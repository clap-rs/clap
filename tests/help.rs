extern crate clap_test;
extern crate clap;

use clap::{App, SubCommand, ErrorKind};

static HELP: &'static str = "clap-test v1.4.8
Kevin K. <kbknapp@gmail.com>
tests clap library

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

FLAGS:
    -f, --flag       tests flags
    -F               tests flags with exclusions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -O, --Option <option3>           specific vals [values: fast, slow]
        --long-option-2 <option2>    tests long options with exclusions
        --maxvals3 <maxvals>...      Tests 3 max vals
        --minvals2 <minvals>...      Tests 2 min vals
        --multvals <one> <two>       Tests mutliple values, not mult occs
        --multvalsmo <one> <two>     Tests mutliple values, and mult occs
    -o, --option <opt>...            tests options

ARGS:
    <positional>        tests positionals
    <positional2>       tests positionals with exclusions
    <positional3>...    tests specific values [values: vi, emacs]

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    subcmd    tests subcommands
";

static SC_HELP: &'static str = "subcmd 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

USAGE:
    subcmd [FLAGS] [OPTIONS] [--] [ARGS]

FLAGS:
    -f, --flag    tests flags

OPTIONS:
    -o, --option <scoption>...    tests options

ARGS:
    <scpositional>    tests positionals
";

#[test]
fn help_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn help_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn help_no_subcommand() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn help_subcommand() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .subcommand(SubCommand::with_name("test")
            .about("tests things")
            .arg_from_usage("-v --verbose 'with verbosity'"))
        .get_matches_from_safe(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_short_help() {
    let m = clap_test::complex_app()
        .get_matches_from_safe(vec!["clap-test", "subcmd", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_long_help() {
    let m = clap_test::complex_app()
        .get_matches_from_safe(vec!["clap-test", "subcmd", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_help_rev() {
    let m = clap_test::complex_app()
        .get_matches_from_safe(vec!["clap-test", "help", "subcmd"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn complex_help_output() {
    clap_test::check_help(clap_test::complex_app(), HELP);
}

#[test]
fn complex_subcommand_help_output() {
    let mut a = clap_test::complex_app();
    let _ = a.get_matches_from_safe_borrow(vec![""]);
    let sc = a.p.subcommands.iter().filter(|s| s.p.meta.name == "subcmd").next().unwrap();
    // Now we check the output of print_help()
    let mut help = vec![];
    sc.write_help(&mut help).ok().expect("failed to print help");
    assert_eq!(&*String::from_utf8(help).unwrap(), SC_HELP);
}
