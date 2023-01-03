#![cfg(feature = "cargo")]

use clap::{
    crate_authors, crate_description, crate_name, crate_version, error::ErrorKind, Command,
};

use crate::utils;

static DESCRIPTION_ONLY: &str = "prog 1
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: prog

Options:
  -h, --help     Print help
  -V, --version  Print version
";

static AUTHORS_ONLY: &str = "prog 1


Usage: prog

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn crate_version() {
    let res = Command::new("prog")
        .version(crate_version!())
        .help_template(utils::FULL_TEMPLATE)
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
        .help_template(utils::FULL_TEMPLATE)
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
        .help_template(utils::FULL_TEMPLATE)
        .try_get_matches_from(vec!["prog", "--help"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(err.to_string(), AUTHORS_ONLY);
}

#[test]
fn crate_authors_with_sep() {
    let res = Command::new("prog")
        .version("1")
        .author(crate_authors!(", "))
        .help_template(utils::FULL_TEMPLATE)
        .try_get_matches_from(vec!["prog", "--help"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(err.to_string(), AUTHORS_ONLY);
}

#[test]
fn crate_name() {
    let res = Command::new(crate_name!())
        .help_template(utils::FULL_TEMPLATE)
        .version("3.0")
        .try_get_matches_from(vec!["clap", "--version"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "clap 3.0\n");
}
