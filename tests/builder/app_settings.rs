use std::ffi::OsString;

use super::utils;

use clap::{arg, error::ErrorKind, Arg, ArgAction, Command};

static ALLOW_EXT_SC: &str = "\
Usage: clap-test [COMMAND]

Options:
  -h, --help     Print help
  -V, --version  Print version
";

static DONT_COLLAPSE_ARGS: &str = "\
Usage: clap-test [arg1] [arg2] [arg3]

Arguments:
  [arg1]  some
  [arg2]  some
  [arg3]  some

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn sub_command_negate_required() {
    Command::new("sub_command_negate")
        .subcommand_negates_reqs(true)
        .arg(Arg::new("test").required(true).index(1))
        .subcommand(Command::new("sub1"))
        .try_get_matches_from(vec!["myprog", "sub1"])
        .unwrap();
}

#[test]
fn sub_command_negate_required_2() {
    let result = Command::new("sub_command_negate")
        .subcommand_negates_reqs(true)
        .arg(Arg::new("test").required(true).index(1))
        .subcommand(Command::new("sub1"))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn sub_command_required() {
    let result = Command::new("sc_required")
        .subcommand_required(true)
        .subcommand(Command::new("sub1"))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingSubcommand);
}

#[test]
#[cfg(feature = "error-context")]
fn sub_command_required_error() {
    static ERROR: &str = "\
error: 'sc_required' requires a subcommand but one was not provided
  [subcommands: sub1, help]

Usage: sc_required <COMMAND>

For more information, try '--help'.
";

    let cmd = Command::new("sc_required")
        .subcommand_required(true)
        .subcommand(Command::new("sub1"));
    utils::assert_output(cmd, "sc_required", ERROR, true);
}

