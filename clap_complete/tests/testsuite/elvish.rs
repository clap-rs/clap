use crate::common;
#[allow(unused_imports)]
use snapbox::assert_data_eq;

#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
const CMD: &str = "elvish";
#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
type RuntimeBuilder = completest_pty::ElvishRuntimeBuilder;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/basic.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/feature_sample.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/special_commands.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/quoting.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/aliases.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/sub_subcommands.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn external_subcommands() {
    let name = "my-app";
    let cmd = common::external_subcommand(name);
    common::assert_matches(
        snapbox::file!["../snapshots/external_subcommands.elvish"],
        clap_complete::shells::Elvish,
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
        snapbox::file!["../snapshots/custom_bin_name.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        bin_name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_hint.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_terminator.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn multi_value_option() {
    let name = "my-app";
    let cmd = common::multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/multi_value_option.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn optional_value_option() {
    let name = "my-app";
    let cmd = common::optional_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_value_option.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn optional_multi_value_option() {
    let name = "my-app";
    let cmd = common::optional_multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_multi_value_option.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/two_multi_valued_arguments.elvish"],
        clap_complete::shells::Elvish,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches(
        snapbox::file!["../snapshots/subcommand_last.elvish"],
        clap_complete::shells::Elvish,
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
% exhaustive --empty-choice
 COMPLETING argument  
--empty-choice empty-choice                                             
--generate     generate                                                 
--help         Print help                                               
-h             Print help                                               
action         action                                                   
alias          alias                                                    
empty          empty                                                    
global         global                                                   
help           Print this message or the help of the given subcommand(s)
hint           hint                                                     
last           last                                                     
pacman         pacman                                                   
quote          quote                                                    
value          value                                                    
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive empty \t";
    let expected = snapbox::str![[r#"
error: no candidates
% exhaustive empty 
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive --empty=\t";
    let expected = snapbox::str![[r#"
error: no candidates
% exhaustive --empty=
"#]];
    let actual = runtime.complete(input, &term).unwrap();
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

    let input = "exhaustive \t";
    let expected = snapbox::str![[r#"
% exhaustive --empty-choice
 COMPLETING argument  
--empty-choice  --help  alias  global  hint  pacman  value
--generate      action  empty  help    last  quote 
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

    let input = "exhaustive quote \t";
    let expected = snapbox::str![[r#"
% exhaustive quote --backslash
 COMPLETING argument  
--backslash  --choice         --help           cmd-backticks      cmd-expansions     help
--backticks  --double-quotes  --single-quotes  cmd-brackets       cmd-single-quotes
--brackets   --expansions     cmd-backslash    cmd-double-quotes  escape-help      
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

    let input = "exhaustive action --choice=\t";
    let expected = snapbox::str![[r#"
% exhaustive action '--choice=first'
 COMPLETING argument  
--choice=first  --choice=second
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive action --choice=f\t";
    let expected = snapbox::str![[r#"
% exhaustive action '--choice=first'
 COMPLETING argument  
--choice=first
"#]];
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

    let input = "exhaustive quote --choice \t";
    let expected = snapbox::str![[r#"
% exhaustive quote --choice 'another shell'
 COMPLETING argument  
another shell  bash  fish  zsh
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive quote --choice an\t";
    let expected = snapbox::str![[r#"
% exhaustive quote --choice 'another shell'
 COMPLETING argument  
another shell
"#]];
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
    let expected = snapbox::str![[r#"
error: no candidates
error: no candidates
% exhaustive empty 
"#]];
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
    let expected = snapbox::str![[r#"
error: no candidates
% exhaustive --empty=
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

/// Static vs dynamic gap analysis for elvish (#3919)
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

        let input = "exhaustive \t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive --empty-choice
 COMPLETING argument
--empty-choice empty-choice
--generate     generate
--help         Print help
-h             Print help
action         action
alias          alias
empty          empty
global         global
help           Print this message or the help of the given subcommand(s)
hint           hint
last           last
pacman         pacman
quote          quote
value          value
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive --empty-choice
 COMPLETING argument
--empty-choice  --help  alias  global  hint  pacman  value
--generate      action  empty  help    last  quote
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

        let input = "exhaustive global \t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive global --global
 COMPLETING argument
--global   everywhere
--help     Print help
--version  Print version
-V         Print version
-h         Print help
help       Print this message or the help of the given subcommand(s)
one        one
two        two
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive global --global
 COMPLETING argument
--global   --help  --version  help  one  two
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

        let input = "exhaustive action --\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive action --choice
 COMPLETING argument
--choice    enum
--count     number
--help      Print help
--set       value
--set-true  bool
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive action --set-true
 COMPLETING argument
--choice    --count  --help  --set  --set-true
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

        let input = "exhaustive action --choice=\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive action --choice=first
 COMPLETING argument
first   second
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive action --choice=--choice=first
 COMPLETING argument
--choice=first  --choice=second
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

        let input = "exhaustive action --choice \t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive action --choice first
 COMPLETING argument
first   second
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive action --choice first
 COMPLETING argument
first   second
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 9: Empty subcommand
    #[test]
    fn empty_subcommand() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive empty \t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
error: no candidates
% exhaustive empty
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
error: no candidates
% exhaustive empty
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 10: Special character handling (quote subcommand options)
    #[test]
    fn special_characters() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive quote \t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive quote --backslash
 COMPLETING argument
--backslash       Avoid '/n'
--backticks       For more information see `echo test`
--brackets        List packages [filter]
--choice
--double-quotes   Can be "always", "auto", or "never"
--expansions      Execute the shell command with $SHELL
--help            Print help
--single-quotes   Can be 'always', 'auto', or 'never'
-h                Print help
cmd-backslash     Avoid '/n'
cmd-backticks     For more information see `echo test`
cmd-brackets      List packages [filter]
cmd-double-quotes Can be "always", "auto", or "never"
cmd-expansions    Execute the shell command with $SHELL
cmd-single-quotes Can be 'always', 'auto', or 'never'
escape-help       /tab/t"'
help              Print this message or the help of the given subcommand(s)
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive quote cmd-single-quotes
 COMPLETING argument
--choice           cmd-backslash      cmd-double-quotes  cmd-single-quotes  help
--help             cmd-backticks      cmd-expansions     escape-help
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

        let input = "exhaustive alias --\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive alias --flag
 COMPLETING argument
--flag    cmd flag
--flg     cmd flag
--help    Print help
--opt     cmd option
--option  cmd option
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive alias --flag
 COMPLETING argument
--flag  --flg  --help  --opt  --option
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }
}
