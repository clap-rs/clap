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

use clap::Clap;

#[test]
fn unique_flag() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long)]
        alice: bool,
    }

    assert_eq!(Opt { alice: false }, Opt::parse_from(&["test"]));
    assert_eq!(Opt { alice: true }, Opt::parse_from(&["test", "-a"]));
    assert_eq!(Opt { alice: true }, Opt::parse_from(&["test", "--alice"]));
    assert!(Opt::try_parse_from(&["test", "-i"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "foo"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "-a"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "--alice"]).is_err());
}

#[test]
fn multiple_flag() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long, parse(from_occurrences))]
        alice: u64,
        #[clap(short, long, parse(from_occurrences))]
        bob: u8,
    }

    assert_eq!(Opt { alice: 0, bob: 0 }, Opt::parse_from(&["test"]));
    assert_eq!(Opt { alice: 1, bob: 0 }, Opt::parse_from(&["test", "-a"]));
    assert_eq!(
        Opt { alice: 2, bob: 0 },
        Opt::parse_from(&["test", "-a", "-a"])
    );
    assert_eq!(
        Opt { alice: 2, bob: 2 },
        Opt::parse_from(&["test", "-a", "--alice", "-bb"])
    );
    assert_eq!(
        Opt { alice: 3, bob: 1 },
        Opt::parse_from(&["test", "-aaa", "--bob"])
    );
    assert!(Opt::try_parse_from(&["test", "-i"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "foo"]).is_err());
}

fn parse_from_flag(b: bool) -> std::sync::atomic::AtomicBool {
    std::sync::atomic::AtomicBool::new(b)
}

#[test]
fn non_bool_flags() {
    #[derive(Clap, Debug)]
    struct Opt {
        #[clap(short, long, parse(from_flag = parse_from_flag))]
        alice: std::sync::atomic::AtomicBool,
        #[clap(short, long, parse(from_flag))]
        bob: std::sync::atomic::AtomicBool,
    }

    let falsey = Opt::parse_from(&["test"]);
    assert!(!falsey.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(!falsey.bob.load(std::sync::atomic::Ordering::Relaxed));

    let alice = Opt::parse_from(&["test", "-a"]);
    assert!(alice.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(!alice.bob.load(std::sync::atomic::Ordering::Relaxed));

    let bob = Opt::parse_from(&["test", "-b"]);
    assert!(!bob.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(bob.bob.load(std::sync::atomic::Ordering::Relaxed));

    let both = Opt::parse_from(&["test", "-b", "-a"]);
    assert!(both.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(both.bob.load(std::sync::atomic::Ordering::Relaxed));
}

#[test]
fn combined_flags() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long)]
        alice: bool,
        #[clap(short, long, parse(from_occurrences))]
        bob: u64,
    }

    assert_eq!(
        Opt {
            alice: false,
            bob: 0
        },
        Opt::parse_from(&["test"])
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 0
        },
        Opt::parse_from(&["test", "-a"])
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 0
        },
        Opt::parse_from(&["test", "-a"])
    );
    assert_eq!(
        Opt {
            alice: false,
            bob: 1
        },
        Opt::parse_from(&["test", "-b"])
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 1
        },
        Opt::parse_from(&["test", "--alice", "--bob"])
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 4
        },
        Opt::parse_from(&["test", "-bb", "-a", "-bb"])
    );
}
