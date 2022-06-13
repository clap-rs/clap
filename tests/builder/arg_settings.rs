#![allow(deprecated)]

use clap::{Arg, ArgSettings, Command};
use std::ffi::OsStr;

#[test]
fn setting() {
    let m = Arg::new("setting").setting(ArgSettings::Required);
    assert!(m.is_required_set());
}

#[test]
fn unset_setting() {
    let m = Arg::new("unset_setting").setting(ArgSettings::Required);
    assert!(m.is_required_set());

    let m = m.unset_setting(ArgSettings::Required);
    assert!(!m.is_required_set(), "{:#?}", m);
}

#[test]
fn setting_bitor() {
    let m = Arg::new("setting_bitor")
        .setting(ArgSettings::Required | ArgSettings::Hidden | ArgSettings::Last);

    assert!(m.is_required_set());
    assert!(m.is_hide_set());
    assert!(m.is_last_set());
}

#[test]
fn unset_setting_bitor() {
    let m = Arg::new("unset_setting_bitor")
        .setting(ArgSettings::Required)
        .setting(ArgSettings::Hidden)
        .setting(ArgSettings::Last);

    assert!(m.is_required_set());
    assert!(m.is_hide_set());
    assert!(m.is_last_set());

    let m = m.unset_setting(ArgSettings::Required | ArgSettings::Hidden | ArgSettings::Last);
    assert!(!m.is_required_set(), "{:#?}", m);
    assert!(!m.is_hide_set(), "{:#?}", m);
    assert!(!m.is_last_set(), "{:#?}", m);
}

#[test]
fn default_value_ifs_os() {
    let cmd = Command::new("my_cargo")
        .arg(
            Arg::new("flag")
                .long("flag")
                .allow_invalid_utf8(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("other")
                .long("other")
                .allow_invalid_utf8(true)
                .default_value_ifs_os(&[(
                    "flag",
                    Some("标记2").map(OsStr::new),
                    Some("flag=标记2").map(OsStr::new),
                )]),
        );
    let result = cmd.try_get_matches_from([
        OsStr::new("my_cargo"),
        OsStr::new("--flag"),
        OsStr::new("标记2"),
    ]);
    assert!(result.is_ok());
    match result {
        Ok(arg_matches) => {
            assert_eq!(arg_matches.value_of_os("flag"), Some(OsStr::new("标记2")));
            assert_eq!(
                arg_matches.value_of_os("other"),
                Some(OsStr::new("flag=标记2")),
            );
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    }
}
