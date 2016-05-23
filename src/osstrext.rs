use std::ffi::OsStr;
#[cfg(not(windows))]
use std::os::unix::ffi::OsStrExt as StdOsStrExt;

#[cfg(windows)]
use INVALID_UTF8;

#[doc(hidden)]
pub trait OsStrExt {
    fn _starts_with(&self, s: &[u8]) -> bool;
    fn _split_at_byte(&self, b: u8) -> (&OsStr, &OsStr);
    fn _split_at(&self, i: usize) -> (&OsStr, &OsStr);
    fn _trim_left_matches(&self, b: u8) -> &OsStr;
    fn _len(&self) -> usize;
    fn _contains_byte(&self, b: u8) -> bool;
    fn _is_empty(&self) -> bool;
    fn _split(&self, b: u8) -> OsSplit;
    #[cfg(windows)]
    fn from_bytes(b: &[u8]) -> &Self;
    #[cfg(windows)]
    fn as_bytes(&self) -> &[u8];
}

impl OsStrExt for OsStr {
    fn _starts_with(&self, s: &[u8]) -> bool {
        Self::as_bytes(self).starts_with(s)
    }

    fn _is_empty(&self) -> bool {
        Self::as_bytes(self).is_empty()
    }

    fn _contains_byte(&self, byte: u8) -> bool {
        for b in self.as_bytes() {
            if b == &byte {
                return true;
            }
        }
        false
    }

    fn _split_at_byte(&self, byte: u8) -> (&OsStr, &OsStr) {
        for (i, b) in self.as_bytes().iter().enumerate() {
            if b == &byte {
                return (&OsStr::from_bytes(&self.as_bytes()[..i]),
                        &OsStr::from_bytes(&self.as_bytes()[i + 1..]));
            }
        }
        (&*self, &OsStr::from_bytes(&self.as_bytes()[self._len()..self._len()]))
    }

    fn _trim_left_matches(&self, byte: u8) -> &OsStr {
        for (i, b) in self.as_bytes().iter().enumerate() {
            if b != &byte {
                return &OsStr::from_bytes(&self.as_bytes()[i..]);
            }
        }
        &*self
    }

    fn _split_at(&self, i: usize) -> (&OsStr, &OsStr) {
        (&OsStr::from_bytes(&self.as_bytes()[..i]), &OsStr::from_bytes(&self.as_bytes()[i..]))
    }

    fn _len(&self) -> usize {
        self.as_bytes().len()
    }

    fn _split(&self, b: u8) -> OsSplit {
        OsSplit {
            sep: b,
            val: self.as_bytes(),
            pos: 0,
        }
    }
    #[cfg(windows)]
    fn from_bytes(b: &[u8]) -> &Self {
        use std::mem;
        unsafe { mem::transmute(b) }
    }
    #[cfg(windows)]
    fn as_bytes(&self) -> &[u8] {
        self.to_str().map(|s| s.as_bytes()).expect(INVALID_UTF8)
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
        debugln!("fn=OsSplit::next;");
        debugln!("OsSplit: {:?}", self);
        if self.pos == self.val.len() {
            return None;
        }
        let start = self.pos;
        for b in &self.val[start..] {
            self.pos += 1;
            if *b == self.sep {
                return Some(&OsStr::from_bytes(&self.val[start..self.pos - 1]));
            }
        }
        Some(&OsStr::from_bytes(&self.val[start..]))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut count = 0;
        for b in &self.val[self.pos..] {
            if *b == self.sep {
                count += 1;
            }
        }
        if count > 0 {
            return (count, Some(count));
        }
        (0, None)
    }
}

impl<'a> DoubleEndedIterator for OsSplit<'a> {
    fn next_back(&mut self) -> Option<&'a OsStr> {
        if self.pos == 0 {
            return None;
        }
        let start = self.pos;
        for b in self.val[..self.pos].iter().rev() {
            self.pos -= 1;
            if *b == self.sep {
                return Some(&OsStr::from_bytes(&self.val[self.pos + 1..start]));
            }
        }
        Some(&OsStr::from_bytes(&self.val[..start]))
    }
}
