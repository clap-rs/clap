use std::ffi::OsStr;

use parse::RawValue;
use parse::KeyType;

pub struct RawOpt<'a> {
    raw_key: &'a OsStr,
    key: KeyType,
    value: Option<RawValue<'a>>,
}

impl<'a> RawOpt<'a> {
    pub(crate) fn had_eq(&self) -> bool {
        self.value.map_or(false, |v| v.had_eq)
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn value_unchecked(&'a self) -> RawValue<'a> {
        self.value.unwrap()
    }
}