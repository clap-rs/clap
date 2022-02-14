use clap::{arg, Arg, ArgMatches, Command};

#[test]
fn opt_missing() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .min_values(0)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "auto");
    assert_eq!(m.occurrences_of("color"), 0);
}

#[test]
fn opt_present_with_missing_value() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .min_values(0)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "always");
    assert_eq!(m.occurrences_of("color"), 1);
}

#[test]
fn opt_present_with_value() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .min_values(0)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color=never"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "never");
    assert_eq!(m.occurrences_of("color"), 1);
}

#[test]
fn opt_present_with_empty_value() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color="]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "");
    assert_eq!(m.occurrences_of("color"), 1);
}

//## `default_value`/`default_missing_value` non-interaction checks

#[test]
fn opt_default() {
    // assert no change to usual argument handling when adding default_missing_value()
    let r = Command::new("cmd")
        .arg(
            arg!(o: -o [opt] "some opt")
                .default_value("default")
                .default_missing_value("default_missing"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "default");
}

#[test]
fn opt_default_user_override() {
    // assert no change to usual argument handling when adding default_missing_value()
    let r = Command::new("cmd")
        .arg(
            arg!(o: -o [opt] "some opt")
                .default_value("default")
                .default_missing_value("default_missing"),
        )
        .try_get_matches_from(vec!["", "-o=value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "value");
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn default_missing_value_flag_value() {
    let cmd = Command::new("test").arg(
        Arg::new("flag")
            .long("flag")
            .takes_value(true)
            .default_missing_value("true"),
    );

    fn flag_value(m: ArgMatches) -> bool {
        match m.value_of("flag") {
            None => false,
            Some(x) => x.parse().expect("non boolean value"),
        }
    }

    assert_eq!(
        flag_value(cmd.clone().try_get_matches_from(&["test"]).unwrap()),
        false
    );
    assert_eq!(
        flag_value(
            cmd.clone()
                .try_get_matches_from(&["test", "--flag"])
                .unwrap()
        ),
        true
    );
    assert_eq!(
        flag_value(
            cmd.clone()
                .try_get_matches_from(&["test", "--flag=true"])
                .unwrap()
        ),
        true
    );
    assert_eq!(
        flag_value(
            cmd.clone()
                .try_get_matches_from(&["test", "--flag=false"])
                .unwrap()
        ),
        false
    );
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument `arg`'s default_missing_value=value doesn't match possible values"]
fn default_missing_values_are_possible_values() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .possible_values(["one", "two"])
                .default_missing_value("value"),
        )
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument `arg`'s default_missing_value=value failed validation: invalid digit found in string"]
fn default_missing_values_are_valid() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .validator(|val| val.parse::<u32>().map_err(|e| e.to_string()))
                .default_missing_value("value"),
        )
        .try_get_matches();
}
