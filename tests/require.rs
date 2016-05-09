extern crate clap;
extern crate clap_test;

use clap::{App, Arg, ErrorKind, ArgGroup};

#[test]
fn flag_required() {
    let result = App::new("flag_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("color"))
        .arg(Arg::from_usage("-c, --color 'third flag'"))
        .get_matches_from_safe(vec!["", "-f"]);
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
        .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn option_required() {
    let result = App::new("option_required")
        .arg(Arg::from_usage("-f [flag] 'some flag'")
            .requires("color"))
        .arg(Arg::from_usage("-c [color] 'third flag'"))
        .get_matches_from_safe(vec!["", "-f", "val"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn option_required_2() {
    let m = App::new("option_required")
        .arg(Arg::from_usage("-f [flag] 'some flag'")
            .requires("c"))
        .arg(Arg::from_usage("-c [color] 'third flag'"))
        .get_matches_from(vec!["", "-f", "val", "-c", "other_val"]);
    assert!(m.is_present("c"));
    assert_eq!(m.value_of("c").unwrap(), "other_val");
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "val");
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
        .get_matches_from(vec!["", "someval"]);
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
        .get_matches_from_safe(vec!["", "-f"]);
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
        .get_matches_from(vec!["", "-f", "--some"]);
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
        .get_matches_from(vec!["", "-f", "--other"]);
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
        .get_matches_from_safe(vec!["", "-f"]);
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
        .get_matches_from(vec!["", "-f", "--some"]);
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
        .get_matches_from(vec!["", "-f", "--other"]);
    assert!(!m.is_present("some"));
    assert!(m.is_present("other"));
    assert!(m.is_present("flag"));
}

// REQUIRED_UNLESS

#[test]
fn required_unless() {
    let res = App::new("unlesstest")
        .arg(Arg::with_name("cfg")
            .required_unless("dbg")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg")
            .long("debug"))
        .get_matches_from_safe(vec![
            "unlesstest", "--debug"
        ]);

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("dbg"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_err() {
    let res = App::new("unlesstest")
        .arg(Arg::with_name("cfg")
            .required_unless("dbg")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg")
            .long("debug"))
        .get_matches_from_safe(vec![
            "unlesstest"
        ]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

// REQUIRED_UNLESS_ALL

#[test]
fn required_unless_all() {
    let res = App::new("unlessall")
        .arg(Arg::with_name("cfg")
            .required_unless_all(&["dbg", "infile"])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg")
            .long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec![
            "unlessall", "--debug", "-i", "file"
        ]);

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("dbg"));
    assert!(m.is_present("infile"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_all_err() {
    let res = App::new("unlessall")
        .arg(Arg::with_name("cfg")
            .required_unless_all(&["dbg", "infile"])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg")
            .long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec![
            "unlessall", "--debug"
        ]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

// REQUIRED_UNLESS_ONE

#[test]
fn required_unless_one() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .required_unless_one(&["dbg", "infile"])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg")
            .long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec![
            "unlessone", "--debug"
        ]);

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("dbg"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_one_err() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .required_unless_one(&["dbg", "infile"])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg")
            .long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec![
            "unlessone"
        ]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn missing_required_output() {
    clap_test::check_err_output(clap_test::complex_app(), "clap-test -F",
"error: The following required arguments were not provided:
    <positional2>
    --long-option-2 <option2>

USAGE:
    clap-test <positional2> -F --long-option-2 <option2>

For more information try --help", true)
}
