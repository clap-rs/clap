use clap::{error::ErrorKind, Arg, ArgAction, Command};

#[cfg(feature = "error-context")]
use super::utils;

#[test]
fn empty_values() {
    let m = Command::new("config")
        .arg(Arg::new("config").long("config").action(ArgAction::Set))
        .try_get_matches_from(["config", "--config", ""])
        .unwrap();
    assert_eq!(m.get_one::<String>("config").map(|v| v.as_str()), Some(""));
}

#[test]
fn empty_values_with_equals() {
    let m = Command::new("config")
        .arg(Arg::new("config").long("config").action(ArgAction::Set))
        .try_get_matches_from(["config", "--config="])
        .unwrap();
    assert_eq!(m.get_one::<String>("config").map(|v| v.as_str()), Some(""));

    let m = Command::new("config")
        .arg(Arg::new("config").short('c').action(ArgAction::Set))
        .try_get_matches_from(["config", "-c="])
        .unwrap();
    assert_eq!(m.get_one::<String>("config").map(|v| v.as_str()), Some(""))
}

#[test]
fn no_empty_values() {
    let m = Command::new("config")
        .arg(
            Arg::new("config")
                .long("config")
                .action(ArgAction::Set)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(["config", "--config", ""]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);

    let m = Command::new("config")
        .arg(
            Arg::new("config")
                .short('c')
                .action(ArgAction::Set)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(["config", "-c", ""]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue)
}

#[test]
fn no_empty_values_with_equals() {
    let m = Command::new("config")
        .arg(
            Arg::new("config")
                .long("config")
                .action(ArgAction::Set)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(["config", "--config="]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);

    let m = Command::new("config")
        .arg(
            Arg::new("config")
                .short('c')
                .action(ArgAction::Set)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(["config", "-c="]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn no_empty_values_without_equals() {
    let m = Command::new("config")
        .arg(
            Arg::new("config")
                .long("config")
                .action(ArgAction::Set)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(["config", "--config"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);

    let m = Command::new("config")
        .arg(
            Arg::new("config")
                .short('c')
                .action(ArgAction::Set)
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(["config", "-c"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue)
}

#[test]
#[cfg(feature = "error-context")]
fn no_empty_values_without_equals_but_requires_equals() {
    let cmd = Command::new("config").arg(
        Arg::new("config")
            .long("config")
            .action(ArgAction::Set)
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .require_equals(true),
    );
    let m = cmd.clone().try_get_matches_from(["config", "--config"]);
    // Should error on no equals rather than empty value.
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::NoEquals);

    static NO_EUQALS_ERROR: &str =
        "error: equal sign is needed when assigning values to '--config=<config>'

Usage: config [OPTIONS]

For more information, try '--help'.
";

    utils::assert_output(cmd, "config --config", NO_EUQALS_ERROR, true);
}
