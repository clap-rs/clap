extern crate clap;

use clap::{App, Arg};

#[test]
#[should_panic]
fn unique_arg_names() {
    App::new("some").args(&[Arg::with_name("arg").short("a"),
                            Arg::with_name("arg").short("b")]);
}

#[test]
#[should_panic]
fn unique_arg_shorts() {
    App::new("some").args(&[Arg::with_name("arg1").short("a"),
                            Arg::with_name("arg2").short("a")]);
}

#[test]
#[should_panic]
fn unique_arg_longs() {
    App::new("some").args(&[Arg::with_name("arg1").long("long"),
                            Arg::with_name("arg2").long("long")]);
}
