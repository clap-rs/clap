extern crate clap;
extern crate regex;

use clap::{App, AppSettings, Arg, ErrorKind, Propagation, SubCommand};

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
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from(vec!["myprog", "sub1"]);
}

#[test]
fn global_version() {
    let mut app = App::new("global_version")
        .setting(AppSettings::GlobalVersion)
        .version("1.1")
        .subcommand(SubCommand::with_name("sub1"));
    app._propagate(Propagation::NextLevel);
    assert_eq!(app.subcommands[0].version, Some("1.1"));
}

#[test]
fn sub_command_negate_required_2() {
    let result = App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("test").required(true).index(1))
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn sub_command_required() {
    let result = App::new("sc_required")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingSubcommand);
}

#[test]
fn arg_required_else_help() {
    let result = App::new("arg_required")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("test").index(1))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingArgumentOrSubcommand);
}

#[test]
fn arg_required_else_help_over_reqs() {
    let result = App::new("arg_required")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("test").index(1).required(true))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingArgumentOrSubcommand);
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("temp"))
        .get_matches_from_safe(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_no_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("temp"))
        .get_matches_from_safe(vec!["prog", "te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidSubcommand);
}

#[test]
fn infer_subcommands_fail_with_args() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .arg(Arg::with_name("some"))
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("temp"))
        .get_matches_from_safe(vec!["prog", "t"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert_eq!(m.unwrap().value_of("some"), Some("t"));
}

#[test]
fn infer_subcommands_fail_with_args2() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .arg(Arg::with_name("some"))
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("temp"))
        .get_matches_from_safe(vec!["prog", "te"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert_eq!(m.unwrap().value_of("some"), Some("te"));
}

#[test]
fn infer_subcommands_pass() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("test"))
        .get_matches_from(vec!["prog", "te"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn infer_subcommands_pass_close() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("temp"))
        .get_matches_from(vec!["prog", "tes"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[cfg(feature = "suggestions")]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("temp"))
        .get_matches_from_safe(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidSubcommand);
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn infer_subcommands_fail_suggestions() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("test"))
        .subcommand(SubCommand::with_name("temp"))
        .get_matches_from_safe(vec!["prog", "temps"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
}

#[test]
fn no_bin_name() {
    let result = App::new("arg_required")
        .setting(AppSettings::NoBinaryName)
        .arg(Arg::with_name("test").required(true).index(1))
        .get_matches_from_safe(vec!["testing"]);
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
        .args_from_usage(
            "-f, --flag 'some flag'
             [arg1] 'some pos arg'
             --option [opt] 'some option'",
        );

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
            Arg::from_usage("-o, --opt [opt] 'some option'").possible_values(&["one", "two"]),
            Arg::from_usage("[arg1] 'some pos arg'").possible_values(&["three", "four"]),
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
        .subcommand(SubCommand::with_name("subcmd"));
    app._propagate(Propagation::NextLevel);
    assert!(
        app.subcommands
            .iter()
            .filter(|s| s.name == "subcmd")
            .next()
            .unwrap()
            .is_set(AppSettings::ColoredHelp)
    );
}

#[test]
fn global_settings() {
    let mut app = App::new("test")
        .global_settings(&[AppSettings::ColoredHelp, AppSettings::TrailingVarArg])
        .subcommand(SubCommand::with_name("subcmd"));
    app._propagate(Propagation::NextLevel);
    assert!(
        app.subcommands
            .iter()
            .filter(|s| s.name == "subcmd")
            .next()
            .unwrap()
            .is_set(AppSettings::ColoredHelp)
    );
    assert!(
        app.subcommands
            .iter()
            .filter(|s| s.name == "subcmd")
            .next()
            .unwrap()
            .is_set(AppSettings::TrailingVarArg)
    );
}

#[test]
fn stop_delim_values_only_pos_follows() {
    let r = App::new("onlypos")
        .setting(AppSettings::DontDelimitTrailingValues)
        .args(&[
            Arg::from_usage("-f [flag] 'some opt'"),
            Arg::from_usage("[arg]... 'some arg'"),
        ])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
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
        .arg(Arg::from_usage("[opt]... 'some pos'"))
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
            Arg::from_usage("-f [flag] 'some opt'"),
            Arg::from_usage("[arg]... 'some arg'"),
        ])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
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
        .arg(Arg::from_usage("[opt]... 'some pos'"))
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
            Arg::from_usage("-f [flag] 'some opt'"),
            Arg::from_usage("[arg]... 'some arg'").use_delimiter(true),
        ])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
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
        .arg(Arg::from_usage("[opt]... 'some pos'").use_delimiter(true))
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
        .arg(Arg::with_name("other").short("o"))
        .get_matches_from_safe(vec!["", "-bar", "-o"]);
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
        .arg(Arg::with_name("other").short("o"))
        .get_matches_from_safe(vec!["", "--bar", "-o"]);
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
        .arg(Arg::with_name("other").short("o"))
        .get_matches_from_safe(vec!["", "--opt", "--bar", "-o"]);
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
        .arg(Arg::with_name("onum").short("o").takes_value(true))
        .get_matches_from_safe(vec!["negnum", "-20", "-o", "-1.2"]);
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
        .arg(Arg::with_name("onum").short("o").takes_value(true))
        .get_matches_from_safe(vec!["negnum", "--foo", "-o", "-1.2"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument)
}

#[test]
fn leading_double_hyphen_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("[opt]... 'some pos'"))
        .get_matches_from(vec!["", "--foo", "-Wl", "bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["--foo", "-Wl", "bar"]
    );
}

