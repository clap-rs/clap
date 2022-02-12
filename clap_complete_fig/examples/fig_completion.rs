use clap::Command;
use clap_complete::generate;
use clap_complete_fig::Fig;
use std::io;

fn main() {
    let mut app = Command::new("myapp")
        .subcommand(Command::new("test").subcommand(Command::new("config")))
        .subcommand(Command::new("hello"));

    generate(Fig, &mut app, "myapp", &mut io::stdout());
}
