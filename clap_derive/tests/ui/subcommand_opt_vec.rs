// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Clap;

#[derive(Clap, Debug)]
struct MakeCookie {
    #[clap(short)]
    s: String,

    #[clap(subcommand)]
    cmd: Option<Vec<Command>>,
}

#[derive(Clap, Debug)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound { acorns: u32 },

    Sparkle {
        #[clap(short)]
        color: String,
    },
}

fn main() {
    let opt = MakeCookie::parse();
    println!("{:?}", opt);
}
