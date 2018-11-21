extern crate clap;

use clap::{App, Arg};

#[test]
#[should_panic]
fn unique_arg_names() {
    let _ = App::new("some")
        .args(&[
            Arg::new("arg").short('a'),
            Arg::new("arg").short('b'),
        ])
        .try_get_matches();
}

#[test]
#[should_panic]
fn unique_arg_shorts() {
    let _ = App::new("some")
        .args(&[
            Arg::new("arg1").short('a'),
            Arg::new("arg2").short('a'),
        ])
        .try_get_matches();
}

#[test]
#[should_panic]
fn unique_arg_longs() {
    let _ = App::new("some")
        .args(&[
            Arg::new("arg1").long("long"),
            Arg::new("arg2").long("long"),
        ])
        .try_get_matches();
}
