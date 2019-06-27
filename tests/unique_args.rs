extern crate clap;

use clap::{App, Arg};

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Arg names must be unique")]
fn unique_arg_names() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").short('a'),
            Arg::with_name("arg1").short('b'),
        ])
        .try_get_matches();
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Argument short must be unique")]
fn unique_arg_shorts() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").short('a'),
            Arg::with_name("arg2").short('a'),
        ])
        .try_get_matches();
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Argument long must be unique")]
fn unique_arg_longs() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").long("long"),
            Arg::with_name("arg2").long("long"),
        ])
        .try_get_matches();
}
