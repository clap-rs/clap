use std::ffi::OsStr;

pub enum HyphenStyle {
    Single,
    Double,
    DoubleOnly,
    None
}

impl<'a> From<&'a OsStr> for HyphenStyle {
    fn from(oss: &'a OsStr) -> Self {
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
