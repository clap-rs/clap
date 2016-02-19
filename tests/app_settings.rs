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
    let app = App::new("global_version")
        .setting(AppSettings::GlobalVersion)
        .version("1.1")
        .subcommand(SubCommand::with_name("sub1"));
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
\ttest [OPTIONS] [ARGS]

OPTIONS:
    -f, --flag            some flag
    -h, --help            Prints help information
        --option <opt>    some option
    -V, --version         Prints version information

ARGS:
    <arg1>    some pos arg\n"));
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
\ttest [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <opt>    some option

ARGS:
    <arg1>    some pos arg\n"));
}

#[test]
fn app_settings_fromstr() {
    assert_eq!("subcommandsnegatereqs".parse::<AppSettings>().unwrap(), AppSettings::SubcommandsNegateReqs);
    assert_eq!("subcommandsrequired".parse::<AppSettings>().unwrap(), AppSettings::SubcommandRequired);
    assert_eq!("argrequiredelsehelp".parse::<AppSettings>().unwrap(), AppSettings::ArgRequiredElseHelp);
    assert_eq!("globalversion".parse::<AppSettings>().unwrap(), AppSettings::GlobalVersion);
    assert_eq!("versionlesssubcommands".parse::<AppSettings>().unwrap(), AppSettings::VersionlessSubcommands);
    assert_eq!("unifiedhelpmessage".parse::<AppSettings>().unwrap(), AppSettings::UnifiedHelpMessage);
    assert_eq!("waitonerror".parse::<AppSettings>().unwrap(), AppSettings::WaitOnError);
    assert_eq!("subcommandrequiredelsehelp".parse::<AppSettings>().unwrap(), AppSettings::SubcommandRequiredElseHelp);
    assert_eq!("allowexternalsubcommands".parse::<AppSettings>().unwrap(), AppSettings::AllowExternalSubcommands);
    assert_eq!("trailingvararg".parse::<AppSettings>().unwrap(), AppSettings::TrailingVarArg);
    assert_eq!("nobinaryname".parse::<AppSettings>().unwrap(), AppSettings::NoBinaryName);
    assert_eq!("strictutf8".parse::<AppSettings>().unwrap(), AppSettings::StrictUtf8);
    assert_eq!("allowinvalidutf8".parse::<AppSettings>().unwrap(), AppSettings::AllowInvalidUtf8);
    assert_eq!("allowleadinghyphen".parse::<AppSettings>().unwrap(), AppSettings::AllowLeadingHyphen);
    assert_eq!("hidepossiblevaluesinhelp".parse::<AppSettings>().unwrap(), AppSettings::HidePossibleValuesInHelp);
    assert_eq!("hidden".parse::<AppSettings>().unwrap(), AppSettings::Hidden);
    assert!("hahahaha".parse::<AppSettings>().is_err());
}
