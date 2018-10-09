extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, SubCommand, ErrorKind};

static VISIBLE_ALIAS_HELP: &'static str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help [aliases: dongle, done]";

static INVISIBLE_ALIAS_HELP: &'static str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help";

#[cfg(feature = "suggestions")]
static DYM_SUBCMD: &'static str = "error: The subcommand 'subcm' wasn't recognized
	Did you mean 'subcmd'?

If you believe you received this message in error, try re-running with 'dym -- subcm'

USAGE:
    dym [SUBCOMMAND]

For more information try --help";

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

    assert_eq!(m.subcommand_name().unwrap(), "some");
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

    assert!(m.subcommand_name().is_none());
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
    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn single_alias() {
    let m = App::new("myprog")
                .subcommand(SubCommand::with_name("test")
                    .alias("do-stuff"))
                .get_matches_from(vec!["myprog", "do-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn multiple_aliases() {
    let m = App::new("myprog")
                .subcommand(SubCommand::with_name("test")
                    .aliases(&["do-stuff", "test-stuff"]))
                .get_matches_from(vec!["myprog", "test-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
#[cfg(feature="suggestions")]
fn subcmd_did_you_mean_output() {
    let app = App::new("dym")
        .subcommand(SubCommand::with_name("subcmd"));
    assert!(test::compare_output(app, "dym subcm", DYM_SUBCMD, true));
}

#[test]
#[cfg(feature="suggestions")]
fn subcmd_did_you_mean_output_arg() {
    static EXPECTED: &'static str = "error: Found argument '--subcmarg' which wasn't expected, or isn't valid in this context
\tDid you mean to put '--subcmdarg' after the subcommand 'subcmd'?

USAGE:
    dym [SUBCOMMAND]

For more information try --help";

    let app = App::new("dym")
        .subcommand(SubCommand::with_name("subcmd")
            .arg_from_usage("-s --subcmdarg [subcmdarg] 'tests'") );
    assert!(test::compare_output(app, "dym --subcmarg subcmd", EXPECTED, true));
}

#[test]
#[cfg(feature="suggestions")]
fn subcmd_did_you_mean_output_arg_false_positives() {
    static EXPECTED: &'static str = "error: Found argument '--subcmarg' which wasn't expected, or isn't valid in this context

USAGE:
    dym [SUBCOMMAND]

For more information try --help";

    let app = App::new("dym")
        .subcommand(SubCommand::with_name("subcmd")
            .arg_from_usage("-s --subcmdarg [subcmdarg] 'tests'") );
    assert!(test::compare_output(app, "dym --subcmarg foo", EXPECTED, true));
}

#[test]
fn alias_help() {
    let m = App::new("myprog")
                .subcommand(SubCommand::with_name("test")
                    .alias("do-stuff"))
                .get_matches_from_safe(vec!["myprog", "help", "do-stuff"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn visible_aliases_help_output() {
    let app = App::new("clap-test")
        .version("2.6")
        .subcommand(SubCommand::with_name("test")
            .about("Some help")
            .alias("invisible")
            .visible_alias("dongle")
            .visible_alias("done"));
    assert!(test::compare_output(app, "clap-test --help", VISIBLE_ALIAS_HELP, false));
}

#[test]
fn invisible_aliases_help_output() {
    let app = App::new("clap-test")
        .version("2.6")
        .subcommand(SubCommand::with_name("test")
            .about("Some help")
            .alias("invisible"));
    assert!(test::compare_output(app, "clap-test --help", INVISIBLE_ALIAS_HELP, false));
}

#[test]
fn issue_1031_args_with_same_name() {
    let res = App::new("prog")
        .arg(Arg::from_usage("--ui-path=<PATH>"))
        .subcommand(SubCommand::with_name("signer"))
        .get_matches_from_safe(vec!["prog", "--ui-path", "signer"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("ui-path"), Some("signer"));
}

#[test]
fn issue_1031_args_with_same_name_no_more_vals() {
    let res = App::new("prog")
        .arg(Arg::from_usage("--ui-path=<PATH>"))
        .subcommand(SubCommand::with_name("signer"))
        .get_matches_from_safe(vec!["prog", "--ui-path", "value", "signer"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("ui-path"), Some("value"));
    assert_eq!(m.subcommand_name(), Some("signer"));
}
