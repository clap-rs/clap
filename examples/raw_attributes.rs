// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
use structopt::clap::AppSettings;

/// An example of raw attributes
#[derive(StructOpt, Debug)]
#[structopt(global_settings_raw = "&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]")]
struct Opt {
    /// Output file
    #[structopt(short = "o", long = "output")]
    output: String,

    /// admin_level to consider
    #[structopt(short = "l", long = "level", aliases_raw = "&[\"set-level\", \"lvl\"]")]
    level: Vec<String>,

    /// Files to process
    ///
    /// `level` is required if a file is called `FILE`.
    #[structopt(name = "FILE", requires_if_raw = "\"FILE\", \"level\"")]
    files: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
