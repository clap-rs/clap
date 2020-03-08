use clap::{App, Arg};

#[test]
fn two_conflicting_arguments() {
    let a = App::new("two_conflicting_arguments")
        .arg(
            Arg::with_name("develop")
                .long("develop")
                .conflicts_with("production"),
        )
        .arg(
            Arg::with_name("production")
                .long("production")
                .conflicts_with("develop"),
        )
        .try_get_matches_from(vec!["", "--develop", "--production"]);

    assert!(a.is_err());
    let a = a.unwrap_err();
    assert_eq!(
        a.cause,
        "The argument \'--develop\' cannot be used with \'--production\'"
    );
}

#[test]
fn three_conflicting_arguments() {
    let a = App::new("two_conflicting_arguments")
        .arg(
            Arg::with_name("one")
                .long("one")
                .conflicts_with_all(&["two", "three"]),
        )
        .arg(
            Arg::with_name("two")
                .long("two")
                .conflicts_with_all(&["one", "three"]),
        )
        .arg(
            Arg::with_name("three")
                .long("three")
                .conflicts_with_all(&["one", "two"]),
        )
        .try_get_matches_from(vec!["", "--one", "--two", "--three"]);

    assert!(a.is_err());
    let a = a.unwrap_err();
    assert_eq!(
        a.cause,
        "The argument \'--one\' cannot be used with \'--two\'"
    );
}
