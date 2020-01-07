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

use clap::Clap;

#[test]
fn required_argument() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        arg: i32,
    }
    assert_eq!(Opt { arg: 42 }, Opt::parse_from(&["test", "42"]));
    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert!(Opt::try_parse_from(&["test", "42", "24"]).is_err());
}

#[test]
fn optional_argument() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        arg: Option<i32>,
    }
    assert_eq!(Opt { arg: Some(42) }, Opt::parse_from(&["test", "42"]));
    assert_eq!(Opt { arg: None }, Opt::parse_from(&["test"]));
    assert!(Opt::try_parse_from(&["test", "42", "24"]).is_err());
}

#[test]
fn argument_with_default() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value = "42")]
        arg: i32,
    }
    assert_eq!(Opt { arg: 24 }, Opt::parse_from(&["test", "24"]));
    assert_eq!(Opt { arg: 42 }, Opt::parse_from(&["test"]));
    assert!(Opt::try_parse_from(&["test", "42", "24"]).is_err());
}

#[test]
fn arguments() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        arg: Vec<i32>,
    }
    assert_eq!(Opt { arg: vec![24] }, Opt::parse_from(&["test", "24"]));
    assert_eq!(Opt { arg: vec![] }, Opt::parse_from(&["test"]));
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::parse_from(&["test", "24", "42"])
    );
}

#[test]
fn arguments_safe() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        arg: Vec<i32>,
    }
    assert_eq!(
        Opt { arg: vec![24] },
        Opt::try_parse_from(&["test", "24"]).unwrap()
    );
    assert_eq!(Opt { arg: vec![] }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::try_parse_from(&["test", "24", "42"]).unwrap()
    );

    assert_eq!(
        clap::ErrorKind::ValueValidation,
        Opt::try_parse_from(&["test", "NOPE"]).err().unwrap().kind
    );
}
