use super::utils;

use clap::{arg, Arg, ArgAction, Command};

static SC_VISIBLE_ALIAS_HELP: &str = "ct-test 1.2
Some help

USAGE:
    ct test [OPTIONS]

OPTIONS:
    -f, --flag         [aliases: v_flg, flag2, flg3]
    -h, --help         Print help information
    -o, --opt <opt>    [aliases: visible]
    -V, --version      Print version information
";

static SC_INVISIBLE_ALIAS_HELP: &str = "ct-test 1.2
Some help

USAGE:
    ct test [OPTIONS]

OPTIONS:
    -f, --flag         
    -h, --help         Print help information
    -o, --opt <opt>    
    -V, --version      Print version information
";

#[test]
fn single_alias_of_option() {
    let a = Command::new("single_alias")
        .arg(
            Arg::new("alias")
                .long("alias")
                .takes_value(true)
                .help("single alias")
                .alias("new-opt"),
        )
        .try_get_matches_from(vec!["", "--new-opt", "cool"]);
    assert!(a.is_ok(), "{}", a.unwrap_err());
    let a = a.unwrap();
    assert!(a.contains_id("alias"));
    assert_eq!(
        a.get_one::<String>("alias").map(|v| v.as_str()).unwrap(),
        "cool"
    );
}

#[test]
fn multiple_aliases_of_option() {
    let a = Command::new("multiple_aliases").arg(
        Arg::new("aliases")
            .long("aliases")
            .takes_value(true)
            .help("multiple aliases")
            .aliases(&["alias1", "alias2", "alias3"]),
    );
    let long = a
        .clone()
        .try_get_matches_from(vec!["", "--aliases", "value"]);
    assert!(long.is_ok(), "{}", long.unwrap_err());
    let long = long.unwrap();

    let als1 = a
        .clone()
        .try_get_matches_from(vec!["", "--alias1", "value"]);
    assert!(als1.is_ok(), "{}", als1.unwrap_err());
    let als1 = als1.unwrap();

    let als2 = a
        .clone()
        .try_get_matches_from(vec!["", "--alias2", "value"]);
    assert!(als2.is_ok(), "{}", als2.unwrap_err());
    let als2 = als2.unwrap();

    let als3 = a
        .clone()
        .try_get_matches_from(vec!["", "--alias3", "value"]);
    assert!(als3.is_ok(), "{}", als3.unwrap_err());
    let als3 = als3.unwrap();

    assert!(long.contains_id("aliases"));
    assert!(als1.contains_id("aliases"));
    assert!(als2.contains_id("aliases"));
    assert!(als3.contains_id("aliases"));
    assert_eq!(
        long.get_one::<String>("aliases")
            .map(|v| v.as_str())
            .unwrap(),
        "value"
    );
    assert_eq!(
        als1.get_one::<String>("aliases")
            .map(|v| v.as_str())
            .unwrap(),
        "value"
    );
    assert_eq!(
        als2.get_one::<String>("aliases")
            .map(|v| v.as_str())
            .unwrap(),
        "value"
    );
    assert_eq!(
        als3.get_one::<String>("aliases")
            .map(|v| v.as_str())
            .unwrap(),
        "value"
    );
}

#[test]
fn single_alias_of_flag() {
    let a = Command::new("test")
        .arg(
            Arg::new("flag")
                .long("flag")
                .alias("alias")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "--alias"]);
    assert!(a.is_ok(), "{}", a.unwrap_err());
    let a = a.unwrap();
    assert!(*a.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn multiple_aliases_of_flag() {
    let a = Command::new("test").arg(
        Arg::new("flag")
            .long("flag")
            .aliases(&["invisible", "set", "of", "cool", "aliases"])
            .action(ArgAction::SetTrue),
    );

    let flag = a.clone().try_get_matches_from(vec!["", "--flag"]);
    assert!(flag.is_ok(), "{}", flag.unwrap_err());
    let flag = flag.unwrap();

    let inv = a.clone().try_get_matches_from(vec!["", "--invisible"]);
    assert!(inv.is_ok(), "{}", inv.unwrap_err());
    let inv = inv.unwrap();

    let cool = a.clone().try_get_matches_from(vec!["", "--cool"]);
    assert!(cool.is_ok(), "{}", cool.unwrap_err());
    let cool = cool.unwrap();

    let als = a.clone().try_get_matches_from(vec!["", "--aliases"]);
    assert!(als.is_ok(), "{}", als.unwrap_err());
    let als = als.unwrap();

    assert!(*flag.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*inv.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*cool.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*als.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn alias_on_a_subcommand_option() {
    let m = Command::new("test")
        .subcommand(
            Command::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .alias("opt")
                    .help("testing testing"),
            ),
        )
        .arg(Arg::new("other").long("other").aliases(&["o1", "o2", "o3"]))
        .try_get_matches_from(vec!["test", "some", "--opt", "awesome"])
        .unwrap();

    assert!(m.subcommand_matches("some").is_some());
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.contains_id("test"));
    assert_eq!(
        sub_m.get_one::<String>("test").map(|v| v.as_str()).unwrap(),
        "awesome"
    );
}

#[test]
fn invisible_arg_aliases_help_output() {
    let cmd = Command::new("ct").author("Salim Afiune").subcommand(
        Command::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .takes_value(true)
                    .aliases(&["invisible", "als1", "more"]),
            )
            .arg(arg!(-f - -flag).aliases(&["unseeable", "flg1", "anyway"])),
    );
    utils::assert_output(cmd, "ct test --help", SC_INVISIBLE_ALIAS_HELP, false);
}

#[test]
fn visible_arg_aliases_help_output() {
    let cmd = Command::new("ct").author("Salim Afiune").subcommand(
        Command::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .takes_value(true)
                    .alias("invisible")
                    .visible_alias("visible"),
            )
            .arg(
                Arg::new("flg")
                    .long("flag")
                    .short('f')
                    .visible_aliases(&["v_flg", "flag2", "flg3"]),
            ),
    );
    utils::assert_output(cmd, "ct test --help", SC_VISIBLE_ALIAS_HELP, false);
}
