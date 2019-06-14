extern crate clap;

use clap::{App, Arg};

// This tests a programmer error and will only succeed with debug_assertions enabled
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Non-unique argument name: arg1 is already in use")]
fn unique_arg_names() {
    App::new("some").args(&[Arg::with_name("arg1").short("a"), Arg::with_name("arg1").short("b")]);
}

// This tests a programmer error and will only succeed with debug_assertions enabled
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Argument short must be unique")]
fn unique_arg_shorts() {
    App::new("some").args(&[Arg::with_name("arg1").short("a"), Arg::with_name("arg2").short("a")]);
}

// This tests a programmer error and will only succeed with debug_assertions enabled
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Argument long must be unique")]
fn unique_arg_longs() {
    App::new("some")
        .args(&[Arg::with_name("arg1").long("long"), Arg::with_name("arg2").long("long")]);
}
