use clap::{arg, Arg, ArgAction, Command};

#[test]
fn multiple_occurrences_of_flags_long() {
    let m = Command::new("mo_flags_long")
        .arg(arg!(--multflag "allowed multiple flag").action(ArgAction::SetTrue))
        .arg(arg!(--flag "disallowed multiple flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "--multflag", "--flag", "--multflag"])
        .unwrap();
    assert!(m.is_present("multflag"));
    assert_eq!(m.get_one::<bool>("multflag").copied(), Some(true));
    assert!(m.is_present("flag"));
    assert_eq!(m.get_one::<bool>("flag").copied(), Some(true));
}

#[test]
fn multiple_occurrences_of_flags_short() {
    let m = Command::new("mo_flags_short")
        .arg(arg!(-m --multflag "allowed multiple flag").action(ArgAction::SetTrue))
        .arg(arg!(-f --flag "disallowed multiple flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-m", "-f", "-m"])
        .unwrap();
    assert!(m.is_present("multflag"));
    assert_eq!(m.get_one::<bool>("multflag").copied(), Some(true));
    assert!(m.is_present("flag"));
    assert_eq!(m.get_one::<bool>("flag").copied(), Some(true));
}

#[test]
fn multiple_occurrences_of_positional() {
    let cmd = Command::new("test").arg(
        Arg::new("multi")
            .multiple_values(true)
            .action(ArgAction::Append),
    );

    let m = cmd
        .clone()
        .try_get_matches_from(&["test"])
        .expect("zero occurrences work");
    assert!(!m.is_present("multi"));
    assert!(m.get_many::<String>("multi").is_none());

    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "one"])
        .expect("single occurrence work");
    assert!(m.is_present("multi"));
    assert_eq!(
        m.get_many::<String>("multi")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["one"]
    );

    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "one", "two", "three", "four"])
        .expect("many occurrences work");
    assert!(m.is_present("multi"));
    assert_eq!(
        m.get_many::<String>("multi")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
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
        .arg(arg!(-m --multflag "allowed multiple flag").action(ArgAction::Count))
        .try_get_matches_from(args)
        .unwrap();
    assert!(m.is_present("multflag"));
    assert_eq!(m.get_one::<u64>("multflag").copied(), Some(1024));
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
            .action(ArgAction::Count),
    );

    let m = cmd.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(0));

    let m = cmd.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(1));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(2));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(3));
}

#[cfg(feature = "env")]
#[test]
fn multiple_occurrences_of_after_env() {
    let cmd = Command::new("mo_after_env").arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .takes_value(false)
            .action(ArgAction::Count)
            .env("VERBOSE"),
    );

    let m = cmd.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(0));

    let m = cmd.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(1));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(2));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u64>("verbose").copied(), Some(3));
}
