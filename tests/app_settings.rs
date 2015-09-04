extern crate clap;

use clap::{App, Arg, SubCommand, AppSettings};

#[test]
fn sub_command_negate_requred() {
    App::new("sub_command_negate")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("test")
               .required(true)
               .index(1))
        .subcommand(SubCommand::with_name("sub1"))
        .subcommand(SubCommand::with_name("sub1"))
        .get_matches_from(vec!["", "sub1"]);
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
    assert!("hahahaha".parse::<AppSettings>().is_err());
}