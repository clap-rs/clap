extern crate clap;

use clap::{App, Arg, SubCommand, AppSettings, ErrorKind};

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
    let mut app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .setting(AppSettings::UnifiedHelpMessage)
        .args_from_usage("-f, --flag 'some flag'
                          [arg1] 'some pos arg'
                          --option [opt] 'some option'");
    // We call a get_matches method to cause --help and --version to be built
    let _ = app.get_matches_from_safe_borrow(vec![""]);

    // Now we check the output of print_help()
    let mut help = vec![];
    app.write_help(&mut help).ok().expect("failed to print help");
    assert_eq!(&*String::from_utf8_lossy(&*help), &*String::from("test 1.3\n\
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
    <arg1>    some pos arg"));
}

#[test]
fn skip_possible_values() {
    let mut app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .setting(AppSettings::HidePossibleValuesInHelp)
        .args(&[Arg::from_usage("-o, --opt [opt] 'some option'").possible_values(&["one", "two"]),
                Arg::from_usage("[arg1] 'some pos arg'").possible_values(&["three", "four"])]);
    // We call a get_matches method to cause --help and --version to be built
    let _ = app.get_matches_from_safe_borrow(vec![""]);

    // Now we check the output of print_help()
    let mut help = vec![];
    app.write_help(&mut help).expect("failed to print help");
    assert_eq!(&*String::from_utf8_lossy(&*help), &*String::from("test 1.3\n\
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
    <arg1>    some pos arg"));
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
