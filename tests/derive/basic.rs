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

use clap::Parser;

#[test]
fn basic() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short = 'a', long = "arg")]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 24 },
        Opt::try_parse_from(&["test", "-a24"]).unwrap()
    );
}

#[test]
fn update_basic() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short = 'a', long = "arg")]
        single_value: i32,
    }

    let mut opt = Opt::try_parse_from(&["test", "-a0"]).unwrap();

    opt.update_from(&["test", "-a42"]);

    assert_eq!(Opt { single_value: 42 }, opt);
}

#[test]
fn unit_struct() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt;

    assert_eq!(Opt {}, Opt::try_parse_from(&["test"]).unwrap());
}
