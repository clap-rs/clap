// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate structopt;

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
    assert!(output.contains("Guillaume Pinot <texitoi@texitoi.eu>, others"));
    assert!(output.contains("Parse command line argument by defining a struct."));
}
