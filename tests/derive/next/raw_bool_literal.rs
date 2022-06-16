// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[test]
fn raw_bool_literal() {
    #[derive(Parser, Debug, PartialEq)]
    #[clap(name = "raw_bool")]
    struct Opt {
        #[clap(raw(false))]
        a: String,
        #[clap(raw(true))]
        b: String,
    }

    assert_eq!(
        Opt {
            a: "one".into(),
            b: "--help".into()
        },
        Opt::try_parse_from(&["test", "one", "--", "--help"]).unwrap()
    );
}
