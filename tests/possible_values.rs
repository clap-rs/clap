mod utils;

use clap::{App, Arg, ErrorKind};

#[cfg(feature = "suggestions")]
static PV_ERROR: &str = "error: 'slo' isn't a valid value for '--Option <option3>'
\t[possible values: fast, slow]

\tDid you mean 'slow'?

USAGE:
    clap-test --Option <option3>

For more information try --help";

#[cfg(not(feature = "suggestions"))]
static PV_ERROR: &'static str = "error: 'slo' isn't a valid value for '--Option <option3>'
\t[possible values: fast, slow]

USAGE:
    clap-test --Option <option3>

For more information try --help";

#[test]
fn possible_values_of_positional() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("positional")
                .index(1)
                .possible_value("test123"),
        )
        .try_get_matches_from(vec!["myprog", "test123"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("positional"));
    assert_eq!(m.value_of("positional"), Some("test123"));
}

#[test]
fn possible_values_of_positional_fail() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("positional")
                .index(1)
                .possible_value("test123"),
        )
        .try_get_matches_from(vec!["myprog", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidValue);
}

#[test]
fn possible_values_of_positional_multiple() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("positional")
                .index(1)
                .possible_value("test123")
                .possible_value("test321")
                .multiple(true),
        )
        .try_get_matches_from(vec!["myprog", "test123", "test321"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("positional"));
    assert_eq!(
        m.values_of("positional").unwrap().collect::<Vec<_>>(),
        vec!["test123", "test321"]
    );
}

#[test]
fn possible_values_of_positional_multiple_fail() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("positional")
                .index(1)
                .possible_value("test123")
                .possible_value("test321")
                .multiple(true),
        )
        .try_get_matches_from(vec!["myprog", "test123", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidValue);
}

#[test]
fn possible_values_of_option() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123"),
        )
        .try_get_matches_from(vec!["myprog", "--option", "test123"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.value_of("option"), Some("test123"));
}

#[test]
fn possible_values_of_option_fail() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123"),
        )
        .try_get_matches_from(vec!["myprog", "--option", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidValue);
}

#[test]
fn possible_values_of_option_multiple() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123")
                .possible_value("test321")
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "--option", "test123", "--option", "test321"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        vec!["test123", "test321"]
    );
}

#[test]
fn possible_values_of_option_multiple_fail() {
    let m = App::new("possible_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123")
                .possible_value("test321")
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "--option", "test123", "--option", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidValue);
}

#[test]
fn possible_values_output() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test -O slo",
        PV_ERROR,
        true
    ));
}

#[test]
fn case_insensitive() {
    let m = App::new("pv")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123")
                .possible_value("test321")
                .case_insensitive(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "TeSt123"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .value_of("option")
        .unwrap()
        .eq_ignore_ascii_case("test123"));
}

#[test]
fn case_insensitive_faili() {
    let m = App::new("pv")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123")
                .possible_value("test321"),
        )
        .try_get_matches_from(vec!["pv", "--option", "TeSt123"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidValue);
}

#[test]
fn case_insensitive_multiple() {
    let m = App::new("pv")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123")
                .possible_value("test321")
                .multiple(true)
                .case_insensitive(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "TeSt123", "teST123", "tESt321"]);

    assert!(m.is_ok());
    assert_eq!(
        m.unwrap().values_of("option").unwrap().collect::<Vec<_>>(),
        &["TeSt123", "teST123", "tESt321"]
    );
}

#[test]
fn case_insensitive_multiple_fail() {
    let m = App::new("pv")
        .arg(
            Arg::with_name("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("test123")
                .possible_value("test321")
                .multiple(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "test123", "teST123", "test321"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidValue);
}
