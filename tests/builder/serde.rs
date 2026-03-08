#![cfg(feature = "serde")]

use clap::{Arg, ArgAction, ArgGroup, Command};

#[test]
fn command_round_trip() {
    let cmd = Command::new("test-app")
        .version("1.0")
        .author("Test Author")
        .about("A test application")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .action(ArgAction::Set)
                .help("Path to config file"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("Increase verbosity"),
        )
        .subcommand(
            Command::new("sub")
                .about("A subcommand")
                .arg(Arg::new("input").help("Input file")),
        );

    let json = serde_json::to_string(&cmd).expect("serialize Command");
    let deserialized: Command = serde_json::from_str(&json).expect("deserialize Command");

    assert_eq!(deserialized.get_name(), cmd.get_name());
    assert_eq!(
        deserialized.get_version(),
        cmd.get_version()
    );
    assert_eq!(
        deserialized.get_author(),
        cmd.get_author()
    );
    assert_eq!(
        deserialized.get_about().map(|s| s.to_string()),
        cmd.get_about().map(|s| s.to_string())
    );
    assert_eq!(
        deserialized.get_subcommands().count(),
        cmd.get_subcommands().count()
    );
    assert_eq!(
        deserialized.get_arguments().count(),
        cmd.get_arguments().count()
    );
}

#[test]
fn arg_round_trip() {
    let arg = Arg::new("test-arg")
        .short('t')
        .long("test")
        .action(ArgAction::Set)
        .help("A test argument")
        .required(true);

    let json = serde_json::to_string(&arg).expect("serialize Arg");
    let deserialized: Arg = serde_json::from_str(&json).expect("deserialize Arg");

    assert_eq!(deserialized.get_id(), arg.get_id());
    assert_eq!(deserialized.get_short(), arg.get_short());
    assert_eq!(deserialized.get_long(), arg.get_long());
    assert_eq!(
        deserialized.get_help().map(|s| s.to_string()),
        arg.get_help().map(|s| s.to_string())
    );
    assert!(deserialized.is_required_set());
}

#[test]
fn arg_group_round_trip() {
    let group = ArgGroup::new("test-group")
        .args(["arg1", "arg2"])
        .required(true);

    let json = serde_json::to_string(&group).expect("serialize ArgGroup");
    let deserialized: ArgGroup = serde_json::from_str(&json).expect("deserialize ArgGroup");

    assert_eq!(deserialized.get_id(), group.get_id());
    assert_eq!(deserialized.is_required_set(), group.is_required_set());
}

#[test]
fn command_with_aliases_round_trip() {
    let cmd = Command::new("app")
        .visible_alias("application")
        .arg(
            Arg::new("flag")
                .short('f')
                .long("flag")
                .visible_alias("fl")
                .action(ArgAction::SetTrue),
        );

    let json = serde_json::to_string(&cmd).expect("serialize Command with aliases");
    let deserialized: Command = serde_json::from_str(&json).expect("deserialize Command with aliases");

    assert_eq!(deserialized.get_name(), "app");
}

#[test]
fn command_with_groups_round_trip() {
    let cmd = Command::new("app")
        .arg(Arg::new("opt1").long("opt1").action(ArgAction::Set))
        .arg(Arg::new("opt2").long("opt2").action(ArgAction::Set))
        .group(
            ArgGroup::new("opts")
                .args(["opt1", "opt2"])
                .required(true),
        );

    let json = serde_json::to_string(&cmd).expect("serialize Command with groups");
    let deserialized: Command = serde_json::from_str(&json).expect("deserialize Command with groups");

    assert_eq!(deserialized.get_name(), "app");
    assert_eq!(deserialized.get_groups().count(), cmd.get_groups().count());
}
