// Copyright 2022 Bill Fraser (@wfraser) <wfraser@codewise.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{Args, Parser};

#[derive(Parser, Debug, PartialEq)]
struct Main {
    #[arg(short)]
    s: String,

    #[arg(long)]
    some_string: String,

    #[arg(long)]
    same_name: String,

    #[command(flatten)]
    foo_args: Foo,

    #[command(flatten)]
    bar_args: Bar,
}

#[derive(Args, Debug, PartialEq)]
#[command(prefix = "foo", next_help_heading = "Foo options")]
struct Foo {
    #[arg(long)]
    some_param: String,

    #[arg(long)]
    same_name: String, // without prefix, would conflict with the one in Main
}

#[derive(Args, Debug, PartialEq)]
#[command(prefix = "bar", rename_all = "pascal", next_help_heading = "Bar options")]
struct Bar {
    #[arg(long)]
    another_param: String,

    #[arg(long = "spaghetti")] // prefix does NOT get applied to this, nor does the rename_all.
    weird_name: String,
}

#[test]
fn test_all() {
    let expected = Main {
        s: "s-value".to_string(),
        some_string: "some-string-value".to_string(),
        same_name: "same-name-value".to_string(),
        foo_args: Foo {
            some_param: "foo-some-param-value".to_string(),
            same_name: "foo-same-name-value".to_string(),
        },
        bar_args: Bar {
            another_param: "bar-another-param-value".to_string(),
            weird_name: "bar-weird-name-value".to_string(),
        }
    };

    let result = Main::parse_from(&[
        "test",
        "-s", "s-value",
        "--some-string", "some-string-value",
        "--same-name", "same-name-value",
        "--foo.some-param", "foo-some-param-value",
        "--foo.same-name=foo-same-name-value",
        "--Bar.AnotherParam", "bar-another-param-value",
        "--spaghetti", "bar-weird-name-value",
    ]);
    assert_eq!(result, expected);
}
