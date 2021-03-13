mod utils;

use clap::{App, Arg, ArgSettings, ErrorKind};

#[test]
fn empty_values() {
    let m = App::new("config")
        .arg(Arg::new("config").long("config").takes_value(true))
        .get_matches_from(&["config", "--config", ""]);
    assert_eq!(m.value_of("config"), Some(""));
}

#[test]
fn empty_values_with_equals() {
    let m = App::new("config")
        .arg(Arg::new("config").long("config").takes_value(true))
        .get_matches_from(&["config", "--config="]);
    assert_eq!(m.value_of("config"), Some(""));

    let m = App::new("config")
        .arg(Arg::new("config").short('c').takes_value(true))
        .get_matches_from(&["config", "-c="]);
    assert_eq!(m.value_of("config"), Some(""))
}

#[test]
fn no_empty_values() {
    let m = App::new("config")
        .arg(
            Arg::new("config")
                .long("config")
                .takes_value(true)
                .forbid_empty_values(true),
        )
        .try_get_matches_from(&["config", "--config", ""]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::EmptyValue);

    let m = App::new("config")
        .arg(
            Arg::new("config")
                .short('c')
                .takes_value(true)
                .forbid_empty_values(true),
        )
        .try_get_matches_from(&["config", "-c", ""]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::EmptyValue)
}

#[test]
fn no_empty_values_with_equals() {
    let m = App::new("config")
        .arg(
            Arg::new("config")
                .long("config")
                .takes_value(true)
                .setting(ArgSettings::ForbidEmptyValues),
        )
        .try_get_matches_from(&["config", "--config="]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::EmptyValue);

    let m = App::new("config")
        .arg(
            Arg::new("config")
                .short('c')
                .takes_value(true)
                .setting(ArgSettings::ForbidEmptyValues),
        )
        .try_get_matches_from(&["config", "-c="]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn no_empty_values_without_equals() {
    let m = App::new("config")
        .arg(
            Arg::new("config")
                .long("config")
                .takes_value(true)
                .setting(ArgSettings::ForbidEmptyValues),
        )
        .try_get_matches_from(&["config", "--config"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::EmptyValue);

    let m = App::new("config")
        .arg(
            Arg::new("config")
                .short('c')
                .takes_value(true)
                .setting(ArgSettings::ForbidEmptyValues),
        )
        .try_get_matches_from(&["config", "-c"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::EmptyValue)
}

#[test]
fn no_empty_values_without_equals_but_requires_equals() {
    let app = App::new("config").arg(
        Arg::new("config")
            .long("config")
            .takes_value(true)
            .setting(ArgSettings::ForbidEmptyValues)
            .setting(ArgSettings::RequireEquals),
    );
    let m = app.clone().try_get_matches_from(&["config", "--config"]);
    // Should error on no equals rather than empty value.
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::NoEquals);

    static NO_EUQALS_ERROR: &str =
        "error: Equal sign is needed when assigning values to '--config=<config>'.

USAGE:
    config [OPTIONS]

For more information try --help";

    assert!(utils::compare_output(
        app,
        "config --config",
        NO_EUQALS_ERROR,
        true
    ));
}
