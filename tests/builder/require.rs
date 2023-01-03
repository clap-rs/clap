use clap::builder::ArgPredicate;
use clap::{arg, error::ErrorKind, Arg, ArgAction, ArgGroup, Command};

#[cfg(feature = "error-context")]
use super::utils;

#[test]
fn flag_required() {
    let result = Command::new("flag_required")
        .arg(arg!(-f --flag "some flag").requires("color"))
        .arg(arg!(-c --color "third flag"))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn flag_required_2() {
    let m = Command::new("flag_required")
        .arg(
            arg!(-f --flag "some flag")
                .requires("color")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-c --color "third flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn option_required() {
    let result = Command::new("option_required")
        .arg(arg!(f: -f <flag> "some flag").requires("c"))
        .arg(arg!(c: -c <color> "third flag"))
        .try_get_matches_from(vec!["", "-f", "val"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn option_required_2() {
    let m = Command::new("option_required")
        .arg(arg!(f: -f <flag> "some flag").requires("c"))
        .arg(arg!(c: -c <color> "third flag"))
        .try_get_matches_from(vec!["", "-f", "val", "-c", "other_val"])
        .unwrap();
    assert!(m.contains_id("c"));
    assert_eq!(
        m.get_one::<String>("c").map(|v| v.as_str()).unwrap(),
        "other_val"
    );
    assert!(m.contains_id("f"));
    assert_eq!(m.get_one::<String>("f").map(|v| v.as_str()).unwrap(), "val");
}

#[test]
fn positional_required() {
    let result = Command::new("positional_required")
        .arg(Arg::new("flag").index(1).required(true))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn positional_required_2() {
    let m = Command::new("positional_required")
        .arg(Arg::new("flag").index(1).required(true))
        .try_get_matches_from(vec!["", "someval"])
        .unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()).unwrap(),
        "someval"
    );
}

#[test]
#[cfg(feature = "error-context")]
fn positional_required_with_requires() {
    static POSITIONAL_REQ: &str = "\
error: the following required arguments were not provided:
  <flag>
  <opt>

Usage: clap-test <flag> <opt> [bar]

For more information, try '--help'.
";

    let cmd = Command::new("positional_required")
        .arg(Arg::new("flag").required(true).requires("opt"))
        .arg(Arg::new("opt"))
        .arg(Arg::new("bar"));

    utils::assert_output(cmd, "clap-test", POSITIONAL_REQ, true);
}

#[test]
#[cfg(feature = "error-context")]
fn positional_required_with_requires_if_no_value() {
    static POSITIONAL_REQ_IF_NO_VAL: &str = "\
error: the following required arguments were not provided:
  <flag>

Usage: clap-test <flag> [opt] [bar]

For more information, try '--help'.
";

    let cmd = Command::new("positional_required")
        .arg(Arg::new("flag").required(true).requires_if("val", "opt"))
        .arg(Arg::new("opt"))
        .arg(Arg::new("bar"));

    utils::assert_output(cmd, "clap-test", POSITIONAL_REQ_IF_NO_VAL, true);
}

#[test]
#[cfg(feature = "error-context")]
fn positional_required_with_requires_if_value() {
    static POSITIONAL_REQ_IF_VAL: &str = "\
error: the following required arguments were not provided:
  <foo>
  <opt>

Usage: clap-test <flag> <foo> <opt> [bar]

For more information, try '--help'.
";

    let cmd = Command::new("positional_required")
        .arg(Arg::new("flag").required(true).requires_if("val", "opt"))
        .arg(Arg::new("foo").required(true))
        .arg(Arg::new("opt"))
        .arg(Arg::new("bar"));

    utils::assert_output(cmd, "clap-test val", POSITIONAL_REQ_IF_VAL, true);
}

#[test]
fn group_required() {
    let result = Command::new("group_required")
        .arg(arg!(-f --flag "some flag"))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_required_2() {
    let m = Command::new("group_required")
        .arg(arg!(-f --flag "some flag").action(ArgAction::SetTrue))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(arg!(--some "some arg").action(ArgAction::SetTrue))
        .arg(arg!(--other "other arg").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-f", "--some"])
        .unwrap();
    assert!(*m.get_one::<bool>("some").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("other").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn group_required_3() {
    let m = Command::new("group_required")
        .arg(arg!(-f --flag "some flag").action(ArgAction::SetTrue))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(arg!(--some "some arg").action(ArgAction::SetTrue))
        .arg(arg!(--other "other arg").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-f", "--other"])
        .unwrap();
    assert!(!*m.get_one::<bool>("some").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("other").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn arg_require_group() {
    let result = Command::new("arg_require_group")
        .arg(arg!(-f --flag "some flag").requires("gr"))
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn arg_require_group_2() {
    let res = Command::new("arg_require_group")
        .arg(
            arg!(-f --flag "some flag")
                .requires("gr")
                .action(ArgAction::SetTrue),
        )
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg").action(ArgAction::SetTrue))
        .arg(arg!(--other "other arg").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-f", "--some"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("some").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("other").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn arg_require_group_3() {
    let res = Command::new("arg_require_group")
        .arg(
            arg!(-f --flag "some flag")
                .requires("gr")
                .action(ArgAction::SetTrue),
        )
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg").action(ArgAction::SetTrue))
        .arg(arg!(--other "other arg").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-f", "--other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!*m.get_one::<bool>("some").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("other").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

// REQUIRED_UNLESS

#[test]
fn issue_753() {
    let m = Command::new("test")
        .arg(arg!(
            -l --list "List available interfaces (and stop there)"
        ))
        .arg(
            arg!(
                -i --iface <INTERFACE> "Ethernet interface for fetching NTP packets"
            )
            .required(false)
            .required_unless_present("list"),
        )
        .arg(
            arg!(-f --file <TESTFILE> "Fetch NTP packets from pcap file")
                .conflicts_with("iface")
                .required_unless_present("list"),
        )
        .arg(arg!(-s --server <SERVER_IP> "NTP server IP address").required_unless_present("list"))
        .try_get_matches_from(vec!["test", "--list"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
}

#[test]
fn required_unless_present() {
    let res = Command::new("unlesstest")
        .arg(
            Arg::new("cfg")
                .required_unless_present("dbg")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["unlesstest", "--debug"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("dbg").expect("defaulted by clap"));
    assert!(!m.contains_id("cfg"));
}

#[test]
fn required_unless_present_err() {
    let res = Command::new("unlesstest")
        .arg(
            Arg::new("cfg")
                .required_unless_present("dbg")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .try_get_matches_from(vec!["unlesstest"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_unless_present_with_optional_value() {
    let res = Command::new("unlesstest")
        .arg(Arg::new("opt").long("opt").num_args(0..=1))
        .arg(
            Arg::new("cfg")
                .required_unless_present("dbg")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .try_get_matches_from(vec!["unlesstest", "--opt"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

// REQUIRED_UNLESS_ALL

#[test]
fn required_unless_present_all() {
    let res = Command::new("unlessall")
        .arg(
            Arg::new("cfg")
                .required_unless_present_all(["dbg", "infile"])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug").action(ArgAction::SetTrue))
        .arg(Arg::new("infile").short('i').action(ArgAction::Set))
        .try_get_matches_from(vec!["unlessall", "--debug", "-i", "file"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("dbg").expect("defaulted by clap"));
    assert!(m.contains_id("infile"));
    assert!(!m.contains_id("cfg"));
}

#[test]
fn required_unless_all_err() {
    let res = Command::new("unlessall")
        .arg(
            Arg::new("cfg")
                .required_unless_present_all(["dbg", "infile"])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug").action(ArgAction::SetTrue))
        .arg(Arg::new("infile").short('i').action(ArgAction::Set))
        .try_get_matches_from(vec!["unlessall", "--debug"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

// REQUIRED_UNLESS_ONE

#[test]
fn required_unless_present_any() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(["dbg", "infile"])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug").action(ArgAction::SetTrue))
        .arg(Arg::new("infile").short('i').action(ArgAction::Set))
        .try_get_matches_from(vec!["unlessone", "--debug"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("dbg").expect("defaulted by clap"));
    assert!(!m.contains_id("cfg"));
}

#[test]
fn required_unless_any_2() {
    // This tests that the required_unless_present_any works when the second arg in the array is used
    // instead of the first.
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(["dbg", "infile"])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug").action(ArgAction::SetTrue))
        .arg(Arg::new("infile").short('i').action(ArgAction::Set))
        .try_get_matches_from(vec!["unlessone", "-i", "file"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("infile"));
    assert!(!m.contains_id("cfg"));
}

#[test]
fn required_unless_any_works_with_short() {
    // GitHub issue: https://github.com/clap-rs/clap/issues/1135
    let res = Command::new("unlessone")
        .arg(
            Arg::new("a")
                .conflicts_with("b")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("b").short('b').action(ArgAction::SetTrue))
        .arg(
            Arg::new("x")
                .short('x')
                .action(ArgAction::SetTrue)
                .required_unless_present_any(["a", "b"]),
        )
        .try_get_matches_from(vec!["unlessone", "-a"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_unless_any_works_with_short_err() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("a")
                .conflicts_with("b")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("b").short('b').action(ArgAction::SetTrue))
        .arg(
            Arg::new("x")
                .short('x')
                .action(ArgAction::SetTrue)
                .required_unless_present_any(["a", "b"]),
        )
        .try_get_matches_from(vec!["unlessone"]);

    assert!(res.is_err());
}

#[test]
fn required_unless_any_works_without() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("a")
                .conflicts_with("b")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("b").short('b').action(ArgAction::SetTrue))
        .arg(Arg::new("x").required_unless_present_any(["a", "b"]))
        .try_get_matches_from(vec!["unlessone", "-a"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_unless_any_works_with_long() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("a")
                .conflicts_with("b")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("b").short('b').action(ArgAction::SetTrue))
        .arg(
            Arg::new("x")
                .long("x_is_the_option")
                .action(ArgAction::SetTrue)
                .required_unless_present_any(["a", "b"]),
        )
        .try_get_matches_from(vec!["unlessone", "-a"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_unless_any_1() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(["dbg", "infile"])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug").action(ArgAction::SetTrue))
        .arg(Arg::new("infile").short('i').action(ArgAction::Set))
        .try_get_matches_from(vec!["unlessone", "--debug"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.contains_id("infile"));
    assert!(!m.contains_id("cfg"));
    assert!(*m.get_one::<bool>("dbg").expect("defaulted by clap"));
}

#[test]
fn required_unless_any_err() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(["dbg", "infile"])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug").action(ArgAction::SetTrue))
        .arg(Arg::new("infile").short('i').action(ArgAction::Set))
        .try_get_matches_from(vec!["unlessone"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
#[cfg(feature = "error-context")]
fn missing_required_output() {
    static MISSING_REQ: &str = "\
error: the following required arguments were not provided:
  --long-option-2 <option2>
  <positional>
  <positional2>

Usage: clap-test --long-option-2 <option2> -F <positional> <positional2> [positional3]...

For more information, try '--help'.
";

    utils::assert_output(utils::complex_app(), "clap-test -F", MISSING_REQ, true);
}

// Conditional external requirements

#[test]
fn requires_if_present_val() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_if("my.cfg", "extra")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["unlessone", "--config=my.cfg"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn requires_if_present_mult() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_ifs([("my.cfg", "extra"), ("other.cfg", "other")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra").action(ArgAction::SetTrue))
        .arg(Arg::new("other").long("other").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["unlessone", "--config=other.cfg"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn requires_if_present_mult_pass() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_ifs([("my.cfg", "extra"), ("other.cfg", "other")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra").action(ArgAction::SetTrue))
        .arg(Arg::new("other").long("other").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["unlessone", "--config=some.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn requires_if_present_val_no_present_pass() {
    let res = Command::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_if("my.cfg", "extra")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["unlessone"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

// Conditionally required

#[test]
fn required_if_val_present_pass() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "val")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .try_get_matches_from(vec!["ri", "--extra", "val", "--config", "my.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_val_present_fail() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "val")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .try_get_matches_from(vec!["ri", "--extra", "val"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_if_val_present_ignore_case_pass() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "Val")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(
            Arg::new("extra")
                .action(ArgAction::Set)
                .long("extra")
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["ri", "--extra", "vaL", "--config", "my.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_val_present_ignore_case_fail() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "Val")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(
            Arg::new("extra")
                .action(ArgAction::Set)
                .long("extra")
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["ri", "--extra", "vaL"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_if_all_values_present_pass() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all([("extra", "val"), ("option", "spec")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec![
            "ri", "--extra", "val", "--option", "spec", "--config", "my.cfg",
        ]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_some_values_present_pass() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all([("extra", "val"), ("option", "spec")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "val"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_all_values_present_fail() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all([("extra", "val"), ("option", "spec")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "val", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_if_any_all_values_present_pass() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all([("extra", "val"), ("option", "spec")])
                .required_if_eq_any([("extra", "val2"), ("option", "spec2")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec![
            "ri", "--extra", "val", "--option", "spec", "--config", "my.cfg",
        ]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_any_all_values_present_fail() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all([("extra", "val"), ("option", "spec")])
                .required_if_eq_any([("extra", "val2"), ("option", "spec2")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "val", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
#[cfg(feature = "error-context")]
fn list_correct_required_args() {
    static COND_REQ_IN_USAGE: &str = "\
error: the following required arguments were not provided:
  --output <output>

Usage: test --target <target> --input <input> --output <output>

For more information, try '--help'.
";

    let cmd = Command::new("Test cmd")
        .version("1.0")
        .author("F0x06")
        .about("Arg test")
        .arg(
            Arg::new("target")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(["file", "stdout"])
                .long("target"),
        )
        .arg(
            Arg::new("input")
                .action(ArgAction::Set)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::new("output")
                .action(ArgAction::Set)
                .required(true)
                .long("output"),
        );

    utils::assert_output(
        cmd,
        "test --input somepath --target file",
        COND_REQ_IN_USAGE,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn required_if_val_present_fail_error_output() {
    static COND_REQ_IN_USAGE: &str = "\
error: the following required arguments were not provided:
  --output <output>

Usage: test --target <target> --input <input> --output <output>

For more information, try '--help'.
";

    let cmd = Command::new("Test cmd")
        .version("1.0")
        .author("F0x06")
        .about("Arg test")
        .arg(
            Arg::new("target")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(["file", "stdout"])
                .long("target"),
        )
        .arg(
            Arg::new("input")
                .action(ArgAction::Set)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::new("output")
                .action(ArgAction::Set)
                .required_if_eq("target", "file")
                .long("output"),
        );

    utils::assert_output(
        cmd,
        "test --input somepath --target file",
        COND_REQ_IN_USAGE,
        true,
    );
}

#[test]
fn required_if_wrong_val() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "val")
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .try_get_matches_from(vec!["ri", "--extra", "other"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_ifs_val_present_pass() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any([("extra", "val"), ("option", "spec")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .try_get_matches_from(vec!["ri", "--option", "spec", "--config", "my.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_ifs_val_present_fail() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any([("extra", "val"), ("option", "spec")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec!["ri", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_ifs_wrong_val() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any([("extra", "val"), ("option", "spec")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec!["ri", "--option", "other"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_ifs_wrong_val_mult_fail() {
    let res = Command::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any([("extra", "val"), ("option", "spec")])
                .action(ArgAction::Set)
                .long("config"),
        )
        .arg(Arg::new("extra").action(ArgAction::Set).long("extra"))
        .arg(Arg::new("option").action(ArgAction::Set).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "other", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
#[cfg(feature = "error-context")]
fn require_eq() {
    static REQUIRE_EQUALS: &str = "\
error: the following required arguments were not provided:
  --opt=<FILE>

Usage: clap-test --opt=<FILE>

For more information, try '--help'.
";

    let cmd = Command::new("clap-test").version("v1.4.8").arg(
        Arg::new("opt")
            .long("opt")
            .short('o')
            .required(true)
            .require_equals(true)
            .value_name("FILE")
            .help("some"),
    );
    utils::assert_output(cmd, "clap-test", REQUIRE_EQUALS, true);
}

#[test]
#[cfg(feature = "error-context")]
fn require_eq_filtered() {
    static REQUIRE_EQUALS_FILTERED: &str = "\
error: the following required arguments were not provided:
  --opt=<FILE>

Usage: clap-test --opt=<FILE> --foo=<FILE>

For more information, try '--help'.
";

    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .arg(
            Arg::new("opt")
                .long("opt")
                .short('o')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some"),
        )
        .arg(
            Arg::new("foo")
                .long("foo")
                .short('f')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some other arg"),
        );
    utils::assert_output(cmd, "clap-test -f=blah", REQUIRE_EQUALS_FILTERED, true);
}

#[test]
#[cfg(feature = "error-context")]
fn require_eq_filtered_group() {
    static REQUIRE_EQUALS_FILTERED_GROUP: &str = "\
error: the following required arguments were not provided:
  --opt=<FILE>

Usage: clap-test --opt=<FILE> --foo=<FILE> <--g1=<FILE>|--g2=<FILE>>

For more information, try '--help'.
";

    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .arg(
            Arg::new("opt")
                .long("opt")
                .short('o')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some"),
        )
        .arg(
            Arg::new("foo")
                .long("foo")
                .short('f')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some other arg"),
        )
        .arg(
            Arg::new("g1")
                .long("g1")
                .require_equals(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("g2")
                .long("g2")
                .require_equals(true)
                .value_name("FILE"),
        )
        .group(
            ArgGroup::new("test_group")
                .args(["g1", "g2"])
                .required(true),
        );
    utils::assert_output(
        cmd,
        "clap-test -f=blah --g1=blah",
        REQUIRE_EQUALS_FILTERED_GROUP,
        true,
    );
}

fn issue_1158_app() -> Command {
    Command::new("example")
        .arg(
            arg!(-c --config <FILE> "Custom config file.")
                .required_unless_present("ID")
                .conflicts_with("ID"),
        )
        .arg(
            arg!([ID] "ID")
                .required_unless_present("config")
                .conflicts_with("config")
                .requires_ifs([
                    (ArgPredicate::IsPresent, "x"),
                    (ArgPredicate::IsPresent, "y"),
                    (ArgPredicate::IsPresent, "z"),
                ]),
        )
        .arg(arg!(x: -x <X> "X"))
        .arg(arg!(y: -y <Y> "Y"))
        .arg(arg!(z: -z <Z> "Z"))
}

#[test]
#[cfg(feature = "error-context")]
fn multiple_required_unless_usage_printing() {
    static MULTIPLE_REQUIRED_UNLESS_USAGE: &str = "\
error: the following required arguments were not provided:
  --a <a>
  --b <b>

Usage: test --c <c> --a <a> --b <b>

For more information, try '--help'.
";
    let cmd = Command::new("test")
        .arg(
            Arg::new("a")
                .long("a")
                .action(ArgAction::Set)
                .required_unless_present("b")
                .conflicts_with("b"),
        )
        .arg(
            Arg::new("b")
                .long("b")
                .action(ArgAction::Set)
                .required_unless_present("a")
                .conflicts_with("a"),
        )
        .arg(
            Arg::new("c")
                .long("c")
                .action(ArgAction::Set)
                .required_unless_present("d")
                .conflicts_with("d"),
        )
        .arg(
            Arg::new("d")
                .long("d")
                .action(ArgAction::Set)
                .required_unless_present("c")
                .conflicts_with("c"),
        );
    utils::assert_output(cmd, "test --c asd", MULTIPLE_REQUIRED_UNLESS_USAGE, true);
}

#[test]
#[cfg(feature = "error-context")]
fn issue_1158_conflicting_requirements() {
    static ISSUE_1158: &str = "\
error: the following required arguments were not provided:
  -x <X>
  -y <Y>
  -z <Z>

Usage: example -x <X> -y <Y> -z <Z> <ID>

For more information, try '--help'.
";

    let cmd = issue_1158_app();

    utils::assert_output(cmd, "example id", ISSUE_1158, true);
}

#[test]
fn issue_1158_conflicting_requirements_rev() {
    let res = issue_1158_app().try_get_matches_from(["", "--config", "some.conf"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn issue_1643_args_mutually_require_each_other() {
    use clap::*;

    let cmd = Command::new("test")
        .arg(
            Arg::new("relation_id")
                .help("The relation id to get the data from")
                .long("relation-id")
                .short('r')
                .action(ArgAction::Set)
                .requires("remote_unit_name"),
        )
        .arg(
            Arg::new("remote_unit_name")
                .help("The name of the remote unit to get data from")
                .long("remote-unit")
                .short('u')
                .action(ArgAction::Set)
                .requires("relation_id"),
        );

    cmd.try_get_matches_from(["test", "-u", "hello", "-r", "farewell"])
        .unwrap();
}

#[test]
fn short_flag_require_equals_with_minvals_zero() {
    let m = Command::new("foo")
        .arg(
            Arg::new("check")
                .short('c')
                .num_args(0..)
                .require_equals(true),
        )
        .arg(Arg::new("unique").short('u').action(ArgAction::SetTrue))
        .try_get_matches_from(["foo", "-cu"])
        .unwrap();
    assert!(m.contains_id("check"));
    assert!(*m.get_one::<bool>("unique").expect("defaulted by clap"));
}

#[test]
fn issue_2624() {
    let matches = Command::new("foo")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .require_equals(true)
                .num_args(0..)
                .value_parser(["silent", "quiet", "diagnose-first"]),
        )
        .arg(
            Arg::new("unique")
                .short('u')
                .long("unique")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(["foo", "-cu"])
        .unwrap();
    assert!(matches.contains_id("check"));
    assert!(*matches
        .get_one::<bool>("unique")
        .expect("defaulted by clap"));
}

#[test]
fn required_unless_all_with_any() {
    let cmd = Command::new("prog")
        .arg(Arg::new("foo").long("foo").action(ArgAction::SetTrue))
        .arg(Arg::new("bar").long("bar").action(ArgAction::SetTrue))
        .arg(Arg::new("baz").long("baz").action(ArgAction::SetTrue))
        .arg(
            Arg::new("flag")
                .long("flag")
                .action(ArgAction::SetTrue)
                .required_unless_present_any(["foo"])
                .required_unless_present_all(["bar", "baz"]),
        );

    let result = cmd.clone().try_get_matches_from(vec!["myprog"]);
    assert!(result.is_err(), "{:?}", result.unwrap());

    let result = cmd.clone().try_get_matches_from(vec!["myprog", "--foo"]);
    assert!(result.is_ok(), "{:?}", result.unwrap_err());
    let matches = result.unwrap();
    assert!(!*matches.get_one::<bool>("flag").expect("defaulted by clap"));

    let result = cmd
        .clone()
        .try_get_matches_from(vec!["myprog", "--bar", "--baz"]);
    assert!(result.is_ok(), "{:?}", result.unwrap_err());
    let matches = result.unwrap();
    assert!(!*matches.get_one::<bool>("flag").expect("defaulted by clap"));

    let result = cmd.try_get_matches_from(vec!["myprog", "--bar"]);
    assert!(result.is_err(), "{:?}", result.unwrap());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command prog: Argument or group 'extra' specified in 'requires*' for 'config' does not exist"]
fn requires_invalid_arg() {
    let _ = Command::new("prog")
        .arg(Arg::new("config").requires("extra").long("config"))
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command prog: Argument or group 'extra' specified in 'requires*' for 'config' does not exist"]
fn requires_if_invalid_arg() {
    let _ = Command::new("prog")
        .arg(
            Arg::new("config")
                .requires_if("val", "extra")
                .long("config"),
        )
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command prog: Argument or group 'extra' specified in 'required_if_eq*' for 'config' does not exist"]
fn required_if_invalid_arg() {
    let _ = Command::new("prog")
        .arg(
            Arg::new("config")
                .required_if_eq("extra", "val")
                .long("config"),
        )
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command prog: Argument or group 'extra' specified in 'required_unless*' for 'config' does not exist"]
fn required_unless_invalid_arg() {
    let _ = Command::new("prog")
        .arg(
            Arg::new("config")
                .required_unless_present("extra")
                .long("config"),
        )
        .try_get_matches_from(vec![""]);
}

#[test]
fn requires_with_default_value() {
    let result = Command::new("prog")
        .arg(
            Arg::new("opt")
                .long("opt")
                .default_value("default")
                .requires("flag"),
        )
        .arg(Arg::new("flag").long("flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["myprog"]);

    assert!(
        result.is_ok(),
        "requires should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn requires_if_with_default_value() {
    let result = Command::new("prog")
        .arg(
            Arg::new("opt")
                .long("opt")
                .default_value("default")
                .requires_if("default", "flag"),
        )
        .arg(Arg::new("flag").long("flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["myprog"]);

    assert!(
        result.is_ok(),
        "requires_if should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn group_requires_with_default_value() {
    let result = Command::new("prog")
        .arg(Arg::new("opt").long("opt").default_value("default"))
        .arg(Arg::new("flag").long("flag").action(ArgAction::SetTrue))
        .group(ArgGroup::new("one").arg("opt").requires("flag"))
        .try_get_matches_from(vec!["myprog"]);

    assert!(
        result.is_ok(),
        "arg group requires should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn required_if_eq_on_default_value() {
    let result = Command::new("prog")
        .arg(Arg::new("opt").long("opt").default_value("default"))
        .arg(
            Arg::new("flag")
                .long("flag")
                .action(ArgAction::SetTrue)
                .required_if_eq("opt", "default"),
        )
        .try_get_matches_from(vec!["myprog"]);

    assert!(
        result.is_ok(),
        "required_if_eq should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn required_if_eq_all_on_default_value() {
    let result = Command::new("prog")
        .arg(Arg::new("opt").long("opt").default_value("default"))
        .arg(
            Arg::new("flag")
                .long("flag")
                .action(ArgAction::SetTrue)
                .required_if_eq_all([("opt", "default")]),
        )
        .try_get_matches_from(vec!["myprog"]);

    assert!(
        result.is_ok(),
        "required_if_eq_all should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn required_unless_on_default_value() {
    let result = Command::new("prog")
        .arg(Arg::new("opt").long("opt").default_value("default"))
        .arg(Arg::new("flag").long("flag").required_unless_present("opt"))
        .try_get_matches_from(vec!["myprog"]);

    assert!(result.is_err(), "{:?}", result.unwrap());
}

#[test]
fn required_unless_all_on_default_value() {
    let result = Command::new("prog")
        .arg(Arg::new("opt").long("opt").default_value("default"))
        .arg(
            Arg::new("flag")
                .long("flag")
                .required_unless_present_all(["opt"]),
        )
        .try_get_matches_from(vec!["myprog"]);

    assert!(result.is_err(), "{:?}", result.unwrap());
}

#[test]
#[cfg(feature = "error-context")]
fn required_error_doesnt_duplicate() {
    let cmd = Command::new("Clap-created-USAGE-string-bug")
        .arg(Arg::new("a").required(true))
        .arg(
            Arg::new("b")
                .short('b')
                .action(ArgAction::Set)
                .conflicts_with("c"),
        )
        .arg(
            Arg::new("c")
                .short('c')
                .action(ArgAction::Set)
                .conflicts_with("b"),
        );
    const EXPECTED: &str = "\
error: the argument '-b <b>' cannot be used with '-c <c>'

Usage: clap-test -b <b> <a>

For more information, try '--help'.
";
    utils::assert_output(cmd, "clap-test aaa -b bbb -c ccc", EXPECTED, true);
}

#[test]
#[cfg(feature = "error-context")]
fn required_require_with_group_shows_flag() {
    let cmd = Command::new("test")
        .arg(arg!(--"require-first").requires("first"))
        .arg(arg!(--first).group("either_or_both"))
        .arg(arg!(--second).group("either_or_both"))
        .group(
            ArgGroup::new("either_or_both")
                .multiple(true)
                .required(true),
        );
    const EXPECTED: &str = "\
error: the following required arguments were not provided:
  --first

Usage: test --require-first <--first|--second>

For more information, try '--help'.
";
    utils::assert_output(cmd, "test --require-first --second", EXPECTED, true);
}
