// REVIEW_NEEDED

use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
};

use super::OsStrExt3;

pub(crate) trait _CowStrExt {
    fn into_os_str(self) -> Cow<'static, OsStr>;
}

impl _CowStrExt for Cow<'static, str> {
    fn into_os_str(self) -> Cow<'static, OsStr> {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(OsStr::from_bytes(s.as_bytes())),
            // TODO: this always copies the data, while it shouldn't
            Cow::Owned(s) => Cow::Owned(OsString::from(s)),
        }
    }
}
