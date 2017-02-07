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

#[derive(StructOpt, Debug)]
#[structopt(name = "basic", about = "A basic example")]
struct Opt {
    #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
    debug: bool,
    #[structopt(short = "v", long = "verbose", help = "Verbose mode")]
    verbose: u64,
    #[structopt(short = "s", long = "speed", help = "Set speed", default_value = "42")]
    speed: f64,
    #[structopt(short = "o", long = "output", help = "Output file")]
    output: String,
    #[structopt(short = "c", long = "car", help = "Number of car")]
    car: Option<i32>,
    #[structopt(short = "l", long = "level")]
    #[structopt(help = "admin_level to consider")]
    level: Vec<String>,
    #[structopt(help = "Files to process")]
    files: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
