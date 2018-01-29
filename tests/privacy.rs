// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

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
