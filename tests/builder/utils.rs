#![allow(unused_imports, dead_code)]

use std::io::{Cursor, Write};
use std::str;

use regex::Regex;

use clap::{arg, App, Arg, ArgGroup};

pub fn compare<S, S2>(l: S, r: S2) -> bool
where
    S: AsRef<str>,
    S2: AsRef<str>,
{
    let re = Regex::new("\x1b[^m]*m").unwrap();
    // Strip out any mismatching \r character on windows that might sneak in on either side
    let ls = l.as_ref().replace('\r', "");
    let rs = r.as_ref().replace('\r', "");
    let left_ = re.replace_all(&*ls, "");
    let right = re.replace_all(&*rs, "");
    let b = left_ == right;
    if !b {
        dbg!(&left_);
        dbg!(&right);
        println!();
        println!("--> left");
        println!("{}", left_);
        println!("--> right");
        println!("{}", right);
        println!("--")
    }
    b
}

pub fn compare_output(l: App, args: &str, right: &str, stderr: bool) -> bool {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    let res = l.try_get_matches_from(args.split(' ').collect::<Vec<_>>());
    let err = res.unwrap_err();
    write!(&mut buf, "{}", err).unwrap();
    let content = buf.into_inner();
    let left = String::from_utf8(content).unwrap();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    compare(left, right)
}

// Legacy tests from the python script days

pub fn complex_app() -> App<'static> {
    let opt3_vals = ["fast", "slow"];
    let pos3_vals = ["vi", "emacs"];

    App::new("clap-test")
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
        ])
        .subcommand(
            App::new("subcmd")
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
