#![cfg(feature = "yaml")]

use clap::{load_yaml, App, ValueHint};

#[test]
fn create_app_from_yaml() {
    let yaml = load_yaml!("fixtures/app.yaml");
    App::from(yaml);
}

// TODO: Uncomment to test yaml with 2 spaces https://github.com/chyh1990/yaml-rust/issues/101
// #[test]
// fn create_app_from_yaml_2spaces() {
//     let yaml = load_yaml!("fixtures/app_2space.yaml");
//     App::from(yaml);
// }

#[test]
fn help_message() {
    let yaml = load_yaml!("fixtures/app.yaml");
    let mut app = App::from(yaml);
    // Generate the full help message!
    let _ = app.try_get_matches_from_mut(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(help_string
        .contains("-h, --help                   prints help with a nonstandard description\n"));
}

#[test]
fn author() {
    let yaml = load_yaml!("fixtures/app.yaml");
    let mut app = App::from(yaml);
    // Generate the full help message!
    let _ = app.try_get_matches_from_mut(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(help_string.contains("Kevin K. <kbknapp@gmail.com>"));
}

// ValueHint must be parsed correctly from Yaml
#[test]
fn value_hint() {
    let yml = load_yaml!("fixtures/app.yaml");
    let app = App::from(yml);

    let arg = app
        .get_arguments()
        .find(|a| a.get_name() == "value_hint")
        .unwrap();
    assert_eq!(arg.get_value_hint(), ValueHint::FilePath);
}

#[test]
fn default_value_if_not_triggered_by_argument() {
    let yml = load_yaml!("fixtures/app.yaml");
    let app = App::from(yml);

    // Fixtures use "other" as value
    let matches = app.try_get_matches_from(vec!["prog", "wrong"]).unwrap();

    assert!(matches.value_of("positional2").is_none());
}

#[test]
fn default_value_if_triggered_by_matching_argument() {
    let yml = load_yaml!("fixtures/app.yaml");
    let app = App::from(yml);

    let matches = app.try_get_matches_from(vec!["prog", "other"]).unwrap();
    assert_eq!(matches.value_of("positional2").unwrap(), "something");
}

#[test]
fn default_value_if_triggered_by_flag() {
    let yml = load_yaml!("fixtures/app.yaml");
    let app = App::from(yml);

    let matches = app
        .try_get_matches_from(vec!["prog", "--flag", "flagvalue"])
        .unwrap();

    assert_eq!(matches.value_of("positional2").unwrap(), "some");
}

#[test]
fn default_value_if_triggered_by_flag_and_argument() {
    let yml = load_yaml!("fixtures/app.yaml");
    let app = App::from(yml);

    let matches = app
        .try_get_matches_from(vec!["prog", "--flag", "flagvalue", "other"])
        .unwrap();

    // First condition triggers, therefore "some"
    assert_eq!(matches.value_of("positional2").unwrap(), "some");
}

#[test]
fn yaml_multiple_occurrences() {
    let yaml = load_yaml!("fixtures/app.yaml");
    let matches = App::from(yaml)
        .try_get_matches_from(vec!["prog", "-vvv"])
        .unwrap();
    assert_eq!(matches.occurrences_of("verbose"), 3);
}

#[test]
fn yaml_multiple_values() {
    let yaml = load_yaml!("fixtures/app.yaml");
    let matches = App::from(yaml)
        .try_get_matches_from(vec!["prog", "-s", "aaa", "bbb"])
        .unwrap();
    assert_eq!(
        matches
            .values_of("settings")
            .unwrap()
            .collect::<Vec<&str>>(),
        vec!["aaa", "bbb"]
    );
}

#[cfg(feature = "regex")]
#[test]
fn regex_with_invalid_string() {
    let yml = load_yaml!("fixtures/app_regex.yaml");
    let app = App::from(yml);
    let res = app.try_get_matches_from(vec!["prog", "not a proper filter"]);

    assert!(res.is_err());
}

#[cfg(feature = "regex")]
#[test]
fn regex_with_valid_string() {
    let yml = load_yaml!("fixtures/app_regex.yaml");
    let app = App::from(yml);

    let matches = app.try_get_matches_from(vec!["prog", "*.txt"]).unwrap();

    assert_eq!(matches.value_of("filter").unwrap(), "*.txt");
}

#[cfg(feature = "regex")]
#[test]
#[should_panic]
fn regex_with_invalid_yaml() {
    let yml = load_yaml!("fixtures/app_regex_invalid.yaml");
    let _app = App::from(yml);
}
