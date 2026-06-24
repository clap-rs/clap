use clap::{arg, Arg, ArgAction, Command};

#[test]
fn multiple_occurrences_of_flags_long() {
    let m = Command::new("mo_flags_long")
        .args_override_self(true)
        .arg(arg!(--multflag "allowed multiple flag").action(ArgAction::SetTrue))
        .arg(arg!(--flag "disallowed multiple flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "--multflag", "--flag", "--multflag"])
        .unwrap();
    assert!(m.contains_id("multflag"));
    assert_eq!(m.get_one::<bool>("multflag").copied(), Some(true));
    assert!(m.contains_id("flag"));
    assert_eq!(m.get_one::<bool>("flag").copied(), Some(true));
}

#[test]
fn multiple_occurrences_of_flags_short() {
    let m = Command::new("mo_flags_short")
        .args_override_self(true)
        .arg(arg!(-m --multflag "allowed multiple flag").action(ArgAction::SetTrue))
        .arg(arg!(-f --flag "disallowed multiple flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-m", "-f", "-m"])
        .unwrap();
    assert!(m.contains_id("multflag"));
    assert_eq!(m.get_one::<bool>("multflag").copied(), Some(true));
    assert!(m.contains_id("flag"));
    assert_eq!(m.get_one::<bool>("flag").copied(), Some(true));
}

#[test]
fn multiple_occurrences_of_positional() {
    let cmd = Command::new("test").arg(Arg::new("multi").num_args(1..).action(ArgAction::Append));

    let m = cmd
        .clone()
        .try_get_matches_from(["test"])
        .expect("zero occurrences work");
    assert!(!m.contains_id("multi"));
    assert!(m.get_many::<String>("multi").is_none());

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "one"])
        .expect("single occurrence work");
    assert!(m.contains_id("multi"));
    assert_eq!(
        m.get_many::<String>("multi")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["one"]
    );

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "one", "two", "three", "four"])
        .expect("many occurrences work");
    assert!(m.contains_id("multi"));
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
    let cmd = Command::new("mo_flags_large_qty")
        .arg(arg!(-m --multflag "allowed multiple flag").action(ArgAction::Count));

    let args: Vec<&str> = vec![""].into_iter().chain(vec!["-m"; 200]).collect();
    let m = cmd.clone().try_get_matches_from(args).unwrap();
    assert!(m.contains_id("multflag"));
    assert_eq!(m.get_one::<u8>("multflag").copied(), Some(200));

    let args: Vec<&str> = vec![""].into_iter().chain(vec!["-m"; 500]).collect();
    let m = cmd.try_get_matches_from(args).unwrap();
    assert!(m.contains_id("multflag"));
    assert_eq!(m.get_one::<u8>("multflag").copied(), Some(u8::MAX));
}

#[cfg(feature = "env")]
#[test]
fn multiple_occurrences_of_before_env() {
    let cmd = Command::new("mo_before_env").arg(
        Arg::new("verbose")
            .env("VERBOSE")
            .short('v')
            .long("verbose")
            .action(ArgAction::Count),
    );

    let m = cmd.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(0));

    let m = cmd.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(1));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(2));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(3));
}

#[cfg(feature = "env")]
#[test]
fn multiple_occurrences_of_after_env() {
    let cmd = Command::new("mo_after_env").arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::Count)
            .env("VERBOSE"),
    );

    let m = cmd.clone().try_get_matches_from(vec![""]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(0));

    let m = cmd.clone().try_get_matches_from(vec!["", "-v"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(1));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(2));

    let m = cmd.clone().try_get_matches_from(vec!["", "-vvv"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(m.get_one::<u8>("verbose").copied(), Some(3));
}
