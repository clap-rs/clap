// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[test]
fn skip_1() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[clap(short, value_parser)]
        x: u32,
        #[clap(skip)]
        s: u32,
    }

    assert!(Opt::try_parse_from(&["test", "-x", "10", "20"]).is_err());

    let mut opt = Opt::try_parse_from(&["test", "-x", "10"]).unwrap();
    assert_eq!(
        opt,
        Opt {
            x: 10,
            s: 0, // default
        }
    );
    opt.s = 42;

    opt.update_from(&["test", "-x", "22"]);

    assert_eq!(opt, Opt { x: 22, s: 42 });
}

#[test]
fn skip_2() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[clap(short, value_parser)]
        x: u32,
        #[clap(skip)]
        ss: String,
        #[clap(skip)]
        sn: u8,
        #[clap(value_parser)]
        y: u32,
        #[clap(skip)]
        sz: u16,
        #[clap(value_parser)]
        t: u32,
    }

    assert_eq!(
        Opt::try_parse_from(&["test", "-x", "10", "20", "30"]).unwrap(),
        Opt {
            x: 10,
            ss: String::from(""),
            sn: 0,
            y: 20,
            sz: 0,
            t: 30,
        }
    );
}

#[test]
fn skip_enum() {
    #[derive(Debug, PartialEq)]
    #[allow(unused)]
    enum Kind {
        A,
        B,
    }

    impl Default for Kind {
        fn default() -> Self {
            Kind::B
        }
    }

    #[derive(Parser, Debug, PartialEq)]
    pub struct Opt {
        #[clap(long, short, value_parser)]
        number: u32,
        #[clap(skip)]
        k: Kind,
        #[clap(skip)]
        v: Vec<u32>,
    }

    assert_eq!(
        Opt::try_parse_from(&["test", "-n", "10"]).unwrap(),
        Opt {
            number: 10,
            k: Kind::B,
            v: vec![],
        }
    );
}

#[test]
fn skip_help_doc_comments() {
    #[derive(Parser, Debug, PartialEq)]
    pub struct Opt {
        #[clap(skip, help = "internal_stuff")]
        a: u32,

        #[clap(skip, long_help = "internal_stuff\ndo not touch")]
        b: u32,

        /// Not meant to be used by clap.
        ///
        /// I want a default here.
        #[clap(skip)]
        c: u32,

        #[clap(short, value_parser)]
        n: u32,
    }

    assert_eq!(
        Opt::try_parse_from(&["test", "-n", "10"]).unwrap(),
        Opt {
            n: 10,
            a: 0,
            b: 0,
            c: 0,
        }
    );
}

#[test]
fn skip_val() {
    #[derive(Parser, Debug, PartialEq)]
    pub struct Opt {
        #[clap(long, short, value_parser)]
        number: u32,

        #[clap(skip = "key")]
        k: String,

        #[clap(skip = vec![1, 2, 3])]
        v: Vec<u32>,
    }

    assert_eq!(
        Opt::try_parse_from(&["test", "-n", "10"]).unwrap(),
        Opt {
            number: 10,
            k: "key".to_string(),
            v: vec![1, 2, 3]
        }
    );
}
