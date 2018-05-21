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
use structopt::clap::AppSettings;

// Check if the global settings compile
#[derive(StructOpt, Debug, PartialEq, Eq)]
#[structopt(raw(global_settings = "&[AppSettings::ColoredHelp]"))]
struct Opt {
    #[structopt(long = "x",
                raw(display_order = "2", next_line_help = "true", default_value = r#""0""#,
                    require_equals = "true"))]
    x: i32,

    #[structopt(short = "l", long = "level", raw(aliases = r#"&["set-level", "lvl"]"#))]
    level: String,

    #[structopt(long = "values")]
    values: Vec<i32>,

    #[structopt(name = "FILE", raw(requires_if = r#""FILE", "values""#))]
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
        Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1"]))
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--level", "1"]))
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--set-level", "1"]))
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: Vec::new(),
            values: vec![],
        },
        Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--lvl", "1"]))
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
        Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1", "file"]))
    );
    assert_eq!(
        Opt {
            x: 0,
            level: "1".to_string(),
            files: vec!["FILE".to_string()],
            values: vec![1],
        },
        Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1", "--values", "1", "--", "FILE"]))
    );
}

#[test]
fn test_raw_multi_args_fail() {
    let result = Opt::clap().get_matches_from_safe(&["test", "-l", "1", "--", "FILE"]);
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
        Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1", "--x=1"]))
    );
    let result = Opt::clap().get_matches_from_safe(&["test", "-l", "1", "--x", "1"]);
    assert!(result.is_err());
}
