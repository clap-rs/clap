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

use clap::CommandFactory;
use clap::Parser;

#[test]
fn required_argument() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_parser)]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 42 },
        Opt::try_parse_from(&["test", "42"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert!(Opt::try_parse_from(&["test", "42", "24"]).is_err());
}

#[test]
fn argument_with_default() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_parser, default_value = "42")]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 24 },
        Opt::try_parse_from(&["test", "24"]).unwrap()
    );
    assert_eq!(Opt { arg: 42 }, Opt::try_parse_from(&["test"]).unwrap());
    assert!(Opt::try_parse_from(&["test", "42", "24"]).is_err());
}

#[test]
fn auto_value_name() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_parser)]
        my_special_arg: i32,
    }

    let mut help = Vec::new();
    Opt::command().write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("MY_SPECIAL_ARG"));
    // Ensure the implicit `num_vals` is just 1
    assert_eq!(
        Opt { my_special_arg: 10 },
        Opt::try_parse_from(&["test", "10"]).unwrap()
    );
}

#[test]
fn explicit_value_name() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_parser, value_name = "BROWNIE_POINTS")]
        my_special_arg: i32,
    }

    let mut help = Vec::new();
    Opt::command().write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("BROWNIE_POINTS"));
    assert!(!help.contains("MY_SPECIAL_ARG"));
    // Ensure the implicit `num_vals` is just 1
    assert_eq!(
        Opt { my_special_arg: 10 },
        Opt::try_parse_from(&["test", "10"]).unwrap()
    );
}

#[test]
fn option_type_is_optional() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_parser)]
        arg: Option<i32>,
    }
    assert_eq!(
        Opt { arg: Some(42) },
        Opt::try_parse_from(&["test", "42"]).unwrap()
    );
    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&["test"]).unwrap());
    assert!(Opt::try_parse_from(&["test", "42", "24"]).is_err());
}

#[test]
fn vec_type_is_multiple_values() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_parser)]
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
        Opt::try_parse_from(&["test", "NOPE"]).err().unwrap().kind()
    );
}
