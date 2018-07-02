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

#![deny(warnings)]
#![cfg(feature = "nightly")] // TODO: remove that when never is stable
#![feature(never_type)]

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

fn try_str(s: &str) -> Result<String, !> { Ok(s.into()) }

#[test]
fn warning_never_struct() {
    #[derive(Debug, PartialEq, StructOpt)]
    struct Opt {
        #[structopt(parse(try_from_str = "try_str"))]
        s: String,
    }
    assert_eq!(
        Opt {
            s: "foo".to_string()
        },
        Opt::from_iter(&["test", "foo"])
    );
}

#[test]
fn warning_never_enum() {
    #[derive(Debug, PartialEq, StructOpt)]
    enum Opt {
        Foo {
            #[structopt(parse(try_from_str = "try_str"))]
            s: String,
        },
    }
    assert_eq!(
        Opt::Foo {
            s: "foo".to_string()
        },
        Opt::from_iter(&["test", "Foo", "foo"])
    );
}
