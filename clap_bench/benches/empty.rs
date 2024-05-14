#![allow(elided_lifetimes_in_paths)] // needed for divan

use clap::ArgMatches;
use clap::Command;

macro_rules! create_app {
    () => {{
        Command::new("claptests")
    }};
}

#[divan::bench]
fn build() -> Command {
    create_app!()
}

#[divan::bench]
fn startup() -> ArgMatches {
    create_app!().get_matches_from(vec![""])
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
