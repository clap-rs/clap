use std::ffi::OsStr;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use std::os::unix::ffi::OsStrExt;
#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
use crate::INVALID_UTF8;

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
pub trait OsStrExt3 {
    fn from_bytes(b: &[u8]) -> &Self;
    fn as_bytes(&self) -> &[u8];
}

#[doc(hidden)]
pub trait OsStrExt2 {
    fn starts_with(&self, s: &[u8]) -> bool;
    fn split_at_byte(&self, b: u8) -> (&OsStr, &OsStr);
    fn split_at(&self, i: usize) -> (&OsStr, &OsStr);
    fn trim_left_matches(&self, b: u8) -> &OsStr;
    fn contains_byte(&self, b: u8) -> bool;
    fn split(&self, b: u8) -> OsSplit;
}

#[cfg(target_os = "windows")]
impl OsStrExt3 for OsStr {
    fn from_bytes(b: &[u8]) -> &Self {
        use std::mem;
        unsafe { mem::transmute(b) }
    }
    fn as_bytes(&self) -> &[u8] { self.to_str().map(|s| s.as_bytes()).expect(INVALID_UTF8) }
}

impl OsStrExt2 for OsStr {
    fn starts_with(&self, s: &[u8]) -> bool { self.as_bytes().starts_with(s) }

    fn contains_byte(&self, byte: u8) -> bool {
        for b in self.as_bytes() {
            if b == &byte {
                return true;
            }
        }
        false
    }

    fn split_at_byte(&self, byte: u8) -> (&OsStr, &OsStr) {
        for (i, b) in self.as_bytes().iter().enumerate() {
            if b == &byte {
                return (
                    OsStr::from_bytes(&self.as_bytes()[..i]),
                    OsStr::from_bytes(&self.as_bytes()[i + 1..]),
                );
            }
        }
        (
            &*self,
            OsStr::from_bytes(&self.as_bytes()[self.len()..self.len()]),
        )
    }

    fn trim_left_matches(&self, byte: u8) -> &OsStr {
        let mut found = false;
        for (i, b) in self.as_bytes().iter().enumerate() {
            if b != &byte {
                return OsStr::from_bytes(&self.as_bytes()[i..]);
            } else {
                found = true;
            }
        }
        if found {
            return OsStr::from_bytes(&self.as_bytes()[self.len()..]);
        }
        &*self
    }

    fn split_at(&self, i: usize) -> (&OsStr, &OsStr) {
        (
            OsStr::from_bytes(&self.as_bytes()[..i]),
            OsStr::from_bytes(&self.as_bytes()[i..]),
        )
    }

    fn split(&self, b: u8) -> OsSplit {
        OsSplit {
            sep: b,
            val: self.as_bytes(),
            pos: 0,
        }
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct OsSplit<'a> {
    sep: u8,
    val: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for OsSplit<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<&'a OsStr> {
        debugln!("OsSplit::next: self={:?}", self);
        if self.pos == self.val.len() {
            return None;
        }
        let start = self.pos;
        for b in &self.val[start..] {
            self.pos += 1;
            if *b == self.sep {
                return Some(OsStr::from_bytes(&self.val[start..self.pos - 1]));
            }
        }
        Some(OsStr::from_bytes(&self.val[start..]))
    }
}
