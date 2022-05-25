// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "make-cookie")]
struct MakeCookie {
    #[clap(short, value_parser)]
    s: String,

    #[clap(skip, flatten)]
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    #[clap(name = "pound")]
    /// Pound acorns into flour for cookie dough.
    Pound {
        #[clap(value_parser)]
        acorns: u32,
    },

    Sparkle {
        #[clap(short, value_parser)]
        color: String,
    },
}

impl Default for Command {
    fn default() -> Self {
        Command::Pound { acorns: 0 }
    }
}

fn main() {
    let opt = MakeCookie::parse();
    println!("{:?}", opt);
}
