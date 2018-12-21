use std::ffi::OsStr;

use parse::RawLong;
use util::{OsStrExt2, OsSplit};

pub struct RawValue<'a> {
    // value1,value2
    pub(crate) raw: &'a OsStr,
    // if --foo=value1,value2 was used
    pub(crate) had_eq: bool,
    // Some(,)
    pub(crate) sep: Option<char>,
    // [6]
    val_idxs: Option<Vec<usize>>,
}

impl<'a> RawValue<'a> {
    // When raw does not contain '='
    pub fn new(oss: &'a OsStr) -> Self {
        RawValue {
            raw: oss,
            had_eq: false,
            sep: None,
            val_idxs: None,
        }
    }
    pub fn from_maybe_empty_osstr(oss: &'a OsStr) -> Option<RawValue<'a>> {
        if oss.is_empty() { return None; }
        Some(oss.into())
    }

    // When raw was =foo but you already trimmed `=`. Implicitly sets had_eq to true.
    pub fn from_trimmed(oss: &'a OsStr) -> Self {
        RawValue {
            raw: oss,
            had_eq: true,
            sep: None,
            val_idxs: None,
        }
    }

    pub fn values(&self) -> OsSplit {
        self.raw.split(self.sep.map_or(0, |c| c as u8))
    }

    pub fn used_sep(&self) -> bool {
        // Defaults to byte value 0 (NULL)...so if the sep *actually* is NULL for delimiter this
        // would be a bug...but I can't imagine anyone using NULL as a valid value delimiter
        self.raw.contains_byte(self.sep.map_or(0, |c| c as u8))
    }
}

impl<'a> From<&'a OsStr> for RawValue<'a> {
    fn from(oss: &'a OsStr) -> Self {
        if oss.contains_byte(b'=') {
            RawValue::from_trimmed(oss.trim_left_matches(b'='))
        } else {
            RawValue::new(oss)
        }
    }
}

