#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument names must be unique, but 'arg1' is in use by more than one argument or group"]
fn unique_arg_names() {
    use clap::{App, Arg};

    let _ = App::new("some")
        .args(&[Arg::new("arg1").short('a'), Arg::new("arg1").short('b')])
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Short option names must be unique for each argument, but '-a' is in use by both 'arg1' and 'arg2'"]
fn unique_arg_shorts() {
    use clap::{App, Arg};

    let _ = App::new("some")
        .args(&[Arg::new("arg1").short('a'), Arg::new("arg2").short('a')])
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Long option names must be unique for each argument, but '--long' is in use by both 'arg1' and 'arg2'"]
fn unique_arg_longs() {
    use clap::{App, Arg};

    let _ = App::new("some")
        .args(&[Arg::new("arg1").long("long"), Arg::new("arg2").long("long")])
        .try_get_matches();
}
