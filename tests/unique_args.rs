extern crate clap;

use clap::{App, Arg};

#[test]
#[should_panic]
fn unique_arg_names() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg").short('a'),
            Arg::with_name("arg").short('b'),
        ])
        .get_matches_safe();
}

#[test]
#[should_panic]
fn unique_arg_shorts() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").short('a'),
            Arg::with_name("arg2").short('a'),
        ])
        .get_matches_safe();
}

#[test]
#[should_panic]
fn unique_arg_longs() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").long("long"),
            Arg::with_name("arg2").long("long"),
        ])
        .get_matches_safe();
}
