#![cfg(not(windows))]

extern crate clap;

use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use clap::{App, Arg, AppSettings, ErrorKind};

#[test]
fn invalid_unicode_safe() {
    let m = App::new("bad_unicode")
        .arg(Arg::from_usage("<arg> 'some arg'"))
        .get_matches_from_safe(vec![OsString::from_vec(vec![0x20]),
                                    OsString::from_vec(vec![0xe9])]);
    assert!(m.is_err());
    if let Err(err) = m {
        assert_eq!(err.kind, ErrorKind::InvalidUtf8);
    }
}

#[test]
fn invalid_unicode_lossy() {
    let m = App::new("bad_unicode")
        .arg(Arg::from_usage("<arg> 'some arg'"))
        .setting(AppSettings::AllowInvalidUtf8)
        .get_matches_from(vec![OsString::from_vec(vec![0x20]),
                                OsString::from_vec(vec![0xe9])]);
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "\u{FFFD}");
}
