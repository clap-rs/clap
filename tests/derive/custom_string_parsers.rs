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

#![allow(deprecated)]

use clap::Parser;

use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Parser, PartialEq, Debug)]
struct PathOpt {
    #[clap(short, long)]
    path: PathBuf,

    #[clap(short, default_value = "../")]
    default_path: PathBuf,

    #[clap(short, multiple_occurrences(true))]
    vector_path: Vec<PathBuf>,

    #[clap(short)]
    option_path_1: Option<PathBuf>,

    #[clap(short = 'q')]
    option_path_2: Option<PathBuf>,
}

#[test]
fn test_path_opt_simple() {
    assert_eq!(
        PathOpt {
            path: PathBuf::from("/usr/bin"),
            default_path: PathBuf::from("../"),
            vector_path: vec![
                PathBuf::from("/a/b/c"),
                PathBuf::from("/d/e/f"),
                PathBuf::from("/g/h/i"),
            ],
            option_path_1: None,
            option_path_2: Some(PathBuf::from("j.zip")),
        },
        PathOpt::try_parse_from(&[
            "test", "-p", "/usr/bin", "-v", "/a/b/c", "-v", "/d/e/f", "-v", "/g/h/i", "-q",
            "j.zip",
        ])
        .unwrap()
    );
}

fn parse_hex(input: &str) -> Result<u64, ParseIntError> {
    u64::from_str_radix(input, 16)
}

#[derive(Parser, PartialEq, Debug)]
struct HexOpt {
    #[clap(short, value_parser = parse_hex)]
    number: u64,
}

#[test]
fn test_parse_hex() {
    assert_eq!(
        HexOpt { number: 5 },
        HexOpt::try_parse_from(&["test", "-n", "5"]).unwrap()
    );
    assert_eq!(
        HexOpt {
            number: 0x00ab_cdef
        },
        HexOpt::try_parse_from(&["test", "-n", "abcdef"]).unwrap()
    );

    let err = HexOpt::try_parse_from(&["test", "-n", "gg"]).unwrap_err();
    assert!(
        err.to_string().contains("invalid digit found in string"),
        "{}",
        err
    );
}

#[derive(Debug)]
struct ErrCode(u32);
impl std::error::Error for ErrCode {}
impl std::fmt::Display for ErrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
fn custom_parser_2(_: &str) -> Result<&'static str, ErrCode> {
    Ok("B")
}

#[derive(Parser, PartialEq, Debug)]
struct NoOpOpt {
    #[clap(short, value_parser = custom_parser_2)]
    b: &'static str,
}

#[test]
fn test_every_custom_parser() {
    assert_eq!(
        NoOpOpt { b: "B" },
        NoOpOpt::try_parse_from(&["test", "-b=?"]).unwrap()
    );
}

#[test]
fn update_every_custom_parser() {
    let mut opt = NoOpOpt { b: "0" };

    opt.try_update_from(&["test", "-b=?"]).unwrap();

    assert_eq!(NoOpOpt { b: "B" }, opt);
}

#[derive(Parser, PartialEq, Debug)]
struct DefaultedOpt {
    #[clap(short)]
    integer: u64,

    #[clap(short)]
    path: PathBuf,
}

#[test]
fn test_parser_with_default_value() {
    assert_eq!(
        DefaultedOpt {
            integer: 9000,
            path: PathBuf::from("src/lib.rs"),
        },
        DefaultedOpt::try_parse_from(&["test", "-i", "9000", "-p", "src/lib.rs",]).unwrap()
    );
}
