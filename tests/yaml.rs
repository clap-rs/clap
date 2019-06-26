#![cfg(feature="yaml")]

#[macro_use]
extern crate clap;

use clap::App;

#[test]
fn create_app_from_yaml() {
    let yml = load_yaml!("app.yml");
    App::from_yaml(yml);
}

#[test]
fn help_message() {
    let yml = load_yaml!("app.yml");
    let mut app = App::from_yaml(yml);
    // Generate the full help message!
    let _ = app.get_matches_from_safe_borrow(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(help_string.contains(
        "-h, --help             prints help with a nonstandard description\n"));
}

#[test]
fn about_subcommand_help() {
    let yml = load_yaml!("app.yml");
    let mut app = App::from_yaml(yml);
    // Generate the full help message!
    let _ = app.get_matches_from_safe_borrow(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(help_string.contains("SUBCOMMANDS:
    help      prints this help or help for sucommand(s) with a nonstandard description"));
}

#[test]
fn author() {
    let yml = load_yaml!("app.yml");
    let mut app = App::from_yaml(yml);
    // Generate the full help message!
    let _ = app.get_matches_from_safe_borrow(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    app.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    assert!(help_string.contains(
        "Kevin K. <kbknapp@gmail.com>"));
}
