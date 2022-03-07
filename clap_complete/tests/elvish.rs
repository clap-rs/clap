mod common;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches_path(
        "tests/snapshots/basic.elvish.log",
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches_path(
        "tests/snapshots/feature_sample.elvish.log",
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches_path(
        "tests/snapshots/special_commands.elvish.log",
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches_path(
        "tests/snapshots/quoting.elvish.log",
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches_path(
        "tests/snapshots/aliases.elvish.log",
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches_path(
        "tests/snapshots/sub_subcommands.elvish.log",
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches_path(
        "tests/snapshots/value_hint.elvish.log",
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}
