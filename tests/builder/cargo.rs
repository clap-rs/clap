#![cfg(feature = "cargo")]

use clap::{
    crate_authors, crate_description, crate_name, crate_version, error::ErrorKind, Command,
};

static DESCRIPTION_ONLY: &str = "prog 1
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    prog

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static AUTHORS_ONLY: &str = "prog 1


USAGE:
    prog

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

#[test]
fn crate_version() {
    let res = Command::new("prog")
        .version(crate_version!())
        .try_get_matches_from(vec!["prog", "--version"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(
        err.to_string(),
        format!("prog {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn crate_description() {
    let res = Command::new("prog")
        .version("1")
        .about(crate_description!())
        .try_get_matches_from(vec!["prog", "--help"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(err.to_string(), DESCRIPTION_ONLY);
}

#[test]
fn crate_authors() {
    let res = Command::new("prog")
        .version("1")
        .author(crate_authors!())
        .try_get_matches_from(vec!["prog", "--help"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(err.to_string(), AUTHORS_ONLY);
}

#[test]
fn crate_name() {
    let res = Command::new(crate_name!())
        .version("3.0")
        .try_get_matches_from(vec!["clap", "--version"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "clap 3.0\n");
}
