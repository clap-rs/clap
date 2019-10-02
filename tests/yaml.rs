#![cfg(feature = "yaml")]

#[macro_use]
extern crate clap;

use clap::App;

#[test]
fn create_app_from_yaml() {
    let yml = load_yaml!("app.yml");
    App::from(yml);
}

// TODO: Uncomment to test yaml with 2 spaces https://github.com/chyh1990/yaml-rust/issues/101
// #[test]
// fn create_app_from_yaml_2spaces() {
//     let yml = load_yaml!("app_2space.yml");
//     App::from(yml);
// }

#[test]
fn help_message() {
    let yml = load_yaml!("app.yml");
    let mut app = App::from(yml);
    // Generate the full help message!
    let _ = app.try_get_matches_from_mut(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(
        help_string.contains("-h, --help             prints help with a nonstandard description\n")
    );
}

#[test]
fn author() {
    let yml = load_yaml!("app.yml");
    let mut app = App::from(yml);
    // Generate the full help message!
    let _ = app.try_get_matches_from_mut(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(help_string.contains("Kevin K. <kbknapp@gmail.com>"));
}
