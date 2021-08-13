use clap::Clap;

mod utils;

use utils::*;

#[test]
fn default_value() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value = "3")]
        arg: i32,
    }
    assert_eq!(Opt { arg: 3 }, Opt::parse_from(&["test"]));
    assert_eq!(Opt { arg: 1 }, Opt::parse_from(&["test", "1"]));

    let help = get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}

#[test]
fn default_value_t() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value_t = 3)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 3 }, Opt::parse_from(&["test"]));
    assert_eq!(Opt { arg: 1 }, Opt::parse_from(&["test", "1"]));

    let help = get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}

#[test]
fn auto_default_value_t() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value_t)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 0 }, Opt::parse_from(&["test"]));
    assert_eq!(Opt { arg: 1 }, Opt::parse_from(&["test", "1"]));

    let help = get_long_help::<Opt>();
    assert!(help.contains("[default: 0]"));
}
