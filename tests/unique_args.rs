use clap::{App, Arg};

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Argument name must be unique\n\n\t'arg1' is already in use")]
fn unique_arg_names() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").short('a'),
            Arg::with_name("arg1").short('b'),
        ])
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Argument short must be unique\n\n\t'-a' is already in use")]
fn unique_arg_shorts() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").short('a'),
            Arg::with_name("arg2").short('a'),
        ])
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Argument long must be unique\n\n\t'--long' is already in use")]
fn unique_arg_longs() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").long("long"),
            Arg::with_name("arg2").long("long"),
        ])
        .try_get_matches();
}
