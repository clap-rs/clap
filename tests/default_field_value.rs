#![feature(default_field_values)]
#![cfg(feature = "derive")]
#![cfg(feature = "help")]
#![cfg(feature = "usage")]
// #![cfg(nightly)]

use clap::{CommandFactory, Parser};

pub(crate) fn get_long_help<T: CommandFactory>() -> String {
    let output = <T as CommandFactory>::command()
        .render_long_help()
        .to_string();

    eprintln!("\n%%% LONG_HELP %%%:=====\n{output}\n=====\n");
    eprintln!("\n%%% LONG_HELP (DEBUG) %%%:=====\n{output:?}\n=====\n");

    output
}

#[test]
fn default_value() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        arg: i32 = 3,
    }
    assert_eq!(Opt { arg: 3 }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(["test", "1"]).unwrap());

    let help = get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}
