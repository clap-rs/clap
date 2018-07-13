// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

#[macro_use]
extern crate clap;

use clap::Clap;

#[test]
fn commets_intead_of_actual_help() {
    use clap::IntoApp;

    /// Lorem ipsum
    #[derive(Clap, PartialEq, Debug)]
    struct LoremIpsum {
        /// Fooify a bar
        /// and a baz
        #[clap(short = "f", long = "foo")]
        foo: bool,
    }

    let mut output = Vec::new();
    LoremIpsum::into_app().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    assert!(output.contains("Lorem ipsum"));
    assert!(output.contains("Fooify a bar and a baz"));
}

#[test]
fn help_is_better_than_comments() {
    use clap::IntoApp;
    /// Lorem ipsum
    #[derive(Clap, PartialEq, Debug)]
    #[clap(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        /// Fooify a bar
        #[clap(
            short = "f",
            long = "foo",
            help = "DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES"
        )]
        foo: bool,
    }

    let mut output = Vec::new();
    LoremIpsum::into_app().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    assert!(output.contains("Dolor sit amet"));
    assert!(!output.contains("Lorem ipsum"));
    assert!(output.contains("DO NOT PASS A BAR"));
}

#[test]
fn empty_line_in_doc_comment_is_double_linefeed() {
    use clap::IntoApp;
    /// Foo.
    ///
    /// Bar
    #[derive(Clap, PartialEq, Debug)]
    #[clap(name = "lorem-ipsum", author = "", version = "")]
    struct LoremIpsum {}

    let mut output = Vec::new();
    LoremIpsum::into_app().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    println!("{}", output);
    assert!(output.starts_with("lorem-ipsum \nFoo.\n\nBar\n\nUSAGE:"));
}
