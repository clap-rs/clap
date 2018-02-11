// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#[macro_use] extern crate structopt;

use structopt::StructOpt;

#[test]
fn no_author_version_about() {
    #[derive(StructOpt, PartialEq, Debug)]
    #[structopt(name = "foo", about = "", author = "", version = "")]
    struct Opt {}

    let mut output = Vec::new();
    Opt::clap().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    assert!(output.starts_with("foo \n\nUSAGE:"));
}

#[test]
fn use_env() {
    #[derive(StructOpt, PartialEq, Debug)]
    #[structopt()]
    struct Opt {}

    let mut output = Vec::new();
    Opt::clap().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();
    assert!(output.starts_with("structopt 0.2."));
    assert!(output.contains("Guillaume Pinot <texitoi@texitoi.eu>"));
    assert!(output.contains("Parse command line argument by defining a struct."));
}
