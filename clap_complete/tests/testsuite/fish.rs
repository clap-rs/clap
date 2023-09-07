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
    common::register_example("static", "exhaustive", completest::Shell::Fish);
}

#[test]
#[cfg(unix)]
fn complete() {
    if !common::has_command("fish") {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime("static", "exhaustive", completest::Shell::Fish);

    let input = "exhaustive \t";
    let expected = r#"% exhaustive
action  complete            (Register shell completions for this program)  hint  pacman  value
alias   help  (Print this message or the help of the given subcommand(s))  last  quote"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);

    let input = "exhaustive quote --choice \t";
    let actual = runtime.complete(input, &term).unwrap();
    let expected = r#"% exhaustive quote --choice
bash  (bash (shell))  fish  (fish shell)  zsh  (zsh shell)"#;
    snapbox::assert_eq(expected, actual);
}

#[cfg(all(unix, feature = "unstable-dynamic"))]
#[test]
fn register_dynamic() {
    common::register_example("dynamic", "exhaustive", completest::Shell::Fish);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
fn complete_dynamic() {
    if !common::has_command("fish") {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime("dynamic", "exhaustive", completest::Shell::Fish);

    let input = "exhaustive \t";
    let expected = r#"% exhaustive
action                                                             last              -V         (Print version)
alias                                                              pacman            --generate      (generate)
complete            (Register shell completions for this program)  quote             --global      (everywhere)
help  (Print this message or the help of the given subcommand(s))  value             --help        (Print help)
hint                                                               -h  (Print help)  --version  (Print version)"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);

    let input = "exhaustive quote \t";
    let expected = r#"% exhaustive quote
cmd-backslash                                        (Avoid '\n')
cmd-backticks              (For more information see `echo test`)
cmd-brackets                             (List packages [filter])
cmd-double-quotes           (Can be "always", "auto", or "never")
cmd-expansions            (Execute the shell command with $SHELL)
cmd-single-quotes           (Can be 'always', 'auto', or 'never')
escape-help                                             (\tab "')
help  (Print this message or the help of the given subcommand(s))
-h                                                   (Print help)
-V                                                (Print version)
--backslash                                          (Avoid '\n')
--backticks                (For more information see `echo test`)
--brackets                               (List packages [filter])
--double-quotes             (Can be "always", "auto", or "never")
--expansions              (Execute the shell command with $SHELL)
--global                                             (everywhere)
--help                                               (Print help)
--single-quotes             (Can be 'always', 'auto', or 'never')
--version                                         (Print version)"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);
}
