extern crate clap;
extern crate regex;

use clap::{App, Arg, SubCommand, AppSettings, ErrorKind};

include!("../clap-test.rs");

static DONT_COLLAPSE_ARGS: &'static str = "clap-test v1.4.8

USAGE:
    clap-test [arg1] [arg2] [arg3]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <arg1>    some
    <arg2>    some
    <arg3>    some";

static UNIFIED_HELP: &'static str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test [OPTIONS] [arg1]

OPTIONS:
    -f, --flag            some flag
    -h, --help            Prints help information
        --option <opt>    some option
    -V, --version         Prints version information

ARGS:
    <arg1>    some pos arg";

static SKIP_POS_VALS: &'static str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test [OPTIONS] [arg1]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <opt>    some option

ARGS:
    <arg1>    some pos arg";

#[test]
fn sub_command_negate_required() {
    App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("test")
               .required(true)
               .index(1))
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from(vec!["myprog", "sub1"]);
}

#[test]
fn global_version() {
    let mut app = App::new("global_version")
        .setting(AppSettings::GlobalVersion)
        .version("1.1")
        .subcommand(SubCommand::with_name("sub1"));
    app.p.propogate_settings();
    assert_eq!(app.p.subcommands[0].p.meta.version, Some("1.1"));
}

#[test]
fn sub_command_negate_required_2() {
    let result = App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("test")
               .required(true)
               .index(1))
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
        .arg(Arg::with_name("test")
               .index(1))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingArgumentOrSubcommand);
}

#[test]
fn no_bin_name() {
    let result = App::new("arg_required")
        .setting(AppSettings::NoBinaryName)
        .arg(Arg::with_name("test")
               .required(true)
               .index(1))
        .get_matches_from_safe(vec!["testing"]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert_eq!(matches.value_of("test").unwrap(), "testing");
}

#[test]
fn unified_help() {
    let app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .setting(AppSettings::UnifiedHelpMessage)
        .args_from_usage("-f, --flag 'some flag'
                          [arg1] 'some pos arg'
                          --option [opt] 'some option'");

    assert!(test::compare_output(app, "test --help", UNIFIED_HELP, false));
}

#[test]
fn skip_possible_values() {
    let app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .setting(AppSettings::HidePossibleValuesInHelp)
        .args(&[Arg::from_usage("-o, --opt [opt] 'some option'").possible_values(&["one", "two"]),
                Arg::from_usage("[arg1] 'some pos arg'").possible_values(&["three", "four"])]);

    assert!(test::compare_output(app, "test --help", SKIP_POS_VALS, false));
}

#[test]
fn global_setting() {
    let mut app = App::new("test")
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("subcmd"));
    app.p.propogate_settings();
    assert!(app.p
               .subcommands
               .iter()
               .filter(|s| s.p
                            .meta
                            .name == "subcmd")
               .next()
               .unwrap()
               .p
               .is_set(AppSettings::ColoredHelp));
}

#[test]
fn global_settings() {
    let mut app = App::new("test")
        .global_settings(&[AppSettings::ColoredHelp, AppSettings::TrailingVarArg])
        .subcommand(SubCommand::with_name("subcmd"));
    app.p.propogate_settings();
    assert!(app.p
               .subcommands
               .iter()
               .filter(|s| s.p
                            .meta
                            .name == "subcmd")
               .next()
               .unwrap()
               .p
               .is_set(AppSettings::ColoredHelp));
    assert!(app.p
               .subcommands
               .iter()
               .filter(|s| s.p
                            .meta
                            .name == "subcmd")
               .next()
               .unwrap()
               .p
               .is_set(AppSettings::TrailingVarArg));

}

#[test]
fn stop_delim_values_only_pos_follows() {
    let r = App::new("onlypos")
        .setting(AppSettings::DontDelimitTrailingValues)
        .args(&[Arg::from_usage("-f [flag] 'some opt'"),
                Arg::from_usage("[arg]... 'some arg'")])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(), &["-f", "-g,x"]);
}

#[test]
fn dont_delim_values_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::DontDelimitTrailingValues)
        .arg(
            Arg::from_usage("[opt]... 'some pos'"),
        )
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(), &["test", "--foo", "-Wl,-bar"]);
}

