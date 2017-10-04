extern crate clap;
extern crate regex;

use std::env;
use std::ffi::OsStr;

use clap::{App, Arg, ErrorKind};

#[test]
fn env_no_default() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(Arg::from_usage("[arg] 'some opt'").env("CLP_TEST_ENV"))
        .get_matches_from_safe(vec![""]);

    env::remove_var("CLP_TEST_ENV");
    assert!(r.is_ok());
    let m = r.unwrap();
    // assert!(m.is_present("arg")); // TODO: should this be true?
    assert_eq!(m.value_of("arg").unwrap(), "env");
}
