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

use std::ffi::{OsStr, OsString};
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Clap, PartialEq, Debug)]
struct PathOpt {
    #[clap(short = "p", long = "path", parse(from_os_str))]
    path: PathBuf,

    #[clap(short = "d", default_value = "../", parse(from_os_str))]
    default_path: PathBuf,

    #[clap(short = "v", parse(from_os_str))]
    vector_path: Vec<PathBuf>,

    #[clap(short = "o", parse(from_os_str))]
    option_path_1: Option<PathBuf>,

    #[clap(short = "q", parse(from_os_str))]
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
        PathOpt::from_argmatches(&PathOpt::into_app().get_matches_from(&[
            "test", "-p", "/usr/bin", "-v", "/a/b/c", "-v", "/d/e/f", "-v", "/g/h/i", "-q",
            "j.zip",
        ]))
    );
}

fn parse_hex(input: &str) -> Result<u64, ParseIntError> { u64::from_str_radix(input, 16) }

#[derive(Clap, PartialEq, Debug)]
struct HexOpt {
    #[clap(short = "n", parse(try_from_str = "parse_hex"))]
    number: u64,
}

#[test]
fn test_parse_hex() {
    assert_eq!(
        HexOpt { number: 5 },
        HexOpt::from_argmatches(&HexOpt::into_app().get_matches_from(&["test", "-n", "5"]))
    );
    assert_eq!(
        HexOpt { number: 0xabcdef },
        HexOpt::from_argmatches(&HexOpt::into_app().get_matches_from(&["test", "-n", "abcdef"]))
    );

    let err = HexOpt::into_app()
        .get_matches_from_safe(&["test", "-n", "gg"])
        .unwrap_err();
    assert!(err.message.contains("invalid digit found in string"), err);
}

fn custom_parser_1(_: &str) -> &'static str { "A" }
fn custom_parser_2(_: &str) -> Result<&'static str, u32> { Ok("B") }
fn custom_parser_3(_: &OsStr) -> &'static str { "C" }
fn custom_parser_4(_: &OsStr) -> Result<&'static str, OsString> { Ok("D") }

#[derive(Clap, PartialEq, Debug)]
struct NoOpOpt {
    #[clap(short = "a", parse(from_str = "custom_parser_1"))]
    a: &'static str,
    #[clap(short = "b", parse(try_from_str = "custom_parser_2"))]
    b: &'static str,
    #[clap(short = "c", parse(from_os_str = "custom_parser_3"))]
    c: &'static str,
    #[clap(short = "d", parse(try_from_os_str = "custom_parser_4"))]
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
        NoOpOpt::from_argmatches(
            &NoOpOpt::into_app().get_matches_from(&["test", "-a=?", "-b=?", "-c=?", "-d=?"])
        )
    );
}

// Note: can't use `Vec<u8>` directly, as structopt would instead look for
// conversion function from `&str` to `u8`.
type Bytes = Vec<u8>;

#[derive(Clap, PartialEq, Debug)]
struct DefaultedOpt {
    #[clap(short = "b", parse(from_str))]
    bytes: Bytes,

    #[clap(short = "i", parse(try_from_str))]
    integer: u64,

    #[clap(short = "p", parse(from_os_str))]
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
        DefaultedOpt::from_argmatches(&DefaultedOpt::into_app().get_matches_from(&[
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

fn foo(value: u64) -> Foo { Foo(value as u8) }

#[derive(Clap, PartialEq, Debug)]
struct Occurrences {
    #[clap(short = "s", long = "signed", parse(from_occurrences))]
    signed: i32,

    #[clap(short = "l", parse(from_occurrences))]
    little_signed: i8,

    #[clap(short = "u", parse(from_occurrences))]
    unsigned: usize,

    #[clap(short = "r", parse(from_occurrences))]
    little_unsigned: u8,

    #[clap(short = "c", long = "custom", parse(from_occurrences = "foo"))]
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
        Occurrences::from_argmatches(&Occurrences::into_app().get_matches_from(&[
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
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = "d", parse(try_from_str = "parse_bool"))]
        debug: bool,
        #[clap(short = "v", default_value = "false", parse(try_from_str = "parse_bool"))]
        verbose: bool,
        #[clap(short = "t", parse(try_from_str = "parse_bool"))]
        tribool: Option<bool>,
        #[clap(short = "b", parse(try_from_str = "parse_bool"))]
        bitset: Vec<bool>,
    }

    assert!(Opt::into_app().get_matches_from_safe(&["test"]).is_err());
    assert!(
        Opt::into_app()
            .get_matches_from_safe(&["test", "-d"])
            .is_err()
    );
    assert!(
        Opt::into_app()
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
        Opt::parse_from(&["test", "-dfalse"])
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::parse_from(&["test", "-dtrue"])
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![],
        },
        Opt::parse_from(&["test", "-dtrue", "-vfalse"])
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: true,
            tribool: None,
            bitset: vec![],
        },
        Opt::parse_from(&["test", "-dtrue", "-vtrue"])
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: Some(false),
            bitset: vec![],
        },
        Opt::parse_from(&["test", "-dtrue", "-tfalse"])
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: Some(true),
            bitset: vec![],
        },
        Opt::parse_from(&["test", "-dtrue", "-ttrue"])
    );
    assert_eq!(
        Opt {
            debug: true,
            verbose: false,
            tribool: None,
            bitset: vec![false, true, false, false],
        },
        Opt::parse_from(&["test", "-dtrue", "-bfalse", "-btrue", "-bfalse", "-bfalse"])
    );
}
