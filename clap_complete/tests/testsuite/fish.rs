use crate::common;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches_path(
        "tests/snapshots/basic.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches_path(
        "tests/snapshots/feature_sample.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches_path(
        "tests/snapshots/special_commands.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches_path(
        "tests/snapshots/quoting.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches_path(
        "tests/snapshots/aliases.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches_path(
        "tests/snapshots/sub_subcommands.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches_path(
        "tests/snapshots/value_hint.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches_path(
        "tests/snapshots/value_terminator.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches_path(
        "tests/snapshots/two_multi_valued_arguments.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches_path(
        "tests/snapshots/subcommand_last.fish",
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
#[cfg(unix)]
fn register_completion() {
    if !common::has_command("fish") {
        return;
    }

    common::register_example("test", completest::Shell::Fish);
}

#[test]
#[cfg(unix)]
fn complete() {
    if !common::has_command("fish") {
        return;
    }

    let term = completest::Term::new();
    let runtime = common::load_runtime("test", completest::Shell::Fish);

    let input = "test \t";
    let expected = r#"% test
action  help  (Print this message or the help of the given subcommand(s))  last    quote
alias   hint                                                               pacman  value"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);
}
