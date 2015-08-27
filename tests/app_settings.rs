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