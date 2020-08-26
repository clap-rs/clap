use ::regex::Regex;
use core::convert::TryFrom;
use core::ops::Deref;
use core::str::FromStr;
use std::borrow::Cow;

/// Contains either a regular expression or a reference to one.
///
/// Essentially a [`Cow`] wrapper with custom convenience traits.
///
/// [`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
#[derive(Debug, Clone)]
pub struct RegexRef<'a>(Cow<'a, Regex>);

impl<'a> Deref for RegexRef<'a> {
    type Target = Regex;

    fn deref(&self) -> &Regex {
        self.0.deref()
    }
}

impl<'a> FromStr for RegexRef<'a> {
    type Err = <Regex as core::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::from_str(s).map(|v| RegexRef(Cow::Owned(v)))
    }
}

impl<'a> TryFrom<&'a str> for RegexRef<'a> {
    type Error = <RegexRef<'a> as FromStr>::Err;
    fn try_from(r: &'a str) -> Result<Self, Self::Error> {
        RegexRef::from_str(r)
    }
}

impl<'a> From<&'a Regex> for RegexRef<'a> {
    fn from(r: &'a Regex) -> Self {
        RegexRef(Cow::Borrowed(r))
    }
}

impl<'a> From<Regex> for RegexRef<'a> {
    fn from(r: Regex) -> Self {
        RegexRef(Cow::Owned(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;
    #[test]
    fn test_try_from_with_valid_string() {
        let t: Result<RegexRef, _> = "^Hello, World$".try_into();
        assert!(t.is_ok())
    }

    #[test]
    fn test_try_from_with_invalid_string() {
        let t: Result<RegexRef, _> = "^Hello, World)$".try_into();
        assert!(t.is_err());
    }

    #[test]
    fn from_str() {
        let t: Result<RegexRef, _> = RegexRef::from_str("^Hello, World");
        assert!(t.is_ok());
    }
}
