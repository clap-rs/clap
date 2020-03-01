use std::cmp::Ordering;

// Third Party
#[cfg(feature = "suggestions")]
use strsim;

// Internal
use crate::build::App;
use crate::output::fmt::Format;

/// Produces multiple strings from a given list of possible values which are similar
/// to the passed in value `v` within a certain confidence by least confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
#[cfg(feature = "suggestions")]
pub fn did_you_mean<T, I>(v: &str, possible_values: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut candidates: Vec<(f64, String)> = possible_values
        .into_iter()
        .map(|pv| (strsim::jaro_winkler(v, pv.as_ref()), pv.as_ref().to_owned()))
        .filter(|(confidence, _pv)| *confidence > 0.8)
        .collect();
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    candidates.into_iter().map(|(_confidence, pv)| pv).collect()
}

#[cfg(not(feature = "suggestions"))]
pub fn did_you_mean<T, I>(_: &str, _: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    Vec::new()
}

/// Returns a suffix that can be empty, or is the standard 'did you mean' phrase
pub fn did_you_mean_flag_suffix<I, T>(
    arg: &str,
    longs: I,
    subcommands: &mut [App],
) -> (String, Option<String>)
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    match did_you_mean(arg, longs).pop() {
        Some(ref candidate) => {
            let suffix = format!(
                "\n\tDid you mean {}{}?",
                Format::Good("--"),
                Format::Good(candidate)
            );
            return (suffix, Some(candidate.to_owned()));
        }
        None => {
            for subcommand in subcommands {
                subcommand._build();
                if let Some(ref candidate) = did_you_mean(
                    arg,
                    longs!(subcommand).map(|x| x.to_string_lossy().into_owned()),
                )
                .pop()
                {
                    let suffix = format!(
                        "\n\tDid you mean to put '{}{}' after the subcommand '{}'?",
                        Format::Good("--"),
                        Format::Good(candidate),
                        Format::Good(subcommand.get_name())
                    );
                    return (suffix, Some(candidate.clone()));
                }
            }
        }
    }
    (String::new(), None)
}

/// Returns a suffix that can be empty, or is the standard 'did you mean' phrase
pub fn did_you_mean_value_suffix<T, I>(arg: &str, values: I) -> (String, Option<String>)
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    match did_you_mean(arg, values).pop() {
        Some(ref candidate) => {
            let suffix = format!("\n\tDid you mean '{}'?", Format::Good(candidate));
            (suffix, Some(candidate.to_owned()))
        }
        None => (String::new(), None),
    }
}

#[cfg(all(test, features = "suggestions"))]
mod test {
    use super::*;

    #[test]
    fn possible_values_match() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(did_you_mean("tst", p_vals.iter()), Some("test"));
    }

    #[test]
    fn possible_values_match() {
        let p_vals = ["test", "temp"];
        assert_eq!(did_you_mean("te", p_vals.iter()), Some("test"));
    }

    #[test]
    fn possible_values_nomatch() {
        let p_vals = ["test", "possible", "values"];
        assert!(did_you_mean("hahaahahah", p_vals.iter()).is_none());
    }

    #[test]
    fn suffix_long() {
        let p_vals = ["test", "possible", "values"];
        let suffix = "\n\tDid you mean \'--test\'?";
        assert_eq!(
            did_you_mean_flag_suffix("tst", p_vals.iter(), []),
            (suffix, Some("test"))
        );
    }

    #[test]
    fn suffix_enum() {
        let p_vals = ["test", "possible", "values"];
        let suffix = "\n\tDid you mean \'test\'?";
        assert_eq!(
            did_you_mean_value_suffix("tst", p_vals.iter()),
            (suffix, Some("test"))
        );
    }
}
