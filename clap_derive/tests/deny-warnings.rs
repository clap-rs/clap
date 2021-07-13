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

use clap::Parser;

fn try_str(s: &str) -> Result<String, std::convert::Infallible> {
    Ok(s.into())
}

#[test]
fn warning_never_struct() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[clap(parse(try_from_str = try_str))]
        s: String,
    }
    assert_eq!(
        Opt {
            s: "foo".to_string()
        },
        Opt::parse_from(&["test", "foo"])
    );
}

#[test]
fn warning_never_enum() {
    #[derive(Parser, Debug, PartialEq)]
    enum Opt {
        Foo {
            #[clap(parse(try_from_str = try_str))]
            s: String,
        },
    }
    assert_eq!(
        Opt::Foo {
            s: "foo".to_string()
        },
        Opt::parse_from(&["test", "foo", "foo"])
    );
}
