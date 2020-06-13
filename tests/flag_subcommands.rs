use clap::{App, Arg, FlagSubCommand};

#[test]
fn flag_subcommand_normal() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new('S', "some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "some", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_normal_with_alias() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new('S', "some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .alias("result"),
        )
        .get_matches_from(vec!["myprog", "result", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new_short("some", 'S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "-S", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_args() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new_short("some", 'S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "-St"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_alias() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new_short("some", 'S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .alias("M")
                .alias("B"),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_long() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new_long("some", "some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "--some", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_long_with_alias() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new_long("some", "some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .alias("result"),
        )
        .get_matches_from(vec!["myprog", "--result", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_multiple() {
    let matches = App::new("test")
        .subcommand(
            FlagSubCommand::new('S', "some")
                .arg(Arg::from("-f, --flag 'some flag'"))
                .arg(Arg::from("-p, --print 'print something'"))
                .subcommand(
                    FlagSubCommand::new('R', "result")
                        .arg(Arg::from("-f, --flag 'some flag'"))
                        .arg(Arg::from("-p, --print 'print something'")),
                ),
        )
        .get_matches_from(vec!["myprog", "-SfpRfp"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("flag"));
    assert!(sub_matches.is_present("print"));
    assert_eq!(sub_matches.subcommand_name().unwrap(), "result");
    let result_matches = sub_matches.subcommand_matches("result").unwrap();
    assert!(result_matches.is_present("flag"));
    assert!(result_matches.is_present("print"));
}
