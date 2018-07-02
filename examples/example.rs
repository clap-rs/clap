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

#[derive(Clap, Debug)]
#[clap(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// A flag, true if used in the command line.
    #[clap(short = "d", long = "debug", help = "Activate debug mode")]
    debug: bool,

    /// An argument of type float, with a default value.
    #[clap(short = "s", long = "speed", help = "Set speed", default_value = "42")]
    speed: f64,

    /// Needed parameter, the first on the command line.
    #[clap(help = "Input file")]
    input: String,

    /// An optional parameter, will be `None` if not present on the
    /// command line.
    #[clap(help = "Output file, stdout if not present")]
    output: Option<String>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
