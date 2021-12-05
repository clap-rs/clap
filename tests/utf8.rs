#![cfg(not(windows))]

use clap::{App, AppSettings, Arg, ErrorKind};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

#[test]
fn invalid_utf8_strict_positional() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg"))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_space() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
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
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
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
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
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
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
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
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
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
        .arg(Arg::new("arg").allow_invalid_utf8(true))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_short_space() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(Arg::new("arg").allow_invalid_utf8(true))
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
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

#[test]
fn refuse_invalid_utf8_subcommand_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0xe9]),
            OsString::from("normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn refuse_invalid_utf8_subcommand_when_args_are_allowed_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0xe9]),
            OsString::from("normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn refuse_invalid_utf8_subcommand_args_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("subcommand"),
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn allow_invalid_utf8_subcommand_args_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("subcommand"),
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]);
    assert!(m.is_ok());
    let m = m.unwrap();
    let (subcommand, args) = m.subcommand().unwrap();
    let args = args.values_of_os("").unwrap().collect::<Vec<_>>();
    assert_eq!(subcommand, OsString::from("subcommand"));
    assert_eq!(
        args,
        vec![
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]
    );
}
