#[macro_use]
extern crate clap;
#[macro_use]
extern crate clap_derive;

use clap::{App, Arg};

#[derive(ArgEnum, Debug, PartialEq)]
#[case_sensitive]
enum ArgChoice {
    Foo,
    Bar,
    Baz,
}

#[test]
fn when_lowercase() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
            .arg(Arg::with_name("arg")
                .required(true)
                .takes_value(true)
                .possible_values(&ArgChoice::variants())
            ).get_matches_from_safe(vec![
                "",
                "foo",
            ]); // We expect this to fail.
    assert!(matches.is_err());
    assert_eq!(matches.unwrap_err().kind, clap::ErrorKind::InvalidValue);
}

#[test]
fn when_capitalized() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
            .arg(Arg::with_name("arg")
                .required(true)
                .takes_value(true)
                .possible_values(&ArgChoice::variants())
            ).get_matches_from_safe(vec![
                "",
                "Foo",
            ]).unwrap();
    let t = value_t!(matches.value_of("arg"), ArgChoice);
    assert!(t.is_ok());
    assert_eq!(t.unwrap(), ArgChoice::Foo);
}