extern crate clap;

use clap::{App, Arg, SubCommand, AppSettings, ClapErrorType};

#[test]
fn sub_command_negate_required() {
    App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("test")
               .required(true)
               .index(1))
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from(vec!["", "sub1"]);
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
    assert_eq!(err.error_type, ClapErrorType::MissingRequiredArgument);
}

#[test]
fn sub_command_required() {
    let result = App::new("sc_required")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.error_type, ClapErrorType::MissingSubcommand);
}

#[test]
fn arg_required_else_help() {
    let result = App::new("arg_required")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("test")
               .required(true)
               .index(1))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.error_type, ClapErrorType::MissingRequiredArgument);
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
    -V, --version         Prints version information
        --option <opt>    some option

ARGS:
    arg1    some pos arg\n"));
}

#[test]
fn app_settings_fromstr() {
    assert_eq!("subcommandsnegatereqs".parse::<AppSettings>().ok().unwrap(), AppSettings::SubcommandsNegateReqs);
    assert_eq!("subcommandsrequired".parse::<AppSettings>().ok().unwrap(), AppSettings::SubcommandRequired);
    assert_eq!("argrequiredelsehelp".parse::<AppSettings>().ok().unwrap(), AppSettings::ArgRequiredElseHelp);
    assert_eq!("globalversion".parse::<AppSettings>().ok().unwrap(), AppSettings::GlobalVersion);
    assert_eq!("versionlesssubcommands".parse::<AppSettings>().ok().unwrap(), AppSettings::VersionlessSubcommands);
    assert_eq!("unifiedhelpmessage".parse::<AppSettings>().ok().unwrap(), AppSettings::UnifiedHelpMessage);
    assert_eq!("waitonerror".parse::<AppSettings>().ok().unwrap(), AppSettings::WaitOnError);
    assert_eq!("subcommandrequiredelsehelp".parse::<AppSettings>().ok().unwrap(), AppSettings::SubcommandRequiredElseHelp);
    assert_eq!("hidden".parse::<AppSettings>().ok().unwrap(), AppSettings::Hidden);
    assert!("hahahaha".parse::<AppSettings>().is_err());
}