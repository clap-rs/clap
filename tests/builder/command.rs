#![cfg(feature = "cargo")]

use clap::{command, error::ErrorKind};

use crate::utils;

static EVERYTHING: &str = "clap {{version}}
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: clap

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn command() {
    let res = command!()
        .help_template(utils::FULL_TEMPLATE)
        .try_get_matches_from(vec!["clap", "--help"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(
        err.to_string(),
        EVERYTHING.replace("{{version}}", env!("CARGO_PKG_VERSION"))
    );
}
