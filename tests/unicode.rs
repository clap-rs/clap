#![cfg(feature = "unicode")]

#[test]
fn possible_values_case_insensitive() {
    let m = clap::App::new("pv")
        .arg(
            clap::Arg::new("option")
                .short('o')
                .long("--option")
                .takes_value(true)
                .possible_value("ä")
                .case_insensitive(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "Ä"]);

    assert!(m.is_ok());
    assert!(m.unwrap().value_of("option").is_some());
}
