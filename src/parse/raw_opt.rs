use std::ffi::OsStr;

use parse::RawValue;

pub(crate) struct RawOpt<'a> {
    pub(crate) raw_key: &'a OsStr,
    pub(crate) key: Key,
    pub(crate) value: Option<RawValue<'a>>,
}