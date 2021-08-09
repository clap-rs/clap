use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    fmt::{self, Debug},
    slice::Windows,
    str,
};

use bstr::{BStr, BString, ByteSlice};
use os_str_bytes::OsStrBytes;

#[derive(PartialEq, Eq)]
pub(crate) struct ArgStr<'a>(Cow<'a, BStr>);

impl<'a> std::ops::Deref for ArgStr<'a> {
    type Target = BStr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ArgStr<'a> {
    pub(crate) fn new(s: &'a OsStr) -> Self {
        Self(match s.to_raw_bytes() {
            Cow::Owned(x) => Cow::Owned(BString::from(x)),
            Cow::Borrowed(x) => Cow::Borrowed(x.into()),
        })
    }

    pub(crate) fn starts_with(&self, s: &str) -> bool {
        self.0.as_ref().starts_with(s.as_bytes())
    }

    pub(crate) fn is_prefix_of(&self, s: &str) -> bool {
        s.as_bytes().starts_with(self.0.as_ref())
    }

    fn to_borrowed(&'a self) -> Self {
        Self(Cow::Borrowed(&self.0))
    }

    pub(crate) fn contains_char(&self, ch: char) -> bool {
        self.0.chars().any(|x| x == ch)
    }

    pub(crate) fn split_at_byte(&self, byte: u8) -> (ArgStr, Option<ArgStr>) {
        debug_assert!(byte.is_ascii());

        if let Some(i) = self.0.iter().position(|&x| x == byte) {
            let (a, b) = self.split_at_unchecked(i);
            (a, Some(b))
        } else {
            (self.to_borrowed(), None)
        }
    }

    pub(crate) fn trim_start_matches(&'a self, byte: u8) -> ArgStr {
        debug_assert!(byte.is_ascii());
        let trimmed = self
            .0
            .iter()
            .position(|x| x != &byte)
            .map(|i| &self.0[i..])
            .unwrap_or_default();
        Self(Cow::Borrowed(trimmed))
    }

    // Like `trim_start_matches`, but trims no more than `n` matches
    pub(crate) fn trim_start_n_matches(&'a self, n: usize, byte: u8) -> ArgStr {
        debug_assert!(byte.is_ascii());
        let i = self.0.iter().take(n).take_while(|&&c| c == byte).count();
        self.split_at_unchecked(i).1
    }

    pub(crate) fn split_at_unchecked(&'a self, i: usize) -> (ArgStr, ArgStr) {
        (
            Self(Cow::Borrowed(&self.0[..i])),
            Self(Cow::Borrowed(&self.0[i..])),
        )
    }

    pub(crate) fn split(&self, ch: char) -> ArgSplit<'_> {
        let mut sep = [0; 4];
        let sep_len = ch.encode_utf8(&mut sep).len();
        ArgSplit {
            sep,
            sep_len,
            val: &self.0,
            pos: 0,
            windows: self.0.windows(sep_len),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn as_raw_bytes(&self) -> &[u8] {
        &self.0
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    #[allow(dead_code)]
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
        OsStr::from_raw_bytes(match self.0 {
            Cow::Owned(x) => Cow::<'_, [u8]>::Owned(x.into()),
            Cow::Borrowed(x) => Cow::<'_, [u8]>::Borrowed(x.as_ref()),
        })
        .unwrap()
        .into_owned()
    }
}

impl<'a> Debug for ArgStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string_lossy())
    }
}

impl<'a> PartialEq<str> for ArgStr<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0.as_ref() == other
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
    val: &'a BStr,
    pos: usize,
    windows: Windows<'a, u8>,
}

impl<'a> Iterator for ArgSplit<'a> {
    type Item = ArgStr<'a>;

    fn next(&mut self) -> Option<ArgStr<'a>> {
        debug!("ArgSplit::next: self={:?}", self);

        let end = self.val.len();
        if self.pos >= end {
            return None;
        }
        let sep = self.sep;
        let new_pos = self
            .windows
            .position(|window| sep.starts_with(window))
            .map(|x| self.pos + x)
            .unwrap_or(end);
        let slice = &self.val[self.pos..new_pos];
        self.pos = new_pos + self.sep_len;
        Some(ArgStr(Cow::Borrowed(slice)))
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
