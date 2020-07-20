#![cfg(feature = "yaml")]

use clap::{load_yaml, App};

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
        .contains("-h, --help                prints help with a nonstandard description\n"));
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
