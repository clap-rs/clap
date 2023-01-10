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
        #[arg(short)]
        x: u32,
        #[arg(skip)]
        s: u32,
    }

    assert!(Opt::try_parse_from(["test", "-x", "10", "20"]).is_err());

    let mut opt = Opt::try_parse_from(["test", "-x", "10"]).unwrap();
    assert_eq!(
        opt,
        Opt {
            x: 10,
            s: 0, // default
        }
    );
    opt.s = 42;

    opt.try_update_from(["test", "-x", "22"]).unwrap();

    assert_eq!(opt, Opt { x: 22, s: 42 });
}

#[test]
fn skip_2() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short)]
        x: u32,
        #[arg(skip)]
        ss: String,
        #[arg(skip)]
        sn: u8,

        y: u32,
        #[arg(skip)]
        sz: u16,

        t: u32,
    }

    assert_eq!(
        Opt::try_parse_from(["test", "-x", "10", "20", "30"]).unwrap(),
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
        #[arg(long, short)]
        number: u32,
        #[arg(skip)]
        k: Kind,
        #[arg(skip)]
        v: Vec<u32>,
    }

    assert_eq!(
        Opt::try_parse_from(["test", "-n", "10"]).unwrap(),
        Opt {
            number: 10,
            k: Kind::B,
            v: vec![],
        }
    );
}

#[test]
fn skip_help_doc_comments() {
    #[derive(Parser, Debug, PartialEq, Eq)]
    pub struct Opt {
        #[arg(skip, help = "internal_stuff")]
        a: u32,

        #[arg(skip, long_help = "internal_stuff\ndo not touch")]
        b: u32,

        /// Not meant to be used by clap.
        ///
        /// I want a default here.
        #[arg(skip)]
        c: u32,

        #[arg(short)]
        n: u32,
    }

    assert_eq!(
        Opt::try_parse_from(["test", "-n", "10"]).unwrap(),
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
    #[derive(Parser, Debug, PartialEq, Eq)]
    pub struct Opt {
        #[arg(long, short)]
        number: u32,

        #[arg(skip = "key")]
        k: String,

        #[arg(skip = vec![1, 2, 3])]
        v: Vec<u32>,
    }

    assert_eq!(
        Opt::try_parse_from(["test", "-n", "10"]).unwrap(),
        Opt {
            number: 10,
            k: "key".to_string(),
            v: vec![1, 2, 3]
        }
    );
}
