// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate clap;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    debug: bool,
    verbose: u64,
    speed: Option<f64>,
    output: String,
    level: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
