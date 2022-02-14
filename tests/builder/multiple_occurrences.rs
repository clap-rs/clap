use clap::{arg, error::ErrorKind, Arg, Command};

#[test]
fn multiple_occurrences_of_flags_long() {
    let m = Command::new("mo_flags_long")
        .arg(arg!(--multflag "allowed multiple flag").multiple_occurrences(true))
        .arg(arg!(--flag "disallowed multiple flag"))
        .try_get_matches_from(vec!["", "--multflag", "--flag", "--multflag"])
        .unwrap();
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1)
}

#[test]
fn multiple_occurrences_of_flags_short() {
    let m = Command::new("mo_flags_short")
        .arg(arg!(-m --multflag "allowed multiple flag").multiple_occurrences(true))
        .arg(arg!(-f --flag "disallowed multiple flag"))
        .try_get_matches_from(vec!["", "-m", "-f", "-m"])
        .unwrap();
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn multiple_occurrences_of_flags_mixed() {
    let m = Command::new("mo_flags_mixed")
        .arg(arg!(-m --multflag1 "allowed multiple flag").multiple_occurrences(true))
        .arg(arg!(-n --multflag2 "another allowed multiple flag").multiple_occurrences(true))
        .arg(arg!(-f --flag "disallowed multiple flag"))
        .try_get_matches_from(vec![
            "",
            "-m",
            "-f",
            "-n",
            "--multflag1",
            "-m",
            "--multflag2",
        ])
        .unwrap();
    assert!(m.is_present("multflag1"));
    assert_eq!(m.occurrences_of("multflag1"), 3);
    assert!(m.is_present("multflag2"));
    assert_eq!(m.occurrences_of("multflag2"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn multiple_occurrences_of_positional() {
    let cmd = Command::new("test").arg(Arg::new("multi").multiple_occurrences(true));

    let m = cmd
        .clone()
        .try_get_matches_from(&["test"])
        .expect("zero occurrences work");
    assert!(!m.is_present("multi"));
    assert_eq!(m.occurrences_of("multi"), 0);
    assert!(m.values_of("multi").is_none());

    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "one"])
        .expect("single occurrence work");
    assert!(m.is_present("multi"));
    assert_eq!(m.occurrences_of("multi"), 1);
    assert_eq!(m.values_of("multi").unwrap().collect::<Vec<_>>(), ["one"]);

    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "one", "two", "three", "four"])
        .expect("many occurrences work");
    assert!(m.is_present("multi"));
    assert_eq!(m.occurrences_of("multi"), 4);
    assert_eq!(
        m.values_of("multi").unwrap().collect::<Vec<_>>(),
        ["one", "two", "three", "four"]
    );
}

#[test]
fn multiple_occurrences_of_flags_large_quantity() {
    let args: Vec<&str> = vec![""]
        .into_iter()
        .chain(vec!["-m"; 1024].into_iter())
        .collect();
    let m = Command::new("mo_flags_large_qty")
        .arg(arg!(-m --multflag "allowed multiple flag").multiple_occurrences(true))
        .try_get_matches_from(args)
        .unwrap();
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 1024);
}

#[cfg(feature = "env")]
#[test]
fn multiple_occurrences_of_before_env() {
    let cmd = Command::new("mo_before_env").arg(
        Arg::new("verbose")
            .env("VERBOSE")
            .short('v')
            .long("verbose")
            .takes_value(false)
            .multiple_occurrences(true),
    );

    let m = cmd.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 0);

    let m = cmd.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 1);

    let m = cmd.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 2);
    let m = cmd.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);
}

#[cfg(feature = "env")]
#[test]
fn multiple_occurrences_of_after_env() {
    let cmd = Command::new("mo_after_env").arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .takes_value(false)
            .multiple_occurrences(true)
            .env("VERBOSE"),
    );

    let m = cmd.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 0);

    let m = cmd.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 1);

    let m = cmd.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 2);
    let m = cmd.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);
}

#[test]
fn max_occurrences_implies_multiple_occurrences() {
    let cmd = Command::new("prog").arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .max_occurrences(3),
    );
    let m = cmd.try_get_matches_from(vec!["prog", "-vvv"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);

    // One max should not imply multiple occurrences
    let cmd = Command::new("prog").arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .max_occurrences(1),
    );

    let m = cmd.try_get_matches_from(vec!["prog", "-vvv"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnexpectedMultipleUsage);
}

#[test]
fn max_occurrences_try_inputs() {
    let cmd = Command::new("prog").arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .max_occurrences(3),
    );
    let m = cmd.clone().try_get_matches_from(vec!["prog", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 1);

    let m = cmd.clone().try_get_matches_from(vec!["prog", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 2);

    let m = cmd.clone().try_get_matches_from(vec!["prog", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);

    let m = cmd.clone().try_get_matches_from(vec!["prog", "-vvvv"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyOccurrences);

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["prog", "-v", "-v", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["prog", "-v", "-vv", "-v"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyOccurrences);
}

#[test]
fn max_occurrences_positional() {
    let cmd = Command::new("prog").arg(Arg::new("verbose").max_occurrences(3));
    let m = cmd.clone().try_get_matches_from(vec!["prog", "v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 1);

    let m = cmd.clone().try_get_matches_from(vec!["prog", "v", "v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 2);

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["prog", "v", "v", "v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["prog", "v", "v", "v", "v"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyOccurrences);
}
