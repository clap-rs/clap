//! These Windows-only tests are ported from the Unix-only tests in
//! tests/utf16.rs. The tests that use StrictUtf8 mode are omitted here,
//! because that's a Unix-only feature.

#![cfg(windows)]

use clap::{arg, value_parser, Command};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

// Take a slice of ASCII bytes, convert them to UTF-16, and then append a
// dangling surrogate character to make the result invalid UTF-16.
fn bad_osstring(ascii: &[u8]) -> OsString {
    let mut wide_chars: Vec<u16> = ascii.iter().map(|&c| c as u16).collect();
    // UTF-16 surrogate characters are only valid in pairs.
    let surrogate_char: u16 = 0xDC00;
    wide_chars.push(surrogate_char);
    let os = OsString::from_wide(&wide_chars);
    assert!(os.to_str().is_none(), "invalid Unicode");
    os
}

#[test]
fn invalid_utf16_positional() {
    let r = Command::new("bad_utf16")
        .arg(arg!(<arg> "some arg").value_parser(value_parser!(OsString)))
        .try_get_matches_from(vec![OsString::from(""), bad_osstring(b"")]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(&*m.get_one::<OsString>("arg").unwrap(), &*bad_osstring(b""));
}

#[test]
fn invalid_utf16_option_short_space() {
    let r = Command::new("bad_utf16")
        .arg(arg!(-a --arg <arg> "some arg").value_parser(value_parser!(OsString)))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            bad_osstring(b""),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(&*m.get_one::<OsString>("arg").unwrap(), &*bad_osstring(b""));
}

#[test]
fn invalid_utf16_option_short_equals() {
    let r = Command::new("bad_utf16")
        .arg(arg!(-a --arg <arg> "some arg").value_parser(value_parser!(OsString)))
        .try_get_matches_from(vec![OsString::from(""), bad_osstring(b"-a=")]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(&*m.get_one::<OsString>("arg").unwrap(), &*bad_osstring(b""));
}

#[test]
fn invalid_utf16_option_short_no_space() {
    let r = Command::new("bad_utf16")
        .arg(arg!(-a --arg <arg> "some arg").value_parser(value_parser!(OsString)))
        .try_get_matches_from(vec![OsString::from(""), bad_osstring(b"-a")]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(&*m.get_one::<OsString>("arg").unwrap(), &*bad_osstring(b""));
}

#[test]
fn invalid_utf16_option_long_space() {
    let r = Command::new("bad_utf16")
        .arg(arg!(-a --arg <arg> "some arg").value_parser(value_parser!(OsString)))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            bad_osstring(b""),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(&*m.get_one::<OsString>("arg").unwrap(), &*bad_osstring(b""));
}

#[test]
fn invalid_utf16_option_long_equals() {
    let r = Command::new("bad_utf16")
        .arg(arg!(-a --arg <arg> "some arg").value_parser(value_parser!(OsString)))
        .try_get_matches_from(vec![OsString::from(""), bad_osstring(b"--arg=")]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(&*m.get_one::<OsString>("arg").unwrap(), &*bad_osstring(b""));
}
