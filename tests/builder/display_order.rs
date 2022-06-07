use super::utils;

use clap::Command;

#[test]
fn very_large_display_order() {
    let cmd = Command::new("test").subcommand(Command::new("sub").display_order(usize::MAX));

    utils::assert_output(
        cmd,
        "test --help",
        "test 

USAGE:
    test [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    sub     
",
        false,
    );
}
