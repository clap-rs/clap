#![allow(unused_imports, dead_code)]

use std::io::{Cursor, Write};
use std::str;

use regex::Regex;

use clap::{App, Arg, ArgGroup};

pub fn compare<S, S2>(l: S, r: S2) -> bool
where
    S: AsRef<str>,
    S2: AsRef<str>,
{
    let re = Regex::new("\x1b[^m]*m").unwrap();
    // Strip out any mismatching \r character on windows that might sneak in on either side
    let ls = l.as_ref().replace("\r", "");
    let rs = r.as_ref().replace("\r", "");
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
        .arg(Arg::from_usage(
            "[option] -o --option=[opt]... 'tests options'",
        ))
        .arg(Arg::from_usage("[positional] 'tests positionals'"))
        .arg(Arg::from_usage("-f --flag... 'tests flags'").global(true))
        .args(&[
            Arg::from_usage("[flag2] -F 'tests flags with exclusions'")
                .conflicts_with("flag")
                .requires("long-option-2"),
            Arg::from_usage("--long-option-2 [option2] 'tests long options with exclusions'")
                .conflicts_with("option")
                .requires("positional2"),
            Arg::from_usage("[positional2] 'tests positionals with exclusions'"),
            Arg::from_usage("-O --option3 [option3] 'specific vals'").possible_values(opt3_vals),
            Arg::from_usage("[positional3]... 'tests specific values'").possible_values(pos3_vals),
            Arg::from_usage("--multvals [one] [two] 'Tests multiple values, not mult occs'"),
            Arg::from_usage("--multvalsmo... [one] [two] 'Tests multiple values, and mult occs'"),
            Arg::from_usage("--minvals2 [minvals]... 'Tests 2 min vals'").min_values(2),
            Arg::from_usage("--maxvals3 [maxvals]... 'Tests 3 max vals'").max_values(3),
        ])
        .subcommand(
            App::new("subcmd")
                .about("tests subcommands")
                .version("0.1")
                .author("Kevin K. <kbknapp@gmail.com>")
                .arg(Arg::from_usage(
                    "[option] -o --option [scoption]... 'tests options'",
                ))
                .arg(Arg::from_usage(
                    "-s --subcmdarg [subcmdarg] 'tests other args'",
                ))
                .arg(Arg::from_usage("[scpositional] 'tests positionals'")),
        )
}
