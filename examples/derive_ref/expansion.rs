//! Idiomatic builder equivalent of a small `#[derive(Parser)]` CLI.
//!
//! This is *not* raw `cargo expand` output: it shows the builder methods the
//! derive maps onto so you can translate any builder docs example into derive
//! attributes (and vice versa).

#![allow(dead_code)]

use clap::{
    Arg, Command, CommandFactory, Parser, Subcommand, ValueEnum, builder::PossibleValue,
    value_parser,
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Output format
    #[arg(long, value_enum, default_value_t = Format::Text)]
    format: Format,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    /// Human-readable text
    Text,
    /// Machine-readable JSON
    Json,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show the current configuration
    Config {
        /// Include defaults
        #[arg(long)]
        show_defaults: bool,
    },
}

/// Builder form roughly produced by the derive above.
fn cli_as_builder() -> Command {
    // `#[command(version, about, long_about = None)]` + doc comment → Command methods
    Command::new("cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Simple program to greet a person")
        .long_about(None)
        // `#[arg(short, long)] name: String` → Arg methods
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .required(true),
        )
        // `#[arg(short, long, default_value_t = 1)] count: u8`
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .default_value("1"),
        )
        // `#[arg(long, value_enum, default_value_t = Format::Text)]`
        // ValueEnum variants become PossibleValue entries (help from doc comments).
        .arg(
            Arg::new("format")
                .long("format")
                .value_parser(value_parser!(Format))
                .default_value("text"),
        )
        // `#[command(subcommand)] command: Option<Commands>`
        .subcommand(
            Command::new("config")
                .about("Show the current configuration")
                .arg(
                    Arg::new("show_defaults")
                        .long("show-defaults")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
}

// Explicit PossibleValue form of `Format` (what `ValueEnum` builds for you):
fn format_possible_values() -> [PossibleValue; 2] {
    [
        PossibleValue::new("text").help("Human-readable text"),
        PossibleValue::new("json").help("Machine-readable JSON"),
    ]
}

fn main() {
    let _ = cli_as_builder();
    let _ = format_possible_values();
    // Derive side: `CommandFactory` is what `Parser` generates for you.
    let _ = Cli::command();
}
