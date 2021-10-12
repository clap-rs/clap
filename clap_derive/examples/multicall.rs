//! Example of multicall program using clap_derive
//!
//! It works just like using subcommands,
//! but you use the setting attribute to set the Multicall flag.

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_CRATE_NAME"), setting = clap::AppSettings::Multicall)]
enum Args {
    Foo,
    Bar,
}

fn main() {
    match Args::parse() {
        Args::Foo => println!("foo"),
        Args::Bar => println!("bar"),
    }
}
