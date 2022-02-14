use clap::Command;
use clap_complete::generate;
use clap_complete_fig::Fig;
use std::io;

fn main() {
    let mut cmd = Command::new("myapp")
        .subcommand(Command::new("test").subcommand(Command::new("config")))
        .subcommand(Command::new("hello"));

    generate(Fig, &mut cmd, "myapp", &mut io::stdout());
}
