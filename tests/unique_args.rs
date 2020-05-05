use clap::{App, Arg};

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument names must be unique, but 'arg1' is in use by more than one argument or group"]
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
#[should_panic = "Short option names must be unique for each argument, but '-a' is in use by both 'arg1' and 'arg2'"]
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
#[should_panic = "Long option names must be unique for each argument, but '--long' is in use by both 'arg1' and 'arg2'"]
fn unique_arg_longs() {
    let _ = App::new("some")
        .args(&[
            Arg::with_name("arg1").long("long"),
            Arg::with_name("arg2").long("long"),
        ])
        .try_get_matches();
}
