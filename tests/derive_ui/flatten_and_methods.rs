// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[derive(Parser, Debug)]
struct DaemonOpts {
    #[clap(short, value_parser)]
    user: String,
    #[clap(short, value_parser)]
    group: String,
}

#[derive(Parser, Debug)]
#[clap(name = "basic")]
struct Opt {
    #[clap(short, flatten)]
    opts: DaemonOpts,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
