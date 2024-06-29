#[cfg(feature = "suggestions")]
use std::cmp::Ordering;

// Internal
use crate::builder::Command;

/// Find strings from an iterable of `possible_values` similar to a given value `v`
/// Returns a Vec of all possible values that exceed a similarity threshold
/// sorted by ascending similarity, most similar comes last
#[cfg(feature = "suggestions")]
pub(crate) fn did_you_mean<T, I>(input: &str, possible_values: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut candidates: Vec<(f64, String)> = possible_values
        .into_iter()
        .map(|pv| {
            let confidence = if words_are_swapped(input, pv.as_ref()) {
                0.9
            } else {
                // GH #4660: using `jaro` because `jaro_winkler` implementation in `strsim-rs` is wrong
                // causing strings with common prefix >=10 to be considered perfectly similar
                strsim::jaro(input, pv.as_ref())
            };

            (confidence, pv.as_ref().to_owned())
        })
        // Confidence of 0.7 so that bar -> baz is suggested
        .filter(|(confidence, _)| *confidence > 0.7)
        .collect();
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    candidates.into_iter().map(|(_, pv)| pv).collect()
}

#[cfg(feature = "suggestions")]
fn words_are_swapped(input: &str, candidate: &str) -> bool {
    let input_words = input.split_once('-');
    let candidate_words = candidate.split_once('-');
    match (input_words, candidate_words) {
        (Some((input1, input2)), Some((candidate1, candidate2))) => {
            input1 == candidate2 && input2 == candidate1
        }
        _ => false,
    }
}

#[cfg(not(feature = "suggestions"))]
pub(crate) fn did_you_mean<T, I>(_: &str, _: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    Vec::new()
}

/// Returns a suffix that can be empty, or is the standard 'did you mean' phrase
pub(crate) fn did_you_mean_flag<'a, 'help, I, T>(
    arg: &str,
    remaining_args: &[&std::ffi::OsStr],
    longs: I,
    subcommands: impl IntoIterator<Item = &'a mut Command>,
) -> Option<(String, Option<String>)>
where
    'help: 'a,
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    use crate::mkeymap::KeyType;

    match did_you_mean(arg, longs).pop() {
        Some(candidate) => Some((candidate, None)),
        None => subcommands
            .into_iter()
            .filter_map(|subcommand| {
                subcommand._build_self(false);

                let longs = subcommand.get_keymap().keys().filter_map(|a| {
                    if let KeyType::Long(v) = a {
                        Some(v.to_string_lossy().into_owned())
                    } else {
                        None
                    }
                });

                let subcommand_name = subcommand.get_name();

                let candidate = some!(did_you_mean(arg, longs).pop());
                let score = some!(remaining_args.iter().position(|x| subcommand_name == *x));
                Some((score, (candidate, Some(subcommand_name.to_string()))))
            })
            .min_by_key(|(x, _)| *x)
            .map(|(_, suggestion)| suggestion),
    }
}

#[cfg(all(test, feature = "suggestions"))]
mod test {
    use super::*;

    #[test]
    fn missing_letter() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(did_you_mean("tst", p_vals.iter()), vec!["test"]);
    }

    #[test]
    fn ambiguous() {
        let p_vals = ["test", "temp", "possible", "values"];
        assert_eq!(did_you_mean("te", p_vals.iter()), vec!["test", "temp"]);
    }

    #[test]
    fn unrelated() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(
            did_you_mean("hahaahahah", p_vals.iter()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn best_fit() {
        let p_vals = [
            "test",
            "possible",
            "values",
            "alignmentStart",
            "alignmentScore",
        ];
        assert_eq!(
            did_you_mean("alignmentScorr", p_vals.iter()),
            vec!["alignmentStart", "alignmentScore"]
        );
    }

    #[test]
    fn best_fit_long_common_prefix_issue_4660() {
        let p_vals = ["alignmentScore", "alignmentStart"];
        assert_eq!(
            did_you_mean("alignmentScorr", p_vals.iter()),
            vec!["alignmentStart", "alignmentScore"]
        );
    }

    #[test]
    fn swapped_words_1() {
        assert_eq!(
            did_you_mean("write-lock", ["lock-write", "no-lock"].iter()),
            vec!["no-lock", "lock-write"]
        );
    }

    #[test]
    fn swapped_words_2() {
        assert_eq!(
            did_you_mean("features-all", ["all-features", "features"].iter()),
            vec!["features", "all-features"]
        );
    }

    #[test]
    fn flag_missing_letter() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(
            did_you_mean_flag("tst", &[], p_vals.iter(), []),
            Some(("test".to_owned(), None))
        );
    }

    #[test]
    fn flag_ambiguous() {
        let p_vals = ["test", "temp", "possible", "values"];
        assert_eq!(
            did_you_mean_flag("te", &[], p_vals.iter(), []),
            Some(("temp".to_owned(), None))
        );
    }

    #[test]
    fn flag_unrelated() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(
            did_you_mean_flag("hahaahahah", &[], p_vals.iter(), []),
            None
        );
    }

    #[test]
    fn flag_best_fit() {
        let p_vals = [
            "test",
            "possible",
            "values",
            "alignmentStart",
            "alignmentScore",
        ];
        assert_eq!(
            did_you_mean_flag("alignmentScorr", &[], p_vals.iter(), []),
            Some(("alignmentScore".to_owned(), None))
        );
    }
}
