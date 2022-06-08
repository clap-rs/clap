#![cfg(feature = "unicode")]

#[test]
fn possible_values_ignore_case() {
    let m = clap::Command::new("pv")
        .arg(
            clap::Arg::new("option")
                .short('o')
                .long("option")
                .takes_value(true)
                .possible_value("ä")
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "Ä"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m.unwrap().value_of("option").is_some());
}
