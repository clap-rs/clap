use clap::{App, Arg, ValueHint};
use clap_generate::{generate, generators::*};
use std::fmt;

mod bash;
mod elvish;
mod fish;
mod powershell;
mod zsh;

#[derive(PartialEq, Eq)]
pub struct PrettyString<'a>(pub &'a str);

impl<'a> fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        pretty_assertions::assert_eq!(PrettyString($left), PrettyString($right));
    };
}

fn build_app() -> App<'static> {
    build_app_with_name("myapp")
}

fn build_app_with_name(s: &'static str) -> App<'static> {
    App::new(s)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .about("some input file"),
        )
        .subcommand(
            App::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .takes_value(true)
                    .about("the case to test"),
            ),
        )
}

fn build_app_special_commands() -> App<'static> {
    build_app_with_name("my_app")
        .subcommand(
            App::new("some_cmd").about("tests other things").arg(
                Arg::new("config")
                    .long("--config")
                    .takes_value(true)
                    .about("the other case to test"),
            ),
        )
        .subcommand(App::new("some-cmd-with-hypens").alias("hyphen"))
}

fn build_app_special_help() -> App<'static> {
    App::new("my_app")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .about("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .about("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .about("For more information see `echo test`"),
        )
        .arg(Arg::new("backslash").long("backslash").about("Avoid '\\n'"))
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .about("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .about("Execute the shell command with $SHELL"),
        )
}

fn build_app_nested_subcommands() -> App<'static> {
    App::new("first").subcommand(App::new("second").subcommand(App::new("third")))
}

pub fn common<G: Generator>(app: &mut App, name: &str, fixture: &str) {
    let mut buf = vec![];
    generate::<G, _>(app, name, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert_eq!(&string, fixture);
}
