use crate::common;
#[allow(unused_imports)]
use snapbox::assert_data_eq;

#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
const CMD: &str = "fish";
#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
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
fn external_subcommands() {
    let name = "my-app";
    let cmd = common::external_subcommand(name);
    common::assert_matches(
        snapbox::file!["../snapshots/external_subcommands.fish"],
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
fn multi_value_option() {
    let name = "my-app";
    let cmd = common::multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/multi_value_option.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn optional_value_option() {
    let name = "my-app";
    let cmd = common::optional_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_value_option.fish"],
        clap_complete::shells::Fish,
        cmd,
        name,
    );
}

#[test]
fn optional_multi_value_option() {
    let name = "my-app";
    let cmd = common::optional_multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_multi_value_option.fish"],
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
#[cfg(feature = "unstable-shell-tests")]
fn register_completion() {
    common::register_example::<RuntimeBuilder>("static", "exhaustive");
}

#[test]
#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
fn complete() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");

    let input = "exhaustive \t";
    let expected = snapbox::str![[r#"
% exhaustive 
action  empty   help  (Print this message or the help of the given subcommand(s))  last    quote
alias   global  hint                                                               pacman  value
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive empty \t";
    let expected = snapbox::str![[r#"
% exhaustive empty 
Cargo.toml    CONTRIBUTING.md  LICENSE-APACHE  README.md  tests/
CHANGELOG.md  examples/        LICENSE-MIT     src/       
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive --empty=\t";
    let expected = snapbox::str!["% exhaustive --empty="];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive quote --choice \t";
    let actual = runtime.complete(input, &term).unwrap();
    let expected = snapbox::str![[r#"
% exhaustive quote --choice 
another  bash  (bash (shell))  fish  (fish shell)  shell  (something with a space)  zsh  (zsh shell)
"#]];
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn register_dynamic_env() {
    common::register_example::<RuntimeBuilder>("dynamic-env", "exhaustive");
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_env_toplevel() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive \t\t";
    let expected = snapbox::str![[r#"
% exhaustive empty 
empty   quote   last   help  (Print this message or the help of the given subcommand(s))  --help  (Print help)
global  value   alias  --generate                                             (generate)  
action  pacman  hint   --empty-choice                                                     
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_env_quoted_help() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive quote \t\t";
    let expected = snapbox::str![[r#"
% exhaustive quote cmd-single-quotes 
cmd-single-quotes           (Can be 'always', 'auto', or 'never')
cmd-double-quotes           (Can be "always", "auto", or "never")
cmd-backticks              (For more information see `echo test`)
cmd-backslash                                        (Avoid '/n')
cmd-brackets                             (List packages [filter])
cmd-expansions            (Execute the shell command with $SHELL)
escape-help                                             (/tab "')
help  (Print this message or the help of the given subcommand(s))
--single-quotes             (Can be 'always', 'auto', or 'never')
--double-quotes             (Can be "always", "auto", or "never")
--backticks                (For more information see `echo test`)
--backslash                                          (Avoid '/n')
--brackets                               (List packages [filter])
--expansions              (Execute the shell command with $SHELL)
--choice                                                         
--help                      (Print help (see more with '--help'))
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
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
#[cfg(feature = "unstable-shell-tests")]
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

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_empty_subcommand() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive empty \t\t";
    let expected = snapbox::str!["% exhaustive empty "];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_empty_option_value() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = "exhaustive --empty=\t";
    let expected = snapbox::str!["% exhaustive --empty="];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

/// Static vs dynamic gap analysis for fish (#3917)
///
/// Documents the differences between static (AOT) and dynamic completion
/// output for systematic comparison before stabilization.
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
mod gap_analysis {
    use super::*;

    /// Scenario 1: Top-level subcommands
    #[test]
    fn toplevel_subcommands() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive \t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive
action  empty   help  (Print this message or the help of the given subcommand(s))  last    quote
alias   global  hint                                                               pacman  value
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive empty
empty   quote   last   help  (Print this message or the help of the given subcommand(s))  --help  (Print help)
global  value   alias  --generate                                             (generate)
action  pacman  hint   --empty-choice
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 2: Nested subcommands
    #[test]
    fn nested_subcommands() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive global \t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive global
help  (Print this message or the help of the given subcommand(s))  one  two
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive global one
one  two  help  (Print this message or the help of the given subcommand(s))  --global  (everywhere)  --version  (Print version)  --help  (Print help (see more with '--help'))
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 3: Long options
    #[test]
    fn long_options() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive action --\t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive action --
--choice  (enum)  --count  (number)  --help  (Print help)  --set  (value)  --set-true  (bool)
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive action --set-true
--set-true  (bool)  --set  (value)  --count  (number)  --choice  (enum)  --help  (Print help (see more with '--help'))
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 4: Option values with =
    #[test]
    fn option_values_equals() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive action --choice=\t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive action --choice=
first   second
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive action --choice=first
first   second
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 5: Option values space-separated
    #[test]
    fn option_values_space() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive action --choice \t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive action --choice
first   second
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive action --choice first
first   second
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 6: Filtered option values
    #[test]
    fn filtered_option_values() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive action --choice=f\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected =
            snapbox::str!["% exhaustive action --choice=first "];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected =
            snapbox::str!["% exhaustive action --choice=first "];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 9: Empty subcommand
    #[test]
    fn empty_subcommand() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive empty \t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive empty
Cargo.toml    CONTRIBUTING.md  LICENSE-APACHE  README.md  tests/
CHANGELOG.md  examples/        LICENSE-MIT     src/
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str!["% exhaustive empty "];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 10: Special character handling (quote subcommand)
    #[test]
    fn special_characters() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive quote --choice \t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive quote --choice
another  bash  (bash (shell))  fish  (fish shell)  shell  (something with a space)  zsh  (zsh shell)
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive quote --choice another/ shell
another shell  (something with a space)  bash  (bash (shell))  fish  (fish shell)  zsh  (zsh shell)
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 12: Aliases
    #[test]
    fn aliases() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive alias --\t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive alias --
--flag  (cmd flag)  --flg  (cmd flag)  --help  (Print help)  --opt  (cmd option)  --option  (cmd option)
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive alias --flag
--flag  (cmd flag)  --flg  (cmd flag)  --option  (cmd option)  --opt  (cmd option)  --help  (Print help (see more with '--help'))
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }
}
