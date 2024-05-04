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

use clap::error::ErrorKind;
use clap::Parser;
use std::num::ParseIntError;

pub(crate) const DISPLAY_ORDER: usize = 2;

// Check if the global settings compile
#[derive(Parser, Debug, PartialEq, Eq)]
#[command(group = clap::ArgGroup::new("foo"))]
struct Opt {
    #[arg(
        long = "x",
        display_order = DISPLAY_ORDER,
        next_line_help = true,
        default_value = "0",
        require_equals = true,
    )]
    x: i32,

    #[arg(short = 'l', long = "level", aliases = ["set-level", "lvl"])]
    level: String,

    #[arg(long("values"))]
    values: Vec<i32>,

    #[arg(id = "FILE", requires_if("FILE", "values"))]
    files: Vec<String>,
}

#[test]
fn test_slice() {
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::try_parse_from(["test", "-l", "1"]).unwrap()
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::try_parse_from(["test", "--level", "1"]).unwrap()
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::try_parse_from(["test", "--set-level", "1"]).unwrap()
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::try_parse_from(["test", "--lvl", "1"]).unwrap()
    );
}

#[test]
fn test_multi_args() {
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: vec!["file".to_string()],
            values: vec![],
        },
        Opt::try_parse_from(["test", "-l", "1", "file"]).unwrap()
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: vec!["FILE".to_string()],
            values: vec![1],
        },
        Opt::try_parse_from(["test", "-l", "1", "--values", "1", "--", "FILE"]).unwrap()
    );
}

#[test]
fn test_multi_args_fail() {
    let result = Opt::try_parse_from(["test", "-l", "1", "--", "FILE"]);
    assert!(result.is_err());
}

#[test]
fn test_bool() {
    assert_eq!(
        Opt {
            x: 1,
            level: "1".to_string(),
            files: vec![],
            values: vec![],
        },
        Opt::try_parse_from(["test", "-l", "1", "--x=1"]).unwrap()
    );
    let result = Opt::try_parse_from(["test", "-l", "1", "--x", "1"]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NoEquals);
}

#[test]
#[cfg(feature = "error-context")]
fn test_parse_hex_function_path() {
    #[derive(Parser, PartialEq, Debug)]
    struct HexOpt {
        #[arg(short, value_parser = parse_hex)]
        number: u64,
    }

    fn parse_hex(input: &str) -> Result<u64, ParseIntError> {
        u64::from_str_radix(input, 16)
    }

    assert_eq!(
        HexOpt { number: 5 },
        HexOpt::try_parse_from(["test", "-n", "5"]).unwrap()
    );
    assert_eq!(
        HexOpt {
            number: 0x00ab_cdef
        },
        HexOpt::try_parse_from(["test", "-n", "abcdef"]).unwrap()
    );

    let err = HexOpt::try_parse_from(["test", "-n", "gg"]).unwrap_err();
    assert!(
        err.to_string().contains("invalid digit found in string"),
        "{}",
        err
    );
}

#[test]
#[cfg(feature = "error-context")]
fn test_const_name() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(id = NAME, short, long)]
        number: u64,
    }

    const NAME: &str = "fun";

    assert_eq!(
        Opt { number: 5 },
        Opt::try_parse_from(["test", "-f", "5"]).unwrap()
    );
}
