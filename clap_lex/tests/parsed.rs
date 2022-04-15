#[test]
#[should_panic] // Our design philosophy is to match X if it can be completed as X which we break here
fn to_long_stdio() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok(""));
    assert_eq!(value, None);
}

#[test]
fn to_long_escape() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok(""));
    assert_eq!(value, None);
}

#[test]
fn to_long_no_value() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--long"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, None);
}

#[test]
fn to_long_with_empty_value() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--long="]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, Some(clap_lex::RawOsStr::from_str("")));
}

#[test]
fn to_long_with_value() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--long=hello"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, Some(clap_lex::RawOsStr::from_str("hello")));
}

#[test]
fn to_short_stdio() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_short());

    let mut shorts = next.to_short().unwrap();
    assert_eq!(shorts.next_value_os(), None);
}

#[test]
fn to_short_escape() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short_long() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--long"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "-short"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_short());

    let shorts = next.to_short().unwrap();
    let actual: String = shorts.map(|s| s.unwrap()).collect();
    assert_eq!(actual, "short");
}

#[test]
fn is_negative_number() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "-10.0"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_number());
}

#[test]
fn is_positive_number() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "10.0"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_number());
}

#[test]
fn is_not_number() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--10.0"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_number());
}

#[test]
fn is_stdio() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_stdio());
}

#[test]
fn is_not_stdio() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_stdio());
}

#[test]
fn is_escape() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_escape());
}

#[test]
fn is_not_escape() {
    let raw = clap_lex::RawArgs::from_iter(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_escape());
}
