extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, ErrorKind, ArgGroup};

static MISSING_REQ: &'static str = "error: The following required arguments were not provided:
    <positional2>
    --long-option-2 <option2>

USAGE:
    clap-test <positional2> -F --long-option-2 <option2>

For more information try --help";

static COND_REQ_IN_USAGE: &'static str = "error: The following required arguments were not provided:
    --output <output>

USAGE:
    test --target <target> --input <input> --output <output>

For more information try --help";

#[test]
fn flag_required() {
    let result = App::new("flag_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'").requires("color"))
        .arg(Arg::from_usage("-c, --color 'third flag'"))
        .get_matches_from_safe(vec!["", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn flag_required_2() {
    let m = App::new("flag_required")
        .arg(Arg::from_usage("-f, --flag 'some flag'").requires("color"))
        .arg(Arg::from_usage("-c, --color 'third flag'"))
        .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn option_required() {
    let result = App::new("option_required")
        .arg(Arg::from_usage("-f [flag] 'some flag'").requires("color"))
        .arg(Arg::from_usage("-c [color] 'third flag'"))
        .get_matches_from_safe(vec!["", "-f", "val"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn option_required_2() {
    let m = App::new("option_required")
        .arg(Arg::from_usage("-f [flag] 'some flag'").requires("c"))
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
        .arg(Arg::from_usage("-f, --flag 'some flag'").requires("gr"))
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
        .arg(Arg::from_usage("-f, --flag 'some flag'").requires("gr"))
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
        .arg(Arg::from_usage("-f, --flag 'some flag'").requires("gr"))
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
fn issue_753() {
    let m = App::new("test")
        .arg(Arg::from_usage("-l, --list 'List available interfaces (and stop there)'"))
        .arg(Arg::from_usage("-i, --iface=[INTERFACE] 'Ethernet interface for fetching NTP packets'")
            .required_unless("list"))
        .arg(Arg::from_usage("-f, --file=[TESTFILE] 'Fetch NTP packets from pcap file'")
            .conflicts_with("iface")
            .required_unless("list"))
        .arg(Arg::from_usage("-s, --server=[SERVER_IP] 'NTP server IP address'")
            .required_unless("list"))
        .arg(Arg::from_usage("-p, --port=[SERVER_PORT] 'NTP server port'")
            .default_value("123"))
        .get_matches_from_safe(vec!["test", "--list"]);
    assert!(m.is_ok());
}

#[test]
fn required_unless() {
    let res = App::new("unlesstest")
        .arg(Arg::with_name("cfg")
            .required_unless("dbg")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg").long("debug"))
        .get_matches_from_safe(vec!["unlesstest", "--debug"]);

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
        .arg(Arg::with_name("dbg").long("debug"))
        .get_matches_from_safe(vec!["unlesstest"]);

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
        .arg(Arg::with_name("dbg").long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec!["unlessall", "--debug", "-i", "file"]);

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
        .arg(Arg::with_name("dbg").long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec!["unlessall", "--debug"]);

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
        .arg(Arg::with_name("dbg").long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec!["unlessone", "--debug"]);

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("dbg"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_one_2() {
    // This tests that the required_unless_one works when the second arg in the array is used
    // instead of the first.
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .required_unless_one(&["dbg", "infile"])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg").long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec!["unlessone", "-i", "file"]);

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("infile"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_one_1() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .required_unless_one(&["dbg", "infile"])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg").long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec!["unlessone", "--debug"]);

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(!m.is_present("infile"));
    assert!(!m.is_present("cfg"));
    assert!(m.is_present("dbg"));
}

#[test]
fn required_unless_one_err() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .required_unless_one(&["dbg", "infile"])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("dbg").long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .takes_value(true))
        .get_matches_from_safe(vec!["unlessone"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn missing_required_output() {
    assert!(test::compare_output(test::complex_app(), "clap-test -F", MISSING_REQ, true));
}

// Conditional external requirements

#[test]
fn requires_if_present_val() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .requires_if("my.cfg", "extra")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra").long("extra"))
        .get_matches_from_safe(vec!["unlessone", "--config=my.cfg"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn requires_if_present_mult() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .requires_ifs(&[("my.cfg", "extra"), ("other.cfg", "other")])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra").long("extra"))
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from_safe(vec!["unlessone", "--config=other.cfg"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn requires_if_present_mult_pass() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .requires_ifs(&[("my.cfg", "extra"), ("other.cfg", "other")])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra").long("extra"))
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from_safe(vec!["unlessone", "--config=some.cfg"]);

    assert!(res.is_ok());
    // assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn requires_if_present_val_no_present_pass() {
    let res = App::new("unlessone")
        .arg(Arg::with_name("cfg")
            .requires_if("my.cfg", "extra")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra").long("extra"))
        .get_matches_from_safe(vec!["unlessone"]);

    assert!(res.is_ok());
}

// Conditionally required

#[test]
fn required_if_val_present_pass() {
    let res = App::new("ri")
        .arg(Arg::with_name("cfg")
            .required_if("extra", "val")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra")
            .takes_value(true)
            .long("extra"))
        .get_matches_from_safe(vec!["ri", "--extra", "val", "--config", "my.cfg"]);

    assert!(res.is_ok());
}

#[test]
fn required_if_val_present_fail() {
    let res = App::new("ri")
        .arg(Arg::with_name("cfg")
            .required_if("extra", "val")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra")
            .takes_value(true)
            .long("extra"))
        .get_matches_from_safe(vec!["ri", "--extra", "val"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_if_val_present_fail_error_output() {
    let app = App::new("Test app")
        .version("1.0")
        .author("F0x06")
        .about("Arg test")
        .arg(Arg::with_name("target")
            .takes_value(true)
            .required(true)
            .possible_values(&["file", "stdout"])
            .long("target"))
        .arg(Arg::with_name("input")
            .takes_value(true)
            .required(true)
            .long("input"))
        .arg(Arg::with_name("output")
            .takes_value(true)
            .required_if("target", "file")
            .long("output"));

    assert!(test::compare_output(app,
                                 "test --input somepath --target file",
                                 COND_REQ_IN_USAGE,
                                 true));
}

#[test]
fn required_if_wrong_val() {
    let res = App::new("ri")
        .arg(Arg::with_name("cfg")
            .required_if("extra", "val")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra")
            .takes_value(true)
            .long("extra"))
        .get_matches_from_safe(vec!["ri", "--extra", "other"]);

    assert!(res.is_ok());
}

#[test]
fn required_ifs_val_present_pass() {
    let res = App::new("ri")
        .arg(Arg::with_name("cfg")
            .required_ifs(&[("extra", "val"), ("option", "spec")])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("option")
            .takes_value(true)
            .long("option"))
        .arg(Arg::with_name("extra")
            .takes_value(true)
            .long("extra"))
        .get_matches_from_safe(vec!["ri", "--option", "spec", "--config", "my.cfg"]);

    assert!(res.is_ok());
    // assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_ifs_val_present_fail() {
    let res = App::new("ri")
        .arg(Arg::with_name("cfg")
            .required_ifs(&[("extra", "val"), ("option", "spec")])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra")
            .takes_value(true)
            .long("extra"))
        .arg(Arg::with_name("option")
            .takes_value(true)
            .long("option"))
        .get_matches_from_safe(vec!["ri", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_ifs_wrong_val() {
    let res = App::new("ri")
        .arg(Arg::with_name("cfg")
            .required_ifs(&[("extra", "val"), ("option", "spec")])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra")
            .takes_value(true)
            .long("extra"))
        .arg(Arg::with_name("option")
            .takes_value(true)
            .long("option"))
        .get_matches_from_safe(vec!["ri", "--option", "other"]);

    assert!(res.is_ok());
}

#[test]
fn required_ifs_wrong_val_mult_fail() {
    let res = App::new("ri")
        .arg(Arg::with_name("cfg")
            .required_ifs(&[("extra", "val"), ("option", "spec")])
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("extra")
            .takes_value(true)
            .long("extra"))
        .arg(Arg::with_name("option")
            .takes_value(true)
            .long("option"))
        .get_matches_from_safe(vec!["ri", "--extra", "other", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}