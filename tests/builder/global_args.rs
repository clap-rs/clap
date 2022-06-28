use clap::{arg, Arg, ArgAction, Command};

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
            .get_one::<String>("arg1")
            .map(|v| v.as_str())
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
        m.subcommand_matches("test")
            .unwrap()
            .get_one::<String>("sub-str")
            .map(|v| v.as_str())
    );
}

#[test]
fn global_arg_available_in_subcommand() {
    let m = Command::new("opt")
        .args(&[
            Arg::new("global")
                .global(true)
                .long("global")
                .action(ArgAction::SetTrue),
            Arg::new("not")
                .global(false)
                .long("not")
                .action(ArgAction::SetTrue),
        ])
        .subcommand(Command::new("ping"))
        .try_get_matches_from(&["opt", "ping", "--global"])
        .unwrap();

    assert!(*m.get_one::<bool>("global").expect("defaulted by clap"));
    assert!(*m
        .subcommand_matches("ping")
        .unwrap()
        .get_one::<bool>("global")
        .expect("defaulted by clap"));
}

#[test]
fn deeply_nested_discovery() {
    let cmd = Command::new("a")
        .arg(arg!(--"long-a").global(true).action(ArgAction::SetTrue))
        .subcommand(
            Command::new("b")
                .arg(arg!(--"long-b").global(true).action(ArgAction::SetTrue))
                .subcommand(
                    Command::new("c")
                        .arg(arg!(--"long-c").global(true).action(ArgAction::SetTrue))
                        .subcommand(Command::new("d")),
                ),
        );

    let m = cmd
        .try_get_matches_from(["a", "b", "c", "d", "--long-a", "--long-b", "--long-c"])
        .unwrap();
    assert!(*m.get_one::<bool>("long-a").expect("defaulted by clap"));
    let m = m.subcommand_matches("b").unwrap();
    assert!(*m.get_one::<bool>("long-b").expect("defaulted by clap"));
    let m = m.subcommand_matches("c").unwrap();
    assert!(*m.get_one::<bool>("long-c").expect("defaulted by clap"));
}

#[test]
fn global_overrides_default() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("name")
                .long("name")
                .global(true)
                .takes_value(true)
                .default_value("from_default"),
        )
        .subcommand(Command::new("sub"));

    let m = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(
        m.get_one::<String>("name").unwrap().as_str(),
        "from_default"
    );

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "--name", "from_arg"])
        .unwrap();
    assert_eq!(m.get_one::<String>("name").unwrap().as_str(), "from_arg");

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "--name", "from_arg", "sub"])
        .unwrap();
    assert_eq!(m.get_one::<String>("name").unwrap().as_str(), "from_arg");

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "sub", "--name", "from_arg"])
        .unwrap();
    assert_eq!(m.get_one::<String>("name").unwrap().as_str(), "from_arg");
}

#[test]
#[cfg(feature = "env")]
fn global_overrides_env() {
    std::env::set_var("GLOBAL_OVERRIDES_ENV", "from_env");

    let cmd = Command::new("test")
        .arg(
            Arg::new("name")
                .long("name")
                .global(true)
                .takes_value(true)
                .env("GLOBAL_OVERRIDES_ENV"),
        )
        .subcommand(Command::new("sub"));

    let m = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(m.get_one::<String>("name").unwrap().as_str(), "from_env");

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "--name", "from_arg"])
        .unwrap();
    assert_eq!(m.get_one::<String>("name").unwrap().as_str(), "from_arg");

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "--name", "from_arg", "sub"])
        .unwrap();
    assert_eq!(m.get_one::<String>("name").unwrap().as_str(), "from_arg");

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "sub", "--name", "from_arg"])
        .unwrap();
    assert_eq!(m.get_one::<String>("name").unwrap().as_str(), "from_arg");
}
