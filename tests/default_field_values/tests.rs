//! A module for the actual test.
//!
//! This is necessary because rustc will choke if we try and inline this module, because the (stable) parser knows
//! that default field values are unstable, and complains even for `cfg()`ed out occurrences.

use clap::{CommandFactory, Parser};

// Copy of the same from tests/derive/util.rs
pub(crate) fn get_long_help<T: CommandFactory>() -> String {
    let output = <T as CommandFactory>::command()
        .render_long_help()
        .to_string();

    eprintln!("\n%%% LONG_HELP %%%:=====\n{output}\n=====\n");
    eprintln!("\n%%% LONG_HELP (DEBUG) %%%:=====\n{output:?}\n=====\n");

    output
}

#[test]
fn default_field_value() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
            arg: i32 = 3,
        }

    assert_eq!(Opt { arg: 3 }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(["test", "1"]).unwrap());

    let help = get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}
