// Copyright (c) 2018 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[test]
fn required_argument() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        arg: i32,
    }
    assert_eq!(Opt { arg: 42 }, Opt::from_iter(&["test", "42"]));
    assert!(Opt::clap().get_matches_from_safe(&["test"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "42", "24"]).is_err());
}

#[test]
fn optional_argument() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        arg: Option<i32>,
    }
    assert_eq!(Opt { arg: Some(42) }, Opt::from_iter(&["test", "42"]));
    assert_eq!(Opt { arg: None }, Opt::from_iter(&["test"]));
    assert!(Opt::clap().get_matches_from_safe(&["test", "42", "24"]).is_err());
}

#[test]
fn argument_with_default() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(default_value = "42")]
        arg: i32,
    }
    assert_eq!(Opt { arg: 24 }, Opt::from_iter(&["test", "24"]));
    assert_eq!(Opt { arg: 42 }, Opt::from_iter(&["test"]));
    assert!(Opt::clap().get_matches_from_safe(&["test", "42", "24"]).is_err());
}

#[test]
fn argument_with_raw_default() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(raw(default_value = "\"42\""))]
        arg: i32,
    }
    assert_eq!(Opt { arg: 24 }, Opt::from_iter(&["test", "24"]));
    assert_eq!(Opt { arg: 42 }, Opt::from_iter(&["test"]));
    assert!(Opt::clap().get_matches_from_safe(&["test", "42", "24"]).is_err());
}

#[test]
fn arguments() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        arg: Vec<i32>,
    }
    assert_eq!(Opt { arg: vec![24] }, Opt::from_iter(&["test", "24"]));
    assert_eq!(Opt { arg: vec![] }, Opt::from_iter(&["test"]));
    assert_eq!(Opt { arg: vec![24, 42] }, Opt::from_iter(&["test", "24", "42"]));
}
