use crate::common;
use snapbox::assert_data_eq;

#[cfg(unix)]
const CMD: &str = "fish";
#[cfg(unix)]
type RuntimeBuilder = completest_pty::FishRuntimeBuilder;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/basic.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/feature_sample.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/special_commands.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/quoting.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/aliases.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/sub_subcommands.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn custom_bin_name() {
    let name = "my-app";
    let bin_name = "bin-name";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/custom_bin_name.fish"],
        clap_complete::shells::Fish,
        cmd,
        bin_name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_hint.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_terminator.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/two_multi_valued_arguments.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches(
        snapbox::file!["../snapshots/subcommand_last.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
#[cfg(unix)]
fn register_completion() {
    common::register_example::<RuntimeBuilder>("static", "exhaustive");
}

#[test]
#[cfg(unix)]
fn complete() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");

    let input = "exhaustive \t";
    let expected = snapbox::str![[r#"
% exhaustive 
action  help  (Print this message or the help of the given subcommand(s))  last    quote
alias   hint                                                               pacman  value
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive quote --choice \t";
    let actual = runtime.complete(input, &term).unwrap();
    let expected = snapbox::str![[r#"
% exhaustive quote --choice 
another shell  (something with a space)  bash  (bash (shell))  fish  (fish shell)  zsh  (zsh shell)
"#]];
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
fn register_dynamic_env() {
    common::register_example::<RuntimeBuilder>("dynamic-env", "exhaustive");
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
fn complete_dynamic_env_toplevel() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive \t\t";
    let expected = snapbox::str![[r#"
% exhaustive action 
action                                                             pacman                  --help        (Print help)
alias                                                              quote                   --version  (Print version)
help  (Print this message or the help of the given subcommand(s))  value                   -h            (Print help)
hint                                                               --global  (everywhere)  -V         (Print version)
last                                                               --generate  (generate)  
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
fn complete_dynamic_env_quoted_help() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive quote \t\t";
    let expected = snapbox::str![[r#"
% exhaustive quote 
cmd-backslash                                        (Avoid '/n')  --backticks    (For more information see `echo test`)
cmd-backticks              (For more information see `echo test`)  --backslash                              (Avoid '/n')
cmd-brackets                             (List packages [filter])  --brackets                   (List packages [filter])
cmd-double-quotes           (Can be "always", "auto", or "never")  --expansions  (Execute the shell command with $SHELL)
cmd-expansions            (Execute the shell command with $SHELL)  --choice                                             
cmd-single-quotes           (Can be 'always', 'auto', or 'never')  --global                                 (everywhere)
escape-help                                             (/tab "')  --help          (Print help (see more with '--help'))
help  (Print this message or the help of the given subcommand(s))  --version                             (Print version)
--single-quotes             (Can be 'always', 'auto', or 'never')  -h              (Print help (see more with '--help'))
--double-quotes             (Can be "always", "auto", or "never")  -V                                    (Print version)
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
fn complete_dynamic_env_option_value() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive action --choice=\t\t";
    let expected = snapbox::str![[r#"
% exhaustive action --choice=first 
--choice=first  --choice=second
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive action --choice=f\t";
    let expected = snapbox::str!["% exhaustive action --choice=first "];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
fn complete_dynamic_env_quoted_value() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive quote --choice \t\t";
    let expected = snapbox::str![[r#"
% exhaustive quote --choice another/ shell 
another shell  (something with a space)  bash  (bash (shell))  fish  (fish shell)  zsh  (zsh shell)
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive quote --choice an\t";
    let expected = snapbox::str!["% exhaustive quote --choice another/ shell "];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}
