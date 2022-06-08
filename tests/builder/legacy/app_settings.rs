use super::utils;

use clap::{arg, error::ErrorKind, AppSettings, Arg, Command};

static ALLOW_EXT_SC: &str = "clap-test v1.4.8

USAGE:
    clap-test [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static DONT_COLLAPSE_ARGS: &str = "clap-test v1.4.8

USAGE:
    clap-test [arg1] [arg2] [arg3]

ARGS:
    <arg1>    some
    <arg2>    some
    <arg3>    some

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static REQUIRE_EQUALS: &str = "clap-test v1.4.8

USAGE:
    clap-test --opt=<FILE>

OPTIONS:
    -h, --help          Print help information
    -o, --opt=<FILE>    some
    -V, --version       Print version information
";

static SKIP_POS_VALS: &str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test [OPTIONS] [arg1]

ARGS:
    <arg1>    some pos arg

OPTIONS:
    -h, --help         Print help information
    -o, --opt <opt>    some option
    -V, --version      Print version information
";

static ARG_REQUIRED_ELSE_HELP: &str = "test 1.0

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -i, --info       Provides more info
    -V, --version    Print version information
";

static SUBCOMMAND_REQUIRED_ELSE_HELP: &str = "test 1.0

USAGE:
    test <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    info    
";

#[test]
fn setting() {
    #![allow(deprecated)]
    let m = Command::new("setting").args_override_self(true);
    assert!(m.is_set(AppSettings::AllArgsOverrideSelf));
}

#[test]
fn global_setting() {
    #![allow(deprecated)]
    let m = Command::new("global_setting").args_override_self(true);
    assert!(m.is_set(AppSettings::AllArgsOverrideSelf));
}

#[test]
fn unset_setting() {
    #![allow(deprecated)]
    let m = Command::new("unset_setting").args_override_self(true);
    assert!(m.is_set(AppSettings::AllArgsOverrideSelf));

    let m = m.args_override_self(false);
    assert!(!m.is_set(AppSettings::AllArgsOverrideSelf), "{:#?}", m);
}

#[test]
fn unset_global_setting() {
    #![allow(deprecated)]
    let m = Command::new("unset_global_setting").args_override_self(true);
    assert!(m.is_set(AppSettings::AllArgsOverrideSelf));

    let m = m.args_override_self(false);
    assert!(!m.is_set(AppSettings::AllArgsOverrideSelf), "{:#?}", m);
}

#[test]
fn setting_bitor() {
    #![allow(deprecated)]
    let m = Command::new("setting_bitor").setting(
        AppSettings::InferSubcommands | AppSettings::Hidden | AppSettings::DisableHelpSubcommand,
    );

    assert!(m.is_set(AppSettings::InferSubcommands));
    assert!(m.is_set(AppSettings::Hidden));
    assert!(m.is_set(AppSettings::DisableHelpSubcommand));
}

#[test]
fn unset_setting_bitor() {
    #![allow(deprecated)]
    let m = Command::new("unset_setting_bitor")
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::Hidden)
        .setting(AppSettings::DisableHelpSubcommand);

    assert!(m.is_set(AppSettings::InferSubcommands));
    assert!(m.is_set(AppSettings::Hidden));
    assert!(m.is_set(AppSettings::DisableHelpSubcommand));

    let m = m.unset_setting(
        AppSettings::InferSubcommands | AppSettings::Hidden | AppSettings::DisableHelpSubcommand,
    );
    assert!(!m.is_set(AppSettings::InferSubcommands), "{:#?}", m);
    assert!(!m.is_set(AppSettings::Hidden), "{:#?}", m);
    assert!(!m.is_set(AppSettings::DisableHelpSubcommand), "{:#?}", m);
}

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
        .arg(arg!(--input <PATH>).required(false).default_value("-"))
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
    let cmd = Command::new("test")
        .arg_required_else_help(true)
        .version("1.0")
        .arg(
            Arg::new("info")
                .help("Provides more info")
                .short('i')
                .long("info"),
        );
    utils::assert_output(
        cmd,
        "test",
        ARG_REQUIRED_ELSE_HELP,
        true, // Unlike normal displaying of help, we should provide a fatal exit code
    );
}

