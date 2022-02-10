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

#![allow(clippy::option_option)]

use crate::utils;

use clap::{Parser, Subcommand};

#[test]
fn required_option() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long)]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 42 },
        Opt::try_parse_from(&["test", "-a42"]).unwrap()
    );
    assert_eq!(
        Opt { arg: 42 },
        Opt::try_parse_from(&["test", "-a", "42"]).unwrap()
    );
    assert_eq!(
        Opt { arg: 42 },
        Opt::try_parse_from(&["test", "--arg", "42"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn option_with_default() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, default_value = "42")]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 24 },
        Opt::try_parse_from(&["test", "-a24"]).unwrap()
    );
    assert_eq!(Opt { arg: 42 }, Opt::try_parse_from(&["test"]).unwrap());
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn option_with_raw_default() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, default_value = "42")]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 24 },
        Opt::try_parse_from(&["test", "-a24"]).unwrap()
    );
    assert_eq!(Opt { arg: 42 }, Opt::try_parse_from(&["test"]).unwrap());
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn option_from_str() {
    #[derive(Debug, PartialEq)]
    struct A;

    impl<'a> From<&'a str> for A {
        fn from(_: &str) -> A {
            A
        }
    }

    #[derive(Debug, Parser, PartialEq)]
    struct Opt {
        #[clap(parse(from_str))]
        a: Option<A>,
    }

    assert_eq!(Opt { a: None }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(
        Opt { a: Some(A) },
        Opt::try_parse_from(&["test", "foo"]).unwrap()
    );
}

#[test]
fn option_type_is_optional() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short)]
        arg: Option<i32>,
    }
    assert_eq!(
        Opt { arg: Some(42) },
        Opt::try_parse_from(&["test", "-a42"]).unwrap()
    );
    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&["test"]).unwrap());
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn required_with_option_type() {
    #[derive(Debug, PartialEq, Eq, Parser)]
    #[clap(subcommand_negates_reqs = true)]
    struct Opt {
        #[clap(required = true)]
        req_str: Option<String>,

        #[clap(subcommand)]
        cmd: Option<SubCommands>,
    }

    #[derive(Debug, PartialEq, Eq, Subcommand)]
    enum SubCommands {
        ExSub {
            #[clap(short, long, parse(from_occurrences))]
            verbose: u8,
        },
    }

    assert_eq!(
        Opt {
            req_str: Some(("arg").into()),
            cmd: None,
        },
        Opt::try_parse_from(&["test", "arg"]).unwrap()
    );

    assert_eq!(
        Opt {
            req_str: None,
            cmd: Some(SubCommands::ExSub { verbose: 1 }),
        },
        Opt::try_parse_from(&["test", "ex-sub", "-v"]).unwrap()
    );

    assert!(Opt::try_parse_from(&["test"]).is_err());
}

#[test]
fn ignore_qualified_option_type() {
    fn parser(s: &str) -> Option<String> {
        Some(s.to_string())
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(parse(from_str = parser))]
        arg: ::std::option::Option<String>,
    }

    assert_eq!(
        Opt {
            arg: Some("success".into())
        },
        Opt::try_parse_from(&["test", "success"]).unwrap()
    );
}

#[test]
fn option_option_type_is_optional_value() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, multiple_occurrences(true))]
        #[allow(clippy::option_option)]
        arg: Option<Option<i32>>,
    }
    assert_eq!(
        Opt {
            arg: Some(Some(42))
        },
        Opt::try_parse_from(&["test", "-a42"]).unwrap()
    );
    assert_eq!(
        Opt { arg: Some(None) },
        Opt::try_parse_from(&["test", "-a"]).unwrap()
    );
    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&["test"]).unwrap());
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn option_option_type_help() {
    #[derive(Parser, Debug)]
    struct Opt {
        #[clap(long, value_name = "val")]
        arg: Option<Option<i32>>,
    }
    let help = utils::get_help::<Opt>();
    assert!(help.contains("--arg [<val>]"));
    assert!(!help.contains("--arg [<val>]..."));
}

