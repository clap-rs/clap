extern crate clap;

use clap::{App, Arg};

#[test]
#[should_panic]
fn unique_arg_names() {
    App::new("some").args(&[Arg::new("arg").short("a"),
                            Arg::new("arg").short("b")]);
}

#[test]
#[should_panic]
fn unique_arg_shorts() {
    App::new("some").args(&[Arg::new("arg1").short("a"),
                            Arg::new("arg2").short("a")]);
}

#[test]
#[should_panic]
fn unique_arg_longs() {
    App::new("some").args(&[Arg::new("arg1").long("long"),
                            Arg::new("arg2").long("long")]);
}
