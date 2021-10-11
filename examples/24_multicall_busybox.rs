//! Example of a `busybox-style` multicall program
//!
//! See the documentation for clap::AppSettings::Multicall for rationale.
//!
//! This example omits every command except true and false,
//! which are the most trivial to implement,
//! but includes the `--install` option as an example of why it can be useful
//! for the main program to take arguments that aren't applet subcommands.

use std::process::exit;

use clap::{App, AppSettings, Arg};

fn main() {
    let mut app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::Multicall)
        .arg(
            Arg::new("install")
                .long("install")
                .about("Install hardlinks for all subcommands in path")
                .exclusive(true)
                .takes_value(true)
                .default_missing_value("/usr/local/bin")
                .use_delimiter(false),
        )
        .subcommand(App::new("true").about("does nothing successfully"))
        .subcommand(App::new("false").about("does nothing unsuccessfully"));
    let matches = app.get_matches_mut();
    if matches.occurrences_of("install") > 0 {
        unimplemented!("Make hardlinks to the executable here");
    }

    exit(match matches.subcommand_name() {
        Some("true") => 0,
        Some("false") => 1,
        _ => 127,
    })
}
