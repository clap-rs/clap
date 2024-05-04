use clap::{arg, Command};
use clap_mangen::Man;
use std::io;

// Run this example as `cargo run --example man | man -l -`.

fn main() -> Result<(), io::Error> {
    let cmd = Command::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>:Ola Nordmann <old@nordmann.no>")
        .about("Does awesome things")
        .long_about(
            "With a longer description to help clarify some things.

And a few newlines.",
        )
        .after_help("This is an extra section added to the end of the manpage.")
        .after_long_help("With even more text added.")
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                .long_help("Some more text about how to set a custom config file")
                .default_value("config.toml")
                .env("CONFIG_FILE"),
        )
        .arg(arg!([output] "Sets an output file").default_value("result.txt"))
        .arg(
            arg!(-d --debug ... "Turn debugging information on")
                .env("DEBUG_ON")
                .hide_env(true),
        )
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "Lists test values")),
        );

    Man::new(cmd).render(&mut io::stdout())
}
