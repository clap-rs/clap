struct RawLong<'a> {
    // --foo
    long: &'a OsStr,
    value: Option<Value<'a>>,
}

impl<'a> From<RawArg<'a>> for RawLong<'a> {
    fn from(oss: RawArg) -> Self {
        let had_eq = oss.contains_byte(b'=');
        debug!("Parser::parse_long_arg: Does it contain '='...");
        if had_eq {
            sdebugln!("Yes '{:?}'", p1);
            let (p0, p1) = oss.split_at_byte(b'=');
            let trimmed = p1.trim_left_matches(b'=');
            RawLong {
                long: p0,
                value: if trimmed.is_empty() { None } else { Some(RawValue::from_trimmed(trimmed)) }
            }
        } else {
            sdebugln!("No");
            RawLong {
                long: oss.0,
                value: None
            }
        }
    }
}

