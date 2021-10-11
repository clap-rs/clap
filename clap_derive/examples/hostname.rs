//! Example of `hostname`-style multicall program
//!
//! `hostname` is a command to just output the configured hostname.
//! It may also render other network-address related information
//! if linked to under a different name
//! e.g. `dnsdomainname` for the domain name part of the FQDN.
//!
//! `hostname`-style differs from `busybox`-style in that the applets
//! should not ever be available as a subcommand
//! and the name of the executable is the same as an applet.
//!
//! i.e. `hostname` doesn't have a `dnsdomainname` subcommand,
//! `dnsdomainname` must only be run via a soft or hard link to the executable.
//!
//! This behaviour is opted-into by naming an applet subcommand
//! the same as the program name.
//!
//! This is desirable when the executable has a primary purpose
//! rather than being a collection of varied applets,
//! so it is appropriate to name the executable after its purpose,
//! but there is other related functionality that would be convenient to provide
//! and it is convenient for the code to implement it to be in the same executable.
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
