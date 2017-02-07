// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[test]
fn unique_flag() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(short = "a", long = "alice")]
        alice: bool,
    }

    assert_eq!(Opt { alice: false },
               Opt::from_clap(Opt::clap().get_matches_from(&["test"])));
    assert_eq!(Opt { alice: true },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: true },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "--alice"])));
    assert!(Opt::clap().get_matches_from_safe(&["test", "-i"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "foo"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "-a"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "--alice"]).is_err());
}

#[test]
fn multiple_flag() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(short = "a", long = "alice")]
        alice: u64,
    }

    assert_eq!(Opt { alice: 0 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test"])));
    assert_eq!(Opt { alice: 1 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: 2 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-a", "-a"])));
    assert_eq!(Opt { alice: 2 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-a", "--alice"])));
    assert_eq!(Opt { alice: 3 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-aaa"])));
    assert!(Opt::clap().get_matches_from_safe(&["test", "-i"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "foo"]).is_err());
}

#[test]
fn combined_flags() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(short = "a", long = "alice")]
        alice: bool,
        #[structopt(short = "b", long = "bob")]
        bob: u64,
    }

    assert_eq!(Opt { alice: false, bob: 0 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test"])));
    assert_eq!(Opt { alice: true, bob: 0 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: true, bob: 0 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: false, bob: 1 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-b"])));
    assert_eq!(Opt { alice: true, bob: 1 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "--alice",  "--bob"])));
    assert_eq!(Opt { alice: true, bob: 4 },
               Opt::from_clap(Opt::clap().get_matches_from(&["test", "-bb", "-a", "-bb"])));
}
