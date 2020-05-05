#[cfg(feature = "suggestions")]
use std::cmp::Ordering;

// Internal
use crate::build::App;

/// Produces multiple strings from a given list of possible values which are similar
/// to the passed in value `v` within a certain confidence by least confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
#[cfg(feature = "suggestions")]
pub(crate) fn did_you_mean<T, I>(v: &str, possible_values: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut candidates: Vec<(f64, String)> = possible_values
        .into_iter()
        .map(|pv| (strsim::jaro_winkler(v, pv.as_ref()), pv.as_ref().to_owned()))
        .filter(|(confidence, _)| *confidence > 0.8)
        .collect();
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    candidates.into_iter().map(|(_, pv)| pv).collect()
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
pub(crate) fn did_you_mean_flag<I, T>(
    arg: &str,
    longs: I,
    subcommands: &mut [App],
) -> Option<(String, Option<String>)>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    match did_you_mean(arg, longs).pop() {
        Some(ref candidate) => {
            return Some((candidate.to_owned(), None));
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
                    return Some((candidate.to_owned(), Some(subcommand.get_name().to_owned())));
                }
            }
        }
    }

    None
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
    fn flag() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(
            did_you_mean_flag("tst", p_vals.iter(), []),
            Some(("test", None))
        );
    }
}
