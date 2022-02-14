#![cfg(feature = "yaml")]
#![allow(deprecated)]

use clap::{load_yaml, Command};

#[test]
fn create_cmd_from_yaml() {
    let yml = load_yaml!("app.yml");
    Command::from_yaml(yml);
}

#[test]
fn help_message() {
    let yml = load_yaml!("app.yml");
    let mut cmd = Command::from_yaml(yml);
    // Generate the full help message!
    let _ = cmd.get_matches_from_safe_borrow(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    cmd.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    println!("{}", &help_string);
    assert!(help_string.contains("tests positionals with exclusions\n"));
}

#[test]
fn author() {
    let yml = load_yaml!("app.yml");
    let mut cmd = Command::from_yaml(yml);
    // Generate the full help message!
    let _ = cmd.get_matches_from_safe_borrow(Vec::<String>::new());

    let mut help_buffer = Vec::new();
    cmd.write_help(&mut help_buffer).unwrap();
    let help_string = String::from_utf8(help_buffer).unwrap();
    println!("{}", &help_string);
    assert!(help_string.contains("Kevin K. <kbknapp@gmail.com>"));
}
