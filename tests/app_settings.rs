extern crate clap;
extern crate regex;

use clap::{App, AppSettings, Arg, ErrorKind, Propagation};

include!("../clap-test.rs");

static ALLOW_EXT_SC: &'static str = "clap-test v1.4.8

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static DONT_COLLAPSE_ARGS: &'static str = "clap-test v1.4.8

USAGE:
    clap-test [arg1] [arg2] [arg3]

ARGS:
    <arg1>    some
    <arg2>    some
    <arg3>    some

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static REQUIRE_EQUALS: &'static str = "clap-test v1.4.8

USAGE:
    clap-test --opt=<FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt=<FILE>    some";

static UNIFIED_HELP: &'static str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test [OPTIONS] [arg1]

ARGS:
    <arg1>    some pos arg

OPTIONS:
    -f, --flag            some flag
    -h, --help            Prints help information
        --option <opt>    some option
    -V, --version         Prints version information";

static SKIP_POS_VALS: &'static str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test [OPTIONS] [arg1]

ARGS:
    <arg1>    some pos arg

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <opt>    some option";

#[test]
fn sub_command_negate_required() {
    App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("test").required(true).index(1))
        .subcommand(App::new("sub1"))
        .get_matches_from(vec!["myprog", "sub1"]);
}

#[test]
fn global_version() {
    let mut app = App::new("global_version")
        .setting(AppSettings::GlobalVersion)
        .version("1.1")
        .subcommand(App::new("sub1"));
    app._propagate(Propagation::NextLevel);
    assert_eq!(app.subcommands[0].version, Some("1.1"));
}

#[test]
fn sub_command_negate_required_2() {
    let result = App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("test").required(true).index(1))
        .subcommand(App::new("sub1"))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn sub_command_required() {
    let result = App::new("sc_required")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(App::new("sub1"))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingSubcommand);
}

#[test]
fn arg_required_else_help() {
    let result = App::new("arg_required")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("test").index(1))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingArgumentOrSubcommand);
}

#[test]
fn arg_required_else_help_over_reqs() {
    let result = App::new("arg_required")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("test").index(1).required(true))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingArgumentOrSubcommand);
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .try_get_matches_from(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .try_get_matches_from(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidSubcommand);
}

