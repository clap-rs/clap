extern crate clap;
extern crate regex;

use clap::{App, Arg};

include!("../clap-test.rs");

static HIDDEN_ARGS: &'static str = "test 1.4
Kevin K.
tests stuff

USAGE:
    test [FLAGS] [OPTIONS] [DUMMY]

FLAGS:
    -F, --flag2      some other flag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --option <opt>    some option";

#[test]
fn hidden_args() {
    let app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.4")
        .args(&[Arg::from_usage("-f, --flag 'some flag'").hidden(true),
                    Arg::from_usage("-F, --flag2 'some other flag'"),
                    Arg::from_usage("--option [opt] 'some option'"),
                    Arg::with_name("DUMMY").required(false).hidden(true)]);
    assert!(test::compare_output(app, "test --help", HIDDEN_ARGS, false));
}
