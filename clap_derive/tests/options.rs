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

#![allow(clippy::option_option)]

mod utils;

use clap::{Parser, Subcommand};
use utils::*;

#[test]
fn required_option() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 42 }, Opt::parse_from(&["test", "-a42"]));
    assert_eq!(Opt { arg: 42 }, Opt::parse_from(&["test", "-a", "42"]));
    assert_eq!(Opt { arg: 42 }, Opt::parse_from(&["test", "--arg", "42"]));
    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn optional_option() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short)]
        arg: Option<i32>,
    }
    assert_eq!(Opt { arg: Some(42) }, Opt::parse_from(&["test", "-a42"]));
    assert_eq!(Opt { arg: None }, Opt::parse_from(&["test"]));
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn option_with_default() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, default_value = "42")]
        arg: i32,
    }
    assert_eq!(Opt { arg: 24 }, Opt::parse_from(&["test", "-a24"]));
    assert_eq!(Opt { arg: 42 }, Opt::parse_from(&["test"]));
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn option_with_raw_default() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, default_value = "42")]
        arg: i32,
    }
    assert_eq!(Opt { arg: 24 }, Opt::parse_from(&["test", "-a24"]));
    assert_eq!(Opt { arg: 42 }, Opt::parse_from(&["test"]));
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn options() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long, multiple_occurrences(true))]
        arg: Vec<i32>,
    }
    assert_eq!(Opt { arg: vec![24] }, Opt::parse_from(&["test", "-a24"]));
    assert_eq!(Opt { arg: vec![] }, Opt::parse_from(&["test"]));
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::parse_from(&["test", "-a24", "--arg", "42"])
    );
}

#[test]
fn default_value() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, default_value = "test")]
        arg: String,
    }
    assert_eq!(Opt { arg: "test".into() }, Opt::parse_from(&["test"]));
    assert_eq!(
        Opt { arg: "foo".into() },
        Opt::parse_from(&["test", "-afoo"])
    );
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

    assert_eq!(Opt { a: None }, Opt::parse_from(&["test"]));
    assert_eq!(Opt { a: Some(A) }, Opt::parse_from(&["test", "foo"]));
}

#[test]
fn optional_argument_for_optional_option() {
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
        Opt::parse_from(&["test", "-a42"])
    );
    assert_eq!(Opt { arg: Some(None) }, Opt::parse_from(&["test", "-a"]));
    assert_eq!(Opt { arg: None }, Opt::parse_from(&["test"]));
    assert!(Opt::try_parse_from(&["test", "-a42", "-a24"]).is_err());
}

#[test]
fn option_option_help() {
    #[derive(Parser, Debug)]
    struct Opt {
        #[clap(long, value_name = "val")]
        arg: Option<Option<i32>>,
    }
    let help = get_help::<Opt>();
    assert!(help.contains("--arg <val>"));
    assert!(!help.contains("--arg <val>..."));
}

#[test]
fn two_option_options() {
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
        Opt::parse_from(&["test", "-a42", "--field", "f"])
    );
    assert_eq!(
        Opt {
            arg: Some(Some(42)),
            field: Some(None)
        },
        Opt::parse_from(&["test", "-a42", "--field"])
    );
    assert_eq!(
        Opt {
            arg: Some(None),
            field: Some(None)
        },
        Opt::parse_from(&["test", "-a", "--field"])
    );
    assert_eq!(
        Opt {
            arg: Some(None),
            field: Some(Some("f".into()))
        },
        Opt::parse_from(&["test", "-a", "--field", "f"])
    );
    assert_eq!(
        Opt {
            arg: None,
            field: Some(None)
        },
        Opt::parse_from(&["test", "--field"])
    );
    assert_eq!(
        Opt {
            arg: None,
            field: None
        },
        Opt::parse_from(&["test"])
    );
}

#[test]
fn optional_vec() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, multiple_occurrences(true))]
        arg: Option<Vec<i32>>,
    }
    assert_eq!(
        Opt { arg: Some(vec![1]) },
        Opt::parse_from(&["test", "-a", "1"])
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2])
        },
        Opt::parse_from(&["test", "-a1", "-a2"])
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2])
        },
        Opt::parse_from(&["test", "-a1", "-a2", "-a"])
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2])
        },
        Opt::parse_from(&["test", "-a1", "-a", "-a2"])
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2])
        },
        Opt::parse_from(&["test", "-a", "1", "2"])
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2, 3])
        },
        Opt::parse_from(&["test", "-a", "1", "2", "-a", "3"])
    );

    assert_eq!(Opt { arg: Some(vec![]) }, Opt::parse_from(&["test", "-a"]));

    assert_eq!(
        Opt { arg: Some(vec![]) },
        Opt::parse_from(&["test", "-a", "-a"])
    );

    assert_eq!(Opt { arg: None }, Opt::parse_from(&["test"]));
}

#[test]
fn two_optional_vecs() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, multiple_occurrences(true))]
        arg: Option<Vec<i32>>,

        #[clap(short, multiple_occurrences(true))]
        b: Option<Vec<i32>>,
    }

    assert_eq!(
        Opt {
            arg: Some(vec![1]),
            b: Some(vec![])
        },
        Opt::parse_from(&["test", "-a", "1", "-b"])
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1]),
            b: Some(vec![])
        },
        Opt::parse_from(&["test", "-a", "-b", "-a1"])
    );

    assert_eq!(
        Opt {
            arg: Some(vec![1, 2]),
            b: Some(vec![1, 2])
        },
        Opt::parse_from(&["test", "-a1", "-a2", "-b1", "-b2"])
    );

    assert_eq!(Opt { arg: None, b: None }, Opt::parse_from(&["test"]));
}

#[test]
fn required_option_type() {
    #[derive(Debug, PartialEq, Eq, Parser)]
    #[clap(setting(clap::AppSettings::SubcommandsNegateReqs))]
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
        Opt::parse_from(&["test", "arg"])
    );

    assert_eq!(
        Opt {
            req_str: None,
            cmd: Some(SubCommands::ExSub { verbose: 1 }),
        },
        Opt::parse_from(&["test", "ex-sub", "-v"])
    );

    assert!(Opt::try_parse_from(&["test"]).is_err());
}
