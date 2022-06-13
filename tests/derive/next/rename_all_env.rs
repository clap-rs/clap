#![cfg(feature = "env")]

use crate::utils;

use clap::Parser;

#[test]
fn it_works() {
    #[derive(Debug, PartialEq, Parser)]
    #[clap(rename_all_env = "kebab")]
    struct BehaviorModel {
        #[clap(env, value_parser)]
        be_nice: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert!(help.contains("[env: be-nice=]"));
}

#[test]
fn default_is_screaming() {
    #[derive(Debug, PartialEq, Parser)]
    struct BehaviorModel {
        #[clap(env, value_parser)]
        be_nice: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert!(help.contains("[env: BE_NICE=]"));
}

#[test]
fn overridable() {
    #[derive(Debug, PartialEq, Parser)]
    #[clap(rename_all_env = "kebab")]
    struct BehaviorModel {
        #[clap(env, value_parser)]
        be_nice: String,

        #[clap(rename_all_env = "pascal", env, value_parser)]
        be_aggressive: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert!(help.contains("[env: be-nice=]"));
    assert!(help.contains("[env: BeAggressive=]"));
}
