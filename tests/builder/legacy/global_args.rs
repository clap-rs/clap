use clap::{arg, Arg, Command};

#[test]
fn issue_1076() {
    let mut cmd = Command::new("myprog")
        .arg(
            Arg::new("GLOBAL_ARG")
                .long("global-arg")
                .help("Specifies something needed by the subcommands")
                .global(true)
                .takes_value(true)
                .default_value("default_value"),
        )
        .arg(
            Arg::new("GLOBAL_FLAG")
                .long("global-flag")
                .help("Specifies something needed by the subcommands")
                .global(true)
                .takes_value(true),
        )
        .subcommand(Command::new("outer").subcommand(Command::new("inner")));
    let _ = cmd.try_get_matches_from_mut(vec!["myprog"]);
    let _ = cmd.try_get_matches_from_mut(vec!["myprog"]);
    let _ = cmd.try_get_matches_from_mut(vec!["myprog"]);
}

#[test]
fn propagate_global_arg_in_subcommand_to_subsubcommand_1385() {
    let m1 = Command::new("foo")
        .subcommand(
            Command::new("sub1")
                .arg(Arg::new("arg1").long("arg1").takes_value(true).global(true))
                .subcommand(Command::new("sub1a")),
        )
        .try_get_matches_from(&["foo", "sub1", "--arg1", "v1", "sub1a"])
        .unwrap();
    assert_eq!(
        "v1",
        m1.subcommand_matches("sub1")
            .unwrap()
            .subcommand_matches("sub1a")
            .unwrap()
            .value_of("arg1")
            .unwrap()
    );
}

#[test]
fn propagate_global_arg_to_subcommand_in_subsubcommand_2053() {
    let m = Command::new("opts")
        .arg(arg!(--"global-flag").global(true))
        .arg(arg!(--"global-str" <str>).required(false).global(true))
        .subcommand(
            Command::new("test")
                .arg(arg!(--"sub-flag").global(true))
                .arg(arg!(--"sub-str" <str>).required(false).global(true))
                .subcommand(Command::new("test")),
        )
        .try_get_matches_from(&[
            "cmd",
            "test",
            "test",
            "--global-flag",
            "--global-str",
            "hello",
            "--sub-flag",
            "--sub-str",
            "world",
        ])
        .unwrap();
    assert_eq!(
        Some("world"),
        m.subcommand_matches("test").unwrap().value_of("sub-str")
    );
}

#[test]
fn global_arg_available_in_subcommand() {
    let m = Command::new("opt")
        .args(&[
            Arg::new("global").global(true).long("global"),
            Arg::new("not").global(false).long("not"),
        ])
        .subcommand(Command::new("ping"))
        .try_get_matches_from(&["opt", "ping", "--global"])
        .unwrap();

    assert!(m.is_present("global"));
    assert!(m.subcommand_matches("ping").unwrap().is_present("global"));
}

#[test]
fn deeply_nested_discovery() {
    let cmd = Command::new("a")
        .arg(arg!(--"long-a").global(true))
        .subcommand(
            Command::new("b")
                .arg(arg!(--"long-b").global(true))
                .subcommand(
                    Command::new("c")
                        .arg(arg!(--"long-c").global(true))
                        .subcommand(Command::new("d")),
                ),
        );

    let m = cmd
        .try_get_matches_from(["a", "b", "c", "d", "--long-a", "--long-b", "--long-c"])
        .unwrap();
    assert!(m.is_present("long-a"));
    let m = m.subcommand_matches("b").unwrap();
    assert!(m.is_present("long-b"));
    let m = m.subcommand_matches("c").unwrap();
    assert!(m.is_present("long-c"));
}
