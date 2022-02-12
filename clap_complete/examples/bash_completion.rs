use clap::Command;
use clap_complete::{generate, shells::Bash};
use std::io;

fn main() {
    let mut app = Command::new("myapp")
        .subcommand(Command::new("test").subcommand(Command::new("config")))
        .subcommand(Command::new("hello"));

    generate(Bash, &mut app, "myapp", &mut io::stdout());
}
