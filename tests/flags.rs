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

#[test]
fn unique_flag() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(short = "a", long = "alice")]
        alice: bool,
    }

    assert_eq!(Opt { alice: false },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test"])));
    assert_eq!(Opt { alice: true },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: true },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--alice"])));
    assert!(Opt::clap().get_matches_from_safe(&["test", "-i"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "foo"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "-a"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "--alice"]).is_err());
}

#[test]
fn multiple_flag() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(short = "a", long = "alice", parse(from_occurrences))]
        alice: u64,
        #[structopt(short = "b", long = "bob", parse(from_occurrences))]
        bob: u8,
    }

    assert_eq!(Opt { alice: 0, bob: 0 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test"])));
    assert_eq!(Opt { alice: 1, bob: 0 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: 2, bob: 0 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-a", "-a"])));
    assert_eq!(Opt { alice: 2, bob: 2 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-a", "--alice", "-bb"])));
    assert_eq!(Opt { alice: 3, bob: 1 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-aaa", "--bob"])));
    assert!(Opt::clap().get_matches_from_safe(&["test", "-i"]).is_err());
    assert!(Opt::clap().get_matches_from_safe(&["test", "-a", "foo"]).is_err());
}

#[test]
fn combined_flags() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(short = "a", long = "alice")]
        alice: bool,
        #[structopt(short = "b", long = "bob", parse(from_occurrences))]
        bob: u64,
    }

    assert_eq!(Opt { alice: false, bob: 0 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test"])));
    assert_eq!(Opt { alice: true, bob: 0 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: true, bob: 0 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-a"])));
    assert_eq!(Opt { alice: false, bob: 1 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-b"])));
    assert_eq!(Opt { alice: true, bob: 1 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "--alice",  "--bob"])));
    assert_eq!(Opt { alice: true, bob: 4 },
               Opt::from_clap(&Opt::clap().get_matches_from(&["test", "-bb", "-a", "-bb"])));
}
