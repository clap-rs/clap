// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate structopt;

mod options {
    #[derive(Debug, StructOpt)]
    pub struct Options {
        #[structopt(subcommand)]
        pub subcommand: ::subcommands::SubCommand,
    }
}

mod subcommands {
    #[derive(Debug, StructOpt)]
    pub enum SubCommand {
        #[structopt(name = "foo", about = "foo")]
        Foo {
            #[structopt(help = "foo")]
            bars: Vec<String>,
        },
    }
}