#[test]
fn two_option_option_types() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short)]
        arg: Option<Option<i32>>,

        #[clap(long)]
        field: Option<Option<String>>,
    }
    assert_eq!(
        Opt {
            arg: Some(Some(42)),
            field: Some(Some("f".into()))
        },
        Opt::try_parse_from(&["test", "-a42", "--field", "f"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: Some(Some(42)),
            field: Some(None)
        },
        Opt::try_parse_from(&["test", "-a42", "--field"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: Some(None),
            field: Some(None)
        },
        Opt::try_parse_from(&["test", "-a", "--field"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: Some(None),
            field: Some(Some("f".into()))
        },
        Opt::try_parse_from(&["test", "-a", "--field", "f"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: None,
            field: Some(None)
        },
        Opt::try_parse_from(&["test", "--field"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: None,
            field: None
        },
        Opt::try_parse_from(&["test"]).unwrap()
    );
}

#[test]
fn vec_type_is_multiple_occurrences() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long)]
        arg: Vec<i32>,
    }
    assert_eq!(
        Opt { arg: vec![24] },
        Opt::try_parse_from(&["test", "-a24"]).unwrap()
    );
    assert_eq!(Opt { arg: vec![] }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::try_parse_from(&["test", "-a", "24", "-a", "42"]).unwrap()
    );
}

#[test]
fn vec_type_with_required() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long, required = true)]
        arg: Vec<i32>,
    }
    assert_eq!(
        Opt { arg: vec![24] },
        Opt::try_parse_from(&["test", "-a24"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::try_parse_from(&["test", "-a", "24", "-a", "42"]).unwrap()
    );
}

#[test]
fn vec_type_with_multiple_values_only() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long, multiple_values(true), multiple_occurrences(false))]
        arg: Vec<i32>,
    }
    assert_eq!(
        Opt { arg: vec![24] },
        Opt::try_parse_from(&["test", "-a24"]).unwrap()
    );
    assert_eq!(Opt { arg: vec![] }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::try_parse_from(&["test", "-a", "24", "42"]).unwrap()
    );
}

#[test]
fn ignore_qualified_vec_type() {
    fn parser(s: &str) -> Vec<String> {
        vec![s.to_string()]
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(parse(from_str = parser))]
        arg: ::std::vec::Vec<String>,
    }

    assert_eq!(
        Opt {
            arg: vec!["success".into()]
        },
        Opt::try_parse_from(&["test", "success"]).unwrap()
    );
}

#[test]
fn option_vec_type() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short)]
        arg: Option<Vec<i32>>,
    }
    assert_eq!(
        Opt { arg: Some(vec![1]) },
        Opt::try_parse_from(&["test", "-a", "1"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2])
        },
        Opt::try_parse_from(&["test", "-a", "1", "-a", "2"]).unwrap()
    );

    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&["test"]).unwrap());
}

#[test]
fn option_vec_type_structopt_behavior() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long, multiple_values(true), min_values(0))]
        arg: Option<Vec<i32>>,
    }
    assert_eq!(
        Opt { arg: Some(vec![1]) },
        Opt::try_parse_from(&["test", "-a", "1"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2])
        },
        Opt::try_parse_from(&["test", "-a", "1", "2"]).unwrap()
    );

    assert_eq!(
        Opt { arg: Some(vec![]) },
        Opt::try_parse_from(&["test", "-a"]).unwrap()
    );

    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&["test"]).unwrap());
}

#[test]
fn two_option_vec_types() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short)]
        arg: Option<Vec<i32>>,

        #[clap(short)]
        b: Option<Vec<i32>>,
    }

    assert_eq!(
        Opt {
            arg: Some(vec![1]),
            b: None,
        },
        Opt::try_parse_from(&["test", "-a", "1"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1]),
            b: Some(vec![1])
        },
        Opt::try_parse_from(&["test", "-a", "1", "-b", "1"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2]),
            b: Some(vec![1, 2])
        },
        Opt::try_parse_from(&["test", "-a", "1", "-a", "2", "-b", "1", "-b", "2"]).unwrap()
    );

    assert_eq!(
        Opt { arg: None, b: None },
        Opt::try_parse_from(&["test"]).unwrap()
    );
}
