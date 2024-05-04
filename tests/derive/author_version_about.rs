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

use clap::Parser;

#[test]
fn no_author_version_about() {
    #[derive(Parser, PartialEq, Debug)]
    #[command(name = "foo")]
    #[command(help_template = utils::FULL_TEMPLATE)]
    struct Opt {}

    let output = utils::get_long_help::<Opt>();
    assert!(output.starts_with("foo \n\nUsage:"));
}

#[test]
fn use_env() {
    #[derive(Parser, PartialEq, Debug)]
    #[command(author, about, version)]
    #[command(help_template = utils::FULL_TEMPLATE)]
    struct Opt {}

    let output = utils::get_long_help::<Opt>();
    assert!(output.starts_with("clap"));
    assert!(output
        .contains("A simple to use, efficient, and full-featured Command Line Argument Parser"));
}

#[test]
fn explicit_version_not_str_lit() {
    const VERSION: &str = "custom version";

    #[derive(Parser)]
    #[command(version = VERSION)]
    #[command(help_template = utils::FULL_TEMPLATE)]
    pub(crate) struct Opt {}

    let output = utils::get_long_help::<Opt>();
    assert!(output.contains("custom version"));
}
