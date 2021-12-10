/// Regression test to ensure that type aliases do not cause compilation failures.
use std::str::FromStr;

use clap::{ArgEnum, Parser, Subcommand};

// Result type alias
#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Wrapper to use for Option type alias
#[derive(Debug, PartialEq, Eq)]
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
    another_string: String,
    #[clap(subcommand)]
    command: Command,
    #[clap(short, long, arg_enum)]
    choice: ArgChoice,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Command {
    DoSomething { arg: Option<String> },
}

#[derive(ArgEnum, PartialEq, Debug, Clone)]
enum ArgChoice {
    Foo,
    Bar,
}

#[test]
fn type_alias_regressions() {
    Opts::try_parse_from(["test", "value", "--choice=foo", "do-something"]).unwrap();
}
