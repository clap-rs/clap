#![allow(elided_lifetimes_in_paths)] // needed for divan

use clap::{arg, ArgMatches, Command};

macro_rules! create_app {
    () => {{
        Command::new("claptests")
            .version("0.1")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .arg(arg!(-f --flag         "tests flags"))
            .arg(arg!(-o --option <opt> "tests options"))
            .arg(arg!([positional]      "tests positional"))
    }};
}

#[divan::bench]
fn build() -> Command {
    create_app!()
}

mod startup {
    use super::{arg, ArgMatches, Command};

    #[divan::bench]
    fn flag() -> ArgMatches {
        create_app!().get_matches_from(vec!["myprog", "-f"])
    }

    #[divan::bench]
    fn opt() -> ArgMatches {
        create_app!().get_matches_from(vec!["myprog", "-o", "option1"])
    }

    #[divan::bench]
    fn pos() -> ArgMatches {
        create_app!().get_matches_from(vec!["myprog", "arg1"])
    }
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

fn main() {
    divan::main();
}
