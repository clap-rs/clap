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

//! A test to check that clap_derive compiles with deny(missing_docs)

#![deny(missing_docs)]

use clap::Clap;

/// The options
#[derive(Clap, Debug, PartialEq)]
pub struct Opt {
    #[clap(short)]
    verbose: bool,
    #[clap(subcommand)]
    cmd: Option<Cmd>,
}

/// Some subcommands
#[derive(Clap, Debug, PartialEq)]
pub enum Cmd {
    /// command A
    A,
    /// command B
    B {
        /// Alice?
        #[clap(short)]
        alice: bool,
    },
    /// command C
    C(COpt),
}

/// The options for C
#[derive(Clap, Debug, PartialEq)]
pub struct COpt {
    #[clap(short)]
    bob: bool,
}

fn main() {
    println!("{:?}", Opt::parse());
}
