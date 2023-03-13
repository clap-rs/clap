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
fn conflicts_with_kebab_case() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(long = "hh-os")]
        hh_os: Option<String>,
        #[arg(long, conflicts_with = "hh-os")]
        lol: Option<String>,
    }

    assert_eq!(
        Opt {
            hh_os: Some("muahaha".to_string()),
            lol: None,
        },
        Opt::try_parse_from(["test", "--hh-os", "muahaha"]).unwrap()
    );

    assert_eq!(
        Opt {
            hh_os: None,
            lol: Some("haha".to_string()),
        },
        Opt::try_parse_from(["test", "--lol", "haha"]).unwrap()
    );

    assert!(Opt::try_parse_from(["test", "--hh-os", "muahaha", "--lol", "haha"]).is_err());
}

#[test]
fn conflicts_with_snake_case() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(long = "hh-os")]
        hh_os: Option<String>,
        #[arg(long, conflicts_with = "hh_os")]
        lol: Option<String>,
    }

    assert_eq!(
        Opt {
            hh_os: Some("muahaha".to_string()),
            lol: None,
        },
        Opt::try_parse_from(["test", "--hh-os", "muahaha"]).unwrap()
    );

    assert_eq!(
        Opt {
            hh_os: None,
            lol: Some("haha".to_string()),
        },
        Opt::try_parse_from(["test", "--lol", "haha"]).unwrap()
    );

    assert!(Opt::try_parse_from(["test", "--hh-os", "muahaha", "--lol", "haha"]).is_err());
}
