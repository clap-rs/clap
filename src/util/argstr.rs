use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    str,
};

use os_str_bytes::{OsStrBytes, OsStringBytes};

#[derive(Debug)]
pub(crate) struct ArgStr<'a>(Cow<'a, [u8]>);

impl<'a> ArgStr<'a> {
    pub(crate) fn new(s: &'a OsStr) -> Self {
        Self(s.to_bytes())
    }

    pub(crate) fn starts_with(&self, s: &str) -> bool {
        self.0.starts_with(s.as_bytes())
    }

    fn to_borrowed(&'a self) -> Self {
        Self(Cow::Borrowed(&self.0))
    }

    pub(crate) fn contains_byte(&self, byte: u8) -> bool {
        assert!(byte.is_ascii());

        self.0.contains(&byte)
    }

    pub(crate) fn contains_char(&self, ch: char) -> bool {
        let mut bytes = [0; 4];
        let bytes = ch.encode_utf8(&mut bytes).as_bytes();
        for i in 0..self.0.len().saturating_sub(bytes.len() - 1) {
            if self.0[i..].starts_with(bytes) {
                return true;
            }
        }
        false
    }

    pub(crate) fn split_at_byte(&self, byte: u8) -> (ArgStr<'_>, ArgStr<'_>) {
        assert!(byte.is_ascii());

        for (i, b) in self.0.iter().enumerate() {
            if b == &byte {
                return self.split_at_unchecked(i);
            }
        }
        (self.to_borrowed(), Self(Cow::Borrowed(&[])))
    }

    pub(crate) fn trim_start_matches(&'a self, byte: u8) -> ArgStr<'_> {
        assert!(byte.is_ascii());

        let mut found = false;
        for (i, b) in self.0.iter().enumerate() {
            if b != &byte {
                return Self(Cow::Borrowed(&self.0[i..]));
            } else {
                found = true;
            }
        }
        if found {
            return Self(Cow::Borrowed(&[]));
        }
        self.to_borrowed()
    }

    // Like `trim_start_matches`, but trims no more than `n` matches
    #[inline]
    pub(crate) fn trim_start_n_matches(&self, n: usize, ch: u8) -> ArgStr<'_> {
        assert!(ch.is_ascii());

        let i = self
            .0
            .iter()
            .take(n)
            .take_while(|c| **c == ch)
            .enumerate()
            .last()
            .map(|(i, _)| i + 1)
            .unwrap_or(0);

        self.split_at_unchecked(i).1
    }

    pub(crate) fn split_at_unchecked(&'a self, i: usize) -> (ArgStr<'_>, ArgStr<'_>) {
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
        OsString::from_bytes(&self.0).unwrap()
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
