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
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
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

    #[cfg(feature = "unstable-multicall")]
    let app = app.setting(AppSettings::Multicall);
    let matches = app.get_matches();
    if matches.occurrences_of("install") > 0 {
        unimplemented!("Make hardlinks to the executable here");
    }

    match matches.subcommand_name() {
        Some("true") => exit(0),
        Some("false") => exit(1),
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
