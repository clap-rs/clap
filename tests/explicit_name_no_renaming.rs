mod utils;

use clap::Clap;
use utils::*;

#[test]
fn explicit_short_long_no_rename() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(short = ".", long = ".foo")]
        foo: Vec<String>,
    }

    assert_eq!(
        Opt {
            foo: vec!["short".into(), "long".into()]
        },
        Opt::parse_from(&["test", "-.", "short", "--.foo", "long"])
    );
}

#[test]
fn explicit_name_no_rename() {
    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(name = ".options")]
        foo: Vec<String>,
    }

    let help = get_long_help::<Opt>();
    assert!(help.contains("[.options]..."))
}
