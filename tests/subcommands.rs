extern crate clap;

use clap::{App, Arg, SubCommand};

#[test]
fn subcommand() {
    let m = App::new("test")
        .subcommand(SubCommand::with_name("some")
            .arg(Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(true)
                .help("testing testing")))
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert_eq!(m.subcommand_name::<&str>().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn subcommand_none_given() {
    let m = App::new("test")
        .subcommand(SubCommand::with_name("some")
            .arg(Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(true)
                .help("testing testing")))
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from(vec![""]);

    assert!(m.subcommand_name::<&str>().is_none());
}

#[test]
fn subcommand_multiple() {
    let m = App::new("test")
        .subcommands(vec![
            SubCommand::with_name("some")
                .arg(Arg::with_name("test")
                    .short("t")
                    .long("test")
                    .takes_value(true)
                    .help("testing testing")),
            SubCommand::with_name("add")
                .arg(Arg::with_name("roster").short("r"))
        ])
        .arg(Arg::with_name("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert!(m.subcommand_matches("some").is_some());
    assert!(m.subcommand_matches("add").is_none());
    assert_eq!(m.subcommand_name::<&str>().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

