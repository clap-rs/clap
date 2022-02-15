// Hi, future me (or whoever you are)!
//
// Yes, we do need this attr.
// No, the warnings cannot be fixed otherwise.
// Accept and endure. Do not touch.
#![allow(unused)]

use clap::CommandFactory;

pub fn get_help<T: CommandFactory>() -> String {
    let mut output = Vec::new();
    <T as CommandFactory>::command()
        .write_help(&mut output)
        .unwrap();
    let output = String::from_utf8(output).unwrap();

    eprintln!("\n%%% HELP %%%:=====\n{}\n=====\n", output);
    eprintln!("\n%%% HELP (DEBUG) %%%:=====\n{:?}\n=====\n", output);

    output
}

pub fn get_long_help<T: CommandFactory>() -> String {
    let mut output = Vec::new();
    <T as CommandFactory>::command()
        .write_long_help(&mut output)
        .unwrap();
    let output = String::from_utf8(output).unwrap();

    eprintln!("\n%%% LONG_HELP %%%:=====\n{}\n=====\n", output);
    eprintln!("\n%%% LONG_HELP (DEBUG) %%%:=====\n{:?}\n=====\n", output);

    output
}

pub fn get_subcommand_long_help<T: CommandFactory>(subcmd: &str) -> String {
    let mut output = Vec::new();
    <T as CommandFactory>::command()
        .get_subcommands_mut()
        .find(|s| s.get_name() == subcmd)
        .unwrap()
        .write_long_help(&mut output)
        .unwrap();
    let output = String::from_utf8(output).unwrap();

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
