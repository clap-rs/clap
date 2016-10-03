extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, SubCommand};

static SC_VISIBLE_ALIAS_HELP: &'static str = "test 
Some help

USAGE:
    test [OPTIONS]

OPTIONS:
        --opt <opt>     [aliases: visible]";

static SC_INVISIBLE_ALIAS_HELP: &'static str = "test 
Some help

USAGE:
    test [OPTIONS]

OPTIONS:
        --opt <opt>    ";

#[test]
fn single_alias_of_option_long() {
    let a = App::new("single_alias")
        .arg(Arg::with_name("alias")
            .long("alias")
            .takes_value(true)
            .help("single alias")
            .alias("new-opt"))
        .get_matches_from_safe(vec![
            "", "--new-opt", "cool"
        ]);
    assert!(a.is_ok());
    let a = a.unwrap();
    assert!(a.is_present("alias"));
    assert_eq!(a.value_of("alias").unwrap(), "cool");
}

#[test]
fn multiple_aliases_of_option_long() {
    let a = App::new("multiple_aliases")
        .arg(Arg::with_name("aliases")
            .long("aliases")
            .takes_value(true)
            .help("multiple aliases")
            .aliases(&vec![
                "alias1",
                "alias2",
                "alias3"
        ]));
    let long = a.clone().get_matches_from_safe(vec![
        "", "--aliases", "value"
    ]);
    assert!(long.is_ok());
    let long = long.unwrap();

    let als1 = a.clone().get_matches_from_safe(vec![
        "", "--alias1", "value"
    ]);
    assert!(als1.is_ok());
    let als1 = als1.unwrap();

    let als2 = a.clone().get_matches_from_safe(vec![
        "", "--alias2", "value"
    ]);
    assert!(als2.is_ok());
    let als2 = als2.unwrap();

    let als3 = a.clone().get_matches_from_safe(vec![
        "", "--alias3", "value"
    ]);
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
fn alias_on_a_subcommand_option() {
    let m = App::new("test")
        .subcommand(SubCommand::with_name("some")
            .arg(Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(true)
                .alias("opt")
                .help("testing testing")))
        .arg(Arg::with_name("other")
            .long("other")
            .aliases(&vec!["o1", "o2", "o3"]))
        .get_matches_from(vec![
            "test", "some", "--opt", "awesome"
        ]);

    assert!(m.subcommand_matches("some").is_some());
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "awesome");
}

#[test]
fn invisible_arg_aliases_help_output() {
    let app = App::new("clap-test")
        .subcommand(SubCommand::with_name("test")
            .about("Some help")
            .arg(Arg::with_name("opt")
                .long("opt")
                .takes_value(true)
                .aliases(&["invisible", "als1", "more"])));
    test::check_subcommand_help(app, "test", SC_INVISIBLE_ALIAS_HELP);
}

#[test]
fn visible_arg_aliases_help_output() {
    let app = App::new("clap-test")
        .subcommand(SubCommand::with_name("test")
            .about("Some help")
            .arg(Arg::with_name("opt")
                .long("opt")
                .takes_value(true)
                .alias("invisible")
                .visible_alias("visible")));
    test::check_subcommand_help(app, "test", SC_VISIBLE_ALIAS_HELP);
}

#[test]
fn visible_arg_flag_aliases() {
    let a = App::new("test")
                .arg(Arg::with_name("opt")
                    .long("opt")
                    .aliases(&["invisible", "set", "of", "aliases"]))
                .get_matches_from_safe(vec!["", "--aliases"]);
    assert!(a.is_ok());
    let a = a.unwrap();
    assert!(a.is_present("opt"));
}
