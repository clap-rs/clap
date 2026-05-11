// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[derive(Debug)]
enum Kind {
    A,
    B,
}

#[derive(Parser, Debug)]
#[command(name = "test")]
pub struct Opt {
    #[arg(short)]
    number: u32,
    #[arg(skip)]
    k: Kind,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
