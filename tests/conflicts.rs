extern crate clap;

use clap::{App, Arg, ErrorKind, ArgGroup};

#[test]
fn flag_conflict() {
    let result = App::new("flag_conflict")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("other"))
        .arg(Arg::from_usage("-o, --other 'some flag'"))
        .get_matches_from_safe(vec!["myprog", "-f", "-o"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn flag_conflict_2() {
    let result = App::new("flag_conflict")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("other"))
        .arg(Arg::from_usage("-o, --other 'some flag'"))
        .get_matches_from_safe(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_conflict() {
    let result = App::new("group_conflict")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("gr"))
        .group(ArgGroup::with_name("gr")
            .required(true)
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from_safe(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_conflict_2() {
    let result = App::new("group_conflict")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("gr"))
        .group(ArgGroup::with_name("gr")
            .required(true)
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from_safe(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}
