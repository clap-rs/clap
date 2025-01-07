#![allow(unused_imports, dead_code)]

use std::io::{BufRead, Cursor, Write};
use std::str;

use clap::{arg, error::Error, error::ErrorKind, Arg, ArgAction, ArgGroup, Command};
use snapbox::assert_data_eq;

pub(crate) const FULL_TEMPLATE: &str = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}";

#[track_caller]
pub(crate) fn assert_output(
    l: Command,
    args: &str,
    expected: impl snapbox::data::IntoData,
    stderr: bool,
) {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    let res = l.try_get_matches_from(args.split(' ').collect::<Vec<_>>());
    let err = res.unwrap_err();
    write!(&mut buf, "{err}").unwrap();
    let actual = buf.into_inner();
    let actual = String::from_utf8(actual).unwrap();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    assert_data_eq!(actual, expected.raw());
}

#[track_caller]
pub(crate) fn assert_error<F: clap::error::ErrorFormatter>(
    err: Error<F>,
    expected_kind: ErrorKind,
    expected_output: impl snapbox::data::IntoData,
    stderr: bool,
) {
    let actual_output = err.to_string();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    assert_eq!(expected_kind, err.kind());
    #[cfg(feature = "error-context")]
    assert_data_eq!(actual_output, expected_output);
}

// Legacy tests from the python script days

pub(crate) fn complex_app() -> Command {
    let opt3_vals = ["fast", "slow"];
    let pos3_vals = ["vi", "emacs"];

    Command::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .author("Kevin K. <kbknapp@gmail.com>")
        .help_template(FULL_TEMPLATE)
        .arg(
            arg!(
                -o --option <opt> "tests options"
            )
            .required(false)
            .num_args(1..)
            .action(ArgAction::Append),
        )
        .arg(arg!([positional] "tests positionals"))
        .arg(
            arg!(-f --flag  "tests flags")
                .action(ArgAction::Count)
                .global(true),
        )
        .args([
            arg!(flag2: -F "tests flags with exclusions")
                .conflicts_with("flag")
                .requires("long-option-2")
                .action(ArgAction::SetTrue),
            arg!(--"long-option-2" <option2> "tests long options with exclusions")
                .conflicts_with("option")
                .requires("positional2"),
            arg!([positional2] "tests positionals with exclusions"),
            arg!(-O --option3 <option3> "specific vals").value_parser(opt3_vals),
            arg!([positional3] ... "tests specific values").value_parser(pos3_vals),
            arg!(--multvals <val> "Tests multiple values, not mult occs")
                .value_names(["one", "two"]),
            arg!(--multvalsmo <val> ... "Tests multiple values, and mult occs")
                .value_names(["one", "two"]),
            arg!(--minvals2 <minvals> "Tests 2 min vals").num_args(2..),
            arg!(--maxvals3 <maxvals> "Tests 3 max vals").num_args(1..=3),
            arg!(--optvaleq <optval> "Tests optional value, require = sign")
                .num_args(0..=1)
                .require_equals(true),
            arg!(--optvalnoeq <optval> "Tests optional value").num_args(0..=1),
        ])
        .subcommand(
            Command::new("subcmd")
                .about("tests subcommands")
                .version("0.1")
                .author("Kevin K. <kbknapp@gmail.com>")
                .help_template(FULL_TEMPLATE)
                .arg(arg!(-o --option <scoption> "tests options").num_args(1..))
                .arg(arg!(-s --subcmdarg <subcmdarg> "tests other args"))
                .arg(arg!([scpositional] "tests positionals")),
        )
}
