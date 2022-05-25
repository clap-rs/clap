#![cfg(feature = "unicode")]

#[test]
fn possible_values_ignore_case() {
    let m = clap::Command::new("pv")
        .arg(
            clap::Arg::new("option")
                .short('o')
                .long("option")
                .takes_value(true)
                .value_parser(["ä"])
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "Ä"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m
        .unwrap()
        .get_one::<String>("option")
        .map(|v| v.as_str())
        .is_some());
}
