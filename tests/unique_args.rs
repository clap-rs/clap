use clap::{App, Arg};

#[test]
#[should_panic]
fn unique_arg_names() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg").short('a'),
            Arg::with_name("arg").short('b'),
        ])
        .try_get_matches();
}

#[test]
#[should_panic]
fn unique_arg_shorts() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").short('a'),
            Arg::with_name("arg2").short('a'),
        ])
        .try_get_matches();
}

#[test]
#[should_panic]
fn unique_arg_longs() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").long("long"),
            Arg::with_name("arg2").long("long"),
        ])
        .try_get_matches();
}
