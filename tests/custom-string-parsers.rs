// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use std::ffi::{OsStr, OsString};
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(StructOpt, PartialEq, Debug)]
struct PathOpt {
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: PathBuf,

    #[structopt(short = "d", default_value = "../", parse(from_os_str))]
    default_path: PathBuf,

    #[structopt(short = "v", parse(from_os_str))]
    vector_path: Vec<PathBuf>,

    #[structopt(short = "o", parse(from_os_str))]
    option_path_1: Option<PathBuf>,

    #[structopt(short = "q", parse(from_os_str))]
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
        PathOpt::from_clap(&PathOpt::clap().get_matches_from(&[
            "test", "-p", "/usr/bin", "-v", "/a/b/c", "-v", "/d/e/f", "-v", "/g/h/i", "-q",
            "j.zip",
        ]))
    );
}

fn parse_hex(input: &str) -> Result<u64, ParseIntError> {
    u64::from_str_radix(input, 16)
}

#[derive(StructOpt, PartialEq, Debug)]
struct HexOpt {
    #[structopt(short = "n", parse(try_from_str = "parse_hex"))]
    number: u64,
}

#[test]
fn test_parse_hex() {
    assert_eq!(
        HexOpt { number: 5 },
        HexOpt::from_clap(&HexOpt::clap().get_matches_from(&["test", "-n", "5"]))
    );
    assert_eq!(
        HexOpt { number: 0xabcdef },
        HexOpt::from_clap(&HexOpt::clap().get_matches_from(&["test", "-n", "abcdef"]))
    );

    let err = HexOpt::clap()
        .get_matches_from_safe(&["test", "-n", "gg"])
        .unwrap_err();
    assert!(err.message.contains("invalid digit found in string"), err);
}

fn custom_parser_1(_: &str) -> &'static str {
    "A"
}
fn custom_parser_2(_: &str) -> Result<&'static str, u32> {
    Ok("B")
}
fn custom_parser_3(_: &OsStr) -> &'static str {
    "C"
}
fn custom_parser_4(_: &OsStr) -> Result<&'static str, OsString> {
    Ok("D")
}

#[derive(StructOpt, PartialEq, Debug)]
struct NoOpOpt {
    #[structopt(short = "a", parse(from_str = "custom_parser_1"))]
    a: &'static str,
    #[structopt(short = "b", parse(try_from_str = "custom_parser_2"))]
    b: &'static str,
    #[structopt(short = "c", parse(from_os_str = "custom_parser_3"))]
    c: &'static str,
    #[structopt(short = "d", parse(try_from_os_str = "custom_parser_4"))]
    d: &'static str,
}

#[test]
fn test_every_custom_parser() {
    assert_eq!(
        NoOpOpt {
            a: "A",
            b: "B",
            c: "C",
            d: "D"
        },
        NoOpOpt::from_clap(&NoOpOpt::clap().get_matches_from(&["test", "-a=?", "-b=?", "-c=?", "-d=?"]))
    );
}

// Note: can't use `Vec<u8>` directly, as structopt would instead look for
// conversion function from `&str` to `u8`.
type Bytes = Vec<u8>;

#[derive(StructOpt, PartialEq, Debug)]
struct DefaultedOpt {
    #[structopt(short = "b", parse(from_str))]
    bytes: Bytes,

    #[structopt(short = "i", parse(try_from_str))]
    integer: u64,

    #[structopt(short = "p", parse(from_os_str))]
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
        DefaultedOpt::from_clap(&DefaultedOpt::clap().get_matches_from(&[
            "test",
            "-b",
            "E²=p²c²+m²c⁴",
            "-i",
            "9000",
            "-p",
            "src/lib.rs",
        ]))
    );
}

#[derive(PartialEq, Debug)]
struct Foo(u8);

fn foo(value: u64) -> Foo {
    Foo(value as u8)
}

#[derive(StructOpt, PartialEq, Debug)]
struct Occurrences {
    #[structopt(short = "s", long = "signed", parse(from_occurrences))]
    signed: i32,

    #[structopt(short = "l", parse(from_occurrences))]
    little_signed: i8,

    #[structopt(short = "u", parse(from_occurrences))]
    unsigned: usize,

    #[structopt(short = "r", parse(from_occurrences))]
    little_unsigned: u8,

    #[structopt(short = "c", long = "custom", parse(from_occurrences = "foo"))]
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
        Occurrences::from_clap(&Occurrences::clap().get_matches_from(&[
            "test", "-s", "--signed", "--signed", "-l", "-rrrr", "-cccc", "--custom",
        ]))
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
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(short = "d", parse(try_from_str = "parse_bool"))]
        debug: bool,
        #[structopt(short = "v", default_value = "false", parse(try_from_str = "parse_bool"))]
        verbose: bool,
        #[structopt(short = "t", parse(try_from_str = "parse_bool"))]
        tribool: Option<bool>,
        #[structopt(short = "b", parse(try_from_str = "parse_bool"))]
        bitset: Vec<bool>,
    }

    assert!(Opt::clap().get_matches_from_safe(&["test"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-d"]).is_err());
    assert!(
        Opt::clap()
            .get_matches_from_safe(&["test", "-dfoo"])
            .is_err()
    );
    assert_eq!(
        Opt {
            debug: false,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::from_iter(&["test", "-dfalse"]),
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::from_iter(&["test", "-dtrue"]),
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::from_iter(&["test", "-dtrue", "-vfalse"]),
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: true,
            tribool: None,
            bitset: vec![],
        },
        Opt::from_iter(&["test", "-dtrue", "-vtrue"]),
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: Some(false),
            bitset: vec![],
        },
        Opt::from_iter(&["test", "-dtrue", "-tfalse"]),
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: Some(true),
            bitset: vec![],
        },
        Opt::from_iter(&["test", "-dtrue", "-ttrue"]),
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![false, true, false, false],
        },
        Opt::from_iter(&["test", "-dtrue", "-bfalse", "-btrue", "-bfalse", "-bfalse"]),
    );
}
