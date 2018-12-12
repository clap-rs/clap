struct RawValue<'a> {
    // value1,value2
    raw: &'a OsStr,
    // if --foo=value1,value2 was used
    had_eq: bool,
    // Some(,)
    sep: Option<char>,
    // [6]
    val_idxs: Option<Vec<usize>>,
}

impl<'a> RawValue<'a> {
    fn from_trimmed(oss: &'a OsStr) -> Self {
        Value {
            raw: oss,
            had_eq: oss.contains_byte(b'='),
            sep: None,
            val_idxs: None,
        }
    }
}

impl<'a> From<&'a OsStr> for RawLong<'a> {
    fn from(oss: &'a OsStr) -> Self {
        RawValue::from_trimmed(oss.trim_left_matches(b'='))
    }
}