// Copyright (c) 2018 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

// This should be in tests but it will not work until
// https://github.com/rust-lang/rust/issues/24584 is fixed

//! A test to check that structopt compiles with deny(missing_docs)

#![deny(missing_docs)]

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

/// The options
#[derive(StructOpt, Debug, PartialEq)]
pub struct Opt {
    #[structopt(short = "v")]
    verbose: bool,
    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

/// Some subcommands
#[derive(StructOpt, Debug, PartialEq)]
pub enum Cmd {
    /// command A
    A,
    /// command B
    B {
        /// Alice?
        #[structopt(short = "a")]
        alice: bool,
    },
    /// command C
    C(COpt),
}

/// The options for C
#[derive(StructOpt, Debug, PartialEq)]
pub struct COpt {
    #[structopt(short = "b")]
    bob: bool,
}

fn main() {
    println!("{:?}", Opt::from_args());
}
