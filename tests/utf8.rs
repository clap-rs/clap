#![cfg(not(windows))]

extern crate clap;

use clap::{App, AppSettings, Arg, ErrorKind};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

#[test]
fn invalid_utf8_strict_positional() {
    let m = App::new("bad_utf8")
        .arg(Arg::from("<arg> 'some arg'"))
        .setting(AppSettings::StrictUtf8)
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_space() {
    let m = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .setting(AppSettings::StrictUtf8)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_equals() {
    let m = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .setting(AppSettings::StrictUtf8)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_no_space() {
    let m = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .setting(AppSettings::StrictUtf8)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_long_space() {
    let m = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .setting(AppSettings::StrictUtf8)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_long_equals() {
    let m = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .setting(AppSettings::StrictUtf8)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_lossy_positional() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("<arg> 'some arg'"))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_short_space() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_short_equals() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_short_no_space() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_long_space() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_long_equals() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_positional() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("<arg> 'some arg'"))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_space() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_equals() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_no_space() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_long_space() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_long_equals() {
    let r = App::new("bad_utf8")
        .arg(Arg::from("-a, --arg <arg> 'some arg'"))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}
