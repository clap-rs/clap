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

use std::ffi::CString;
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

// Note: can't use `Vec<u8>` directly, as clap would instead look for
// conversion function from `&str` to `u8`.
type Bytes = Vec<u8>;

#[derive(Parser, PartialEq, Debug)]
struct DefaultedOpt {
    #[clap(short, parse(from_str))]
    bytes: Bytes,

    #[clap(short)]
    integer: u64,

    #[clap(short)]
    path: PathBuf,
}

#[test]
fn test_parser_with_default_value() {
    assert_eq!(
        DefaultedOpt {
            bytes: b"E\xc2\xb2=p\xc2\xb2c\xc2\xb2+m\xc2\xb2c\xe2\x81\xb4".to_vec(),
            integer: 9000,
            path: PathBuf::from("src/lib.rs"),
        },
        DefaultedOpt::try_parse_from(&[
            "test",
            "-b",
            "E²=p²c²+m²c⁴",
            "-i",
            "9000",
            "-p",
            "src/lib.rs",
        ])
        .unwrap()
    );
}

#[derive(PartialEq, Debug)]
struct Foo(u8);

fn foo(value: u64) -> Foo {
    Foo(value as u8)
}

#[derive(Parser, PartialEq, Debug)]
struct Occurrences {
    #[clap(short, long, parse(from_occurrences))]
    signed: i32,

    #[clap(short, parse(from_occurrences))]
    little_signed: i8,

    #[clap(short, parse(from_occurrences))]
    unsigned: usize,

    #[clap(short = 'r', parse(from_occurrences))]
    little_unsigned: u8,

    #[clap(short, long, parse(from_occurrences = foo))]
    custom: Foo,
}

#[test]
fn test_parser_occurrences() {
    assert_eq!(
        Occurrences {
            signed: 3,
            little_signed: 1,
            unsigned: 0,
            little_unsigned: 4,
            custom: Foo(5),
        },
        Occurrences::try_parse_from(&[
            "test", "-s", "--signed", "--signed", "-l", "-rrrr", "-cccc", "--custom",
        ])
        .unwrap()
    );
}

#[test]
fn test_custom_bool() {
    fn parse_bool(s: &str) -> Result<bool, String> {
        match s {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("invalid bool {}", s)),
        }
    }
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, parse(try_from_str = parse_bool))]
        debug: bool,
        #[clap(
            short,
            default_value = "false",
            parse(try_from_str = parse_bool)
        )]
        verbose: bool,
        #[clap(short, parse(try_from_str = parse_bool))]
        tribool: Option<bool>,
        #[clap(short, parse(try_from_str = parse_bool), multiple_occurrences(true))]
        bitset: Vec<bool>,
    }

    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-d"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-dfoo"]).is_err());
    assert_eq!(
        Opt {
            debug: false,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::try_parse_from(&["test", "-dfalse"]).unwrap()
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::try_parse_from(&["test", "-dtrue"]).unwrap()
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::try_parse_from(&["test", "-dtrue", "-vfalse"]).unwrap()
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: true,
            tribool: None,
            bitset: vec![],
        },
        Opt::try_parse_from(&["test", "-dtrue", "-vtrue"]).unwrap()
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: Some(false),
            bitset: vec![],
        },
        Opt::try_parse_from(&["test", "-dtrue", "-tfalse"]).unwrap()
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: Some(true),
            bitset: vec![],
        },
        Opt::try_parse_from(&["test", "-dtrue", "-ttrue"]).unwrap()
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![false, true, false, false],
        },
        Opt::try_parse_from(&["test", "-dtrue", "-bfalse", "-btrue", "-bfalse", "-bfalse"])
            .unwrap()
    );
}

#[test]
fn test_cstring() {
    #[derive(Parser)]
    struct Opt {
        #[clap(parse(try_from_str = CString::new))]
        c_string: CString,
    }

    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert_eq!(
        Opt::try_parse_from(&["test", "bla"])
            .unwrap()
            .c_string
            .to_bytes(),
        b"bla"
    );
    assert!(Opt::try_parse_from(&["test", "bla\0bla"]).is_err());
}
