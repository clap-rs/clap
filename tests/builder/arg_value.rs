use clap::{arg, Arg, Command};

#[test]
fn arg_value_dashed_with_short() {
    let arg = arg!(-a <"some-val"> "some arg");
    assert_eq!(
        arg,
        Arg::new("some-val")
            .short('a')
            .long("arg")
            .value_name("some-val")
    );

    let m = Command::new("cmd")
        .arg(arg)
        .try_get_matches_from(vec!["", "-a", "val"])
        .unwrap();
    assert!(m.is_present("some-val"));
    assert_eq!(m.value_of("some-val").unwrap(), "val");
}

#[test]
fn arg_value_dashed_with_long() {
    let arg = arg!(-a --arg <"some-val"> "some arg");
    assert_eq!(
        arg,
        Arg::new("arg")
            .short('a')
            .long("arg")
            .value_name("some-val")
    );

    let m = Command::new("cmd")
        .arg(arg)
        .try_get_matches_from(vec!["", "--arg", "some-val"])
        .unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "some-val");
}
