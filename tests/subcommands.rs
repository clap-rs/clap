// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#[macro_use] extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
enum Opt {
    #[structopt(name = "fetch", about = "Fetch stuff from GitHub.")]
    Fetch {
        #[structopt(long = "all")]
        all: bool,
        #[structopt(short = "f", long = "force")]
        /// Overwrite local branches.
        force: bool,
        repo: String
    },
    
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "i", long = "interactive")]
        interactive: bool,
        #[structopt(short = "v", long = "verbose")]
        verbose: bool
    }
}

#[test]
fn test_fetch() {
    assert_eq!(Opt::Fetch { all: true, force: false, repo: "origin".to_string() },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "fetch", "--all", "origin"])));
    assert_eq!(Opt::Fetch { all: false, force: true, repo: "origin".to_string() },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "fetch", "-f", "origin"])));
}

#[test]
fn test_add() {
    assert_eq!(Opt::Add { interactive: false, verbose: false },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "add"])));
    assert_eq!(Opt::Add { interactive: true, verbose: true },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "add", "-i", "-v"])));
}

#[test]
fn test_no_parse() {
    let result = Opt::clap().get_matches_from_safe(&["test", "badcmd", "-i", "-v"]);
    assert!(result.is_err());

    let result = Opt::clap().get_matches_from_safe(&["test", "add", "--badoption"]);
    assert!(result.is_err());

    let result = Opt::clap().get_matches_from_safe(&["test"]);
    assert!(result.is_err());
}

#[derive(StructOpt, PartialEq, Debug)]
enum Opt2 {
    #[structopt(name = "do-something")]
    DoSomething {
        arg: String
    }
}

#[test]
/// This test is specifically to make sure that hyphenated subcommands get
/// processed correctly.
fn test_hyphenated_subcommands() {
    assert_eq!(Opt2::DoSomething { arg: "blah".to_string() },
               Opt2::from_clap(&Opt2::clap().get_matches_from(&["test", "do-something", "blah"])));
}

#[derive(StructOpt, PartialEq, Debug)]
enum Opt3 {
    #[structopt(name = "add")]
    Add,
    #[structopt(name = "init")]
    Init,
    #[structopt(name = "fetch")]
    Fetch
}

#[test]
fn test_null_commands() {
    assert_eq!(Opt3::Add, Opt3::from_clap(&Opt3::clap().get_matches_from(&["test", "add"])));
    assert_eq!(Opt3::Init, Opt3::from_clap(&Opt3::clap().get_matches_from(&["test", "init"])));
    assert_eq!(Opt3::Fetch, Opt3::from_clap(&Opt3::clap().get_matches_from(&["test", "fetch"])));
}

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Not shown")]
struct Add {
    file: String,
}
/// Not shown
#[derive(StructOpt, PartialEq, Debug)]
struct Fetch {
    remote: String,
}
#[derive(StructOpt, PartialEq, Debug)]
enum Opt4 {
    /// Not shown
    #[structopt(name = "add", about = "Add a file")]
    Add(Add),
    #[structopt(name = "init")]
    Init,
    /// download history from remote
    #[structopt(name = "fetch")]
    Fetch(Fetch),
}

#[test]
fn test_tuple_commands() {
    assert_eq!(
        Opt4::Add(Add { file: "f".to_string() }),
        Opt4::from_clap(&Opt4::clap().get_matches_from(&["test", "add", "f"]))
    );
    assert_eq!(Opt4::Init, Opt4::from_clap(&Opt4::clap().get_matches_from(&["test", "init"])));
    assert_eq!(
        Opt4::Fetch(Fetch { remote: "origin".to_string() }),
        Opt4::from_clap(&Opt4::clap().get_matches_from(&["test", "fetch", "origin"]))
    );

    let mut output = Vec::new();
    Opt4::clap().write_long_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    assert!(output.contains("download history from remote"));
    assert!(output.contains("Add a file"));
    assert!(!output.contains("Not shown"));
}
