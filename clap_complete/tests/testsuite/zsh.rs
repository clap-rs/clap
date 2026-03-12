#[allow(unused_imports)]
use snapbox::assert_data_eq;

use crate::common;

#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
const CMD: &str = "zsh";
#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
type RuntimeBuilder = completest_pty::ZshRuntimeBuilder;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/basic.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/feature_sample.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/special_commands.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/quoting.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/aliases.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/sub_subcommands.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn external_subcommands() {
    let name = "my-app";
    let cmd = common::external_subcommand(name);
    common::assert_matches(
        snapbox::file!["../snapshots/external_subcommands.zsh"],
        clap_complete::shells::Zsh,
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
        snapbox::file!["../snapshots/custom_bin_name.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        bin_name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_hint.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_terminator.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn multi_value_option() {
    let name = "my-app";
    let cmd = common::multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/multi_value_option.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn optional_value_option() {
    let name = "my-app";
    let cmd = common::optional_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_value_option.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn optional_multi_value_option() {
    let name = "my-app";
    let cmd = common::optional_multi_value_option_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/optional_multi_value_option.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/two_multi_valued_arguments.zsh"],
        clap_complete::shells::Zsh,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches(
        snapbox::file!["../snapshots/subcommand_last.zsh"],
        clap_complete::shells::Zsh,
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
help                                                      -- Print this message or the help of the given subcommand(s)
hint                                                      
pacman  action  global  alias  value  quote  empty  last  --                                                          
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive empty \t";
    let expected = snapbox::str!["% exhaustive empty "];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive --empty=\t";
    let expected = snapbox::str!["% exhaustive --empty="];
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

    let input = "exhaustive \t\t";
    let expected = snapbox::str![[r#"
% exhaustive
--generate      -- generate
--help          -- Print help
help            -- Print this message or the help of the given subcommand(s)
--empty-choice  alias           global          last            quote           
action          empty           hint            pacman          value           
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
% exhaustive quote
--help                              -- Print help (see more with '--help')                                            
cmd-backslash      --backslash      -- Avoid '/n'                                                                     
cmd-backticks      --backticks      -- For more information see `echo test`                                           
cmd-brackets       --brackets       -- List packages [filter]                                                         
cmd-double-quotes  --double-quotes  -- Can be "always", "auto", or "never"                                            
cmd-expansions     --expansions     -- Execute the shell command with $SHELL                                          
cmd-single-quotes  --single-quotes  -- Can be 'always', 'auto', or 'never'                                            
escape-help                         -- /tab/t"'                                                                       
help                                -- Print this message or the help of the given subcommand(s)                      
--choice
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
% exhaustive action --choice=
--choice=first   --choice=second
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive action --choice=f\t\t";
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
% exhaustive quote --choice
another shell  -- something with a space
bash           -- bash (shell)
fish           -- fish shell
zsh            -- zsh shell
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive quote --choice an\t\t";
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

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_empty_space() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    // Press left arrow twice to place cursor between the two spaces
    let input = "exhaustive quote  -\x1b[D\x1b[D\t\t";
    let expected = snapbox::str![[r#"
% exhaustive quote  -
--help                              -- Print help (see more with '--help')                                            
cmd-backslash      --backslash      -- Avoid '/n'                                                                     
cmd-backticks      --backticks      -- For more information see `echo test`                                           
cmd-brackets       --brackets       -- List packages [filter]                                                         
cmd-double-quotes  --double-quotes  -- Can be "always", "auto", or "never"                                            
cmd-expansions     --expansions     -- Execute the shell command with $SHELL                                          
cmd-single-quotes  --single-quotes  -- Can be 'always', 'auto', or 'never'                                            
escape-help                         -- /tab/t"'                                                                       
help                                -- Print this message or the help of the given subcommand(s)                      
--choice
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_dir_no_trailing_space() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    // First, complete to the directory name with slash.
    // A trailing slash should not be added after the slash.
    let input = "exhaustive hint --file tes\t\t";
    let expected = snapbox::str!["% exhaustive hint --file tests/"];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    // Verify hitting tab again shows the directory contents.
    // This only works if there is no trailing space after the slash.
    let input = "exhaustive hint --file tests/\t\t";
    let expected = snapbox::str![[r#"
% exhaustive hint --file tests/
tests/examples.rs  tests/snapshots    tests/testsuite
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

/// Static vs dynamic gap analysis for zsh (#3916)
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
help                                                      -- Print this message or the help of the given subcommand(s)
hint
pacman  action  global  alias  value  quote  empty  last  --
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive
--generate      -- generate
--help          -- Print help
help            -- Print this message or the help of the given subcommand(s)
--empty-choice  alias           global          last            quote
action          empty           hint            pacman          value
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
help                                                      -- Print this message or the help of the given subcommand(s)
one  two  --
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive global
--global   -- everywhere
--help     -- Print help (see more with '--help')
--version  -- Print version
help       -- Print this message or the help of the given subcommand(s)
one        two
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
--choice    -- enum
--count     -- number
--help      -- Print help
--set       -- value
--set-true  -- bool
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive action --
--choice    -- enum
--count     -- number
--help      -- Print help (see more with '--help')
--set       -- value
--set-true  -- bool
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
% exhaustive action --choice=
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
% exhaustive action --choice
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
            snapbox::str!["% exhaustive action --choice=first"];
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
        let static_expected = snapbox::str!["% exhaustive empty "];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str!["% exhaustive empty "];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }

    /// Scenario 10: Special character handling (quote subcommand options)
    #[test]
    fn special_characters() {
        if !common::has_command(CMD) {
            return;
        }

        let term = completest::Term::new();

        let input = "exhaustive quote \t\t";

        let mut static_runtime = common::load_runtime::<RuntimeBuilder>("static", "exhaustive");
        let static_actual = static_runtime.complete(input, &term).unwrap();
        let static_expected = snapbox::str![[r#"
% exhaustive quote
help                                                      -- Print this message or the help of the given subcommand(s)
cmd-backslash      cmd-brackets       cmd-single-quotes  --
cmd-backticks      cmd-double-quotes  cmd-expansions  escape-help
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive quote
--help                              -- Print help (see more with '--help')
cmd-backslash      --backslash      -- Avoid '/n'
cmd-backticks      --backticks      -- For more information see `echo test`
cmd-brackets       --brackets       -- List packages [filter]
cmd-double-quotes  --double-quotes  -- Can be "always", "auto", or "never"
cmd-expansions     --expansions     -- Execute the shell command with $SHELL
cmd-single-quotes  --single-quotes  -- Can be 'always', 'auto', or 'never'
escape-help                         -- /tab/t"'
help                                -- Print this message or the help of the given subcommand(s)
--choice
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
--flag    -- cmd flag
--flg     -- cmd flag
--help    -- Print help
--opt     -- cmd option
--option  -- cmd option
"#]];
        assert_data_eq!(static_actual, static_expected);

        let mut dynamic_runtime =
            common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");
        let dynamic_actual = dynamic_runtime.complete(input, &term).unwrap();
        let dynamic_expected = snapbox::str![[r#"
% exhaustive alias --
--flag    -- cmd flag
--flg     -- cmd flag
--help    -- Print help (see more with '--help')
--opt     -- cmd option
--option  -- cmd option
"#]];
        assert_data_eq!(dynamic_actual, dynamic_expected);
    }
}
