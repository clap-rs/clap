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

use clap::{AppSettings, Clap};

/// An example of raw attributes
#[derive(Clap, Debug)]
#[clap(raw(global_setting = "AppSettings::ColoredHelp"))]
#[clap(raw(global_setting = "AppSettings::VersionlessSubcommands"))]
struct Opt {
    /// Output file
    #[clap(short = "o", long = "output")]
    output: String,

    /// admin_level to consider
    #[clap(short = "l", long = "level", raw(aliases = r#"&["set-level", "lvl"]"#))]
    level: Vec<String>,

    /// Files to process
    ///
    /// `level` is required if a file is called `FILE`.
    #[clap(name = "FILE", raw(requires_if = r#""FILE", "level""#))]
    files: Vec<String>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
