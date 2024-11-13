use clap::{arg, error::ErrorKind, Arg, ArgAction, Command, CommandGroup};

use super::utils;
use snapbox::assert_data_eq;
use snapbox::str;

#[test]
fn command_group_help_output_one_ungrouped_command() {
    let visible_help: &str = "\
Usage: clap-test [COMMAND]

Commands:
  help  Print this message or the help of the given subcommand(s)

  test  Some help

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("clap-test")
        .version("2.6")
        .subcommand(
            Command::new("test")
                .about("Some help")
        )
        .command_group(CommandGroup::new("Test commands")
            .commands(&["test"]));


    utils::assert_output(cmd, "clap-test --help", visible_help, false);
}

#[test]
fn command_group_help_output_one_ungrouped_command_one_group() {
    let visible_help: &str = "\
Usage: clap-test [COMMAND]

Commands:
  help  Print this message or the help of the given subcommand(s)

Test:
  test  Some help

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("clap-test")
        .version("2.6")
        .subcommand(
            Command::new("test")
                .about("Some help")
        )
        .command_group(CommandGroup::new("Test commands")
            .help_heading("Test")
            .commands(&["test"]));


    utils::assert_output(cmd, "clap-test --help", visible_help, false);
}

#[test]
fn command_group_help_output_no_ungrouped_commands_no_heading() {
    let visible_help: &str = "\
Usage: clap-test [COMMAND]

  test  Some help
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("clap-test")
        .version("2.6")
        .subcommand(
            Command::new("test")
                .about("Some help")
        )
        .command_group(CommandGroup::new("test_commands")
            .commands(&["test", "help"]));


    utils::assert_output(cmd, "clap-test --help", visible_help, false);
}

#[test]
fn command_group_help_output_group_with_heading() {
    let visible_help: &str = "\
Usage: clap-test [COMMAND]

Test:
  test  Some help
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("clap-test")
        .version("2.6")
        .subcommand(
            Command::new("test")
                .about("Some help")
        )
        .command_group(CommandGroup::new("test_commands")
            .help_heading("Test")
            .commands(&["test", "help"]));


    utils::assert_output(cmd, "clap-test --help", visible_help, false);
}


#[test]
fn command_group_help_output_two_groups_with_headings() {
    let visible_help: &str = "\
Usage: clap-test [COMMAND]

TestGroup1:
  test1  Some help
  test2  Some help

TestGroup2:
  test3  Some help
  test4  Some help
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("clap-test")
        .version("2.6")
        .subcommand(
            Command::new("test1").about("Some help")
        )
        .subcommand(
            Command::new("test2").about("Some help")
        )
        .subcommand(
            Command::new("test3").about("Some help")
        )
        .subcommand(
            Command::new("test4").about("Some help")
        )
        .command_group(CommandGroup::new("test_commands1")
            .help_heading("TestGroup1")
            .commands(&["test1", "test2"]))
        .command_group(CommandGroup::new("test_commands2")
            .help_heading("TestGroup2")
            .commands(&["test3", "test4", "help"]));


    utils::assert_output(cmd, "clap-test --help", visible_help, false);
}

