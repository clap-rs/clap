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

use clap::builder::BoolishValueParser;
use clap::builder::TypedValueParser as _;
use clap::ArgAction;
use clap::CommandFactory;
use clap::Parser;

#[test]
fn bool_type_is_flag() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    #[command(args_override_self = true)]
    struct Opt {
        #[arg(short, long)]
        alice: bool,
    }

    assert_eq!(Opt { alice: false }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(
        Opt { alice: true },
        Opt::try_parse_from(["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt { alice: true },
        Opt::try_parse_from(["test", "-a", "-a"]).unwrap()
    );
    assert_eq!(
        Opt { alice: true },
        Opt::try_parse_from(["test", "--alice"]).unwrap()
    );
    assert!(Opt::try_parse_from(["test", "-i"]).is_err());
    assert!(Opt::try_parse_from(["test", "-a", "foo"]).is_err());
}

#[test]
fn non_bool_type_flag() {
    fn parse_from_flag(b: bool) -> usize {
        if b {
            10
        } else {
            5
        }
    }

    #[derive(Parser, Debug)]
    struct Opt {
        #[arg(short, long, action = ArgAction::SetTrue, value_parser = BoolishValueParser::new().map(parse_from_flag))]
        alice: usize,
        #[arg(short, long, action = ArgAction::SetTrue, value_parser = BoolishValueParser::new().map(parse_from_flag))]
        bob: usize,
    }

    let opt = Opt::try_parse_from(["test"]).unwrap();
    assert_eq!(opt.alice, 5);
    assert_eq!(opt.bob, 5);

    let opt = Opt::try_parse_from(["test", "-a"]).unwrap();
    assert_eq!(opt.alice, 10);
    assert_eq!(opt.bob, 5);

    let opt = Opt::try_parse_from(["test", "-b"]).unwrap();
    assert_eq!(opt.alice, 5);
    assert_eq!(opt.bob, 10);

    let opt = Opt::try_parse_from(["test", "-b", "-a"]).unwrap();
    assert_eq!(opt.alice, 10);
    assert_eq!(opt.bob, 10);
}

#[test]
#[ignore] // Not a good path for supporting this atm
fn inferred_help() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        /// Foo
        #[arg(short, long)]
        help: bool,
    }

    let mut cmd = Opt::command();
    cmd.build();
    let arg = cmd.get_arguments().find(|a| a.get_id() == "help").unwrap();
    assert_eq!(
        arg.get_help().map(|s| s.to_string()),
        Some("Foo".to_owned()),
        "Incorrect help"
    );
    assert!(matches!(arg.get_action(), ArgAction::Help));
}

#[test]
#[ignore] // Not a good path for supporting this atm
fn inferred_version() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        /// Foo
        #[arg(short, long)]
        version: bool,
    }

    let mut cmd = Opt::command();
    cmd.build();
    let arg = cmd
        .get_arguments()
        .find(|a| a.get_id() == "version")
        .unwrap();
    assert_eq!(
        arg.get_help().map(|s| s.to_string()),
        Some("Foo".to_owned()),
        "Incorrect help"
    );
    assert!(matches!(arg.get_action(), ArgAction::Version));
}

#[test]
fn count() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        #[arg(short, long, action = clap::ArgAction::Count)]
        alice: u8,
        #[arg(short, long, action = clap::ArgAction::Count)]
        bob: u8,
    }

    assert_eq!(
        Opt { alice: 0, bob: 0 },
        Opt::try_parse_from(["test"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 1, bob: 0 },
        Opt::try_parse_from(["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 2, bob: 0 },
        Opt::try_parse_from(["test", "-a", "-a"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 2, bob: 2 },
        Opt::try_parse_from(["test", "-a", "--alice", "-bb"]).unwrap()
    );
    assert_eq!(
        Opt { alice: 3, bob: 1 },
        Opt::try_parse_from(["test", "-aaa", "--bob"]).unwrap()
    );
    assert!(Opt::try_parse_from(["test", "-i"]).is_err());
    assert!(Opt::try_parse_from(["test", "-a", "foo"]).is_err());
}

#[test]
fn mixed_type_flags() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        #[arg(short, long)]
        alice: bool,
        #[arg(short, long, action = clap::ArgAction::Count)]
        bob: u8,
    }

    assert_eq!(
        Opt {
            alice: false,
            bob: 0
        },
        Opt::try_parse_from(["test"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 0
        },
        Opt::try_parse_from(["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 0
        },
        Opt::try_parse_from(["test", "-a"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: false,
            bob: 1
        },
        Opt::try_parse_from(["test", "-b"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 1
        },
        Opt::try_parse_from(["test", "--alice", "--bob"]).unwrap()
    );
    assert_eq!(
        Opt {
            alice: true,
            bob: 4
        },
        Opt::try_parse_from(["test", "-bb", "-a", "-bb"]).unwrap()
    );
}

#[test]
fn ignore_qualified_bool_type() {
    mod inner {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Debug, Clone)]
        pub(crate) struct bool(pub(crate) String);

        impl std::str::FromStr for bool {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(bool(s.into()))
            }
        }
    }

    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        arg: inner::bool,
    }

    assert_eq!(
        Opt {
            arg: inner::bool("success".into())
        },
        Opt::try_parse_from(["test", "success"]).unwrap()
    );
}

#[test]
fn override_implicit_action() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        #[arg(long, action = clap::ArgAction::Set)]
        arg: bool,
    }

    assert_eq!(
        Opt { arg: false },
        Opt::try_parse_from(["test", "--arg", "false"]).unwrap()
    );

    assert_eq!(
        Opt { arg: true },
        Opt::try_parse_from(["test", "--arg", "true"]).unwrap()
    );
}

#[test]
fn override_implicit_from_flag_positional() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        #[arg(action = clap::ArgAction::Set)]
        arg: bool,
    }

    assert_eq!(
        Opt { arg: false },
        Opt::try_parse_from(["test", "false"]).unwrap()
    );

    assert_eq!(
        Opt { arg: true },
        Opt::try_parse_from(["test", "true"]).unwrap()
    );
}

#[test]
fn unit_for_negation() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        #[arg(long)]
        arg: bool,
        #[arg(long, action = ArgAction::SetTrue, overrides_with = "arg")]
        no_arg: (),
    }

    assert_eq!(
        Opt {
            arg: false,
            no_arg: ()
        },
        Opt::try_parse_from(["test"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: true,
            no_arg: ()
        },
        Opt::try_parse_from(["test", "--arg"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: false,
            no_arg: ()
        },
        Opt::try_parse_from(["test", "--no-arg"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: true,
            no_arg: ()
        },
        Opt::try_parse_from(["test", "--no-arg", "--arg"]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: false,
            no_arg: ()
        },
        Opt::try_parse_from(["test", "--arg", "--no-arg"]).unwrap()
    );
}
