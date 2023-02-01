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

use crate::utils;

use clap::{Args, Parser, Subcommand};

#[test]
fn flatten() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[command(flatten)]
        common: Common,
    }
    assert_eq!(
        Opt {
            common: Common { arg: 42 }
        },
        Opt::try_parse_from(["test", "42"]).unwrap()
    );
    assert!(Opt::try_parse_from(["test"]).is_err());
    assert!(Opt::try_parse_from(["test", "42", "24"]).is_err());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn flatten_twice() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[command(flatten)]
        c1: Common,
        // Defines "arg" twice, so this should not work.
        #[command(flatten)]
        c2: Common,
    }
    Opt::try_parse_from(["test", "42", "43"]).unwrap();
}

#[test]
fn flatten_in_subcommand() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Args, PartialEq, Debug)]
    struct Add {
        #[arg(short)]
        interactive: bool,
        #[command(flatten)]
        common: Common,
    }

    #[derive(Parser, PartialEq, Debug)]
    enum Opt {
        Fetch {
            #[arg(short)]
            all: bool,
            #[command(flatten)]
            common: Common,
        },

        Add(Add),
    }

    assert_eq!(
        Opt::Fetch {
            all: false,
            common: Common { arg: 42 }
        },
        Opt::try_parse_from(["test", "fetch", "42"]).unwrap()
    );
    assert_eq!(
        Opt::Add(Add {
            interactive: true,
            common: Common { arg: 43 }
        }),
        Opt::try_parse_from(["test", "add", "-i", "43"]).unwrap()
    );
}

#[test]
fn update_args_with_flatten() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[command(flatten)]
        common: Common,
    }

    let mut opt = Opt {
        common: Common { arg: 42 },
    };
    opt.try_update_from(["test"]).unwrap();
    assert_eq!(Opt::try_parse_from(["test", "42"]).unwrap(), opt);

    let mut opt = Opt {
        common: Common { arg: 42 },
    };
    opt.try_update_from(["test", "52"]).unwrap();
    assert_eq!(Opt::try_parse_from(["test", "52"]).unwrap(), opt);
}

#[derive(Subcommand, PartialEq, Debug)]
enum BaseCli {
    Command1(Command1),
}

#[derive(Args, PartialEq, Debug)]
struct Command1 {
    arg1: i32,

    arg2: i32,
}

#[derive(Args, PartialEq, Debug)]
struct Command2 {
    arg2: i32,
}

#[derive(Parser, PartialEq, Debug)]
enum Opt {
    #[command(flatten)]
    BaseCli(BaseCli),
    Command2(Command2),
}

#[test]
fn merge_subcommands_with_flatten() {
    assert_eq!(
        Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 42, arg2: 44 })),
        Opt::try_parse_from(["test", "command1", "42", "44"]).unwrap()
    );
    assert_eq!(
        Opt::Command2(Command2 { arg2: 43 }),
        Opt::try_parse_from(["test", "command2", "43"]).unwrap()
    );
}

#[test]
fn update_subcommands_with_flatten() {
    let mut opt = Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "command1", "42", "44"])
        .unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command1", "42", "44"]).unwrap(),
        opt
    );

    let mut opt = Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "command1", "42"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command1", "42", "14"]).unwrap(),
        opt
    );

    let mut opt = Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "command2", "43"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command2", "43"]).unwrap(),
        opt
    );
}

#[test]
fn flatten_with_doc_comment() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        /// This is an arg. Arg means "argument". Command line argument.
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        /// The very important comment that clippy had me put here.
        /// It knows better.
        #[command(flatten)]
        common: Common,
    }
    assert_eq!(
        Opt {
            common: Common { arg: 42 }
        },
        Opt::try_parse_from(["test", "42"]).unwrap()
    );

    let help = utils::get_help::<Opt>();
    assert!(help.contains("This is an arg."));
    assert!(!help.contains("The very important"));
}

#[test]
fn docstrings_ordering_with_multiple_command() {
    /// This is the docstring for Flattened
    #[derive(Args)]
    struct Flattened {
        #[arg(long)]
        foo: bool,
    }

    /// This is the docstring for Command
    #[derive(Parser)]
    struct Command {
        #[command(flatten)]
        flattened: Flattened,
    }

    let short_help = utils::get_help::<Command>();

    assert!(short_help.contains("This is the docstring for Command"));
}

#[test]
fn docstrings_ordering_with_multiple_clap_partial() {
    /// This is the docstring for Flattened
    #[derive(Args)]
    struct Flattened {
        #[arg(long)]
        foo: bool,
    }

    #[derive(Parser)]
    struct Command {
        #[command(flatten)]
        flattened: Flattened,
    }

    let short_help = utils::get_help::<Command>();

    assert!(short_help.contains("This is the docstring for Flattened"));
}
