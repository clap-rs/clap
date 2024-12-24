use crate::common;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(snapbox::file!["../snapshots/basic.bash.roff"], cmd);
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches(snapbox::file!["../snapshots/feature_sample.bash.roff"], cmd);
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/special_commands.bash.roff"],
        cmd,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches(snapbox::file!["../snapshots/quoting.bash.roff"], cmd);
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches(snapbox::file!["../snapshots/aliases.bash.roff"], cmd);
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/sub_subcommands.bash.roff"],
        cmd,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches(snapbox::file!["../snapshots/value_hint.bash.roff"], cmd);
}

#[test]
fn hidden_options() {
    let name = "my-app";
    let cmd = common::hidden_option_command(name);
    common::assert_matches(snapbox::file!["../snapshots/hidden_option.bash.roff"], cmd);
}

#[test]
fn value_env() {
    let name = "my-app";
    let cmd = common::env_value_command(name);
    common::assert_matches(snapbox::file!["../snapshots/value_env.bash.roff"], cmd);
}

#[test]
fn possible_values() {
    let name = "my-app";
    let cmd = common::possible_values_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/possible_values.bash.roff"],
        cmd,
    );
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
        common::assert_matches(
            snapbox::file!["../snapshots/sub_subcommand_help.roff"],
            cmd.clone(),
        );
    }
}

#[test]
fn help_headings() {
    let name = "my-app";
    let cmd = common::help_headings(name);
    common::assert_matches(snapbox::file!["../snapshots/help_headings.bash.roff"], cmd);
}

#[test]
fn value_name_without_arg() {
    let name = "my-app";
    let cmd = common::value_name_without_arg(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_name_without_arg.bash.roff"],
        cmd,
    );
}
