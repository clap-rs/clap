// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

#[macro_use]
extern crate clap;

use clap::Clap;
use std::path::PathBuf;

/// A basic example
#[derive(Clap, Debug)]
#[clap(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate debug mode
    #[clap(short = "d", long = "debug")]
    debug: bool,

    // The number of occurences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Set speed
    #[clap(short = "s", long = "speed", default_value = "42")]
    speed: f64,

    /// Output file
    #[clap(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// Number of cars
    #[clap(short = "c", long = "nb-cars")]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[clap(short = "l", long = "level")]
    level: Vec<String>,

    /// Files to process
    #[clap(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
