use std::ffi::OsStr;

use parse::RawArg;

pub enum HyphenStyle {
    Single,
    Double,
    DoubleOnly,
    None
}

impl<'a> From<&'a RawArg> for HyphenStyle {
    fn from(oss: &'a RawArg) -> Self {
        use util::OsStrExt2;
        if oss.starts_with(b"--") {
            if oss.len() == 2 {
                return HyphenStyle::DoubleOnly;
            } else {
                return HyphenStyle::Double;
            }
        } else if oss.starts_with(b"-") && oss.len() != 1 {
            return HyphenStyle::Single;
        }
        HyphenStyle::None
    }
}
