use super::utils;

use clap::Command;

#[test]
fn very_large_display_order() {
    let cmd = Command::new("test").subcommand(Command::new("sub").display_order(usize::MAX));

    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test [COMMAND]

Commands:
  help  Print this message or the help of the given subcommand(s)
  sub   

Options:
  -h, --help  Print help
",
        false,
    );
}