#[test]
fn subcommand_required_else_help() {
    #![allow(deprecated)]
    let result = Command::new("test")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(Command::new("info"))
        .try_get_matches_from(&[""]);

    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn subcommand_required_else_help_error_message() {
    #![allow(deprecated)]
    let cmd = Command::new("test")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("1.0")
        .subcommand(Command::new("info").arg(Arg::new("filename")));
    utils::assert_output(
        cmd,
        "test",
        SUBCOMMAND_REQUIRED_ELSE_HELP,
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
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnrecognizedSubcommand);
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
    assert_eq!(m.unwrap().value_of("some"), Some("t"));
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
    assert_eq!(m.unwrap().value_of("some"), Some("te"));
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
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnrecognizedSubcommand);
}

#[test]
fn no_bin_name() {
    let result = Command::new("arg_required")
        .no_binary_name(true)
        .arg(Arg::new("test").required(true).index(1))
        .try_get_matches_from(vec!["testing"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let matches = result.unwrap();
    assert_eq!(matches.value_of("test").unwrap(), "testing");
}

#[test]
fn skip_possible_values() {
    let cmd = Command::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .hide_possible_values(true)
        .args(&[
            arg!(-o --opt <opt> "some option")
                .required(false)
                .possible_values(["one", "two"]),
            arg!([arg1] "some pos arg").possible_values(["three", "four"]),
        ]);

    utils::assert_output(cmd, "test --help", SKIP_POS_VALS, false);
}

#[test]
fn stop_delim_values_only_pos_follows() {
    let r = Command::new("onlypos")
        .dont_delimit_trailing_values(true)
        .args(&[
            arg!(f: -f <flag> "some opt").required(false),
            arg!([arg] ... "some arg"),
        ])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(
        m.values_of("arg").unwrap().collect::<Vec<_>>(),
        &["-f", "-g,x"]
    );
}

#[test]
fn dont_delim_values_trailingvararg() {
    let m = Command::new("positional")
        .trailing_var_arg(true)
        .dont_delimit_trailing_values(true)
        .arg(arg!([opt] ... "some pos"))
        .try_get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"])
        .unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["test", "--foo", "-Wl,-bar"]
    );
}

#[test]
fn delim_values_only_pos_follows() {
    let r = Command::new("onlypos")
        .args(&[arg!(f: -f [flag] "some opt"), arg!([arg] ... "some arg")])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(
        m.values_of("arg").unwrap().collect::<Vec<_>>(),
        &["-f", "-g,x"]
    );
}

#[test]
fn delim_values_trailingvararg() {
    let m = Command::new("positional")
        .trailing_var_arg(true)
        .arg(arg!([opt] ... "some pos"))
        .try_get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"])
        .unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["test", "--foo", "-Wl,-bar"]
    );
}

#[test]
fn delim_values_only_pos_follows_with_delim() {
    let r = Command::new("onlypos")
        .args(&[
            arg!(f: -f [flag] "some opt"),
            arg!([arg] ... "some arg").use_value_delimiter(true),
        ])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(
        m.values_of("arg").unwrap().collect::<Vec<_>>(),
        &["-f", "-g", "x"]
    );
}

#[test]
fn delim_values_trailingvararg_with_delim() {
    let m = Command::new("positional")
        .trailing_var_arg(true)
        .arg(arg!([opt] ... "some pos").use_value_delimiter(true))
        .try_get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"])
        .unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["test", "--foo", "-Wl", "-bar"]
    );
}

