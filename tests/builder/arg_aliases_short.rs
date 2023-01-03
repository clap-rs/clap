use super::utils;

use clap::{arg, Arg, ArgAction, Command};

#[test]
fn single_short_alias_of_option() {
    let a = Command::new("single_alias")
        .arg(
            Arg::new("alias")
                .long("alias")
                .action(ArgAction::Set)
                .help("single short alias")
                .short_alias('a'),
        )
        .try_get_matches_from(vec!["", "-a", "cool"]);
    assert!(a.is_ok(), "{}", a.unwrap_err());
    let a = a.unwrap();
    assert!(a.contains_id("alias"));
    assert_eq!(
        a.get_one::<String>("alias").map(|v| v.as_str()).unwrap(),
        "cool"
    );
}

#[test]
fn multiple_short_aliases_of_option() {
    let a = Command::new("multiple_aliases").arg(
        Arg::new("aliases")
            .long("aliases")
            .action(ArgAction::Set)
            .help("multiple aliases")
            .short_aliases(['1', '2', '3']),
    );
    let long = a
        .clone()
        .try_get_matches_from(vec!["", "--aliases", "value"]);
    assert!(long.is_ok(), "{}", long.unwrap_err());
    let long = long.unwrap();

    let als1 = a.clone().try_get_matches_from(vec!["", "-1", "value"]);
    assert!(als1.is_ok(), "{}", als1.unwrap_err());
    let als1 = als1.unwrap();

    let als2 = a.clone().try_get_matches_from(vec!["", "-2", "value"]);
    assert!(als2.is_ok(), "{}", als2.unwrap_err());
    let als2 = als2.unwrap();

    let als3 = a.clone().try_get_matches_from(vec!["", "-3", "value"]);
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
fn single_short_alias_of_flag() {
    let a = Command::new("test")
        .arg(
            Arg::new("flag")
                .long("flag")
                .short_alias('f')
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "-f"]);
    assert!(a.is_ok(), "{}", a.unwrap_err());
    let a = a.unwrap();
    assert!(*a.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn multiple_short_aliases_of_flag() {
    let a = Command::new("test").arg(
        Arg::new("flag")
            .long("flag")
            .short_aliases(['a', 'b', 'c', 'd', 'e'])
            .action(ArgAction::SetTrue),
    );

    let flag = a.clone().try_get_matches_from(vec!["", "--flag"]);
    assert!(flag.is_ok(), "{}", flag.unwrap_err());
    let flag = flag.unwrap();

    let als1 = a.clone().try_get_matches_from(vec!["", "-a"]);
    assert!(als1.is_ok(), "{}", als1.unwrap_err());
    let als1 = als1.unwrap();

    let als2 = a.clone().try_get_matches_from(vec!["", "-b"]);
    assert!(als2.is_ok(), "{}", als2.unwrap_err());
    let als2 = als2.unwrap();

    let als3 = a.clone().try_get_matches_from(vec!["", "-c"]);
    assert!(als3.is_ok(), "{}", als3.unwrap_err());
    let als3 = als3.unwrap();

    assert!(*flag.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*als1.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*als2.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*als3.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn short_alias_on_a_subcommand_option() {
    let m = Command::new("test")
        .subcommand(
            Command::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .action(ArgAction::Set)
                    .short_alias('o')
                    .help("testing testing"),
            ),
        )
        .arg(
            Arg::new("other")
                .long("other")
                .short_aliases(['1', '2', '3']),
        )
        .try_get_matches_from(vec!["test", "some", "-o", "awesome"])
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
fn invisible_short_arg_aliases_help_output() {
    static SC_INVISIBLE_ALIAS_HELP: &str = "\
Some help

Usage: ct test [OPTIONS]

Options:
  -o, --opt <opt>  
  -f, --flag       
  -h, --help       Print help
  -V, --version    Print version
";

    let cmd = Command::new("ct").author("Salim Afiune").subcommand(
        Command::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .action(ArgAction::Set)
                    .short_aliases(['a', 'b', 'c']),
            )
            .arg(arg!(-f - -flag).short_aliases(['x', 'y', 'z'])),
    );
    utils::assert_output(cmd, "ct test --help", SC_INVISIBLE_ALIAS_HELP, false);
}

#[test]
fn visible_short_arg_aliases_help_output() {
    static SC_VISIBLE_ALIAS_HELP: &str = "\
Some help

Usage: ct test [OPTIONS]

Options:
  -o, --opt <opt>  [short aliases: v]
  -f, --flag       [aliases: flag1] [short aliases: a, b, 🦆]
  -h, --help       Print help
  -V, --version    Print version
";

    let cmd = Command::new("ct").author("Salim Afiune").subcommand(
        Command::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .action(ArgAction::Set)
                    .short_alias('i')
                    .visible_short_alias('v'),
            )
            .arg(
                Arg::new("flg")
                    .long("flag")
                    .short('f')
                    .action(ArgAction::SetTrue)
                    .visible_alias("flag1")
                    .visible_short_aliases(['a', 'b', '🦆']),
            ),
    );
    utils::assert_output(cmd, "ct test --help", SC_VISIBLE_ALIAS_HELP, false);
}
