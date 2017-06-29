// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate structopt;
#[macro_use] extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
enum Opt {
    #[structopt(name = "fetch")]
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
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "fetch", "--all", "origin"])));
    assert_eq!(Opt::Fetch { all: false, force: true, repo: "origin".to_string() },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "fetch", "-f", "origin"])));
}

#[test]
fn test_add() {
    assert_eq!(Opt::Add { interactive: false, verbose: false },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "add"])));
    assert_eq!(Opt::Add { interactive: true, verbose: true },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "add", "-i", "-v"])));
}

#[test]
fn test_no_parse() {
    let result = Opt::clap().get_matches_from_safe(&["test", "badcmd", "-i", "-v"]);
    assert!(result.is_err());

    let result = Opt::clap().get_matches_from_safe(&["test", "add", "--badoption"]);
    assert!(result.is_err());
}
