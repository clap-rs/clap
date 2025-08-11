#![cfg(feature = "env")]

use clap::Parser;
use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

use crate::utils;

#[test]
fn it_works() {
    #[derive(Debug, PartialEq, Parser)]
    #[command(rename_all_env = "kebab")]
    struct BehaviorModel {
        #[arg(env)]
        be_nice: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert_data_eq!(help, str![[r#"
Usage: clap <BE_NICE>

Arguments:
  <BE_NICE>  [env: be-nice=]

Options:
  -h, --help  Print help

"#]].raw());
}

#[test]
fn default_is_screaming() {
    #[derive(Debug, PartialEq, Parser)]
    struct BehaviorModel {
        #[arg(env)]
        be_nice: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert_data_eq!(help, str![[r#"
Usage: clap <BE_NICE>

Arguments:
  <BE_NICE>  [env: BE_NICE=]

Options:
  -h, --help  Print help

"#]].raw());
}

#[test]
fn overridable() {
    #[derive(Debug, PartialEq, Parser)]
    #[command(rename_all_env = "kebab")]
    struct BehaviorModel {
        #[arg(env)]
        be_nice: String,

        #[arg(rename_all_env = "pascal", env)]
        be_aggressive: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert_data_eq!(help, str![[r#"
Usage: clap <BE_NICE> <BE_AGGRESSIVE>

Arguments:
  <BE_NICE>        [env: be-nice=]
  <BE_AGGRESSIVE>  [env: BeAggressive=]

Options:
  -h, --help  Print help

"#]].raw());
}
