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

use clap::{AppSettings, Clap};

// Check if the global settings compile
#[derive(Clap, Debug, PartialEq, Eq)]
#[clap(raw(global_setting = "AppSettings::ColoredHelp"))]
struct Opt {
    #[clap(
        long = "x",
        raw(
            display_order = "2",
            next_line_help = "true",
            default_value = r#""0""#,
            require_equals = "true"
        )
    )]
    x: i32,

    #[clap(short = "l", long = "level", raw(aliases = r#"&["set-level", "lvl"]"#))]
    level: String,

    #[clap(long = "values")]
    values: Vec<i32>,

    #[clap(name = "FILE", raw(requires_if = r#""FILE", "values""#))]
    files: Vec<String>,
}

#[test]
fn test_raw_slice() {
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::parse_from(&["test", "-l", "1"])
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::parse_from(&["test", "--level", "1"])
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::parse_from(&["test", "--set-level", "1"])
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::parse_from(&["test", "--lvl", "1"])
    );
}

#[test]
fn test_raw_multi_args() {
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: vec!["file".to_string()],
            values: vec![],
        },
        Opt::parse_from(&["test", "-l", "1", "file"])
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: vec!["FILE".to_string()],
            values: vec![1],
        },
        Opt::parse_from(&["test", "-l", "1", "--values", "1", "--", "FILE"])
    );
}

#[test]
fn test_raw_multi_args_fail() {
    let result = Opt::try_parse_from(&["test", "-l", "1", "--", "FILE"]);
    assert!(result.is_err());
}

#[test]
fn test_raw_bool() {
    assert_eq!(
        Opt {
            x: 1,
            level: "1".to_string(),
            files: vec![],
            values: vec![],
        },
        Opt::parse_from(&["test", "-l", "1", "--x=1"])
    );
    let result = Opt::try_parse_from(&["test", "-l", "1", "--x", "1"]);
    assert!(result.is_err());
}
