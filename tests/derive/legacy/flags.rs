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

use clap::Parser;

#[test]
fn bool_type_is_flag() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long)]
        alice: bool,
    }

    assert_eq!(
        Opt { alice: false },
        Opt::try_parse_from(&["test"]).unwrap()
    );
    assert_eq!(
        Opt { alice: true },
        Opt::try_parse_from(&["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt { alice: true },
        Opt::try_parse_from(&["test", "--alice"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["test", "-i"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "foo"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "-a"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "--alice"]).is_err());
}

#[test]
fn from_occurrences() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short, long, parse(from_occurrences))]
        alice: u64,
        #[clap(short, long, parse(from_occurrences))]
        bob: u8,
    }

    assert_eq!(
        Opt { alice: 0, bob: 0 },
        Opt::try_parse_from(&["test"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 1, bob: 0 },
        Opt::try_parse_from(&["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 2, bob: 0 },
        Opt::try_parse_from(&["test", "-a", "-a"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 2, bob: 2 },
        Opt::try_parse_from(&["test", "-a", "--alice", "-bb"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 3, bob: 1 },
        Opt::try_parse_from(&["test", "-aaa", "--bob"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["test", "-i"]).is_err());
    assert!(Opt::try_parse_from(&["test", "-a", "foo"]).is_err());
}

#[test]
fn non_bool_type_flag() {
    fn parse_from_flag(b: bool) -> std::sync::atomic::AtomicBool {
        std::sync::atomic::AtomicBool::new(b)
    }

    #[derive(Parser, Debug)]
    struct Opt {
        #[clap(short, long, parse(from_flag = parse_from_flag))]
        alice: std::sync::atomic::AtomicBool,
        #[clap(short, long, parse(from_flag))]
        bob: std::sync::atomic::AtomicBool,
    }

    let falsey = Opt::try_parse_from(&["test"]).unwrap();
    assert!(!falsey.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(!falsey.bob.load(std::sync::atomic::Ordering::Relaxed));

    let alice = Opt::try_parse_from(&["test", "-a"]).unwrap();
    assert!(alice.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(!alice.bob.load(std::sync::atomic::Ordering::Relaxed));

    let bob = Opt::try_parse_from(&["test", "-b"]).unwrap();
    assert!(!bob.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(bob.bob.load(std::sync::atomic::Ordering::Relaxed));

    let both = Opt::try_parse_from(&["test", "-b", "-a"]).unwrap();
    assert!(both.alice.load(std::sync::atomic::Ordering::Relaxed));
    assert!(both.bob.load(std::sync::atomic::Ordering::Relaxed));
}

#[test]
fn mixed_type_flags() {
    #[derive(Parser, PartialEq, Debug)]
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
        Opt::try_parse_from(&["test"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 0
        },
        Opt::try_parse_from(&["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 0
        },
        Opt::try_parse_from(&["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: false,
            bob: 1
        },
        Opt::try_parse_from(&["test", "-b"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 1
        },
        Opt::try_parse_from(&["test", "--alice", "--bob"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 4
        },
        Opt::try_parse_from(&["test", "-bb", "-a", "-bb"]).unwrap()
    );
}

#[test]
fn ignore_qualified_bool_type() {
    mod inner {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Debug)]
        pub struct bool(pub String);

        impl std::str::FromStr for self::bool {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(self::bool(s.into()))
            }
        }
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        arg: inner::bool,
    }

    assert_eq!(
        Opt {
            arg: inner::bool("success".into())
        },
        Opt::try_parse_from(&["test", "success"]).unwrap()
    );
}