#[test]
fn infer_subcommands_fail_with_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .arg(Arg::with_name("some"))
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .try_get_matches_from(vec!["prog", "t"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert_eq!(m.unwrap().value_of("some"), Some("t"));
}

#[test]
fn infer_subcommands_fail_with_args2() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .arg(Arg::with_name("some"))
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .try_get_matches_from(vec!["prog", "te"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert_eq!(m.unwrap().value_of("some"), Some("te"));
}

#[test]
fn infer_subcommands_pass() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .get_matches_from(vec!["prog", "te"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn infer_subcommands_pass_close() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .get_matches_from(vec!["prog", "tes"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .try_get_matches_from(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidSubcommand);
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"))
        .try_get_matches_from(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
}

#[test]
fn no_bin_name() {
    let result = App::new("arg_required")
        .setting(AppSettings::NoBinaryName)
        .arg(Arg::with_name("test").required(true).index(1))
        .try_get_matches_from(vec!["testing"]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert_eq!(matches.value_of("test").unwrap(), "testing");
}

#[test]
fn unified_help() {
    let app = App::new("myTest")
        .name("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .setting(AppSettings::UnifiedHelpMessage)
        .arg("-f, --flag 'some flag'")
        .arg("[arg1] 'some pos arg'")
        .arg("--option [opt] 'some option'");

    assert!(test::compare_output(
        app,
        "test --help",
        UNIFIED_HELP,
        false
    ));
}

#[test]
fn skip_possible_values() {
    let app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .setting(AppSettings::HidePossibleValuesInHelp)
        .args(&[
            Arg::from("-o, --opt [opt] 'some option'").possible_values(&["one", "two"]),
            Arg::from("[arg1] 'some pos arg'").possible_values(&["three", "four"]),
        ]);

    assert!(test::compare_output(
        app,
        "test --help",
        SKIP_POS_VALS,
        false
    ));
}

#[test]
fn global_setting() {
    let mut app = App::new("test")
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(App::new("subcmd"));
    app._propagate(Propagation::NextLevel);
    assert!(app
        .subcommands
        .iter()
        .filter(|s| s.name == "subcmd")
        .next()
        .unwrap()
        .is_set(AppSettings::ColoredHelp));
}

#[test]
fn global_settings() {
    let mut app = App::new("test")
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::TrailingVarArg)
        .subcommand(App::new("subcmd"));
    app._propagate(Propagation::NextLevel);
    assert!(app
        .subcommands
        .iter()
        .filter(|s| s.name == "subcmd")
        .next()
        .unwrap()
        .is_set(AppSettings::ColoredHelp));
    assert!(app
        .subcommands
        .iter()
        .filter(|s| s.name == "subcmd")
        .next()
        .unwrap()
        .is_set(AppSettings::TrailingVarArg));
}

#[test]
fn stop_delim_values_only_pos_follows() {
    let r = App::new("onlypos")
        .setting(AppSettings::DontDelimitTrailingValues)
        .args(&[
            Arg::from("-f [flag] 'some opt'"),
            Arg::from("[arg]... 'some arg'"),
        ])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
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
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::DontDelimitTrailingValues)
        .arg(Arg::from("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["test", "--foo", "-Wl,-bar"]
    );
}

#[test]
fn delim_values_only_pos_follows() {
    let r = App::new("onlypos")
        .args(&[
            Arg::from("-f [flag] 'some opt'"),
            Arg::from("[arg]... 'some arg'"),
        ])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
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
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::from("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["test", "--foo", "-Wl,-bar"]
    );
}

#[test]
fn delim_values_only_pos_follows_with_delim() {
    let r = App::new("onlypos")
        .args(&[
            Arg::from("-f [flag] 'some opt'"),
            Arg::from("[arg]... 'some arg'").use_delimiter(true),
        ])
        .try_get_matches_from(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
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
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::from("[opt]... 'some pos'").use_delimiter(true))
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["test", "--foo", "-Wl", "-bar"]
    );
}

#[test]
fn leading_hyphen_short() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("some"))
        .arg(Arg::with_name("other").short('o'))
        .try_get_matches_from(vec!["", "-bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "-bar");
}

#[test]
fn leading_hyphen_long() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("some"))
        .arg(Arg::with_name("other").short('o'))
        .try_get_matches_from(vec!["", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn leading_hyphen_opt() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("some").takes_value(true).long("opt"))
        .arg(Arg::with_name("other").short('o'))
        .try_get_matches_from(vec!["", "--opt", "--bar", "-o"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("some").unwrap(), "--bar");
}

#[test]
fn allow_negative_numbers() {
    let res = App::new("negnum")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(Arg::with_name("panum"))
        .arg(Arg::with_name("onum").short('o').takes_value(true))
        .try_get_matches_from(vec!["negnum", "-20", "-o", "-1.2"]);
    assert!(res.is_ok(), "Error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("panum").unwrap(), "-20");
    assert_eq!(m.value_of("onum").unwrap(), "-1.2");
}

#[test]
fn allow_negative_numbers_fail() {
    let res = App::new("negnum")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(Arg::with_name("panum"))
        .arg(Arg::with_name("onum").short('o').takes_value(true))
        .try_get_matches_from(vec!["negnum", "--foo", "-o", "-1.2"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument)
}

#[test]
fn leading_double_hyphen_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "--foo", "-Wl", "bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["--foo", "-Wl", "bar"]
    );
}

#[test]
fn unset_setting() {
    let m = App::new("unset_setting").setting(AppSettings::AllArgsOverrideSelf);
    assert!(m.is_set(AppSettings::AllArgsOverrideSelf));

    let m = m.unset_setting(AppSettings::AllArgsOverrideSelf);
    assert!(!m.is_set(AppSettings::AllArgsOverrideSelf));
}

#[test]
fn unset_settings() {
    let m = App::new("unset_settings");
    assert!(&m.is_set(AppSettings::AllowInvalidUtf8));
    assert!(&m.is_set(AppSettings::ColorAuto));

    let m = m
        .unset_setting(AppSettings::AllowInvalidUtf8)
        .unset_setting(AppSettings::ColorAuto);
    assert!(
        !m.is_set(AppSettings::AllowInvalidUtf8),
        "l: {:?}\ng:{:?}",
        m.settings,
        m.g_settings
    );
    assert!(!m.is_set(AppSettings::ColorAuto));
}

#[test]
fn disable_help_subcommand() {
    let result = App::new("disablehelp")
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(App::new("sub1"))
        .try_get_matches_from(vec!["", "help"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn dont_collapse_args() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .setting(AppSettings::DontCollapseArgsInUsage)
        .args(&[
            Arg::with_name("arg1").help("some"),
            Arg::with_name("arg2").help("some"),
            Arg::with_name("arg3").help("some"),
        ]);
    assert!(test::compare_output(
        app,
        "clap-test --help",
        DONT_COLLAPSE_ARGS,
        false
    ));
}

#[test]
fn require_eq() {
    let app = App::new("clap-test").version("v1.4.8").arg(
        Arg::with_name("opt")
            .long("opt")
            .short('o')
            .required(true)
            .require_equals(true)
            .value_name("FILE")
            .help("some"),
    );
    assert!(test::compare_output(
        app,
        "clap-test --help",
        REQUIRE_EQUALS,
        false
    ));
}

#[test]
fn args_negate_subcommands_one_level() {
    let res = App::new("disablehelp")
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg("<arg1> 'some arg'")
        .arg("<arg2> 'some arg'")
        .subcommand(App::new("sub1").subcommand(App::new("sub2").subcommand(App::new("sub3"))))
        .try_get_matches_from(vec!["", "pickles", "sub1"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("arg2"), Some("sub1"));
}

#[test]
fn args_negate_subcommands_two_levels() {
    let res = App::new("disablehelp")
        .global_setting(AppSettings::ArgsNegateSubcommands)
        .global_setting(AppSettings::SubcommandsNegateReqs)
        .arg("<arg1> 'some arg'")
        .arg("<arg2> 'some arg'")
        .subcommand(
            App::new("sub1")
                .arg("<arg> 'some'")
                .arg("<arg2> 'some'")
                .subcommand(App::new("sub2").subcommand(App::new("sub3"))),
        )
        .try_get_matches_from(vec!["", "sub1", "arg", "sub2"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(
        m.subcommand_matches("sub1").unwrap().value_of("arg2"),
        Some("sub2")
    );
}

#[test]
fn propagate_vals_down() {
    let m = App::new("myprog")
        .arg(Arg::from("[cmd] 'command to run'").global(true))
        .subcommand(App::new("foo"))
        .try_get_matches_from(vec!["myprog", "set", "foo"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();
    assert_eq!(m.value_of("cmd"), Some("set"));
    let sub_m = m.subcommand_matches("foo").unwrap();
    assert_eq!(sub_m.value_of("cmd"), Some("set"));
}

#[test]
fn allow_missing_positional() {
    let m = App::new("test")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::from("[src] 'some file'").default_value("src"))
        .arg("<dest> 'some file'")
        .try_get_matches_from(vec!["test", "file"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();
    assert_eq!(m.value_of("src"), Some("src"));
    assert_eq!(m.value_of("dest"), Some("file"));
}

#[test]
fn allow_missing_positional_no_default() {
    let m = App::new("test")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::from("[src] 'some file'"))
        .arg("<dest> 'some file'")
        .try_get_matches_from(vec!["test", "file"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();
    assert_eq!(m.value_of("src"), None);
    assert_eq!(m.value_of("dest"), Some("file"));
}

#[test]
fn missing_positional_no_hyphen() {
    let r = App::new("bench")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::from("[BENCH] 'some bench'"))
        .arg(Arg::from("[ARGS]... 'some args'"))
        .try_get_matches_from(vec!["bench", "foo", "arg1", "arg2", "arg3"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind);

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
    let r = App::new("bench")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::from("[BENCH] 'some bench'"))
        .arg(Arg::from("[ARGS]... 'some args'"))
        .try_get_matches_from(vec!["bench", "--", "arg1", "arg2", "arg3"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind);

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
    let r = App::new("bench")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::from("[BENCH1] 'some bench'"))
        .arg(Arg::from("[BENCH2] 'some bench'"))
        .arg(Arg::from("[BENCH3] 'some bench'"))
        .arg(Arg::from("[ARGS]... 'some args'"))
        .try_get_matches_from(vec!["bench", "foo", "--", "arg1", "arg2", "arg3"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind);

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
    let r = App::new("bench")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::from("[BENCH1] 'some bench'"))
        .arg(Arg::from("<BENCH2> 'some bench'"))
        .arg(Arg::from("[ARGS]... 'some args'"))
        .try_get_matches_from(vec!["bench", "foo", "--", "arg1", "arg2", "arg3"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("--some-argument"))
        .try_get_matches_from(vec!["prog", "hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_no_vals() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("--some-argument"))
        .try_get_matches_from(vec!["prog", "--hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_option() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from("--some-argument=[val]"))
        .try_get_matches_from(vec!["prog", "-hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn issue_1093_allow_ext_sc() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .setting(AppSettings::AllowExternalSubcommands);
    assert!(test::compare_output(
        app,
        "clap-test --help",
        ALLOW_EXT_SC,
        false
    ));
}

#[test]
fn external_subcommand_looks_like_built_in() {
    let res = App::new("cargo")
        .version("1.26.0")
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(App::new("install"))
        .try_get_matches_from(vec!["cargo", "install-update", "foo"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    match m.subcommand() {
        (name, Some(args)) => {
            assert_eq!(name, "install-update");
            assert_eq!(args.values_of_lossy(""), Some(vec!["foo".to_string()]));
        }
        _ => assert!(false),
    }
}

#[test]
fn aaos_flags() {
    // flags
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--flag  'some flag'"))
        .try_get_matches_from(vec!["", "--flag", "--flag"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn aaos_flags_mult() {
    // flags with multiple
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--flag...  'some flag'"))
        .try_get_matches_from(vec!["", "--flag", "--flag", "--flag", "--flag"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 4);
}

#[test]
fn aaos_opts() {
    // opts
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--opt [val] 'some option'"))
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn aaos_opts_w_other_overrides() {
    // opts with other overrides
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--opt [val] 'some option'"))
        .arg(Arg::from("--other [val] 'some other option'").overrides_with("opt"))
        .try_get_matches_from(vec!["", "--opt=some", "--other=test", "--opt=other"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert!(!m.is_present("other"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn aaos_opts_w_other_overrides_rev() {
    // opts with other overrides, rev
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--opt [val] 'some option'"))
        .arg(Arg::from("--other [val] 'some other option'").overrides_with("opt"))
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--other=val"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(!m.is_present("opt"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("other"), Some("val"));
}

#[test]
fn aaos_opts_w_other_overrides_2() {
    // opts with other overrides
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--opt [val] 'some option'").overrides_with("other"))
        .arg(Arg::from("--other [val] 'some other option'"))
        .try_get_matches_from(vec!["", "--opt=some", "--other=test", "--opt=other"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert!(!m.is_present("other"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn aaos_opts_w_other_overrides_rev_2() {
    // opts with other overrides, rev
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--opt [val] 'some option'").overrides_with("other"))
        .arg(Arg::from("--other [val] 'some other option'"))
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--other=val"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(!m.is_present("opt"));
    assert!(m.is_present("other"));
    assert_eq!(m.value_of("other"), Some("val"));
}

#[test]
fn aaos_opts_mult() {
    // opts with multiple
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(
            Arg::from("--opt [val]... 'some option'")
                .number_of_values(1)
                .require_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
    assert!(res.is_ok());
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
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--opt [val]... 'some option'"))
        .try_get_matches_from(vec![
            "", "--opt", "first", "overides", "--opt", "some", "other", "val",
        ]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 2);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["first", "overides", "some", "other", "val"]
    );
}

#[test]
fn aaos_pos_mult() {
    // opts with multiple
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("[val]... 'some pos'"))
        .try_get_matches_from(vec!["", "some", "other", "value"]);
    assert!(res.is_ok());
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
    let m = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from("--opt [val] 'some option'").use_delimiter(false))
        .get_matches_from(vec!["", "--opt=some,other", "--opt=one,two"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["one,two"]
    );
}
