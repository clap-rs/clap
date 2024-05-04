// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
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

#![deny(unused_qualifications)]
#![deny(warnings)]

use clap::Parser;

fn try_str(s: &str) -> Result<String, std::convert::Infallible> {
    Ok(s.into())
}

#[test]
fn warning_never_struct() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(value_parser = try_str, default_value_t)]
        s: String,
    }
    assert_eq!(
        Opt {
            s: "foo".to_string()
        },
        Opt::try_parse_from(["test", "foo"]).unwrap()
    );
}

#[test]
fn warning_never_enum() {
    #[derive(Parser, Debug, PartialEq)]
    enum Opt {
        Foo {
            #[arg(value_parser = try_str, default_value_t)]
            s: String,
        },
    }
    assert_eq!(
        Opt::Foo {
            s: "foo".to_string()
        },
        Opt::try_parse_from(["test", "foo", "foo"]).unwrap()
    );
}

#[test]
fn warning_unused_qualifications() {
    // This causes `clap::Args` within the derive to be unused qualifications
    use clap::Args;

    #[derive(Args, Clone, Copy, Debug, Default)]
    #[group(skip)]
    pub(crate) struct Compose<L: Args> {
        #[command(flatten)]
        pub(crate) left: L,
    }
}