#[test]
fn arg_required_else_help() {
    let result = Command::new("arg_required")
        .arg_required_else_help(true)
        .arg(Arg::new("test").index(1))
        .try_get_matches_from(vec![""]);

    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn arg_required_else_help_over_req_arg() {
    let result = Command::new("arg_required")
        .arg_required_else_help(true)
        .arg(Arg::new("test").index(1).required(true))
        .try_get_matches_from(vec![""]);

    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn arg_required_else_help_over_req_subcommand() {
    let result = Command::new("sub_required")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("sub1"))
        .try_get_matches_from(vec![""]);

    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn arg_required_else_help_with_default() {
    let result = Command::new("arg_required")
        .arg_required_else_help(true)
        .arg(arg!(--input <PATH>).default_value("-"))
        .try_get_matches_from(vec![""]);

    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn arg_required_else_help_error_message() {
    static ARG_REQUIRED_ELSE_HELP: &str = "\
Usage: test [OPTIONS]

Options:
  -i, --info     Provides more info
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("test")
        .arg_required_else_help(true)
        .version("1.0")
        .arg(
            Arg::new("info")
                .help("Provides more info")
                .short('i')
                .long("info")
                .action(ArgAction::SetTrue),
        );
    utils::assert_output(
        cmd,
        "test",
        ARG_REQUIRED_ELSE_HELP,
        true, // Unlike normal displaying of help, we should provide a fatal exit code
    );
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"))
        .try_get_matches_from(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"))
        .try_get_matches_from(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn infer_subcommands_fail_with_args() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .arg(Arg::new("some"))
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"))
        .try_get_matches_from(vec!["prog", "t"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    assert_eq!(
        m.unwrap().get_one::<String>("some").map(|v| v.as_str()),
        Some("t")
    );
}

#[test]
fn infer_subcommands_fail_with_args2() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .arg(Arg::new("some"))
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"))
        .try_get_matches_from(vec!["prog", "te"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    assert_eq!(
        m.unwrap().get_one::<String>("some").map(|v| v.as_str()),
        Some("te")
    );
}

#[test]
fn infer_subcommands_pass() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test"))
        .try_get_matches_from(vec!["prog", "te"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn infer_subcommands_pass_close() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"))
        .try_get_matches_from(vec!["prog", "tes"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn infer_subcommands_pass_exact_match() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test"))
        .subcommand(Command::new("testa"))
        .subcommand(Command::new("testb"))
        .try_get_matches_from(vec!["prog", "test"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"))
        .try_get_matches_from(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"))
        .try_get_matches_from(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn no_bin_name() {
    let result = Command::new("arg_required")
        .no_binary_name(true)
        .arg(Arg::new("test").required(true).index(1))
        .try_get_matches_from(vec!["testing"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let matches = result.unwrap();
    assert_eq!(
        matches
            .get_one::<String>("test")
            .map(|v| v.as_str())
            .unwrap(),
        "testing"
    );
}

#[test]
fn skip_possible_values() {
    static SKIP_POS_VALS: &str = "\
tests stuff

Usage: test [OPTIONS] [arg1]

Arguments:
  [arg1]  some pos arg

Options:
  -o, --opt <opt>  some option
  -h, --help       Print help
  -V, --version    Print version
";

    let cmd = Command::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .hide_possible_values(true)
        .args([
            arg!(-o --opt <opt> "some option").value_parser(["one", "two"]),
            arg!([arg1] "some pos arg").value_parser(["three", "four"]),
        ]);

    utils::assert_output(cmd, "test --help", SKIP_POS_VALS, false);
}

#[test]
fn stop_delim_values_only_pos_follows() {
    let r = Command::new("onlypos")
        .dont_delimit_trailing_values(true)
        .args([arg!(f: -f <flag> "some opt"), arg!([arg] ... "some arg")])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert!(!m.contains_id("f"));
    assert_eq!(
        m.get_many::<String>("arg")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["-f", "-g,x"]
    );
}

#[test]
fn dont_delim_values_trailingvararg() {
    let m = Command::new("positional")
        .dont_delimit_trailing_values(true)
        .arg(arg!([opt] ... "some pos").trailing_var_arg(true))
        .try_get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"])
        .unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["test", "--foo", "-Wl,-bar"]
    );
}

#[test]
fn delim_values_only_pos_follows() {
    let r = Command::new("onlypos")
        .args([arg!(f: -f [flag] "some opt"), arg!([arg] ... "some arg")])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert!(!m.contains_id("f"));
    assert_eq!(
        m.get_many::<String>("arg")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["-f", "-g,x"]
    );
}

#[test]
fn delim_values_trailingvararg() {
    let m = Command::new("positional")
        .arg(arg!([opt] ... "some pos").trailing_var_arg(true))
        .try_get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"])
        .unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["test", "--foo", "-Wl,-bar"]
    );
}

#[test]
fn delim_values_only_pos_follows_with_delim() {
    let r = Command::new("onlypos")
        .args([
            arg!(f: -f [flag] "some opt"),
            arg!([arg] ... "some arg").value_delimiter(','),
        ])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert!(!m.contains_id("f"));
    assert_eq!(
        m.get_many::<String>("arg")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["-f", "-g", "x"]
    );
}

#[test]
fn delim_values_trailingvararg_with_delim() {
    let m = Command::new("positional")
        .arg(
            arg!([opt] ... "some pos")
                .value_delimiter(',')
                .trailing_var_arg(true),
        )
        .try_get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"])
        .unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["test", "--foo", "-Wl", "-bar"]
    );
}

#[test]
fn leading_hyphen_short() {
    let res = Command::new("leadhy")
        .arg(Arg::new("some").allow_hyphen_values(true))
        .arg(Arg::new("other").short('o').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert!(m.contains_id("some"));
    assert!(m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("some").map(|v| v.as_str()).unwrap(),
        "-bar"
    );
    assert_eq!(
        *m.get_one::<bool>("other").expect("defaulted by clap"),
        true
    );
}

#[test]
fn leading_hyphen_long() {
    let res = Command::new("leadhy")
        .arg(Arg::new("some").allow_hyphen_values(true))
        .arg(Arg::new("other").short('o').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert!(m.contains_id("some"));
    assert!(m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("some").map(|v| v.as_str()).unwrap(),
        "--bar"
    );
    assert_eq!(
        *m.get_one::<bool>("other").expect("defaulted by clap"),
        true
    );
}

#[test]
fn leading_hyphen_opt() {
    let res = Command::new("leadhy")
        .arg(
            Arg::new("some")
                .action(ArgAction::Set)
                .long("opt")
                .allow_hyphen_values(true),
        )
        .arg(Arg::new("other").short('o').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "--opt", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert!(m.contains_id("some"));
    assert!(m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("some").map(|v| v.as_str()).unwrap(),
        "--bar"
    );
    assert_eq!(
        *m.get_one::<bool>("other").expect("defaulted by clap"),
        true
    );
}

#[test]
fn allow_negative_numbers_success() {
    let res = Command::new("negnum")
        .arg(Arg::new("panum").allow_negative_numbers(true))
        .arg(
            Arg::new("onum")
                .short('o')
                .action(ArgAction::Set)
                .allow_negative_numbers(true),
        )
        .try_get_matches_from(vec!["negnum", "-20", "-o", "-1.2"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(
        m.get_one::<String>("panum").map(|v| v.as_str()).unwrap(),
        "-20"
    );
    assert_eq!(
        m.get_one::<String>("onum").map(|v| v.as_str()).unwrap(),
        "-1.2"
    );
}

#[test]
fn allow_negative_numbers_fail() {
    let res = Command::new("negnum")
        .arg(Arg::new("panum").allow_negative_numbers(true))
        .arg(
            Arg::new("onum")
                .short('o')
                .action(ArgAction::Set)
                .allow_negative_numbers(true),
        )
        .try_get_matches_from(vec!["negnum", "--foo", "-o", "-1.2"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument)
}

#[test]
fn trailing_var_arg_with_hyphen_values_escape_first() {
    assert_trailing_var_args(&["test", "--", "foo", "bar"], Some(&["foo", "bar"]), false);
}

#[test]
fn trailing_var_arg_with_hyphen_values_escape_middle() {
    assert_trailing_var_args(
        &["test", "foo", "--", "bar"],
        Some(&["foo", "--", "bar"]),
        false,
    );
}

#[test]
fn trailing_var_arg_with_hyphen_values_short_first() {
    assert_trailing_var_args(&["test", "-p", "foo", "bar"], Some(&["foo", "bar"]), true);
}

#[test]
fn trailing_var_arg_with_hyphen_values_short_middle() {
    assert_trailing_var_args(
        &["test", "foo", "-p", "bar"],
        Some(&["foo", "-p", "bar"]),
        false,
    );
}

#[test]
fn trailing_var_arg_with_hyphen_values_long_first() {
    assert_trailing_var_args(
        &["test", "--prog", "foo", "bar"],
        Some(&["foo", "bar"]),
        true,
    );
}

#[test]
fn trailing_var_arg_with_hyphen_values_long_middle() {
    assert_trailing_var_args(
        &["test", "foo", "--prog", "bar"],
        Some(&["foo", "--prog", "bar"]),
        false,
    );
}

#[track_caller]
fn assert_trailing_var_args(
    input: &[&str],
    expected_var_arg: Option<&[&str]>,
    expected_flag: bool,
) {
    let cmd = Command::new("test").arg(arg!(-p - -prog)).arg(
        arg!([opt] ... "some pos")
            .trailing_var_arg(true)
            .allow_hyphen_values(true),
    );
    let m = cmd.try_get_matches_from(input);
    assert!(
        m.is_ok(),
        "failed with args {:?}: {}",
        input,
        m.unwrap_err()
    );
    let m = m.unwrap();

    let actual_var_args = m
        .get_many::<String>("opt")
        .map(|v| v.map(|s| s.as_str()).collect::<Vec<_>>());
    assert_eq!(actual_var_args.as_deref(), expected_var_arg);
    assert_eq!(m.get_flag("prog"), expected_flag);
}

#[test]
fn disable_help_subcommand() {
    let result = Command::new("disablehelp")
        .disable_help_subcommand(true)
        .subcommand(Command::new("sub1"))
        .try_get_matches_from(vec!["", "help"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn dont_collapse_args() {
    let cmd = Command::new("clap-test").version("v1.4.8").args([
        Arg::new("arg1").help("some"),
        Arg::new("arg2").help("some"),
        Arg::new("arg3").help("some"),
    ]);
    utils::assert_output(cmd, "clap-test --help", DONT_COLLAPSE_ARGS, false);
}

#[test]
fn require_eq() {
    static REQUIRE_EQUALS: &str = "\
Usage: clap-test --opt=<FILE>

Options:
  -o, --opt=<FILE>  some
  -h, --help        Print help
  -V, --version     Print version
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
    utils::assert_output(cmd, "clap-test --help", REQUIRE_EQUALS, false);
}

#[test]
fn propagate_vals_down() {
    let m = Command::new("myprog")
        .arg(arg!([cmd] "command to run").global(true))
        .subcommand(Command::new("foo"))
        .try_get_matches_from(vec!["myprog", "set", "foo"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();
    assert_eq!(m.get_one::<String>("cmd").map(|v| v.as_str()), Some("set"));
    let sub_m = m.subcommand_matches("foo").unwrap();
    assert_eq!(
        sub_m.get_one::<String>("cmd").map(|v| v.as_str()),
        Some("set")
    );
}

#[test]
fn allow_missing_positional() {
    let m = Command::new("test")
        .allow_missing_positional(true)
        .arg(arg!([src] "some file").default_value("src"))
        .arg(arg!(<dest> "some file"))
        .try_get_matches_from(vec!["test", "file"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();
    assert_eq!(m.get_one::<String>("src").map(|v| v.as_str()), Some("src"));
    assert_eq!(
        m.get_one::<String>("dest").map(|v| v.as_str()),
        Some("file")
    );
}

#[test]
fn allow_missing_positional_no_default() {
    let m = Command::new("test")
        .allow_missing_positional(true)
        .arg(arg!([src] "some file"))
        .arg(arg!(<dest> "some file"))
        .try_get_matches_from(vec!["test", "file"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();
    assert_eq!(m.get_one::<String>("src").map(|v| v.as_str()), None);
    assert_eq!(
        m.get_one::<String>("dest").map(|v| v.as_str()),
        Some("file")
    );
}

#[test]
fn missing_positional_no_hyphen() {
    let r = Command::new("bench")
        .allow_missing_positional(true)
        .arg(arg!([BENCH] "some bench"))
        .arg(arg!([ARGS] ... "some args"))
        .try_get_matches_from(vec!["bench", "foo", "arg1", "arg2", "arg3"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());

    let m = r.unwrap();

    let expected_bench = Some("foo");
    let expected_args = vec!["arg1", "arg2", "arg3"];

    assert_eq!(
        m.get_one::<String>("BENCH").map(|v| v.as_str()),
        expected_bench
    );
    assert_eq!(
        m.get_many::<String>("ARGS")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &*expected_args
    );
}

#[test]
fn missing_positional_hyphen() {
    let r = Command::new("bench")
        .allow_missing_positional(true)
        .arg(arg!([BENCH] "some bench"))
        .arg(arg!([ARGS] ... "some args"))
        .try_get_matches_from(vec!["bench", "--", "arg1", "arg2", "arg3"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());

    let m = r.unwrap();

    let expected_bench = None;
    let expected_args = vec!["arg1", "arg2", "arg3"];

    assert_eq!(
        m.get_one::<String>("BENCH").map(|v| v.as_str()),
        expected_bench
    );
    assert_eq!(
        m.get_many::<String>("ARGS")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &*expected_args
    );
}

#[test]
fn missing_positional_hyphen_far_back() {
    let r = Command::new("bench")
        .allow_missing_positional(true)
        .arg(arg!([BENCH1] "some bench"))
        .arg(arg!([BENCH2] "some bench"))
        .arg(arg!([BENCH3] "some bench"))
        .arg(arg!([ARGS] ... "some args"))
        .try_get_matches_from(vec!["bench", "foo", "--", "arg1", "arg2", "arg3"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());

    let m = r.unwrap();

    let expected_bench1 = Some("foo");
    let expected_bench2 = None;
    let expected_bench3 = None;
    let expected_args = vec!["arg1", "arg2", "arg3"];

    assert_eq!(
        m.get_one::<String>("BENCH1").map(|v| v.as_str()),
        expected_bench1
    );
    assert_eq!(
        m.get_one::<String>("BENCH2").map(|v| v.as_str()),
        expected_bench2
    );
    assert_eq!(
        m.get_one::<String>("BENCH3").map(|v| v.as_str()),
        expected_bench3
    );
    assert_eq!(
        m.get_many::<String>("ARGS")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &*expected_args
    );
}

#[test]
fn missing_positional_hyphen_req_error() {
    let r = Command::new("bench")
        .allow_missing_positional(true)
        .arg(arg!([BENCH1] "some bench"))
        .arg(arg!(<BENCH2> "some bench"))
        .arg(arg!([ARGS] ... "some args"))
        .try_get_matches_from(vec!["bench", "foo", "--", "arg1", "arg2", "arg3"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_option() {
    let res = Command::new("prog")
        .arg(
            arg!(--"some-argument" <val>)
                .required(true)
                .allow_hyphen_values(true),
        )
        .try_get_matches_from(vec!["prog", "-fish"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn issue_1437_allow_hyphen_values_for_positional_arg() {
    let m = Command::new("tmp")
        .arg(
            Arg::new("pat")
                .allow_hyphen_values(true)
                .required(true)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(["tmp", "-file"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("pat").map(|v| v.as_str()),
        Some("-file")
    );
}

#[test]
fn issue_3880_allow_long_flag_hyphen_value_for_positional_arg() {
    let m = Command::new("prog")
        .arg(
            Arg::new("pat")
                .allow_hyphen_values(true)
                .required(true)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(["", "--file"])
        .unwrap();

    assert_eq!(
        m.get_one::<String>("pat").map(|v| v.as_str()),
        Some("--file")
    );
}

#[test]
fn issue_1093_allow_ext_sc() {
    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .allow_external_subcommands(true);
    utils::assert_output(cmd, "clap-test --help", ALLOW_EXT_SC, false);
}

#[test]
fn allow_ext_sc_empty_args() {
    let res = Command::new("clap-test")
        .version("v1.4.8")
        .allow_external_subcommands(true)
        .try_get_matches_from(vec!["clap-test", "external-cmd"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());

    match res.unwrap().subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "external-cmd");
            assert_eq!(
                args.get_many::<OsString>("").unwrap().collect::<Vec<_>>(),
                Vec::<&OsString>::new(),
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn allow_ext_sc_when_sc_required() {
    let res = Command::new("clap-test")
        .version("v1.4.8")
        .allow_external_subcommands(true)
        .subcommand_required(true)
        .try_get_matches_from(vec!["clap-test", "external-cmd", "foo"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());

    match res.unwrap().subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "external-cmd");
            assert_eq!(
                args.get_many::<OsString>("")
                    .unwrap()
                    .cloned()
                    .collect::<Vec<_>>(),
                vec![OsString::from("foo")]
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn external_subcommand_looks_like_built_in() {
    let res = Command::new("cargo")
        .version("1.26.0")
        .allow_external_subcommands(true)
        .subcommand(Command::new("install"))
        .try_get_matches_from(vec!["cargo", "install-update", "foo"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    match m.subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "install-update");
            assert_eq!(
                args.get_many::<OsString>("")
                    .unwrap()
                    .cloned()
                    .collect::<Vec<_>>(),
                vec![OsString::from("foo")]
            );
        }
        _ => panic!("external_subcommand didn't work"),
    }
}

#[test]
fn built_in_subcommand_escaped() {
    let res = Command::new("cargo")
        .version("1.26.0")
        .allow_external_subcommands(true)
        .subcommand(Command::new("install"))
        .try_get_matches_from(vec!["cargo", "--", "install", "foo"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    match m.subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "install");
            assert_eq!(
                args.get_many::<OsString>("")
                    .unwrap()
                    .cloned()
                    .collect::<Vec<_>>(),
                vec![OsString::from("foo")]
            );
        }
        _ => panic!("external_subcommand didn't work"),
    }
}

#[test]
fn aaos_opts_w_other_overrides() {
    // opts with other overrides
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--opt <val> "some option").action(ArgAction::Set))
        .arg(
            arg!(--other <val> "some other option")
                .overrides_with("opt")
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--other=test", "--opt=other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("opt"));
    assert!(!m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("other")
    );
}

#[test]
fn aaos_opts_w_other_overrides_rev() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--other <val> "some other option")
                .required(true)
                .overrides_with("opt")
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--other=val"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.contains_id("opt"));
    assert!(m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("other").map(|v| v.as_str()),
        Some("val")
    );
}

#[test]
fn aaos_opts_w_other_overrides_2() {
    // opts with other overrides
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .overrides_with("other")
                .action(ArgAction::Set),
        )
        .arg(arg!(--other <val> "some other option").action(ArgAction::Set))
        .try_get_matches_from(vec!["", "--opt=some", "--other=test", "--opt=other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("opt"));
    assert!(!m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("other")
    );
}

#[test]
fn aaos_opts_w_other_overrides_rev_2() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .overrides_with("other")
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--other <val> "some other option")
                .required(true)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--other=val"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.contains_id("opt"));
    assert!(m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("other").map(|v| v.as_str()),
        Some("val")
    );
}

#[test]
fn aaos_opts_w_override_as_conflict_1() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .overrides_with("other")
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--other <val> "some other option")
                .required(true)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "--opt=some"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("opt"));
    assert!(!m.contains_id("other"));
    assert_eq!(m.get_one::<String>("opt").map(|v| v.as_str()), Some("some"));
}

#[test]
fn aaos_opts_w_override_as_conflict_2() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .overrides_with("other")
                .action(ArgAction::Set),
        )
        .arg(
            arg!(--other <val> "some other option")
                .required(true)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "--other=some"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.contains_id("opt"));
    assert!(m.contains_id("other"));
    assert_eq!(
        m.get_one::<String>("other").map(|v| v.as_str()),
        Some("some")
    );
}

#[test]
fn aaos_opts_mult_req_delims() {
    // opts with multiple and require delims
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> ... "some option")
                .action(ArgAction::Set)
                .value_delimiter(',')
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["some", "other", "one", "two"]
    );
}

#[test]
fn aaos_opts_mult() {
    // opts with multiple
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> ... "some option")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "",
            "--opt",
            "first",
            "overrides",
            "--opt",
            "some",
            "other",
            "val",
        ]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["first", "overrides", "some", "other", "val"]
    );
}

#[test]
fn aaos_pos_mult() {
    // opts with multiple
    let res = Command::new("posix")
        .arg(arg!([val] ... "some pos"))
        .try_get_matches_from(vec!["", "some", "other", "value"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("val"));
    assert_eq!(
        m.get_many::<String>("val")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["some", "other", "value"]
    );
}

#[test]
fn aaos_option_use_delim_false() {
    let m = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "--opt=some,other", "--opt=one,two"])
        .unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["one,two"]
    );
}

#[test]
#[cfg(feature = "color")]
fn color_is_global() {
    let mut cmd = Command::new("myprog")
        .color(clap::ColorChoice::Never)
        .subcommand(Command::new("foo"));
    cmd.build();
    assert_eq!(cmd.get_color(), clap::ColorChoice::Never);

    let sub = cmd.get_subcommands().collect::<Vec<_>>()[0];
    assert_eq!(sub.get_color(), clap::ColorChoice::Never);
}
