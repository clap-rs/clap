use clap::{arg, Arg, Command};

#[test]
fn arg_name_dashed() {
    let arg = arg!(["some-arg"] "some arg");
    assert_eq!(arg, Arg::new("some-arg").help("some arg"));

    let m = Command::new("flag")
        .arg(arg)
        .try_get_matches_from(vec!["", "some-val"])
        .unwrap();
    assert!(m.is_present("some-arg"));
    assert_eq!(m.value_of("some-arg").unwrap(), "some-val");
}
