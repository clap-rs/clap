#![cfg(not(windows))]

use clap::{arg, error::ErrorKind, value_parser, Arg, Command};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

#[test]
fn invalid_utf8_strict_positional() {
    let m = Command::new("bad_utf8")
        .arg(Arg::new("arg"))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_space() {
    let m = Command::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_equals() {
    let m = Command::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_no_space() {
    let m = Command::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_long_space() {
    let m = Command::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_long_equals() {
    let m = Command::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_positional() {
    let r = Command::new("bad_utf8")
        .arg(Arg::new("arg").value_parser(value_parser!(OsString)))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        &*m.get_one::<OsString>("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_space() {
    let r = Command::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .value_parser(value_parser!(OsString)),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        &*m.get_one::<OsString>("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_equals() {
    let r = Command::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .value_parser(value_parser!(OsString)),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        &*m.get_one::<OsString>("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_no_space() {
    let r = Command::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .value_parser(value_parser!(OsString)),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        &*m.get_one::<OsString>("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_long_space() {
    let r = Command::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .value_parser(value_parser!(OsString)),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        &*m.get_one::<OsString>("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_long_equals() {
    let r = Command::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .value_parser(value_parser!(OsString)),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        &*m.get_one::<OsString>("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn refuse_invalid_utf8_subcommand_with_allow_external_subcommands() {
    let m = Command::new("bad_utf8")
        .allow_external_subcommands(true)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0xe9]),
            OsString::from("normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn refuse_invalid_utf8_subcommand_when_args_are_allowed_with_allow_external_subcommands() {
    let m = Command::new("bad_utf8")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0xe9]),
            OsString::from("normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn refuse_invalid_utf8_subcommand_args_with_allow_external_subcommands() {
    let m = Command::new("bad_utf8")
        .allow_external_subcommands(true)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("subcommand"),
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidUtf8);
}

#[test]
fn allow_invalid_utf8_subcommand_args_with_allow_external_subcommands() {
    let m = Command::new("bad_utf8")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("subcommand"),
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    let (subcommand, args) = m.subcommand().unwrap();
    let args = args
        .get_many::<OsString>("")
        .unwrap()
        .cloned()
        .collect::<Vec<_>>();
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

#[test]
fn allow_validated_utf8_value_of() {
    #![allow(deprecated)]
    let a = Command::new("test").arg(arg!(--name <NAME>));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.get_one::<String>("name").map(|v| v.as_str());
}

#[test]
#[should_panic = "Must use `Arg::allow_invalid_utf8` with `_os` lookups at `name`"]
fn panic_validated_utf8_value_of_os() {
    #![allow(deprecated)]
    let a = Command::new("test").arg(arg!(--name <NAME>));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.value_of_os("name");
}

#[test]
#[should_panic = "Must use `Arg::allow_invalid_utf8` with `_os` lookups at `value`"]
fn panic_validated_utf8_with_defaults() {
    #![allow(deprecated)]
    let a = Command::new("test").arg(arg!(--value <VALUE>).required(false).default_value("foo"));
    let m = a.try_get_matches_from(["test"]).unwrap();
    let _ = m.get_one::<String>("value").map(|v| v.as_str());
    let _ = m.value_of_os("value");
}

#[test]
fn allow_invalid_utf8_value_of_os() {
    #![allow(deprecated)]
    let a = Command::new("test").arg(arg!(--name <NAME>).allow_invalid_utf8(true));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.value_of_os("name");
}

#[test]
#[should_panic = "Mismatch between definition and access of `name`. Could not downcast to alloc::string::String, need to downcast to std::ffi::os_str::OsString"]
fn panic_invalid_utf8_value_of() {
    #![allow(deprecated)]
    let a = Command::new("test").arg(arg!(--name <NAME>).allow_invalid_utf8(true));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.get_one::<String>("name").map(|v| v.as_str());
}

#[test]
#[should_panic = "Mismatch between definition and access of `value`. Could not downcast to alloc::string::String, need to downcast to std::ffi::os_str::OsString"]
fn panic_invalid_utf8_with_defaults() {
    #![allow(deprecated)]
    let a = Command::new("test").arg(
        arg!(--value <VALUE>)
            .required(false)
            .default_value("foo")
            .allow_invalid_utf8(true),
    );
    let m = a.try_get_matches_from(["test"]).unwrap();
    let _ = m.value_of_os("value");
    let _ = m.get_one::<String>("value").map(|v| v.as_str());
}

#[test]
fn allow_validated_utf8_external_subcommand_values_of() {
    let a = Command::new("test").allow_external_subcommands(true);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    args.get_many::<String>("").unwrap_or_default().count();
}

#[test]
#[should_panic = "Mismatch between definition and access of ``. Could not downcast to std::ffi::os_str::OsString, need to downcast to alloc::string::String"]
fn panic_validated_utf8_external_subcommand_values_of_os() {
    let a = Command::new("test").allow_external_subcommands(true);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    args.get_many::<OsString>("").unwrap_or_default().count();
}

#[test]
fn allow_invalid_utf8_external_subcommand_values_of_os() {
    let a = Command::new("test")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    args.get_many::<OsString>("").unwrap_or_default().count();
}

#[test]
#[should_panic = "Mismatch between definition and access of ``. Could not downcast to alloc::string::String, need to downcast to std::ffi::os_str::OsString"]
fn panic_invalid_utf8_external_subcommand_values_of() {
    let a = Command::new("test")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    args.get_many::<String>("").unwrap_or_default().count();
}
