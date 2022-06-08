#![allow(unused_imports, dead_code)]

use std::io::{BufRead, Cursor, Write};
use std::str;

use regex::Regex;

use clap::{arg, Arg, ArgGroup, Command};

#[track_caller]
pub fn assert_eq<S, S2>(expected: S, actual: S2)
where
    S: AsRef<str>,
    S2: AsRef<str>,
{
    let expected = expected.as_ref();
    let actual = actual.as_ref();
    snapbox::assert_eq(expected, actual);
}

#[track_caller]
pub fn assert_output(l: Command, args: &str, expected: &str, stderr: bool) {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    let res = l.try_get_matches_from(args.split(' ').collect::<Vec<_>>());
    let err = res.unwrap_err();
    write!(&mut buf, "{}", err).unwrap();
    let actual = buf.into_inner();
    let actual = String::from_utf8(actual).unwrap();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    assert_eq(expected, actual)
}

// Legacy tests from the python script days

pub fn complex_app() -> Command<'static> {
    let opt3_vals = ["fast", "slow"];
    let pos3_vals = ["vi", "emacs"];

    Command::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .author("Kevin K. <kbknapp@gmail.com>")
        .arg(
            arg!(
                -o --option <opt> "tests options"
            )
            .required(false)
            .multiple_values(true)
            .multiple_occurrences(true),
        )
        .arg(arg!([positional] "tests positionals"))
        .arg(arg!(-f --flag ... "tests flags").global(true))
        .args(&[
            arg!(flag2: -F "tests flags with exclusions")
                .conflicts_with("flag")
                .requires("long-option-2"),
            arg!(--"long-option-2" <option2> "tests long options with exclusions")
                .required(false)
                .conflicts_with("option")
                .requires("positional2"),
            arg!([positional2] "tests positionals with exclusions"),
            arg!(-O --option3 <option3> "specific vals")
                .required(false)
                .possible_values(opt3_vals),
            arg!([positional3] ... "tests specific values").possible_values(pos3_vals),
            arg!(--multvals "Tests multiple values, not mult occs")
                .value_names(&["one", "two"])
                .required(false),
            arg!(--multvalsmo ... "Tests multiple values, and mult occs")
                .value_names(&["one", "two"])
                .required(false),
            arg!(--minvals2 <minvals> "Tests 2 min vals")
                .required(false)
                .min_values(2),
            arg!(--maxvals3 <maxvals> "Tests 3 max vals")
                .required(false)
                .max_values(3),
            arg!(--optvaleq <optval> "Tests optional value, require = sign")
                .required(false)
                .min_values(0)
                .number_of_values(1)
                .require_equals(true),
            arg!(--optvalnoeq <optval> "Tests optional value")
                .required(false)
                .min_values(0)
                .number_of_values(1),
        ])
        .subcommand(
            Command::new("subcmd")
                .about("tests subcommands")
                .version("0.1")
                .author("Kevin K. <kbknapp@gmail.com>")
                .arg(
                    arg!(-o --option <scoption> "tests options")
                        .required(false)
                        .multiple_values(true),
                )
                .arg(arg!(-s --subcmdarg <subcmdarg> "tests other args").required(false))
                .arg(arg!([scpositional] "tests positionals")),
        )
}
