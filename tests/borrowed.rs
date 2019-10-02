extern crate clap;
extern crate regex;

use clap::{App, Arg};

include!("../clap-test.rs");

#[test]
fn borrowed_args() {
    let arg = Arg::with_name("some")
        .short('s')
        .long("some")
        .help("other help");
    let arg2 = Arg::with_name("some2")
        .short('S')
        .long("some-thing")
        .help("other help");
    let result = App::new("sub_command_negate")
        .arg(Arg::with_name("test").index(1))
        .arg(&arg)
        .arg(&arg2)
        .subcommand(App::new("sub1").arg(&arg))
        .try_get_matches_from(vec!["prog"]);
    assert!(result.is_ok());
}
