extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, ArgGroup, ErrorKind};

static REQ_GROUP_USAGE: &'static str = "error: The following required arguments were not provided:
    <base|--delete>

USAGE:
    clap-test <base|--delete>

For more information try --help";

static REQ_GROUP_CONFLICT_USAGE: &'static str =
    "error: The argument '<base>' cannot be used with '--delete'

USAGE:
    clap-test <base|--delete>

For more information try --help";

static REQ_GROUP_CONFLICT_REV: &'static str =
    "error: The argument '--delete' cannot be used with '<base>'

USAGE:
    clap-test <base|--delete>

For more information try --help";

#[test]
fn required_group_missing_arg() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg(" -c, --color 'some other flag'")
        .group(
            ArgGroup::with_name("req")
                .args(&["flag", "color"])
                .required(true),
        )
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

// This tests a programmer error and will only succeed with debug_assertions
// #[cfg(debug_assertions)]
#[test]
// This used to provide a nice, programmer-friendly error.
// Now the error directs the programmer to file a bug report with clap.
// #[should_panic(expected = "The group 'req' contains the arg 'flg' that doesn't actually exist.")]
#[should_panic(expected = "internal error")]
fn non_existing_arg() {
    let _ = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some other flag'")
        .group(
            ArgGroup::with_name("req")
                .args(&["flg", "color"])
                .required(true),
        )
        .try_get_matches_from(vec![""]);
}

#[test]
fn group_single_value() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-c", "blue"]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert_eq!(m.value_of("grp").unwrap(), "blue");
}

#[test]
fn group_single_flag() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_empty() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec![""]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(!m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_reqired_flags_empty() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some option'")
        .group(
            ArgGroup::with_name("grp")
                .required(true)
                .args(&["flag", "color"]),
        )
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_multi_value_single_arg() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color]... 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-c", "blue", "red", "green"]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert_eq!(
        &*m.values_of("grp").unwrap().collect::<Vec<_>>(),
        &["blue", "red", "green"]
    );
}

#[test]
fn empty_group() {
    let r = App::new("empty_group")
        .arg(Arg::from("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("vers").required(true))
        .try_get_matches_from(vec!["empty_prog"]);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn req_group_usage_string() {
    let app = App::new("req_group")
        .arg("[base] 'Base commit'")
        .arg("-d, --delete 'Remove the base commit information'")
        .group(
            ArgGroup::with_name("base_or_delete")
                .args(&["base", "delete"])
                .required(true),
        );

    assert!(test::compare_output(
        app,
        "clap-test",
        REQ_GROUP_USAGE,
        true
    ));
}

#[test]
fn req_group_with_conflict_usage_string() {
    let app = App::new("req_group")
        .arg(Arg::from("[base] 'Base commit'").conflicts_with("delete"))
        .arg(Arg::from(
            "-d, --delete 'Remove the base commit information'",
        ))
        .group(
            ArgGroup::with_name("base_or_delete")
                .args(&["base", "delete"])
                .required(true),
        );

    assert!(test::compare_output2(
        app,
        "clap-test --delete base",
        REQ_GROUP_CONFLICT_REV,
        REQ_GROUP_CONFLICT_USAGE,
        true
    ));
}

#[test]
fn required_group_multiple_args() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some other flag'")
        .group(
            ArgGroup::with_name("req")
                .args(&["flag", "color"])
                .required(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_ok());
    let m = result.unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn group_multiple_args_error() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some other flag'")
        .group(ArgGroup::with_name("req").args(&["flag", "color"]))
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_acts_like_arg() {
    let m = App::new("prog")
        .arg(Arg::with_name("debug").long("debug").group("mode"))
        .arg(Arg::with_name("verbose").long("verbose").group("mode"))
        .get_matches_from(vec!["prog", "--debug"]);
    assert!(m.is_present("mode"));
}
