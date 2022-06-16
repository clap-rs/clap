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

use clap::{Parser, Subcommand};

#[derive(Parser, PartialEq, Debug)]
struct Opt {
    #[clap(short, long)]
    force: bool,
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[clap(subcommand)]
    cmd: Sub,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Sub {
    Fetch {},
    Add {},
}

#[derive(Parser, PartialEq, Debug)]
struct Opt2 {
    #[clap(short, long)]
    force: bool,
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[clap(subcommand)]
    cmd: Option<Sub>,
}

#[test]
fn test_no_cmd() {
    let result = Opt::try_parse_from(&["test"]);
    assert!(result.is_err());

    assert_eq!(
        Opt2 {
            force: false,
            verbose: 0,
            cmd: None
        },
        Opt2::try_parse_from(&["test"]).unwrap()
    );
}

#[test]
fn test_fetch() {
    assert_eq!(
        Opt {
            force: false,
            verbose: 3,
            cmd: Sub::Fetch {}
        },
        Opt::try_parse_from(&["test", "-vvv", "fetch"]).unwrap()
    );
    assert_eq!(
        Opt {
            force: true,
            verbose: 0,
            cmd: Sub::Fetch {}
        },
        Opt::try_parse_from(&["test", "--force", "fetch"]).unwrap()
    );
}

#[test]
fn test_add() {
    assert_eq!(
        Opt {
            force: false,
            verbose: 0,
            cmd: Sub::Add {}
        },
        Opt::try_parse_from(&["test", "add"]).unwrap()
    );
    assert_eq!(
        Opt {
            force: false,
            verbose: 2,
            cmd: Sub::Add {}
        },
        Opt::try_parse_from(&["test", "-vv", "add"]).unwrap()
    );
}

#[test]
fn test_badinput() {
    let result = Opt::try_parse_from(&["test", "badcmd"]);
    assert!(result.is_err());
    let result = Opt::try_parse_from(&["test", "add", "--verbose"]);
    assert!(result.is_err());
    let result = Opt::try_parse_from(&["test", "--badopt", "add"]);
    assert!(result.is_err());
    let result = Opt::try_parse_from(&["test", "add", "--badopt"]);
    assert!(result.is_err());
}

#[derive(Parser, PartialEq, Debug)]
struct Opt3 {
    #[clap(short, long)]
    all: bool,
    #[clap(subcommand)]
    cmd: Sub2,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Sub2 {
    Foo {
        file: String,
        #[clap(subcommand)]
        cmd: Sub3,
    },
    Bar {},
}

#[derive(Subcommand, PartialEq, Debug)]
enum Sub3 {
    Baz {},
    Quux {},
}

#[test]
fn test_subsubcommand() {
    assert_eq!(
        Opt3 {
            all: true,
            cmd: Sub2::Foo {
                file: "lib.rs".to_string(),
                cmd: Sub3::Quux {}
            }
        },
        Opt3::try_parse_from(&["test", "--all", "foo", "lib.rs", "quux"]).unwrap()
    );
}

#[derive(Parser, PartialEq, Debug)]
enum SubSubCmdWithOption {
    Remote {
        #[clap(subcommand)]
        cmd: Option<Remote>,
    },
    Stash {
        #[clap(subcommand)]
        cmd: Stash,
    },
}
#[derive(Subcommand, PartialEq, Debug)]
enum Remote {
    Add { name: String, url: String },
    Remove { name: String },
}

#[derive(Subcommand, PartialEq, Debug)]
enum Stash {
    Save,
    Pop,
}

#[test]
fn sub_sub_cmd_with_option() {
    fn make(args: &[&str]) -> Option<SubSubCmdWithOption> {
        SubSubCmdWithOption::try_parse_from(args).ok()
    }
    assert_eq!(
        Some(SubSubCmdWithOption::Remote { cmd: None }),
        make(&["", "remote"])
    );
    assert_eq!(
        Some(SubSubCmdWithOption::Remote {
            cmd: Some(Remote::Add {
                name: "origin".into(),
                url: "http".into()
            })
        }),
        make(&["", "remote", "add", "origin", "http"])
    );
    assert_eq!(
        Some(SubSubCmdWithOption::Stash { cmd: Stash::Save }),
        make(&["", "stash", "save"])
    );
    assert_eq!(None, make(&["", "stash"]));
}
