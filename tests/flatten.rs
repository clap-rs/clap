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

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[test]
fn flatten() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(flatten)]
        common: Common,
    }
    assert_eq!(
        Opt {
            common: Common { arg: 42 }
        },
        Opt::from_iter(&["test", "42"])
    );
    assert!(Opt::clap().get_matches_from_safe(&["test"]).is_err());
    assert!(
        Opt::clap()
            .get_matches_from_safe(&["test", "42", "24"])
            .is_err()
    );
}

#[test]
#[should_panic]
fn flatten_twice() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(StructOpt, PartialEq, Debug)]
    struct Opt {
        #[structopt(flatten)]
        c1: Common,
        // Defines "arg" twice, so this should not work.
        #[structopt(flatten)]
        c2: Common,
    }
    Opt::from_iter(&["test", "42", "43"]);
}

#[test]
fn flatten_in_subcommand() {
    #[derive(StructOpt, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(StructOpt, PartialEq, Debug)]
    struct Add {
        #[structopt(short = "i")]
        interactive: bool,
        #[structopt(flatten)]
        common: Common,
    }

    #[derive(StructOpt, PartialEq, Debug)]
    enum Opt {
        #[structopt(name = "fetch")]
        Fetch {
            #[structopt(short = "a")]
            all: bool,
            #[structopt(flatten)]
            common: Common,
        },

        #[structopt(name = "add")]
        Add(Add),
    }

    assert_eq!(
        Opt::Fetch {
            all: false,
            common: Common { arg: 42 }
        },
        Opt::from_iter(&["test", "fetch", "42"])
    );
    assert_eq!(
        Opt::Add(Add {
            interactive: true,
            common: Common { arg: 43 }
        }),
        Opt::from_iter(&["test", "add", "-i", "43"])
    );
}
