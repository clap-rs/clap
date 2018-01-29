// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use structopt::clap::AppSettings;

// Check if the global settings compile
#[derive(StructOpt, Debug, PartialEq, Eq)]
#[structopt(global_settings_raw = "&[AppSettings::ColoredHelp]")]
struct Opt {
    #[structopt(long = "x", display_order_raw = "2", next_line_help_raw = "true",
        default_value_raw = "\"0\"", require_equals_raw = "true")]
    x: i32,

    #[structopt(short = "l", long = "level", aliases_raw = "&[\"set-level\", \"lvl\"]")]
    level: String,

    #[structopt(long = "values")]
    values: Vec<i32>,

    #[structopt(name = "FILE", requires_if_raw = "\"FILE\", \"values\"")]
    files: Vec<String>,
}

#[test]
fn test_raw_slice() {
    assert_eq!(Opt { x: 0, level: "1".to_string(), files: Vec::new(), values: vec![] },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1"])));
    assert_eq!(Opt { x: 0, level: "1".to_string(), files: Vec::new(), values: vec![] },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--level", "1"])));
    assert_eq!(Opt { x: 0, level: "1".to_string(), files: Vec::new(), values: vec![] },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--set-level", "1"])));
    assert_eq!(Opt { x: 0, level: "1".to_string(), files: Vec::new(), values: vec![] },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--lvl", "1"])));
}

#[test]
fn test_raw_multi_args() {
    assert_eq!(Opt { x: 0, level: "1".to_string(), files: vec!["file".to_string()], values: vec![] },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1", "file"])));
    assert_eq!(Opt { x: 0, level: "1".to_string(), files: vec!["FILE".to_string()], values: vec![1] },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1", "--values", "1", "--", "FILE"])));
}

#[test]
fn test_raw_multi_args_fail() {
    let result = Opt::clap().get_matches_from_safe(&["test", "-l", "1", "--", "FILE"]);
    assert!(result.is_err());
}

#[test]
fn test_raw_bool() {
    assert_eq!(Opt { x: 1, level: "1".to_string(), files: vec![], values: vec![] },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-l", "1", "--x=1"])));
    let result = Opt::clap().get_matches_from_safe(&["test", "-l", "1", "--x", "1"]);
    assert!(result.is_err());
}
