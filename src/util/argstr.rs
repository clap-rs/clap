use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    fmt::{self, Debug},
    str,
};

use os_str_bytes::{raw, OsStrBytes};

#[derive(PartialEq, Eq)]
pub(crate) struct ArgStr<'a>(Cow<'a, [u8]>);

impl<'a> ArgStr<'a> {
    pub(crate) fn new(s: &'a OsStr) -> Self {
        Self(s.to_raw_bytes())
    }

    pub(crate) fn starts_with(&self, s: &str) -> bool {
        self.0.starts_with(s.as_bytes())
    }

    pub(crate) fn is_prefix_of(&self, s: &str) -> bool {
        raw::starts_with(s, &self.0)
    }

    fn to_borrowed(&'a self) -> Self {
        Self(Cow::Borrowed(&self.0))
    }

    pub(crate) fn contains_byte(&self, byte: u8) -> bool {
        debug_assert!(byte.is_ascii());

        self.0.contains(&byte)
    }

    pub(crate) fn contains_char(&self, ch: char) -> bool {
        self.to_string_lossy().contains(|x| x == ch)
    }

    pub(crate) fn split_at_byte(&self, byte: u8) -> (ArgStr, ArgStr) {
        debug_assert!(byte.is_ascii());

        if let Some(i) = self.0.iter().position(|&x| x == byte) {
            self.split_at_unchecked(i)
        } else {
            (self.to_borrowed(), Self(Cow::Borrowed(&[])))
        }
    }

    pub(crate) fn trim_start_matches(&'a self, byte: u8) -> ArgStr {
        debug_assert!(byte.is_ascii());

        if let Some(i) = self.0.iter().position(|x| x != &byte) {
            Self(Cow::Borrowed(&self.0[i..]))
        } else {
            Self(Cow::Borrowed(&[]))
        }
    }

    // Like `trim_start_matches`, but trims no more than `n` matches
    pub(crate) fn trim_start_n_matches(&'a self, n: usize, byte: u8) -> ArgStr {
        debug_assert!(byte.is_ascii());

        if let Some(i) = self.0.iter().take(n).position(|x| x != &byte) {
            Self(Cow::Borrowed(&self.0[i..]))
        } else {
            match self.0.get(n..) {
                Some(x) => Self(Cow::Borrowed(x)),
                None => Self(Cow::Borrowed(&[])),
            }
        }
    }

    pub(crate) fn split_at_unchecked(&'a self, i: usize) -> (ArgStr, ArgStr) {
        (
            Self(Cow::Borrowed(&self.0[..i])),
            Self(Cow::Borrowed(&self.0[i..])),
        )
    }

    pub(crate) fn split(&self, ch: char) -> ArgSplit<'_> {
        let mut sep = [0; 4];
        ArgSplit {
            sep_len: ch.encode_utf8(&mut sep).as_bytes().len(),
            sep,
            val: &self.0,
            pos: 0,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn as_raw_bytes(&self) -> &[u8] {
        &self.0
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn to_str(&self) -> Option<&str> {
        str::from_utf8(&self.0).ok()
    }

    pub(crate) fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.0)
    }

    pub(crate) fn to_os_string(&self) -> OsString {
        self.to_borrowed().into_os_string()
    }

    pub(crate) fn into_os_string(self) -> OsString {
        OsStr::from_raw_bytes(self.0).unwrap().into_owned()
    }
}

impl<'a> Debug for ArgStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string_lossy())
    }
}

impl<'a> PartialEq<str> for ArgStr<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl<'a> PartialEq<&str> for ArgStr<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl<'a> PartialEq<ArgStr<'a>> for str {
    fn eq(&self, other: &ArgStr<'a>) -> bool {
        other.eq(self)
    }
}

impl<'a> PartialEq<ArgStr<'a>> for &str {
    fn eq(&self, other: &ArgStr<'a>) -> bool {
        other.eq(self)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ArgSplit<'a> {
    sep: [u8; 4],
    sep_len: usize,
    val: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for ArgSplit<'a> {
    type Item = ArgStr<'a>;

    fn next(&mut self) -> Option<ArgStr<'a>> {
        debug!("ArgSplit::next: self={:?}", self);

        if self.pos == self.val.len() {
            return None;
        }
        let start = self.pos;
        while self.pos < self.val.len() {
            if self.val[self.pos..].starts_with(&self.sep[..self.sep_len]) {
                let arg = ArgStr(Cow::Borrowed(&self.val[start..self.pos]));
                self.pos += self.sep_len;
                return Some(arg);
            }
            self.pos += 1;
        }
        Some(ArgStr(Cow::Borrowed(&self.val[start..])))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_trim_start_matches() {
        let raw = OsString::from("hello? world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_matches(b'-');
        assert_eq!(trimmed, a);

        let raw = OsString::from("------------hello? world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_matches(b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("hello? world")));

        let raw = OsString::from("------------hel-lo? -world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_matches(b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("hel-lo? -world")));

        let raw = OsString::from("hel-lo? -world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_matches(b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("hel-lo? -world")));

        let raw = OsString::from("");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_matches(b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("")));
    }

    #[test]
    #[rustfmt::skip]
    fn test_trim_start_n_matches() {
        let raw = OsString::from("hello? world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(2, b'-');
        assert_eq!(trimmed, a);

        let raw = OsString::from("------------hello? world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(2, b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("----------hello? world")));

        let raw = OsString::from("------------hello? world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(1000, b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("hello? world")));

        let raw = OsString::from("------------hel-lo? -world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(2, b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("----------hel-lo? -world")));

        let raw = OsString::from("-hel-lo? -world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(5, b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("hel-lo? -world")));

        let raw = OsString::from("hel-lo? -world");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(10, b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("hel-lo? -world")));

        let raw = OsString::from("");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(10, b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("")));

        let raw = OsString::from("");
        let a = ArgStr::new(&raw);
        let trimmed = a.trim_start_n_matches(0, b'-');
        assert_eq!(trimmed, ArgStr::new(&OsString::from("")));
    }
}
