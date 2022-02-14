use clap::{CommandFactory, Parser};

use crate::utils;

#[test]
fn default_value() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value = "3")]
        arg: i32,
    }
    assert_eq!(Opt { arg: 3 }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(&["test", "1"]).unwrap());

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}

#[test]
fn default_value_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value_t = 3)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 3 }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(&["test", "1"]).unwrap());

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}

#[test]
fn auto_default_value_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value_t)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 0 }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(&["test", "1"]).unwrap());

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: 0]"));
}

#[test]
fn detect_os_variant() {
    #![allow(unused_parens)] // needed for `as_ref` call

    #[derive(clap::Parser)]
    pub struct Options {
        #[clap(default_value_os = ("123".as_ref()))]
        x: String,
    }
    Options::command().debug_assert();
}
