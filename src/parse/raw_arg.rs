use std::mem;

struct RawArg<'a>(&'a OsStr);

impl<'a> RawArg<'a> {

}

impl<'a> OsStrExt3 for RawArg<'a> {
    fn from_bytes(b: &[u8]) -> &Self {
        unsafe { mem::transmute(b) }
    }
    fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }
}

impl<'a> OsStrExt2 for RawArg<'a> {
    fn starts_with(&self, s: &[u8]) -> bool { self.0.starts_with(s) }

    fn contains_byte(&self, byte: u8) -> bool { self.0.contains_byte(byte) }

    fn split_at_byte(&self, byte: u8) -> (&OsStr, &OsStr) { self.0.split_at_byte(byte) }

    fn trim_left_matches(&self, byte: u8) -> &OsStr { self.0.trim_left_matches(byte) }

    fn split_at(&self, i: usize) -> (&OsStr, &OsStr) { self.0.split_at(i) }

    fn split(&self, b: u8) -> OsSplit { self.0.split(b) }
}

impl<'a> From<&'a OsStr> for RawArg<'a> {
    fn from(oss: &'a OsStr) -> Self {
        RawArg(oss)
    }
}