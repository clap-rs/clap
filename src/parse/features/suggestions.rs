// Third Party
#[cfg(feature = "suggestions")]
use strsim;

// Internal
use crate::build::App;
use crate::output::fmt::Format;

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
#[cfg(feature = "suggestions")]
#[cfg_attr(feature = "lints", allow(needless_lifetimes))]
pub fn did_you_mean<T, I>(v: &str, possible_values: I) -> Option<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut candidate: Option<(f64, String)> = None;
    for pv in possible_values {
        let confidence = strsim::jaro_winkler(v, pv.as_ref());
        if confidence > 0.8 && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
        {
            candidate = Some((confidence, pv.as_ref().to_owned()));
        }
    }

    candidate.map(|(_, candidate)| candidate)
}

#[cfg(not(feature = "suggestions"))]
pub fn did_you_mean<T, I>(_: &str, _: I) -> Option<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    None
}

/// Returns a suffix that can be empty, or is the standard 'did you mean' phrase
#[cfg_attr(feature = "lints", allow(needless_lifetimes))]
pub fn did_you_mean_flag_suffix<I, T>(
    arg: &str,
    longs: I,
    subcommands: &mut [App],
) -> (String, Option<String>)
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    match did_you_mean(arg, longs) {
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
                ) {
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
    match did_you_mean(arg, values) {
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
