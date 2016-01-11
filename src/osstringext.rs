use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

pub trait OsStrExt2 {
    fn starts_with(&self, s: &[u8]) -> bool;
    fn split_at_byte(&self, b: u8) -> (&OsStr, &OsStr);
    fn split_at(&self, i: usize) -> (&OsStr, &OsStr);
    fn trim_left_matches(&self, b: u8) -> &OsStr;
    fn len(&self) -> usize;
    fn contains_byte(&self, b: u8) -> bool;
}

impl OsStrExt2 for OsStr {
    fn starts_with(&self, s: &[u8]) -> bool {
        let mut i = 0;
        let sab = self.as_bytes();
        for b in s {
            if *b != sab[i] { return false; }
            i += 1;
        }
        return true;
    }

    fn contains_byte(&self, byte: u8) -> bool {
        for b in self.as_bytes() {
            if b == &byte { return true; }
        }
        false
    }

    fn split_at_byte(&self, byte: u8) -> (&OsStr, &OsStr) {
        let mut i = 0;
        for b in self.as_bytes() {
            if b == &byte { return (OsStr::from_bytes(&self.as_bytes()[..i]), OsStr::from_bytes(&self.as_bytes()[i..])); }
            i += 1;
        }
        (&*self, OsStr::from_bytes(&self.as_bytes()[self.len()..self.len()]))
    }

    fn trim_left_matches(&self, byte: u8) -> &OsStr {
        let mut i = 0;
        for b in self.as_bytes() {
            if b != &byte { return OsStr::from_bytes(&self.as_bytes()[i..]); }
            i += 1;
        }
        &*self
    }

    fn split_at(&self, i: usize) -> (&OsStr, &OsStr) {
        (OsStr::from_bytes(&self.as_bytes()[..i]), OsStr::from_bytes(&self.as_bytes()[i..]))
    }

    fn len(&self) -> usize {
        self.as_bytes().len()
    }
}
