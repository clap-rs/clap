// Hi, future me (or whoever you are)!
//
// Yes, we do need this attr.
// No, the warnings cannot be fixed otherwise.
// Accept and endure. Do not touch.
#![allow(unused)]

use clap::IntoApp;

pub fn get_help<T: IntoApp>() -> String {
    let mut output = Vec::new();
    <T as IntoApp>::into_app().write_help(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    eprintln!("\n%%% HELP %%%:=====\n{}\n=====\n", output);
    eprintln!("\n%%% HELP (DEBUG) %%%:=====\n{:?}\n=====\n", output);

    output
}

pub fn get_long_help<T: IntoApp>() -> String {
    let mut output = Vec::new();
    <T as IntoApp>::into_app()
        .write_long_help(&mut output)
        .unwrap();
    let output = String::from_utf8(output).unwrap();

    eprintln!("\n%%% LONG_HELP %%%:=====\n{}\n=====\n", output);
    eprintln!("\n%%% LONG_HELP (DEBUG) %%%:=====\n{:?}\n=====\n", output);

    output
}

pub fn get_subcommand_long_help<T: IntoApp>(subcmd: &str) -> String {
    let output = <T as IntoApp>::into_app()
        .try_get_matches_from(vec!["test", subcmd, "--help"])
        .expect_err("")
        .message;

    eprintln!(
        "\n%%% SUBCOMMAND `{}` HELP %%%:=====\n{}\n=====\n",
        subcmd, output
    );
    eprintln!(
        "\n%%% SUBCOMMAND `{}` HELP (DEBUG) %%%:=====\n{:?}\n=====\n",
        subcmd, output
    );

    output
}
