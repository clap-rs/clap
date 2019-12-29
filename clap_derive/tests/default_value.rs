use clap::Clap;

mod utils;

use utils::*;

#[test]
fn auto_default_value() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(default_value)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 0 }, Opt::parse_from(&["test"]));
    assert_eq!(Opt { arg: 1 }, Opt::parse_from(&["test", "1"]));

    let help = get_long_help::<Opt>();
    assert!(help.contains("[default: 0]"));
}
