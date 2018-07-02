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
fn required_option() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = "a", long = "arg")]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 42 },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a42"]))
    );
    assert_eq!(
        Opt { arg: 42 },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a", "42"]))
    );
    assert_eq!(
        Opt { arg: 42 },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "--arg", "42"]))
    );
    assert!(Opt::into_app().get_matches_from_safe(&["test"]).is_err());
    assert!(
        Opt::into_app()
            .get_matches_from_safe(&["test", "-a42", "-a24"])
            .is_err()
    );
}

#[test]
fn optional_option() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = "a")]
        arg: Option<i32>,
    }
    assert_eq!(
        Opt { arg: Some(42) },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a42"]))
    );
    assert_eq!(
        Opt { arg: None },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test"]))
    );
    assert!(
        Opt::into_app()
            .get_matches_from_safe(&["test", "-a42", "-a24"])
            .is_err()
    );
}

#[test]
fn option_with_default() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = "a", default_value = "42")]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 24 },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a24"]))
    );
    assert_eq!(
        Opt { arg: 42 },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test"]))
    );
    assert!(
        Opt::into_app()
            .get_matches_from_safe(&["test", "-a42", "-a24"])
            .is_err()
    );
}

#[test]
fn option_with_raw_default() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = "a", raw(default_value = r#""42""#))]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 24 },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a24"]))
    );
    assert_eq!(
        Opt { arg: 42 },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test"]))
    );
    assert!(
        Opt::into_app()
            .get_matches_from_safe(&["test", "-a42", "-a24"])
            .is_err()
    );
}

#[test]
fn options() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = "a", long = "arg")]
        arg: Vec<i32>,
    }
    assert_eq!(
        Opt { arg: vec![24] },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a24"]))
    );
    assert_eq!(
        Opt { arg: vec![] },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test"]))
    );
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a24", "--arg", "42"]))
    );
}

#[test]
fn empy_default_value() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = "a", default_value = "")]
        arg: String,
    }
    assert_eq!(Opt { arg: "".into() }, Opt::parse_from(&["test"]));
    assert_eq!(
        Opt { arg: "foo".into() },
        Opt::parse_from(&["test", "-afoo"])
    );
}
