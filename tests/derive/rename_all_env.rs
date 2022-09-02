#![cfg(feature = "env")]

use crate::utils;

use clap::Parser;

#[test]
fn it_works() {
    #[derive(Debug, PartialEq, Parser)]
    #[command(rename_all_env = "kebab")]
    struct BehaviorModel {
        #[arg(env)]
        be_nice: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert!(help.contains("[env: be-nice=]"));
}

#[test]
fn default_is_screaming() {
    #[derive(Debug, PartialEq, Parser)]
    struct BehaviorModel {
        #[arg(env)]
        be_nice: String,
    }

    let help = utils::get_help::<BehaviorModel>();
    assert!(help.contains("[env: BE_NICE=]"));
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
    assert!(help.contains("[env: be-nice=]"));
    assert!(help.contains("[env: BeAggressive=]"));
}
