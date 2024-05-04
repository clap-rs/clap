#![allow(elided_lifetimes_in_paths)] // needed for divan

use clap::{arg, ArgMatches, Command};

macro_rules! create_app {
    () => {{
        Command::new("claptests")
            .version("0.1")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .arg(arg!(-o --option <opt> ... "tests options"))
            .arg(arg!([positional] "tests positionals"))
            .arg(arg!(-f --flag ... "tests flags").global(true))
            .args([
                arg!(flag2: -F "tests flags with exclusions")
                    .conflicts_with("flag")
                    .requires("option2"),
                arg!(option2: --"long-option-2" <option2> "tests long options with exclusions")
                    .conflicts_with("option")
                    .requires("positional2"),
                arg!([positional2] "tests positionals with exclusions"),
                arg!(-O --Option <option3> "tests options with specific value sets")
                    .value_parser(["fast", "slow"]),
                arg!([positional3] ... "tests positionals with specific values")
                    .value_parser(["vi", "emacs"]),
                arg!(--multvals <s> "Tests multiple values not mult occs").value_names(["one", "two"]),
                arg!(
                    --multvalsmo <s> "Tests multiple values, not mult occs"
                ).required(false).value_names(["one", "two"]),
                arg!(--minvals2 <minvals> ... "Tests 2 min vals").num_args(2..),
                arg!(--maxvals3 <maxvals> ... "Tests 3 max vals").num_args(1..=3),
            ])
            .subcommand(
                Command::new("subcmd")
                    .about("tests subcommands")
                    .version("0.1")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .arg(arg!(-o --option <scoption> ... "tests options"))
                    .arg(arg!([scpositional] "tests positionals"))
            )
    }};
}

#[divan::bench]
fn build() -> Command {
    create_app!()
}

#[divan::bench(args=COMPLEX_ARGS)]
fn startup(args: &Args) -> ArgMatches {
    create_app!().get_matches_from(args.args())
}

#[divan::bench]
fn render_help(bencher: divan::Bencher) {
    let mut cmd = create_app!();
    bencher.bench_local(|| build_help(&mut cmd));
}

fn build_help(cmd: &mut Command) -> String {
    let help = cmd.render_help();
    help.to_string()
}

const COMPLEX_ARGS: &[Args] = &[
    Args("empty", &[""]),
    Args("flag", &["myprog", "-f"]),
    Args("opt", &["myprog", "-o", "option1"]),
    Args("pos", &["myprog", "arg1"]),
    Args("sc", &["myprog", "subcmd"]),
    Args("sc_flag", &["myprog", "subcmd", "-f"]),
    Args("sc_opt", &["myprog", "subcmd", "-o", "option1"]),
    Args("sc_pos", &["myprog", "subcmd", "arg1"]),
    Args(
        "sc_nested",
        &["myprog", "subcmd", "-f", "-o", "option1", "arg1"],
    ),
    Args(
        "mixed1",
        &[
            "myprog",
            "-ff",
            "-o",
            "option1",
            "arg1",
            "-O",
            "fast",
            "arg2",
            "--multvals",
            "one",
            "two",
            "emacs",
        ],
    ),
    Args(
        "mixed2",
        &[
            "myprog",
            "arg1",
            "-f",
            "arg2",
            "--long-option-2",
            "some",
            "-O",
            "slow",
            "--multvalsmo",
            "one",
            "two",
            "--minvals2",
            "3",
            "2",
            "1",
        ],
    ),
];

#[derive(Debug)]
pub struct Args(&'static str, &'static [&'static str]);

impl Args {
    pub const fn name(&self) -> &'static str {
        self.0
    }

    pub const fn args(&self) -> &[&str] {
        self.1
    }
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

fn main() {
    divan::main();
}
