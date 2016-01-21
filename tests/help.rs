extern crate clap;

use clap::{App, SubCommand, ErrorKind};

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
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnexpectedArgument);
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
fn print_app_help() {
    let mut app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .args_from_usage("-f, --flag 'some flag'
                          --option [opt] 'some option'");
    // We call a get_matches method to cause --help and --version to be built
    let _ = app.get_matches_from_safe_borrow(vec![""]);

    // Now we check the output of print_help()
    let mut help = vec![];
    app.write_help(&mut help).ok().expect("failed to print help");
    assert_eq!(&*String::from_utf8_lossy(&*help), &*String::from("test 1.3\n\
Kevin K.
tests stuff

USAGE:
\ttest [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       some flag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --option <opt>    some option\n"));
}
