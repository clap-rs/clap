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

mod utils;

use clap::Clap;
use utils::*;

#[test]
fn no_author_version_about() {
    #[derive(Clap, PartialEq, Debug)]
    #[clap(name = "foo")]
    struct Opt {}

    let output = get_long_help::<Opt>();
    assert!(output.starts_with("foo \n\nUSAGE:"));
}

#[test]
fn use_env() {
    #[derive(Clap, PartialEq, Debug)]
    #[clap(author, about, version)]
    struct Opt {}

    let output = get_long_help::<Opt>();
    assert!(output.starts_with("clap_derive"));
    assert!(output.contains("Guillaume Pinot <texitoi@texitoi.eu>, Clap Maintainers"));
    assert!(output.contains("Parse command line argument by defining a struct, derive crate"));
}

#[test]
fn explicit_version_not_str_lit() {
    const VERSION: &str = "custom version";

    #[derive(Clap)]
    #[clap(version = VERSION)]
    pub struct Opt {}

    let output = get_long_help::<Opt>();
    assert!(output.contains("custom version"));
}
