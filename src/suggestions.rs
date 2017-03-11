// Third Party
#[cfg(feature = "suggestions")]
use strsim;

// Internal
use fmt::Format;

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
#[cfg(feature = "suggestions")]
#[cfg_attr(feature = "lints", allow(needless_lifetimes))]
pub fn did_you_mean<'a, T: ?Sized, I>(v: &str, possible_values: I) -> Option<&'a str>
    where T: AsRef<str> + 'a,
          I: IntoIterator<Item = &'a T>
{

    let mut candidate: Option<(f64, &str)> = None;
    for pv in possible_values {
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
pub fn did_you_mean<'a, T: ?Sized, I>(_: &str, _: I) -> Option<&'a str>
    where T: AsRef<str> + 'a,
          I: IntoIterator<Item = &'a T>
{
    None
}

/// Returns a suffix that can be empty, or is the standard 'did you mean' phrase
#[cfg_attr(feature = "lints", allow(needless_lifetimes))]
pub fn did_you_mean_suffix<'z, T, I>(arg: &str,
                                     values: I,
                                     style: DidYouMeanMessageStyle)
                                     -> (String, Option<&'z str>)
    where T: AsRef<str> + 'z,
          I: IntoIterator<Item = &'z T>
{
    match did_you_mean(arg, values) {
        Some(candidate) => {
            let mut suffix = "\n\tDid you mean ".to_owned();
            match style {
                DidYouMeanMessageStyle::LongFlag => {
                    suffix.push_str(&Format::Good("--").to_string())
                }
                DidYouMeanMessageStyle::EnumValue => suffix.push('\''),
            }
            suffix.push_str(&Format::Good(candidate).to_string()[..]);
            if let DidYouMeanMessageStyle::EnumValue = style {
                suffix.push('\'');
            }
            suffix.push_str("?");
            (suffix, Some(candidate))
        }
        None => (String::new(), None),
    }
}

/// A helper to determine message formatting
#[derive(Copy, Clone, Debug)]
pub enum DidYouMeanMessageStyle {
    /// Suggested value is a long flag
    LongFlag,
    /// Suggested value is one of various possible values
    EnumValue,
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
        assert_eq!(did_you_mean_suffix("tst", p_vals.iter(), DidYouMeanMessageStyle::LongFlag),
                   (suffix, Some("test")));
    }

    #[test]
    fn suffix_enum() {
        let p_vals = ["test", "possible", "values"];
        let suffix = "\n\tDid you mean \'test\'?";
        assert_eq!(did_you_mean_suffix("tst", p_vals.iter(), DidYouMeanMessageStyle::EnumValue),
                   (suffix, Some("test")));
    }
}
