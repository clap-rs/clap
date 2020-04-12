use clap::{App, Arg, ArgSettings};

#[test]
fn multiple_occurrences_of_flags_long() {
    let m = App::new("mo_flags_long")
        .arg(
            Arg::from("--multflag 'allowed multiple flag'")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .arg(Arg::from("--flag 'disallowed multiple flag'"))
        .get_matches_from(vec!["", "--multflag", "--flag", "--multflag"]);
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1)
}

#[test]
fn multiple_occurrences_of_flags_short() {
    let m = App::new("mo_flags_short")
        .arg(
            Arg::from("-m --multflag 'allowed multiple flag'")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .arg(Arg::from("-f --flag 'disallowed multiple flag'"))
        .get_matches_from(vec!["", "-m", "-f", "-m"]);
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn multiple_occurrences_of_flags_mixed() {
    let m = App::new("mo_flags_mixed")
        .arg(
            Arg::from("-m, --multflag1 'allowed multiple flag'")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .arg(
            Arg::from("-n, --multflag2 'another allowed multiple flag'")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .arg(Arg::from("-f, --flag 'disallowed multiple flag'"))
        .get_matches_from(vec![
            "",
            "-m",
            "-f",
            "-n",
            "--multflag1",
            "-m",
            "--multflag2",
        ]);
    assert!(m.is_present("multflag1"));
    assert_eq!(m.occurrences_of("multflag1"), 3);
    assert!(m.is_present("multflag2"));
    assert_eq!(m.occurrences_of("multflag2"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn multiple_occurrences_of_flags_large_quantity() {
    let args: Vec<&str> = vec![""]
        .into_iter()
        .chain(vec!["-m"; 1024].into_iter())
        .collect();
    let m = App::new("mo_flags_larg_qty")
        .arg(
            Arg::from("-m --multflag 'allowed multiple flag'")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .get_matches_from(args);
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 1024);
}

#[test]
fn multiple_occurrences_of_before_env() {
    let app = App::new("mo_before_env").arg(
        Arg::with_name("verbose")
            .env("VERBOSE")
            .short('v')
            .long("verbose")
            .takes_value(false)
            .multiple_occurrences(true),
    );

    let m = app.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 0);

    let m = app.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 1);

    let m = app.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 2);
    let m = app.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);
}

#[test]
fn multiple_occurrences_of_after_env() {
    let app = App::new("mo_after_env").arg(
        Arg::with_name("verbose")
            .short('v')
            .long("verbose")
            .takes_value(false)
            .multiple_occurrences(true)
            .env("VERBOSE"),
    );

    let m = app.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 0);

    let m = app.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 1);

    let m = app.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 2);
    let m = app.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(m.unwrap().occurrences_of("verbose"), 3);
}
