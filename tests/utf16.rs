//! These Windows-only tests are ported from the non-Windows tests in
//! tests/utf8.rs. Two categories of tests are omitted, however:
//!
//! 1. We don't include the tests that use StrictUtf8 mode, since that's
//!    eplicitly Unix-only.
//! 2. We don't include tests that require splitting invalid Unicode strings.
//!
//! Elaborating on that second point: We have fixed invalid Unicode handling in
//! the OsStr::starts_with() method. That means that examples like these can
//! should parse successfully ("X" represents invalid Unicode):
//!
//!     clap.exe X
//!     clap.exe --some-arg X
//!
//! However, other string handling methods like OsStr::split_at_byte() and
//! OsStr::trim_left_matches() still panic in the face of invalid Unicode. That
//! means examples like these *don't* work:
//!
//!     clap.exe --some-arg=X
//!     clap.exe -sX
//!
//! Thus this file omits tests for those cases. All of this OsStr handling is
//! being rewritten for the 3.0 release in any case, so this limitation is
//! specific to the 2.x series.

#![cfg(windows)]

extern crate clap;

use clap::{App, Arg};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

fn bad_utf16_string() -> OsString {
    // UTF-16 surrogate characters are only valid in pairs. Including one on
    // the end by itself makes this invalid UTF-16.
    let surrogate_char: u16 = 0xDC00;
    let os = OsString::from_wide(&[surrogate_char]);
    assert!(os.to_str().is_none(), "should be invalid Unicode");
    os
}

#[test]
fn invalid_utf16_lossy_positional() {
    let r = App::new("bad_utf16")
        .arg(Arg::from_usage("<arg> 'some arg'"))
        .get_matches_from_safe(vec![OsString::from(""), bad_utf16_string()]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf16_lossy_option_short_space() {
    let r = App::new("bad_utf16")
        .arg(Arg::from_usage("-a, --arg <arg> 'some arg'"))
        .get_matches_from_safe(vec![
            OsString::from(""),
            OsString::from("-a"),
            bad_utf16_string(),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf16_lossy_option_long_space() {
    let r = App::new("bad_utf16")
        .arg(Arg::from_usage("-a, --arg <arg> 'some arg'"))
        .get_matches_from_safe(vec![
            OsString::from(""),
            OsString::from("--arg"),
            bad_utf16_string(),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf16_positional() {
    let r = App::new("bad_utf16")
        .arg(Arg::from_usage("<arg> 'some arg'"))
        .get_matches_from_safe(vec![OsString::from(""), bad_utf16_string()]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_os("arg").unwrap(), &*bad_utf16_string());
}

#[test]
fn invalid_utf16_option_short_space() {
    let r = App::new("bad_utf16")
        .arg(Arg::from_usage("-a, --arg <arg> 'some arg'"))
        .get_matches_from_safe(vec![
            OsString::from(""),
            OsString::from("-a"),
            bad_utf16_string(),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_os("arg").unwrap(), &*bad_utf16_string());
}

#[test]
fn invalid_utf16_option_long_space() {
    let r = App::new("bad_utf16")
        .arg(Arg::from_usage("-a, --arg <arg> 'some arg'"))
        .get_matches_from_safe(vec![
            OsString::from(""),
            OsString::from("--arg"),
            bad_utf16_string(),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_os("arg").unwrap(), &*bad_utf16_string());
}
