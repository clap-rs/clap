use crate::common;
use clap::{Arg, Command};

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
fn value_name_without_arg() {
    let name = "my-app";
    let cmd = common::value_name_without_arg(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_name_without_arg.bash.roff"],
        cmd,
    );
}

#[test]
fn flatten_help_false() {
    let name = "my-app";
    let cmd = common::basic_command(name).flatten_help(false);
    common::assert_matches(snapbox::file!["../snapshots/basic.bash.roff"], cmd);
}

#[test]
fn flatten_help_true() {
    let name = "my-app";
    let cmd = common::basic_command(name).flatten_help(true);
    common::assert_matches(snapbox::file!["../snapshots/flatten_help.roff"], cmd);
}

#[test]
fn flatten_help_true_subcommand_required_true() {
    let name = "my-app";
    let cmd = common::basic_command(name)
        .flatten_help(true)
        .subcommand_required(true);
    common::assert_matches(
        snapbox::file!["../snapshots/flatten_help_subcommand_required.roff"],
        cmd,
    );
}

#[test]
fn flatten_help_true_subcommand_args_conflicts_with_subcommands() {
    let name = "my-app";
    let cmd = common::basic_command(name)
        .flatten_help(true)
        .subcommand_required(false)
        .args_conflicts_with_subcommands(false);
    common::assert_matches(snapbox::file!["../snapshots/flatten_help.roff"], cmd);
}

#[test]
fn flatten_basic() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    common::assert_matches(snapbox::file!["../snapshots/flatten_basic.roff"], cmd);
}

#[test]
fn flatten_help_cmd() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(
            Arg::new("parent")
                .long("parent")
                .help("foo")
                .long_help("bar"),
        )
        .subcommand(
            Command::new("test")
                .about("test command")
                .long_about("long some")
                .arg(Arg::new("child").long("child").help("foo").long_help("bar")),
        );

    common::assert_matches(snapbox::file!["../snapshots/flatten_help_cmd.roff"], cmd);
}

#[test]
fn flatten_with_global() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent").global(true))
        .subcommand(
            Command::new("test")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    common::assert_matches(snapbox::file!["../snapshots/flatten_with_global.roff"], cmd);
}

#[test]
fn flatten_arg_required() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent").required(true))
        .subcommand(
            Command::new("test")
                .about("test command")
                .arg(Arg::new("child").long("child").required(true)),
        );

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_arg_required.roff"],
        cmd,
    );
}

#[test]
fn flatten_with_external_subcommand() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .allow_external_subcommands(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_with_external_subcommand.roff"],
        cmd,
    );
}

#[test]
fn flatten_without_subcommands() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"));

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_without_subcommands.roff"],
        cmd,
    );
}

#[test]
fn flatten_with_subcommand_required() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .subcommand_required(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_with_subcommand_required.roff"],
        cmd,
    );
}

#[test]
fn flatten_with_args_conflicts_with_subcommands() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .subcommand_required(true)
        .args_conflicts_with_subcommands(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_with_args_conflicts_with_subcommands.roff"],
        cmd,
    );
}

#[test]
fn flatten_single_hidden_command() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .hide(true)
                .about("child1 command")
                .arg(Arg::new("child").long("child1")),
        );

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_single_hidden_command.roff"],
        cmd,
    );
}

#[test]
fn flatten_hidden_command() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .about("child1 command")
                .arg(Arg::new("child").long("child1")),
        )
        .subcommand(
            Command::new("child2")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
                .hide(true)
                .about("child3 command")
                .arg(Arg::new("child").long("child3")),
        );

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_hidden_command.roff"],
        cmd,
    );
}

#[test]
fn flatten_recursive() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .flatten_help(true)
                .about("child1 command")
                .arg(Arg::new("child").long("child1"))
                .subcommand(
                    Command::new("grandchild1")
                        .flatten_help(true)
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1"))
                        .subcommand(
                            Command::new("greatgrandchild1")
                                .about("greatgrandchild1 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild1")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild2")
                                .about("greatgrandchild2 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild2")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild3")
                                .about("greatgrandchild3 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild3")),
                        ),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        )
        .subcommand(
            Command::new("child2")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
                .hide(true)
                .about("child3 command")
                .arg(Arg::new("child").long("child3"))
                .subcommand(
                    Command::new("grandchild1")
                        .flatten_help(true)
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1"))
                        .subcommand(
                            Command::new("greatgrandchild1")
                                .about("greatgrandchild1 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild1")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild2")
                                .about("greatgrandchild2 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild2")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild3")
                                .about("greatgrandchild3 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild3")),
                        ),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        );

    common::assert_matches(snapbox::file!["../snapshots/flatten_recursive.roff"], cmd);
}

#[test]
fn flatten_not_recursive() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .about("child1 command")
                .arg(Arg::new("child").long("child1"))
                .subcommand(
                    Command::new("grandchild1")
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1")),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        )
        .subcommand(
            Command::new("child2")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
                .about("child3 command")
                .arg(Arg::new("child").long("child3")),
        );

    common::assert_matches(
        snapbox::file!["../snapshots/flatten_not_recursive.roff"],
        cmd,
    );
}
