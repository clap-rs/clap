use std::ffi::OsStr;

use parse::RawValue;
use parse::KeyType;

pub struct RawOpt<'a> {
    pub(crate) raw_key: &'a OsStr,
    pub(crate) key: KeyType,
    pub(crate) value: Option<RawValue<'a>>,
}