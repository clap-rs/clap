// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

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
