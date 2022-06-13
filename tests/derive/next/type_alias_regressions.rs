//! Regression test to ensure that type aliases do not cause compilation failures.
#![allow(deprecated)]

use clap::{Parser, Subcommand, ValueEnum};

// Result type alias
#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Option<T> = std::option::Option<T>;

#[derive(Parser)]
pub struct Opts {
    #[clap(value_parser)]
    another_string: String,
    #[clap(subcommand)]
    command: Command,
    #[clap(short, long, value_enum, value_parser)]
    choice: ArgChoice,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Command {
    DoSomething { arg: Option<String> },
}

#[derive(ValueEnum, PartialEq, Debug, Clone)]
enum ArgChoice {
    Foo,
    Bar,
}

#[test]
fn type_alias_regressions() {
    Opts::try_parse_from(["test", "value", "--choice=foo", "do-something"]).unwrap();
}
