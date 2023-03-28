use std::ffi::OsStr;

// Despite our design philosophy being to support completion generation, we aren't considering `-`
// the start of a long because there is no valid value to return.
#[test]
fn to_long_stdio() {
    let raw = clap_lex::RawArgs::new(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_long());

    assert_eq!(next.to_long(), None);
}

#[test]
fn to_long_no_escape() {
    let raw = clap_lex::RawArgs::new(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_long());

    assert_eq!(next.to_long(), None);
}

#[test]
fn to_long_no_value() {
    let raw = clap_lex::RawArgs::new(["bin", "--long"]);
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
    let raw = clap_lex::RawArgs::new(["bin", "--long="]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, Some(OsStr::new("")));
}

#[test]
fn to_long_with_value() {
    let raw = clap_lex::RawArgs::new(["bin", "--long=hello"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_long());

    let (key, value) = next.to_long().unwrap();
    assert_eq!(key, Ok("long"));
    assert_eq!(value, Some(OsStr::new("hello")));
}

#[test]
fn to_short_stdio() {
    let raw = clap_lex::RawArgs::new(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short_escape() {
    let raw = clap_lex::RawArgs::new(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short_long() {
    let raw = clap_lex::RawArgs::new(["bin", "--long"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_short());

    assert!(next.to_short().is_none());
}

#[test]
fn to_short() {
    let raw = clap_lex::RawArgs::new(["bin", "-short"]);
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
    let raw = clap_lex::RawArgs::new(["bin", "-10.0"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_number());
}

#[test]
fn is_positive_number() {
    let raw = clap_lex::RawArgs::new(["bin", "10.0"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_number());
}

#[test]
fn is_not_number() {
    let raw = clap_lex::RawArgs::new(["bin", "--10.0"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_number());
}

#[test]
fn is_stdio() {
    let raw = clap_lex::RawArgs::new(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_stdio());
}

#[test]
fn is_not_stdio() {
    let raw = clap_lex::RawArgs::new(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_stdio());
}

#[test]
fn is_escape() {
    let raw = clap_lex::RawArgs::new(["bin", "--"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(next.is_escape());
}

#[test]
fn is_not_escape() {
    let raw = clap_lex::RawArgs::new(["bin", "-"]);
    let mut cursor = raw.cursor();
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    let next = raw.next(&mut cursor).unwrap();

    assert!(!next.is_escape());
}
