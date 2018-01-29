// Copyright (c) 2017 structopt Developers
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[test]
fn commets_intead_of_actual_help() {
    /// Lorem ipsum
    #[derive(StructOpt, PartialEq, Debug)]
    struct LoremIpsum {
        /// Fooify a bar
        /// and a baz
        #[structopt(short = "f", long = "foo")]
        foo: bool,
    }

    let mut output = Vec::new();
    LoremIpsum::clap().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    assert!(output.contains("Lorem ipsum"));
    assert!(output.contains("Fooify a bar and a baz"));
}

#[test]
fn help_is_better_than_comments() {
    /// Lorem ipsum
    #[derive(StructOpt, PartialEq, Debug)]
    #[structopt(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        /// Fooify a bar
        #[structopt(short = "f", long = "foo", help = "DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES")]
        foo: bool,
    }

    let mut output = Vec::new();
    LoremIpsum::clap().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    println!("{}", output);
    assert!(output.contains("Dolor sit amet"));
    assert!(!output.contains("Lorem ipsum"));
    assert!(output.contains("DO NOT PASS A BAR"));
}
