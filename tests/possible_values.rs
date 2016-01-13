extern crate clap;

use clap::{App, Arg, ClapErrorType};

#[test]
fn possible_values_of_positional() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("positional")
            .index(1)
            .possible_value("test123"))
        .get_matches_from_safe(vec!["", "test123"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("positional"));
    assert_eq!(m.value_of("positional"), Some("test123"));
}

#[test]
fn possible_values_of_positional_fail() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("positional")
            .index(1)
            .possible_value("test123"))
        .get_matches_from_safe(vec!["", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::InvalidValue);
}

#[test]
fn possible_values_of_positional_multiple() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("positional")
            .index(1)
            .possible_value("test123")
            .possible_value("test321")
            .multiple(true))
        .get_matches_from_safe(vec!["", "test123", "test321"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("positional"));
    assert_eq!(m.values_of("positional"), Some(vec!["test123", "test321"]));
}

#[test]
fn possible_values_of_positional_multiple_fail() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("positional")
            .index(1)
            .possible_value("test123")
            .possible_value("test321")
            .multiple(true))
        .get_matches_from_safe(vec!["", "test123", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::InvalidValue);
}

#[test]
fn possible_values_of_option() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("option")
            .short("-o")
            .long("--option")
            .takes_value(true)
            .possible_value("test123"))
        .get_matches_from_safe(vec!["", "--option", "test123"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.value_of("option"), Some("test123"));
}

#[test]
fn possible_values_of_option_fail() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("option")
            .short("-o")
            .long("--option")
            .takes_value(true)
            .possible_value("test123"))
        .get_matches_from_safe(vec!["", "--option", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::InvalidValue);
}

#[test]
fn possible_values_of_option_multiple() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("option")
            .short("-o")
            .long("--option")
            .takes_value(true)
            .possible_value("test123")
            .possible_value("test321")
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "--option", "test123",
            "--option", "test321",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.values_of("option"), Some(vec!["test123", "test321"]));
}

#[test]
fn possible_values_of_option_multiple_fail() {
    let m = App::new("possible_values")
        .arg(Arg::with_name("option")
            .short("-o")
            .long("--option")
            .takes_value(true)
            .possible_value("test123")
            .possible_value("test321")
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "--option", "test123",
            "--option", "notest",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::InvalidValue);
}
