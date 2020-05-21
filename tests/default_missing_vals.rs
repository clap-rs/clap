use clap::{App, Arg};

#[test]
fn opt_missing() {
    let r = App::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .min_values(0)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "auto");
    assert_eq!(m.occurrences_of("color"), 0);
}

#[test]
fn opt_present_with_missing_value() {
    let r = App::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .min_values(0)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "always");
    assert_eq!(m.occurrences_of("color"), 1);
}

#[test]
fn opt_present_with_value() {
    let r = App::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .min_values(0)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color=never"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "never");
    assert_eq!(m.occurrences_of("color"), 1);
}

// ToDO: [2020-05-20; rivy] test currently fails as empty values are still a work-in-progress (see <https://github.com/clap-rs/clap/pull/1587#issuecomment-631648788>)
// #[test]
// fn opt_present_with_empty_value() {
//     let r = App::new("df")
//         .arg(Arg::new("color").long("color")
//             .default_value("auto")
//             .min_values(0)
//             .require_equals(true)
//             .default_missing_value("always")
//         )
//         .try_get_matches_from(vec!["", "--color="]);
//     assert!(r.is_ok());
//     let m = r.unwrap();
//     assert!(m.is_present("color"));
//     assert_eq!(m.value_of("color").unwrap(), "");
//     assert_eq!(m.occurrences_of("color"), 1);
// }

//## `default_value`/`default_missing_value` non-interaction checks

#[test]
fn opt_default() {
    // assert no change to usual argument handling when adding default_missing_value()
    let r = App::new("app")
        .arg(
            Arg::from("-o [opt] 'some opt'")
                .default_value("default")
                .default_missing_value("default_missing"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "default");
}

#[test]
fn opt_default_user_override() {
    // assert no change to usual argument handling when adding default_missing_value()
    let r = App::new("app")
        .arg(
            Arg::from("-o [opt] 'some opt'")
                .default_value("default")
                .default_missing_value("default_missing"),
        )
        .try_get_matches_from(vec!["", "-o=value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "value");
}
