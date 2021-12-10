/// Regression test to ensure that type aliases do not cause compilation failures.
use clap::Parser;
use std::str::FromStr;

// Result type alias
#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Wrapper to use for Option type alias
struct Wrapper<T>(T);

impl<T: FromStr> FromStr for Wrapper<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        T::from_str(s).map(Wrapper)
    }
}

type Option<T> = std::option::Option<Wrapper<T>>;

#[derive(Parser)]
pub struct Opts {
    another_string: Option<String>,
    strings: Vec<String>,
}

#[test]
fn type_alias_regressions() {
    Opts::parse_from(["test"]);
}
