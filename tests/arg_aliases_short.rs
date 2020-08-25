mod utils;

use clap::{App, Arg};

static SC_VISIBLE_ALIAS_HELP: &str = "ct-test 1.2
Some help

USAGE:
    ct test [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       [aliases: flag1] [short aliases: a, b, 🦆]
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <opt>    [short aliases: v]";

static SC_INVISIBLE_ALIAS_HELP: &str = "ct-test 1.2
Some help

USAGE:
    ct test [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <opt>    ";

#[test]
fn single_short_alias_of_option() {
    let a = App::new("single_alias")
        .arg(
            Arg::new("alias")
                .long("alias")
                .takes_value(true)
                .about("single short alias")
                .short_alias('a'),
        )
        .try_get_matches_from(vec!["", "-a", "cool"]);
    assert!(a.is_ok());
    let a = a.unwrap();
    assert!(a.is_present("alias"));
    assert_eq!(a.value_of("alias").unwrap(), "cool");
}

#[test]
fn multiple_short_aliases_of_option() {
    let a = App::new("multiple_aliases").arg(
        Arg::new("aliases")
            .long("aliases")
            .takes_value(true)
            .about("multiple aliases")
            .short_aliases(&['1', '2', '3']),
    );
    let long = a
        .clone()
        .try_get_matches_from(vec!["", "--aliases", "value"]);
    assert!(long.is_ok());
    let long = long.unwrap();

    let als1 = a.clone().try_get_matches_from(vec!["", "-1", "value"]);
    assert!(als1.is_ok());
    let als1 = als1.unwrap();

    let als2 = a.clone().try_get_matches_from(vec!["", "-2", "value"]);
    assert!(als2.is_ok());
    let als2 = als2.unwrap();

    let als3 = a.clone().try_get_matches_from(vec!["", "-3", "value"]);
    assert!(als3.is_ok());
    let als3 = als3.unwrap();

    assert!(long.is_present("aliases"));
    assert!(als1.is_present("aliases"));
    assert!(als2.is_present("aliases"));
    assert!(als3.is_present("aliases"));
    assert_eq!(long.value_of("aliases").unwrap(), "value");
    assert_eq!(als1.value_of("aliases").unwrap(), "value");
    assert_eq!(als2.value_of("aliases").unwrap(), "value");
    assert_eq!(als3.value_of("aliases").unwrap(), "value");
}

#[test]
fn single_short_alias_of_flag() {
    let a = App::new("test")
        .arg(Arg::new("flag").long("flag").short_alias('f'))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(a.is_ok());
    let a = a.unwrap();
    assert!(a.is_present("flag"));
}

#[test]
fn multiple_short_aliases_of_flag() {
    let a = App::new("test").arg(
        Arg::new("flag")
            .long("flag")
            .short_aliases(&['a', 'b', 'c', 'd', 'e']),
    );

    let flag = a.clone().try_get_matches_from(vec!["", "--flag"]);
    assert!(flag.is_ok());
    let flag = flag.unwrap();

    let als1 = a.clone().try_get_matches_from(vec!["", "-a"]);
    assert!(als1.is_ok());
    let als1 = als1.unwrap();

    let als2 = a.clone().try_get_matches_from(vec!["", "-b"]);
    assert!(als2.is_ok());
    let als2 = als2.unwrap();

    let als3 = a.clone().try_get_matches_from(vec!["", "-c"]);
    assert!(als3.is_ok());
    let als3 = als3.unwrap();

    assert!(flag.is_present("flag"));
    assert!(als1.is_present("flag"));
    assert!(als2.is_present("flag"));
    assert!(als3.is_present("flag"));
}

#[test]
fn short_alias_on_a_subcommand_option() {
    let m = App::new("test")
        .subcommand(
            App::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .short_alias('o')
                    .about("testing testing"),
            ),
        )
        .arg(
            Arg::new("other")
                .long("other")
                .short_aliases(&['1', '2', '3']),
        )
        .get_matches_from(vec!["test", "some", "-o", "awesome"]);

    assert!(m.subcommand_matches("some").is_some());
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "awesome");
}

#[test]
fn invisible_short_arg_aliases_help_output() {
    let app = App::new("ct").author("Salim Afiune").subcommand(
        App::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .takes_value(true)
                    .short_aliases(&['a', 'b', 'c']),
            )
            .arg(Arg::from("-f, --flag").short_aliases(&['x', 'y', 'z'])),
    );
    assert!(utils::compare_output(
        app,
        "ct test --help",
        SC_INVISIBLE_ALIAS_HELP,
        false
    ));
}

#[test]
fn visible_short_arg_aliases_help_output() {
    let app = App::new("ct").author("Salim Afiune").subcommand(
        App::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .takes_value(true)
                    .short_alias('i')
                    .visible_short_alias('v'),
            )
            .arg(
                Arg::new("flg")
                    .long("flag")
                    .short('f')
                    .visible_alias("flag1")
                    .visible_short_aliases(&['a', 'b', '🦆']),
            ),
    );
    assert!(utils::compare_output(
        app,
        "ct test --help",
        SC_VISIBLE_ALIAS_HELP,
        false
    ));
}
