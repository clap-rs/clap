use clap::{arg, Arg, Command};

#[test]
fn single_short_arg_without_value() {
    let cmd = Command::new("cmd").ignore_errors(true).arg(arg!(
        -c --config [FILE] "Sets a custom config file"
    ));

    let r = cmd.try_get_matches_from(vec!["cmd", "-c" /* missing: , "config file" */]);

    assert!(r.is_ok(), "unexpected error: {:?}", r);
    let m = r.unwrap();
    assert!(m.is_present("config"));
}

#[test]
fn single_long_arg_without_value() {
    let cmd = Command::new("cmd").ignore_errors(true).arg(arg!(
        -c --config [FILE] "Sets a custom config file"
    ));

    let r = cmd.try_get_matches_from(vec!["cmd", "--config" /* missing: , "config file" */]);

    assert!(r.is_ok(), "unexpected error: {:?}", r);
    let m = r.unwrap();
    assert!(m.is_present("config"));
}

#[test]
fn multiple_args_and_final_arg_without_value() {
    let cmd = Command::new("cmd")
        .ignore_errors(true)
        .arg(arg!(
            -c --config [FILE] "Sets a custom config file"
        ))
        .arg(arg!(
            -x --stuff [FILE] "Sets a custom stuff file"
        ))
        .arg(arg!(f: -f "Flag"));

    let r = cmd.try_get_matches_from(vec![
        "cmd", "-c", "file", "-f", "-x", /* missing: , "some stuff" */
    ]);

    assert!(r.is_ok(), "unexpected error: {:?}", r);
    let m = r.unwrap();
    assert_eq!(
        m.get_one::<String>("config").map(|v| v.as_str()),
        Some("file")
    );
    assert!(m.is_present("f"));
    assert!(matches!(
        m.try_get_one::<String>("stuff").unwrap_err(),
        clap::parser::MatchesError::ExpectedOne { .. }
    ));
}

#[test]
fn multiple_args_and_intermittent_arg_without_value() {
    let cmd = Command::new("cmd")
        .ignore_errors(true)
        .arg(arg!(
            -c --config[FILE] "Sets a custom config file"
        ))
        .arg(arg!(
            -x --stuff[FILE] "Sets a custom stuff file"
        ))
        .arg(arg!(f: -f "Flag"));

    let r = cmd.try_get_matches_from(vec![
        "cmd", "-x", /* missing: ,"some stuff" */
        "-c", "file", "-f",
    ]);

    assert!(r.is_ok(), "unexpected error: {:?}", r);
    let m = r.unwrap();
    assert_eq!(
        m.get_one::<String>("config").map(|v| v.as_str()),
        Some("file")
    );
    assert!(m.is_present("f"));
    assert!(matches!(
        m.try_get_one::<String>("stuff").unwrap_err(),
        clap::parser::MatchesError::ExpectedOne { .. }
    ));
}

#[test]
fn subcommand() {
    let cmd = Command::new("test")
        .ignore_errors(true)
        .subcommand(
            Command::new("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .takes_value(true)
                        .help("testing testing"),
                )
                .arg(
                    Arg::new("stuff")
                        .short('x')
                        .long("stuff")
                        .takes_value(true)
                        .help("stuf value"),
                ),
        )
        .arg(Arg::new("other").long("other"));

    let m = cmd
        .try_get_matches_from(vec![
            "myprog",
            "some",
            "--test", /* missing: ,"some val" */
            "-x",
            "some other val",
        ])
        .unwrap();

    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(
        sub_m.is_present("test"),
        "expected subcommand to be present due to partial parsing"
    );
    assert!(matches!(
        sub_m.try_get_one::<String>("test").unwrap_err(),
        clap::parser::MatchesError::ExpectedOne { .. }
    ));
    assert_eq!(
        sub_m.get_one::<String>("stuff").map(|v| v.as_str()),
        Some("some other val")
    );
}