#[test]
fn delim_values_only_pos_follows() {
    let r = App::new("onlypos")
        .args(&[Arg::from_usage("-f [flag] 'some opt'"),
                Arg::from_usage("[arg]... 'some arg'")])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(), &["-f", "-g,x"]);
}

#[test]
fn delim_values_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::from_usage("[opt]... 'some pos'"),
        )
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(), &["test", "--foo", "-Wl,-bar"]);
}

#[test]
fn delim_values_only_pos_follows_with_delim() {
    let r = App::new("onlypos")
        .args(&[Arg::from_usage("-f [flag] 'some opt'"),
                Arg::from_usage("[arg]... 'some arg'").use_delimiter(true)])
        .get_matches_from_safe(vec!["", "--", "-f", "-g,x"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(), &["-f", "-g", "x"]);
}

#[test]
fn delim_values_trailingvararg_with_delim() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::from_usage("[opt]... 'some pos'").use_delimiter(true),
        )
        .get_matches_from(vec!["", "test", "--foo", "-Wl,-bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(), &["test", "--foo", "-Wl", "-bar"]);
}

#[test]
fn leading_hyphen_short() {
    let res = App::new("leadhy")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("some"))
        .arg(Arg::with_name("other")
            .short("o"))
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
        .arg(Arg::with_name("other")
            .short("o"))
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
        .arg(Arg::with_name("some")
            .takes_value(true)
            .long("opt"))
        .arg(Arg::with_name("other")
            .short("o"))
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
        .arg(Arg::with_name("onum")
            .short("o")
            .takes_value(true))
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
        .arg(Arg::with_name("onum")
            .short("o")
            .takes_value(true))
        .get_matches_from_safe(vec!["negnum", "--foo", "-o", "-1.2"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument)
}

#[test]
fn leading_double_hyphen_trailingvararg() {
    let m = App::new("positional")
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(
            Arg::from_usage("[opt]... 'some pos'"),
        )
        .get_matches_from(vec!["", "--foo", "-Wl", "bar"]);
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>(), &["--foo", "-Wl", "bar"]);
}

#[test]
fn test_unset_setting() {
    let m = App::new("unset_setting");
    assert!(m.p.is_set(AppSettings::AllowInvalidUtf8));

    let m = m.unset_setting(AppSettings::AllowInvalidUtf8);
    assert!(!m.p.is_set(AppSettings::AllowInvalidUtf8));
}

#[test]
fn test_unset_settings() {
    let m = App::new("unset_settings");
    assert!(&m.p.is_set(AppSettings::AllowInvalidUtf8));
    assert!(&m.p.is_set(AppSettings::ColorAuto));

    let m = m.unset_settings(&[AppSettings::AllowInvalidUtf8,
                               AppSettings::ColorAuto]);
    assert!(!m.p.is_set(AppSettings::AllowInvalidUtf8));
    assert!(!m.p.is_set(AppSettings::ColorAuto));
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
    assert!(test::compare_output(app, "clap-test --help", DONT_COLLAPSE_ARGS, false));
}

#[test]
fn args_negate_subcommands_one_level() {
    let res = App::new("disablehelp")
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg_from_usage("<arg1> 'some arg'")
        .arg_from_usage("<arg2> 'some arg'")
        .subcommand(SubCommand::with_name("sub1")
            .subcommand(SubCommand::with_name("sub2")
                .subcommand(SubCommand::with_name("sub3"))
            )
        )
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
        .subcommand(SubCommand::with_name("sub1")
            .arg_from_usage("<arg> 'some'")
            .arg_from_usage("<arg2> 'some'")
            .subcommand(SubCommand::with_name("sub2")
                .subcommand(SubCommand::with_name("sub3"))
            )
        )
        .get_matches_from_safe(vec!["", "sub1", "arg", "sub2"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.subcommand_matches("sub1").unwrap().value_of("arg2"), Some("sub2"));
}


#[test]
fn propagate_vals_down() {
    let m = App::new("myprog")
        .setting(AppSettings::PropagateGlobalValuesDown)
        .arg(Arg::from_usage("[cmd] 'command to run'").global(true))
        .subcommand(SubCommand::with_name("foo"))
        .get_matches_from(vec!["myprog", "set", "foo"]);
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
        .get_matches_from(vec!["test", "file"]);
    assert_eq!(m.value_of("src"), Some("src"));
    assert_eq!(m.value_of("dest"), Some("file"));
}