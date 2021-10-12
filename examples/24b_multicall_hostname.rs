//! Example of a `hostname-style` multicall program
//!
//! See the documentation for clap::AppSettings::Multicall for rationale.
//!
//! This example omits the implementation of displaying address config

use std::process::exit;

use clap::{App, AppSettings};

fn main() {
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::Multicall)
        .subcommand(App::new("hostname").about("shot hostname part of FQDN"))
        .subcommand(App::new("dnsdomainname").about("show domain name part of FQDN"));

    match app.get_matches().subcommand_name() {
        Some("hostname") => println!("www"),
        Some("dnsdomainname") => println!("example.com"),
        _ => exit(127),
    }
}