#[test]
fn leading_hyphen_short() {
    let res = Command::new("leadhy")
        .allow_hyphen_values(true)
        .arg(Arg::new("some"))
        .arg(Arg::new("other").short('o'))
        .try_get_matches_from(vec!["", "-bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "-bar");
}

#[test]
fn leading_hyphen_long() {
    let res = Command::new("leadhy")
        .allow_hyphen_values(true)
        .arg(Arg::new("some"))
        .arg(Arg::new("other").short('o'))
        .try_get_matches_from(vec!["", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn leading_hyphen_opt() {
    let res = Command::new("leadhy")
        .allow_hyphen_values(true)
        .arg(Arg::new("some").takes_value(true).long("opt"))
        .arg(Arg::new("other").short('o'))
        .try_get_matches_from(vec!["", "--opt", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn allow_negative_numbers() {
    let res = Command::new("negnum")
        .allow_negative_numbers(true)
        .arg(Arg::new("panum"))
        .arg(Arg::new("onum").short('o').takes_value(true))
        .try_get_matches_from(vec!["negnum", "-20", "-o", "-1.2"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(m.value_of("panum").unwrap(), "-20");
    assert_eq!(m.value_of("onum").unwrap(), "-1.2");
}

#[test]
fn allow_negative_numbers_fail() {
    let res = Command::new("negnum")
        .allow_negative_numbers(true)
        .arg(Arg::new("panum"))
        .arg(Arg::new("onum").short('o').takes_value(true))
        .try_get_matches_from(vec!["negnum", "--foo", "-o", "-1.2"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument)
}

#[test]
fn leading_double_hyphen_trailingvararg() {
    let m = Command::new("positional")
        .trailing_var_arg(true)
        .allow_hyphen_values(true)
        .arg(arg!([opt] ... "some pos"))
        .try_get_matches_from(vec!["", "--foo", "-Wl", "bar"])
        .unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["--foo", "-Wl", "bar"]
    );
}

#[test]
fn disable_help_subcommand() {
    let result = Command::new("disablehelp")
        .disable_help_subcommand(true)
        .subcommand(Command::new("sub1"))
        .try_get_matches_from(vec!["", "help"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn dont_collapse_args() {
    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .dont_collapse_args_in_usage(true)
        .args(&[
            Arg::new("arg1").help("some"),
            Arg::new("arg2").help("some"),
            Arg::new("arg3").help("some"),
        ]);
    utils::assert_output(cmd, "clap-test --help", DONT_COLLAPSE_ARGS, false);
}

#[test]
fn require_eq() {
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
fn args_negate_subcommands_one_level() {
    let res = Command::new("disablehelp")
        .args_conflicts_with_subcommands(true)
        .subcommand_negates_reqs(true)
        .arg(arg!(<arg1> "some arg"))
        .arg(arg!(<arg2> "some arg"))
        .subcommand(
            Command::new("sub1").subcommand(Command::new("sub2").subcommand(Command::new("sub3"))),
        )
        .try_get_matches_from(vec!["", "pickles", "sub1"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(m.value_of("arg2"), Some("sub1"));
}

#[test]
fn args_negate_subcommands_two_levels() {
    let res = Command::new("disablehelp")
        .args_conflicts_with_subcommands(true)
        .subcommand_negates_reqs(true)
        .arg(arg!(<arg1> "some arg"))
        .arg(arg!(<arg2> "some arg"))
        .subcommand(
            Command::new("sub1")
                .args_conflicts_with_subcommands(true)
                .subcommand_negates_reqs(true)
                .arg(arg!(<arg> "some"))
                .arg(arg!(<arg2> "some"))
                .subcommand(Command::new("sub2").subcommand(Command::new("sub3"))),
        )
        .try_get_matches_from(vec!["", "sub1", "arg", "sub2"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(
        m.subcommand_matches("sub1").unwrap().value_of("arg2"),
        Some("sub2")
    );
}

#[test]
fn propagate_vals_down() {
    let m = Command::new("myprog")
        .arg(arg!([cmd] "command to run").global(true))
        .subcommand(Command::new("foo"))
        .try_get_matches_from(vec!["myprog", "set", "foo"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();
    assert_eq!(m.value_of("cmd"), Some("set"));
    let sub_m = m.subcommand_matches("foo").unwrap();
    assert_eq!(sub_m.value_of("cmd"), Some("set"));
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
    assert_eq!(m.value_of("src"), Some("src"));
    assert_eq!(m.value_of("dest"), Some("file"));
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
    assert_eq!(m.value_of("src"), None);
    assert_eq!(m.value_of("dest"), Some("file"));
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

    assert_eq!(m.value_of("BENCH"), expected_bench);
    assert_eq!(
        m.values_of("ARGS").unwrap().collect::<Vec<_>>(),
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

    assert_eq!(m.value_of("BENCH"), expected_bench);
    assert_eq!(
        m.values_of("ARGS").unwrap().collect::<Vec<_>>(),
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

    assert_eq!(m.value_of("BENCH1"), expected_bench1);
    assert_eq!(m.value_of("BENCH2"), expected_bench2);
    assert_eq!(m.value_of("BENCH3"), expected_bench3);
    assert_eq!(
        m.values_of("ARGS").unwrap().collect::<Vec<_>>(),
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
fn issue_1066_allow_leading_hyphen_and_unknown_args() {
    let res = Command::new("prog")
        .allow_hyphen_values(true)
        .arg(arg!(--"some-argument"))
        .try_get_matches_from(vec!["prog", "hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_no_vals() {
    let res = Command::new("prog")
        .allow_hyphen_values(true)
        .arg(arg!(--"some-argument"))
        .try_get_matches_from(vec!["prog", "--hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_option() {
    let res = Command::new("prog")
        .allow_hyphen_values(true)
        .arg(arg!(--"some-argument" <val>))
        .try_get_matches_from(vec!["prog", "-hello"]);

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
                .takes_value(true),
        )
        .try_get_matches_from(["tmp", "-file"])
        .unwrap();
    assert_eq!(m.value_of("pat"), Some("-file"));
}

#[test]
fn issue_1093_allow_ext_sc() {
    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .allow_external_subcommands(true);
    utils::assert_output(cmd, "clap-test --help", ALLOW_EXT_SC, false);
}

#[test]
#[cfg(not(feature = "unstable-v4"))]
fn allow_ext_sc_empty_args() {
    let res = Command::new("clap-test")
        .version("v1.4.8")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .try_get_matches_from(vec!["clap-test", "external-cmd"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());

    match res.unwrap().subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "external-cmd");
            assert_eq!(args.values_of_lossy(""), None);
        }
        _ => unreachable!(),
    }
}

#[test]
#[cfg(feature = "unstable-v4")]
fn allow_ext_sc_empty_args() {
    let res = Command::new("clap-test")
        .version("v1.4.8")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .try_get_matches_from(vec!["clap-test", "external-cmd"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());

    match res.unwrap().subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "external-cmd");
            assert_eq!(args.values_of_lossy(""), Some(vec![]));
        }
        _ => unreachable!(),
    }
}

#[test]
fn allow_ext_sc_when_sc_required() {
    let res = Command::new("clap-test")
        .version("v1.4.8")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand_required(true)
        .try_get_matches_from(vec!["clap-test", "external-cmd", "foo"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());

    match res.unwrap().subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "external-cmd");
            assert_eq!(args.values_of_lossy(""), Some(vec!["foo".to_string()]));
        }
        _ => unreachable!(),
    }
}

#[test]
fn external_subcommand_looks_like_built_in() {
    let res = Command::new("cargo")
        .version("1.26.0")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(Command::new("install"))
        .try_get_matches_from(vec!["cargo", "install-update", "foo"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    match m.subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "install-update");
            assert_eq!(args.values_of_lossy(""), Some(vec!["foo".to_string()]));
        }
        _ => panic!("external_subcommand didn't work"),
    }
}

#[test]
fn built_in_subcommand_escaped() {
    let res = Command::new("cargo")
        .version("1.26.0")
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(Command::new("install"))
        .try_get_matches_from(vec!["cargo", "--", "install", "foo"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    match m.subcommand() {
        Some((name, args)) => {
            assert_eq!(name, "install");
            assert_eq!(args.values_of_lossy(""), Some(vec!["foo".to_string()]));
        }
        _ => panic!("external_subcommand didn't work"),
    }
}

#[test]
fn aaos_flags() {
    // flags
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--flag  "some flag"))
        .try_get_matches_from(vec!["", "--flag", "--flag"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn aaos_flags_mult() {
    // flags with multiple
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--flag ...  "some flag"))
        .try_get_matches_from(vec!["", "--flag", "--flag", "--flag", "--flag"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 4);
}

#[test]
fn aaos_opts() {
    // opts
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--opt <val> "some option"))
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn aaos_opts_w_other_overrides() {
    // opts with other overrides
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--opt <val> "some option").required(false))
        .arg(
            arg!(--other <val> "some other option")
                .required(false)
                .overrides_with("opt"),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--other=test", "--opt=other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert!(!m.is_present("other"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn aaos_opts_w_other_overrides_rev() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--opt <val> "some option").required(true))
        .arg(
            arg!(--other <val> "some other option")
                .required(true)
                .overrides_with("opt"),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--other=val"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.is_present("opt"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("other"), Some("val"));
}

#[test]
fn aaos_opts_w_other_overrides_2() {
    // opts with other overrides
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .required(false)
                .overrides_with("other"),
        )
        .arg(arg!(--other <val> "some other option").required(false))
        .try_get_matches_from(vec!["", "--opt=some", "--other=test", "--opt=other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert!(!m.is_present("other"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn aaos_opts_w_other_overrides_rev_2() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .overrides_with("other"),
        )
        .arg(arg!(--other <val> "some other option").required(true))
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--other=val"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.is_present("opt"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("other"), Some("val"));
}

#[test]
fn aaos_opts_w_override_as_conflict_1() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .overrides_with("other"),
        )
        .arg(arg!(--other <val> "some other option").required(true))
        .try_get_matches_from(vec!["", "--opt=some"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert!(!m.is_present("other"));
    assert_eq!(m.value_of("opt"), Some("some"));
}

#[test]
fn aaos_opts_w_override_as_conflict_2() {
    // opts with other overrides, rev
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> "some option")
                .required(true)
                .overrides_with("other"),
        )
        .arg(arg!(--other <val> "some other option").required(true))
        .try_get_matches_from(vec!["", "--other=some"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.is_present("opt"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("other"), Some("some"));
}

#[test]
fn aaos_opts_mult() {
    // opts with multiple
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> ... "some option")
                .number_of_values(1)
                .takes_value(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 3);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["some", "other", "one", "two"]
    );
}

#[test]
fn aaos_opts_mult_req_delims() {
    // opts with multiple and require delims
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--opt <val> ... "some option").multiple_values(true))
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
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 2);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["first", "overrides", "some", "other", "val"]
    );
}

#[test]
fn aaos_pos_mult() {
    // opts with multiple
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(arg!([val] ... "some pos"))
        .try_get_matches_from(vec!["", "some", "other", "value"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("val"));
    assert_eq!(m.occurrences_of("val"), 3);
    assert_eq!(
        m.values_of("val").unwrap().collect::<Vec<_>>(),
        &["some", "other", "value"]
    );
}

#[test]
fn aaos_option_use_delim_false() {
    let m = Command::new("posix")
        .args_override_self(true)
        .arg(arg!(--opt <val> "some option").use_value_delimiter(false))
        .try_get_matches_from(vec!["", "--opt=some,other", "--opt=one,two"])
        .unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["one,two"]
    );
}

#[test]
fn no_auto_help() {
    let cmd = Command::new("myprog")
        .setting(AppSettings::NoAutoHelp)
        .subcommand(Command::new("foo"));

    let result = cmd.clone().try_get_matches_from("myprog --help".split(' '));

    assert!(result.is_ok(), "{}", result.unwrap_err());
    assert!(result.unwrap().is_present("help"));

    let result = cmd.clone().try_get_matches_from("myprog -h".split(' '));

    assert!(result.is_ok(), "{}", result.unwrap_err());
    assert!(result.unwrap().is_present("help"));

    let result = cmd.clone().try_get_matches_from("myprog help".split(' '));

    assert!(result.is_ok(), "{}", result.unwrap_err());
    assert_eq!(result.unwrap().subcommand_name(), Some("help"));
}

#[test]
fn no_auto_version() {
    let cmd = Command::new("myprog")
        .version("3.0")
        .setting(AppSettings::NoAutoVersion);

    let result = cmd
        .clone()
        .try_get_matches_from("myprog --version".split(' '));

    assert!(result.is_ok(), "{}", result.unwrap_err());
    assert!(result.unwrap().is_present("version"));

    let result = cmd.clone().try_get_matches_from("myprog -V".split(' '));

    assert!(result.is_ok(), "{}", result.unwrap_err());
    assert!(result.unwrap().is_present("version"));
}

#[test]
fn no_auto_version_mut_arg() {
    let cmd = Command::new("myprog")
        .version("3.0")
        .mut_arg("version", |v| v.help("custom help"))
        .setting(AppSettings::NoAutoVersion);

    let result = cmd
        .clone()
        .try_get_matches_from("myprog --version".split(' '));

    assert!(result.is_ok(), "{}", result.unwrap_err());
    assert!(result.unwrap().is_present("version"));

    let result = cmd.clone().try_get_matches_from("myprog -V".split(' '));

    assert!(result.is_ok(), "{}", result.unwrap_err());
    assert!(result.unwrap().is_present("version"));
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
