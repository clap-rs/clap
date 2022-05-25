#![cfg(feature = "regex")]
#![allow(deprecated)]

use clap::{error::ErrorKind, Arg, Command};
use regex::{Regex, RegexSet};

#[test]
fn validator_regex() {
    let priority = Regex::new(r"[A-C]").unwrap();

    let m = Command::new("prog")
        .arg(
            Arg::new("priority")
                .index(1)
                .validator_regex(priority, "A, B or C are allowed"),
        )
        .try_get_matches_from(vec!["prog", "12345"]);

    assert!(m.is_err());
    assert_eq!(m.err().unwrap().kind(), ErrorKind::ValueValidation)
}

#[test]
fn validator_regex_with_regex_set() {
    let priority = RegexSet::new(&[r"[A-C]", r"[X-Z]"]).unwrap();

    let m = Command::new("prog")
        .arg(
            Arg::new("priority")
                .index(1)
                .validator_regex(priority, "A, B, C, X, Y or Z are allowed"),
        )
        .try_get_matches_from(vec!["prog", "12345"]);

    assert!(m.is_err());
    assert_eq!(m.err().unwrap().kind(), ErrorKind::ValueValidation)
}
