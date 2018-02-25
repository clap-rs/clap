// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use structopt::clap::AppSettings;

/// An example of raw attributes
#[derive(StructOpt, Debug)]
#[structopt(raw(global_settings = "&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]"))]
struct Opt {
    /// Output file
    #[structopt(short = "o", long = "output")]
    output: String,

    /// admin_level to consider
    #[structopt(short = "l", long = "level", raw(aliases = r#"&["set-level", "lvl"]"#))]
    level: Vec<String>,

    /// Files to process
    ///
    /// `level` is required if a file is called `FILE`.
    #[structopt(name = "FILE", raw(requires_if = r#""FILE", "level""#))]
    files: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