#[test]
fn unset_setting() {
    let m = App::new("unset_setting");
    assert!(m.is_set(AppSettings::AllowInvalidUtf8));

    let m = m.unset_setting(AppSettings::AllowInvalidUtf8);
    assert!(!m.is_set(AppSettings::AllowInvalidUtf8));
}

#[test]
fn unset_settings() {
    let m = App::new("unset_settings");
    assert!(&m.is_set(AppSettings::AllowInvalidUtf8));
    assert!(&m.is_set(AppSettings::ColorAuto));

    let m = m.unset_settings(&[AppSettings::AllowInvalidUtf8, AppSettings::ColorAuto]);
    assert!(!m.is_set(AppSettings::AllowInvalidUtf8), "{:?}", m.settings);
    assert!(!m.is_set(AppSettings::ColorAuto));
}

#[test]
fn disable_help_subcommand() {
    let result = App::new("disablehelp")
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from_safe(vec!["", "help"]);
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
            .short("o")
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
    let res =
        App::new("disablehelp")
            .setting(AppSettings::ArgsNegateSubcommands)
            .setting(AppSettings::SubcommandsNegateReqs)
            .arg_from_usage("<arg1> 'some arg'")
            .arg_from_usage("<arg2> 'some arg'")
            .subcommand(SubCommand::with_name("sub1").subcommand(
                SubCommand::with_name("sub2").subcommand(SubCommand::with_name("sub3")),
            ))
            .get_matches_from_safe(vec!["", "pickles", "sub1"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("arg2"), Some("sub1"));
}

#[test]
fn args_negate_subcommands_two_levels() {
    let res = App::new("disablehelp")
        .global_setting(AppSettings::ArgsNegateSubcommands)
        .global_setting(AppSettings::SubcommandsNegateReqs)
        .arg_from_usage("<arg1> 'some arg'")
        .arg_from_usage("<arg2> 'some arg'")
        .subcommand(
            SubCommand::with_name("sub1")
                .arg_from_usage("<arg> 'some'")
                .arg_from_usage("<arg2> 'some'")
                .subcommand(
                    SubCommand::with_name("sub2").subcommand(SubCommand::with_name("sub3")),
                ),
        )
        .get_matches_from_safe(vec!["", "sub1", "arg", "sub2"]);
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
        .arg(Arg::from_usage("[cmd] 'command to run'").global(true))
        .subcommand(SubCommand::with_name("foo"))
        .get_matches_from_safe(vec!["myprog", "set", "foo"]);
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
        .arg(Arg::from_usage("[src] 'some file'").default_value("src"))
        .arg_from_usage("<dest> 'some file'")
        .get_matches_from_safe(vec!["test", "file"]);
    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();
    assert_eq!(m.value_of("src"), Some("src"));
    assert_eq!(m.value_of("dest"), Some("file"));
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("--some-argument"))
        .get_matches_from_safe(vec!["prog", "hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_no_vals() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("--some-argument"))
        .get_matches_from_safe(vec!["prog", "--hello"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn issue_1066_allow_leading_hyphen_and_unknown_args_option() {
    let res = App::new("prog")
        .global_setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("--some-argument=[val]"))
        .get_matches_from_safe(vec!["prog", "-hello"]);

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
fn aaos_flags() {
    // flags
    let res = App::new("posix")
        .setting(AppSettings::AllArgsOverrideSelf)
        .arg(Arg::from_usage("--flag  'some flag'"))
        .get_matches_from_safe(vec!["", "--flag", "--flag"]);
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
        .arg(Arg::from_usage("--flag...  'some flag'"))
        .get_matches_from_safe(vec!["", "--flag", "--flag", "--flag", "--flag"]);
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
        .arg(Arg::from_usage("--opt [val] 'some option'"))
        .get_matches_from_safe(vec!["", "--opt=some", "--opt=other"]);
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
        .arg(Arg::from_usage("--opt [val] 'some option'"))
        .arg(Arg::from_usage("--other [val] 'some other option'").overrides_with("opt"))
        .get_matches_from_safe(vec!["", "--opt=some", "--other=test", "--opt=other"]);
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
        .arg(Arg::from_usage("--opt [val] 'some option'"))
        .arg(Arg::from_usage("--other [val] 'some other option'").overrides_with("opt"))
        .get_matches_from_safe(vec!["", "--opt=some", "--opt=other", "--other=val"]);
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
        .arg(Arg::from_usage("--opt [val] 'some option'").overrides_with("other"))
        .arg(Arg::from_usage("--other [val] 'some other option'"))
        .get_matches_from_safe(vec!["", "--opt=some", "--other=test", "--opt=other"]);
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
        .arg(Arg::from_usage("--opt [val] 'some option'").overrides_with("other"))
        .arg(Arg::from_usage("--other [val] 'some other option'"))
        .get_matches_from_safe(vec!["", "--opt=some", "--opt=other", "--other=val"]);
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
            Arg::from_usage("--opt [val]... 'some option'")
                .number_of_values(1)
                .require_delimiter(true),
        )
        .get_matches_from_safe(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
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
        .arg(Arg::from_usage("--opt [val]... 'some option'"))
        .get_matches_from_safe(vec![
            "", "--opt", "first", "overides", "--opt", "some", "other", "val"
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
        .arg(Arg::from_usage("[val]... 'some pos'"))
        .get_matches_from_safe(vec!["", "some", "other", "value"]);
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
        .arg(Arg::from_usage("--opt [val] 'some option'").use_delimiter(false))
        .get_matches_from(vec!["", "--opt=some,other", "--opt=one,two"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["one,two"]
    );
}
