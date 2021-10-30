mod utils;

use clap::Parser;
use utils::*;

#[test]
fn explicit_short_long_no_rename() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(short = '.', long = ".foo", multiple_occurrences(true))]
        foo: Vec<String>,
    }

    assert_eq!(
        Opt {
            foo: vec!["short".into(), "long".into()]
        },
        Opt::try_parse_from(&["test", "-.", "short", "--.foo", "long"]).unwrap()
    );
}

#[test]
fn explicit_name_no_rename() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(name = ".options")]
        foo: Vec<String>,
    }

    let help = get_long_help::<Opt>();
    assert!(help.contains("[.options]..."))
}
