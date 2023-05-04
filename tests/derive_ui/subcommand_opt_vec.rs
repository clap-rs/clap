// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[derive(Parser, Debug)]
struct MakeCookie {
    #[arg(short)]
    s: String,

    #[command(subcommand)]
    cmd: Option<Vec<Command>>,
}

#[derive(Parser, Debug)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound { acorns: u32 },

    Sparkle {
        #[arg(short)]
        color: String,
    },
}

fn main() {
    let opt = MakeCookie::parse();
    println!("{opt:?}");
}
