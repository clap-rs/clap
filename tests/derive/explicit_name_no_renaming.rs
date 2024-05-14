use crate::utils;

use clap::Parser;

#[test]
fn explicit_short_long_no_rename() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(short = '.', long = ".foo")]
        foo: String,
    }

    assert_eq!(
        Opt { foo: "long".into() },
        Opt::try_parse_from(["test", "--.foo", "long"]).unwrap()
    );

    assert_eq!(
        Opt {
            foo: "short".into(),
        },
        Opt::try_parse_from(["test", "-.", "short"]).unwrap()
    );
}

#[test]
fn explicit_name_no_rename() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(id = ".options")]
        foo: String,
    }

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("<.options>"));
}
