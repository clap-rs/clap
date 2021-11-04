use clap::{Arg, Parser};

mod utils;

use utils::*;

#[test]
fn modify_fn_default_value_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(modify_fn(clap::default_value_t))]
        arg: i32,
    }
    assert_eq!(Opt { arg: 0 }, Opt::try_parse_from(&["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(&["test", "1"]).unwrap());

    let help = get_long_help::<Opt>();
    assert!(help.contains("[default: 0]"));
}

#[test]
fn modify_fn_generate_about() {
    const MY_ABOUT: &str = "This could be generated";
    fn generate_about_and_def<T: ToString + Default>(arg: Arg) -> Arg {
        arg.about(MY_ABOUT)
           .modify_fn(clap::default_value_t::<T>)
    }
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(modify_fn(generate_about_and_def))]
        arg: String,
    }
    let help = get_long_help::<Opt>();
    assert!(help.contains(MY_ABOUT));
    assert!(help.contains("[default: ]"));
}
