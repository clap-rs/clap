use clap::{App, Arg};

#[test]
fn flag_subcommand_normal() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").short_flag('S').long_flag("some").arg(
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
            App::new("some")
                .short_flag('S')
                .long_flag("S")
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
            App::new("some").short_flag('S').arg(
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
            App::new("some").short_flag('S').arg(
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
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_alias('M')
                .short_flag_alias('B'),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_aliases_vis_and_hidden() {
    let app = App::new("test").subcommand(
        App::new("some")
            .short_flag('S')
            .arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            )
            .visible_short_flag_aliases(&['M', 'B'])
            .short_flag_alias('C'),
    );
    let app1 = app.clone();
    let matches1 = app1.get_matches_from(vec!["test", "-M"]);
    assert_eq!(matches1.subcommand_name().unwrap(), "some");

    let app2 = app.clone();
    let matches2 = app2.get_matches_from(vec!["test", "-C"]);
    assert_eq!(matches2.subcommand_name().unwrap(), "some");

    let app3 = app.clone();
    let matches3 = app3.get_matches_from(vec!["test", "-B"]);
    assert_eq!(matches3.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_short_with_aliases() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_aliases(&['M', 'B']),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_alias_hyphen() {
    let _ = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_alias('-'),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_aliases_hyphen() {
    let _ = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_aliases(&['-', '-', '-']),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
}

#[test]
fn flag_subcommand_long() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").long_flag("some").arg(
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
            App::new("some")
                .long_flag("some")
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
            App::new("some")
                .short_flag('S')
                .long_flag("some")
                .arg(Arg::from("-f, --flag 'some flag'"))
                .arg(Arg::from("-p, --print 'print something'"))
                .subcommand(
                    App::new("result")
                        .short_flag('R')
                        .long_flag("result")
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

#[test]
#[should_panic = "Short option names must be unique for each argument, but \'-f\' is used by both an App named \'some\' and an Arg named \'test\'"]
fn flag_subcommand_short_conflict_with_arg() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").short('f'))
        .get_matches_from(vec!["myprog", "-ff"]);
}

#[test]
#[should_panic = "Long option names must be unique for each argument, but \'--flag\' is used by both an App named \'some\' and an Arg named \'flag\'"]
fn flag_subcommand_long_conflict_with_arg() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('a').long_flag("flag"))
        .arg(Arg::new("flag").long("flag"))
        .get_matches_from(vec!["myprog", "--flag", "--flag"]);
}

#[test]
fn flag_subcommand_conflict_with_help() {
    let _ = App::new("test")
        .subcommand(App::new("help").short_flag('h').long_flag("help"))
        .get_matches_from(vec!["myprog", "--help"]);
}

#[test]
fn flag_subcommand_conflict_with_version() {
    let _ = App::new("test")
        .subcommand(App::new("ver").short_flag('V').long_flag("version"))
        .get_matches_from(vec!["myprog", "--version"]);
}
