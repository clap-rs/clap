#[test]
fn insert() {
    let mut raw = clap_lex::RawArgs::new(["bin", "a", "b", "c"]);
    let mut cursor = raw.cursor();

    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("a")));
    raw.insert(&cursor, ["1", "2", "3"]);
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("1")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("2")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("3")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("b")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("c")));

    let mut cursor = raw.cursor();
    let rest = raw
        .remaining(&mut cursor)
        .map(|s| s.to_string_lossy())
        .collect::<Vec<_>>();
    assert_eq!(rest, vec!["bin", "a", "1", "2", "3", "b", "c"]);
}

#[test]
fn zero_copy_parsing() {
    use clap_lex::RawArgs;
    use std::ffi::OsStr;
    #[derive(Debug, PartialEq)]
    struct Args<'s> {
        bin_name: &'s OsStr,
        verbose_flag: bool,
        remainder: Vec<&'s OsStr>,
    }
    fn parse(raw: &RawArgs) -> Result<Args<'_>, &'static str> {
        let mut cursor = raw.cursor();
        let Some(bin_arg) = raw.next(&mut cursor) else {
            return Err("missing bin name");
        };
        let bin_name = bin_arg.to_value_os();
        let Some(first_arg) = raw.next(&mut cursor) else {
            return Ok(Args {
                bin_name,
                verbose_flag: false,
                remainder: Vec::new(),
            });
        };
        let verbose_flag = if let Some(flag) = first_arg.to_long() {
            match flag {
                (Ok("verbose"), None) => true,
                _ => return Err("unexpected flag"),
            }
        } else {
            false
        };
        let mut remainder = Vec::new();
        if !verbose_flag {
            remainder.push(first_arg.to_value_os());
        }
        remainder.extend(raw.remaining(&mut cursor));
        Ok(Args {
            bin_name,
            verbose_flag,
            remainder,
        })
    }

    let raw1 = RawArgs::new(["bin", "--verbose", "a", "b", "c"]);
    let parsed1 = parse(&raw1).unwrap();
    assert_eq!(
        parsed1,
        Args {
            bin_name: OsStr::new("bin"),
            verbose_flag: true,
            remainder: vec![OsStr::new("a"), OsStr::new("b"), OsStr::new("c")]
        }
    );

    let raw2 = RawArgs::new(["bin", "a", "b", "c"]);
    let parsed2 = parse(&raw2).unwrap();
    assert_eq!(
        parsed2,
        Args {
            bin_name: OsStr::new("bin"),
            verbose_flag: false,
            remainder: vec![OsStr::new("a"), OsStr::new("b"), OsStr::new("c")]
        }
    );
}
