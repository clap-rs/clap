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

#[test]
fn value_with_required_equals() {
    let name = "my-app";
    let cmd = common::value_with_required_equals(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_with_required_equals.bash.roff"],
        cmd,
    );
}

#[test]
fn optional_value_with_required_equals() {
    let name = "my-app";
    let cmd = common::optional_value_with_required_equals(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_with_required_equals_value.bash.roff"],
        cmd,
    );
}

#[test]
fn optional_value() {
    let name = "my-app";
    let cmd = common::optional_value(name);
    common::assert_matches(snapbox::file!["../snapshots/optional_value.bash.roff"], cmd);
}

#[test]
fn multiple_optional_values() {
    let name = "my-app";
    let cmd = common::multiple_optional_values(name);
    common::assert_matches(
        snapbox::file!["../snapshots/multiple_optional_values.bash.roff"],
        cmd,
    );
}

#[test]
fn variadic_values() {
    let name = "my-app";
    let cmd = common::variadic_values(name);
    common::assert_matches(
        snapbox::file!["../snapshots/variadic_values.bash.roff"],
        cmd,
    );
}

#[test]
fn configured_display_order_args() {
    let name = "my-app";
    let cmd = common::configured_display_order_args(name);
    common::assert_matches(
        snapbox::file!["../snapshots/configured_display_order_args.roff"],
        cmd,
    );
}

#[test]
fn configured_subcmd_order() {
    let name = "my-app";
    let cmd = common::configured_subcmd_order(name);
    common::assert_matches(
        snapbox::file!["../snapshots/configured_subcmd_order.roff"],
        cmd,
    );
}

#[test]
fn default_subcmd_order() {
    let name = "my-app";
    let cmd = common::default_subcmd_order(name);
    common::assert_matches(
        snapbox::file!["../snapshots/default_subcmd_order.roff"],
        cmd,
    );
}

#[test]
fn styled() {
    let name = "my-app";
    let cmd = common::styled(name);
    common::assert_matches(snapbox::file!["../snapshots/styled.bash.roff"], cmd);
}
