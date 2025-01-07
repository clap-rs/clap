use clap::{error::ErrorKind, Arg, ArgAction, Command};
use snapbox::str;

use crate::utils;

fn cmd() -> Command {
    Command::new("prog")
        .arg(
            Arg::new("a")
                .short('a')
                .action(ArgAction::SetTrue)
                .required_unless_present_any(["b", "c"])
                .conflicts_with_all(["b", "c"]),
        )
        .arg(
            Arg::new("b")
                .short('b')
                .action(ArgAction::SetTrue)
                .required_unless_present("a")
                .requires("c"),
        )
        .arg(
            Arg::new("c")
                .short('c')
                .action(ArgAction::SetTrue)
                .required_unless_present("a")
                .requires("b"),
        )
}

#[test]
fn valid_cases() {
    let res = cmd().try_get_matches_from(vec!["", "-a"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = cmd().clone().try_get_matches_from(vec!["", "-b", "-c"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = cmd().try_get_matches_from(vec!["", "-c", "-b"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn help_text() {
    let res = cmd().try_get_matches_from(vec!["prog", "--help"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    utils::assert_error(err, ErrorKind::DisplayHelp, str![[r#"
Usage: prog [OPTIONS]

Options:
  -a          
  -b          
  -c          
  -h, --help  Print help

"#]], false);
}

#[test]
#[cfg(feature = "error-context")]
fn no_duplicate_error() {
    let res = cmd().try_get_matches_from(vec!["", "-b"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    utils::assert_error(
        err,
        ErrorKind::MissingRequiredArgument,
        str![[r#"
error: the following required arguments were not provided:
  -c

Usage: prog -b -c

For more information, try '--help'.

"#]],
        true,
    );

    let res = cmd().try_get_matches_from(vec!["", "-c"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    utils::assert_error(
        err,
        ErrorKind::MissingRequiredArgument,
        str![[r#"
error: the following required arguments were not provided:
  -b

Usage: prog -c -b

For more information, try '--help'.

"#]],
        true,
    );
}
