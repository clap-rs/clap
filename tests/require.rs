extern crate clap;

use clap::{App, Arg, ErrorKind, ArgGroup};

#[test]
fn flag_required() {
    let result = App::new("flag_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("color"))
        .arg(Arg::from_usage("-c, --color 'third flag'"))
        .get_matches_from_safe(vec!["myprog", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn flag_required_2() {
    let m = App::new("flag_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("color"))
        .arg(Arg::from_usage("-c, --color 'third flag'"))
        .get_matches_from(vec!["myprog", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn option_required() {
    let result = App::new("option_required")
        .arg(Arg::from_usage("-f [flag] 'some flag'")
            .requires("color"))
        .arg(Arg::from_usage("-c [color] 'third flag'"))
        .get_matches_from_safe(vec!["myprog", "-f", "val"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn option_required_2() {
    let m = App::new("option_required")
        .arg(Arg::from_usage("-f [flag] 'some flag'")
            .requires("color"))
        .arg(Arg::from_usage("-c [color] 'third flag'"))
        .get_matches_from(vec!["myprog", "-f", "val", "-c", "other_val"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other_val");
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "val");
}

#[test]
fn positional_required() {
    let result = App::new("positional_required")
        .arg(Arg::with_name("flag")
            .index(1)
            .required(true))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn positional_required_2() {
    let m = App::new("positional_required")
        .arg(Arg::with_name("flag")
            .index(1)
            .required(true))
        .get_matches_from(vec!["myprog", "someval"]);
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "someval");
}

#[test]
fn group_required() {
    let result = App::new("group_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("gr")
            .required(true)
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from_safe(vec!["myprog", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_required_2() {
    let m = App::new("group_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("gr")
            .required(true)
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from(vec!["myprog", "-f", "--some"]);
    assert!(m.is_present("some"));
    assert!(!m.is_present("other"));
    assert!(m.is_present("flag"));
}

#[test]
fn group_required_3() {
    let m = App::new("group_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("gr")
            .required(true)
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from(vec!["myprog", "-f", "--other"]);
    assert!(!m.is_present("some"));
    assert!(m.is_present("other"));
    assert!(m.is_present("flag"));
}

#[test]
fn arg_require_group() {
    let result = App::new("arg_require_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("gr"))
        .group(ArgGroup::with_name("gr")
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from_safe(vec!["myprog", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn arg_require_group_2() {
    let m = App::new("arg_require_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("gr"))
        .group(ArgGroup::with_name("gr")
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from(vec!["myprog", "-f", "--some"]);
    assert!(m.is_present("some"));
    assert!(!m.is_present("other"));
    assert!(m.is_present("flag"));
}

#[test]
fn arg_require_group_3() {
    let m = App::new("arg_require_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("gr"))
        .group(ArgGroup::with_name("gr")
            .arg("some")
            .arg("other"))
        .arg(Arg::from_usage("--some 'some arg'"))
        .arg(Arg::from_usage("--other 'other arg'"))
        .get_matches_from(vec!["myprog", "-f", "--other"]);
    assert!(!m.is_present("some"));
    assert!(m.is_present("other"));
    assert!(m.is_present("flag"));
}