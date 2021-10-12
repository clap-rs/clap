//! Example of `hostname`-style multicall program
//!
//! See the documentation for clap::AppSettings::Multicall for rationale.
//!
//! This example omits the implementation of displaying address config.

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_CRATE_NAME"), setting = clap::AppSettings::Multicall)]
enum Args {
    /// Show the configured hostname
    Hostname,
    /// Show the domain name part of the configured hostname
    #[clap(about = "show domain")]
    DNSDomainName,
}

fn main() {
    match Args::parse() {
        Args::Hostname => println!("www"),
        Args::DNSDomainName => println!("example.com"),
    }
}
