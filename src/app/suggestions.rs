#[cfg(feature = "suggestions")]
use strsim;

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
#[cfg(feature = "suggestions")]
#[cfg_attr(feature = "lints", allow(needless_lifetimes))]
pub fn did_you_mean<'a, T, I>(v: &str,
                              possible_values: I)
                              -> Option<&'a str>
    where T: AsRef<str> + 'a,
          I: IntoIterator<Item = &'a T>
{

    let mut candidate: Option<(f64, &str)> = None;
    for pv in possible_values.into_iter() {
        let confidence = strsim::jaro_winkler(v, pv.as_ref());
        if confidence > 0.8 &&
           (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence)) {
            candidate = Some((confidence, pv.as_ref()));
        }
    }
    match candidate {
        None => None,
        Some((_, candidate)) => Some(candidate),
    }
}

#[cfg(not(feature = "suggestions"))]
pub fn did_you_mean<'a, T, I>(_: &str,
                              _: I)
                              -> Option<&'a str>
    where T: AsRef<str> + 'a,
          I: IntoIterator<Item = &'a T>
{
    None
}

/// A helper to determine message formatting
pub enum DidYouMeanMessageStyle {
    /// Suggested value is a long flag
    LongFlag,
    /// Suggested value is one of various possible values
    EnumValue,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn did_you_mean_possible_values() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(did_you_mean("tst", p_vals.iter()), Some("test"));
        assert!(did_you_mean("hahaahahah", p_vals.iter()).is_none());

    }
}
