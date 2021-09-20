#![cfg(feature="yaml_inline_macros")]

use clap::{load_yaml, App};

mod common;

#[test]
fn replaces_env() {
    common::setup();

    let yaml = load_yaml!("fixtures/inline-env.yml");
    let mut app = App::from(yaml);
    assert_eq!(app.get_name(), "AppName");
}

#[test]
fn replaces_macros() {
    common::setup();

    let yaml = load_yaml!("fixtures/inline-macros.yml");
    let mut app = App::from(yaml);
    assert_eq!(app.get_name(), "clap");
    // Generate the full help message!
    let _ = app.try_get_matches_from_mut(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(help_string.contains("Kevin K. <kbknapp@gmail.com> Clap Maintainers"));
    assert!(help_string.contains("A simple to use, efficient, and full-featured"));
}
