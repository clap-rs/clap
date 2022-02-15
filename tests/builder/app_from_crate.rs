#![cfg(feature = "cargo")]
#![allow(deprecated)]

use clap::{app_from_crate, error::ErrorKind};

static EVERYTHING: &str = "clap {{version}}
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    clap

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

#[test]
fn app_from_crate() {
    let res = app_from_crate!().try_get_matches_from(vec!["clap", "--help"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(
        err.to_string(),
        EVERYTHING.replace("{{version}}", env!("CARGO_PKG_VERSION"))
    );
}
