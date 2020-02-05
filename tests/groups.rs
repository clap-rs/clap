extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, ArgGroup, ErrorKind, SubCommand};

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
    "error: The argument '--delete' cannot be used with 'base'

USAGE:
    clap-test <base|--delete>

For more information try --help";

#[test]
fn required_group_missing_arg() {
    let result = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color 'some other flag'",
        )
        .group(
            ArgGroup::with_name("req")
                .args(&["flag", "color"])
                .required(true),
        )
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
#[should_panic]
fn non_existing_arg() {
    let _ = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color 'some other flag'",
        )
        .group(
            ArgGroup::with_name("req")
                .args(&["flg", "color"])
                .required(true),
        )
        .get_matches_from_safe(vec![""]);
}

#[test]
#[should_panic(expected = "The group 'c' contains the arg 'd' that doesn't actually exist.")]
fn non_existing_arg_in_subcommand_help() {
    let _ = App::new("a")
        .subcommand(
            SubCommand::with_name("b").group(ArgGroup::with_name("c").args(&["d"]).required(true)),
        )
        .get_matches_from_safe(vec!["a", "help", "b"]);
}

#[test]
fn group_single_value() {
    let res = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color [color] 'some option'",
        )
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .get_matches_from_safe(vec!["", "-c", "blue"]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert_eq!(m.value_of("grp").unwrap(), "blue");
}

#[test]
fn group_single_flag() {
    let res = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color [color] 'some option'",
        )
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .get_matches_from_safe(vec!["", "-f"]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_empty() {
    let res = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color [color] 'some option'",
        )
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .get_matches_from_safe(vec![""]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(!m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_reqired_flags_empty() {
    let result = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color 'some option'",
        )
        .group(
            ArgGroup::with_name("grp")
                .required(true)
                .args(&["flag", "color"]),
        )
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_multi_value_single_arg() {
    let res = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color [color]... 'some option'",
        )
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .get_matches_from_safe(vec!["", "-c", "blue", "red", "green"]);
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
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("vers").required(true))
        .get_matches_from_safe(vec!["empty_prog"]);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn req_group_usage_string() {
    let app = App::new("req_group")
        .args_from_usage(
            "[base] 'Base commit'
                          -d, --delete 'Remove the base commit information'",
        )
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
        .arg(Arg::from_usage("[base] 'Base commit'").conflicts_with("delete"))
        .arg(Arg::from_usage(
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
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color 'some other flag'",
        )
        .group(
            ArgGroup::with_name("req")
                .args(&["flag", "color"])
                .required(true)
                .multiple(true),
        )
        .get_matches_from_safe(vec!["group", "-f", "-c"]);
    assert!(result.is_ok());
    let m = result.unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn group_multiple_args_error() {
    let result = App::new("group")
        .args_from_usage(
            "-f, --flag 'some flag'
                          -c, --color 'some other flag'",
        )
        .group(ArgGroup::with_name("req").args(&["flag", "color"]))
        .get_matches_from_safe(vec!["group", "-f", "-c"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}
