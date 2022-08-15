//! Example to test subcommands.
//!
//! Usage with bash:
//! ```sh
//! cargo run --example completion_subcommands -- --generate=bash > completion_subcommands.bash
//! . ./completion_subcommands.bash
//! ./target/debug/examples/completion_subcommands <TAB>
//! ./target/debug/examples/completion_subcommands help <TAB>
//! ```
use clap::{value_parser, Arg, Command};
use clap_complete::{generate, Generator, Shell};
use std::io;

fn build_cli() -> Command<'static> {
    Command::new("completion_subcommands")
        .arg(
            Arg::new("generator")
                .long("generate")
                .value_parser(value_parser!(Shell)),
        )
        .subcommand(
            Command::new("subcmd")
                .about("run a subcommand")
                .subcommand(Command::new("subsubcmd")),
        )
        .subcommand(Command::new("othersubcmd"))
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let matches = build_cli().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator") {
        let mut cmd = build_cli();
        eprintln!("Generating completion file for {}...", generator);
        print_completions(*generator, &mut cmd);
    }
}
