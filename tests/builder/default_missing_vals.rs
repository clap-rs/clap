use clap::{arg, Arg, ArgAction, Command};

#[test]
fn opt_missing() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "auto"
    );
    assert_eq!(
        m.value_source("color").unwrap(),
        clap::parser::ValueSource::DefaultValue
    );
    assert_eq!(m.index_of("color"), Some(1));
}

#[test]
fn opt_present_with_missing_value() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "always"
    );
    assert_eq!(
        m.value_source("color").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
    assert_eq!(m.index_of("color"), Some(2));
}

#[test]
fn opt_present_with_value() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color=never"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "never"
    );
    assert_eq!(
        m.value_source("color").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
    assert_eq!(m.index_of("color"), Some(2));
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
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        ""
    );
    assert_eq!(
        m.value_source("color").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
    assert_eq!(m.index_of("color"), Some(2));
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
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_one::<String>("o").map(|v| v.as_str()).unwrap(),
        "default"
    );
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
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_one::<String>("o").map(|v| v.as_str()).unwrap(),
        "value"
    );
}

#[test]
fn default_missing_value_per_occurrence() {
    // assert no change to usual argument handling when adding default_missing_value()
    let r = Command::new("cmd")
        .arg(
            arg!(o: -o [opt] ... "some opt")
                .default_value("default")
                .default_missing_value("default_missing"),
        )
        .try_get_matches_from(vec!["", "-o", "-o=value", "-o"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(
        m.get_many::<String>("o")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        vec!["default_missing", "value", "default_missing"]
    );
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn default_missing_value_flag_value() {
    let cmd = Command::new("test").arg(
        Arg::new("flag")
            .long("flag")
            .action(ArgAction::Set)
            .num_args(0..=1)
            .default_value("false")
            .default_missing_value("true"),
    );

    let m = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()),
        Some("false")
    );
    assert_eq!(
        m.value_source("flag").unwrap(),
        clap::parser::ValueSource::DefaultValue
    );

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "--flag"])
        .unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()),
        Some("true")
    );
    assert_eq!(
        m.value_source("flag").unwrap(),
        clap::parser::ValueSource::CommandLine
    );

    let m = cmd
        .clone()
        .try_get_matches_from(["test", "--flag=true"])
        .unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()),
        Some("true")
    );
    assert_eq!(
        m.value_source("flag").unwrap(),
        clap::parser::ValueSource::CommandLine
    );

    let m = cmd.try_get_matches_from(["test", "--flag=false"]).unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()),
        Some("false")
    );
    assert_eq!(
        m.value_source("flag").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
}

#[test]
fn delimited_missing_value() {
    let cmd = Command::new("test").arg(
        Arg::new("flag")
            .long("flag")
            .default_value("one,two")
            .default_missing_value("three,four")
            .num_args(0..)
            .value_delimiter(',')
            .require_equals(true),
    );

    let m = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(
        m.get_many::<String>("flag")
            .unwrap()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
        vec!["one", "two"]
    );

    let m = cmd.try_get_matches_from(["test", "--flag"]).unwrap();
    assert_eq!(
        m.get_many::<String>("flag")
            .unwrap()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
        vec!["three", "four"]
    );
}

#[cfg(debug_assertions)]
#[test]
#[cfg(feature = "error-context")]
#[should_panic = "Argument `arg`'s default_missing_value=\"value\" failed validation: error: invalid value 'value' for '[arg]'"]
fn default_missing_values_are_possible_values() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(["one", "two"])
                .default_missing_value("value"),
        )
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[cfg(feature = "error-context")]
#[should_panic = "Argument `arg`'s default_missing_value=\"value\" failed validation: error: invalid value 'value' for '[arg]"]
fn default_missing_values_are_valid() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(clap::value_parser!(u32))
                .default_missing_value("value"),
        )
        .try_get_matches();
}

#[test]
fn valid_index() {
    let m = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .arg(Arg::new("sync").long("sync").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["df", "--color", "--sync"])
        .unwrap();
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "always"
    );
    assert_eq!(
        m.value_source("color").unwrap(),
        clap::parser::ValueSource::CommandLine
    );

    // Make sure the index reflects `--color`s position and not something else
    assert_eq!(m.index_of("color"), Some(2));
}
