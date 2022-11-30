mod common;
mod derive;

use clap::CommandFactory;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches_path("tests/snapshots/basic.bash.roff", cmd);
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches_path("tests/snapshots/feature_sample.bash.roff", cmd);
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches_path("tests/snapshots/special_commands.bash.roff", cmd);
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches_path("tests/snapshots/quoting.bash.roff", cmd);
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches_path("tests/snapshots/aliases.bash.roff", cmd);
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches_path("tests/snapshots/sub_subcommands.bash.roff", cmd);
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches_path("tests/snapshots/value_hint.bash.roff", cmd);
}

#[test]
fn hidden_options() {
    let name = "my-app";
    let cmd = common::hidden_option_command(name);
    common::assert_matches_path("tests/snapshots/hidden_option.bash.roff", cmd);
}

#[test]
fn value_env() {
    let name = "my-app";
    let cmd = common::env_value_command(name);
    common::assert_matches_path("tests/snapshots/value_env.bash.roff", cmd);
}

#[test]
fn possible_values() {
    let name = "my-app";
    let cmd = common::possible_values_command(name);
    common::assert_matches_path("tests/snapshots/possible_values.bash.roff", cmd);
}

#[test]
fn flag_without_value() {
    let cmd = derive::FlagWithoutValue::command();
    common::assert_matches_path("tests/snapshots/flag_without_value.bash.roff", cmd);
}

#[test]
fn derive_basic_command() {
    let cmd = derive::BasicCommand::command();
    common::assert_matches_path("tests/snapshots/basic.bash.roff", cmd);
}

#[test]
fn sub_subcommands_help() {
    let name = "my-app";
    let mut cmd = common::sub_subcommands_command(name);
    cmd.build();
    let cmd = cmd
        .get_subcommands()
        .find(|cmd| cmd.get_display_name() == Some("my-app-help"));
    assert!(cmd.is_some(), "help subcommand not found in command");
    if let Some(cmd) = cmd {
        common::assert_matches_path("tests/snapshots/sub_subcommand_help.roff", cmd.clone());
    }
}
