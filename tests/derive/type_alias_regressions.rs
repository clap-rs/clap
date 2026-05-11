//! Regression test to ensure that type aliases do not cause compilation failures.

use clap::{Parser, Subcommand, ValueEnum};

// Result type alias
#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Option<T> = std::option::Option<T>;

#[derive(Parser)]
pub(crate) struct Opts {
    another_string: String,
    #[command(subcommand)]
    command: Command,
    #[arg(short, long, value_enum)]
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
