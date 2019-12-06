mod utils;

use clap::Clap;
use utils::*;

#[test]
fn it_works() {
    #[derive(Debug, PartialEq, Clap)]
    #[clap(rename_all_env = "kebab")]
    struct BehaviorModel {
        #[clap(env)]
        be_nice: String,
    }

    let help = get_help::<BehaviorModel>();
    assert!(help.contains("[env: be-nice=]"));
}

#[test]
fn default_is_screaming() {
    #[derive(Debug, PartialEq, Clap)]
    struct BehaviorModel {
        #[clap(env)]
        be_nice: String,
    }

    let help = get_help::<BehaviorModel>();
    assert!(help.contains("[env: BE_NICE=]"));
}

#[test]
fn overridable() {
    #[derive(Debug, PartialEq, Clap)]
    #[clap(rename_all_env = "kebab")]
    struct BehaviorModel {
        #[clap(env)]
        be_nice: String,

        #[clap(rename_all_env = "pascal", env)]
        be_agressive: String,
    }

    let help = get_help::<BehaviorModel>();
    assert!(help.contains("[env: be-nice=]"));
    assert!(help.contains("[env: BeAgressive=]"));
}
