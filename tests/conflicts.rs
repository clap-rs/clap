extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, ArgGroup, ErrorKind};

static CONFLICT_ERR: &'static str = "error: The argument '-F' cannot be used with '--flag'

USAGE:
    clap-test <positional> <positional2> --flag --long-option-2 <option2>

For more information try --help";

static CONFLICT_ERR_REV: &'static str = "error: The argument '--flag' cannot be used with '-F'

USAGE:
    clap-test <positional> <positional2> -F --long-option-2 <option2>

For more information try --help";

#[test]
fn flag_conflict() {
    let result = App::new("flag_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("other"))
        .arg(Arg::from("-o, --other 'some flag'"))
        .try_get_matches_from(vec!["myprog", "-f", "-o"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn flag_conflict_2() {
    let result = App::new("flag_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("other"))
        .arg(Arg::from("-o, --other 'some flag'"))
        .try_get_matches_from(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_conflict() {
    let result = App::new("group_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("gr"))
        .group(
            ArgGroup::with_name("gr")
                .required(true)
                .arg("some")
                .arg("other"),
        )
        .arg(Arg::from("--some 'some arg'"))
        .arg(Arg::from("--other 'other arg'"))
        .try_get_matches_from(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_conflict_2() {
    let result = App::new("group_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("gr"))
        .group(
            ArgGroup::with_name("gr")
                .required(true)
                .arg("some")
                .arg("other"),
        )
        .arg(Arg::from("--some 'some arg'"))
        .arg(Arg::from("--other 'other arg'"))
        .try_get_matches_from(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn conflict_output() {
    test::compare_output(
        test::complex_app(),
        "clap-test val1 --flag --long-option-2 val2 -F",
        CONFLICT_ERR,
        true,
    );
}

#[test]
fn conflict_output_rev() {
    test::compare_output(
        test::complex_app(),
        "clap-test val1 -F --long-option-2 val2 --flag",
        CONFLICT_ERR_REV,
        true,
    );
}

#[test]
fn conflict_with_unused_default_value() {
    let result = App::new("conflict")
        .arg(Arg::from("-o, --opt=[opt] 'some opt'").default_value("default"))
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("opt"))
        .try_get_matches_from(vec!["myprog", "-f"]);
    assert!(result.is_ok());
    let m = result.unwrap();
    assert_eq!(m.value_of("opt"), Some("default"));
    assert!(m.is_present("flag"));
}
