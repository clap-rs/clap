#![cfg(all(unix, feature = "unstable-shell-tests", feature = "unstable-dynamic"))]

use snapbox::assert_data_eq;

use crate::common;

/// Macro for concise E2E completion test definitions.
///
/// Generates a test function that:
/// 1. Checks if the required shell is available
/// 2. Loads a PTY runtime with the `exhaustive` example binary registered for dynamic completion
/// 3. Sends the given input to the shell and triggers tab completion
/// 4. Compares the actual completion output against the expected snapshot
///
/// # Arguments
///
/// * `$name` - Test function name
/// * `$cmd` - Shell command name (e.g. `"bash"`, `"fish"`, `"zsh"`, `"elvish"`)
/// * `$runtime_builder` - The `completest_pty` runtime builder type
/// * `$input` - The input string to type, including `\t` for tab completion
/// * `$expected` - The expected output as a `snapbox::str!` snapshot
macro_rules! e2e_test {
    ($name:ident, $cmd:expr, $runtime_builder:ty, $input:expr, $expected:expr $(,)?) => {
        #[test]
        fn $name() {
            if !common::has_command($cmd) {
                return;
            }

            let term = completest::Term::new();
            let mut runtime =
                common::load_runtime::<$runtime_builder>("dynamic-env", "exhaustive");

            let actual = runtime.complete($input, &term).unwrap();
            assert_data_eq!(actual, $expected);
        }
    };
}

// =============================================================================
// Bash E2E tests
// =============================================================================

e2e_test!(
    bash_subcommand_completion,
    "bash",
    completest_pty::BashRuntimeBuilder,
    "exhaustive \t\t",
    snapbox::str![[r#"
%
empty           action          value           last            hint            --generate      --help
global          quote           pacman          alias           help            --empty-choice
"#]],
);

e2e_test!(
    bash_long_option_completion,
    "bash",
    completest_pty::BashRuntimeBuilder,
    "exhaustive action --\t\t",
    snapbox::str![[r#"
%
--set-true  --set       --count     --choice    --help
"#]],
);

e2e_test!(
    bash_option_value_completion,
    "bash",
    completest_pty::BashRuntimeBuilder,
    "exhaustive action --choice=\t\t",
    snapbox::str![[r#"
%
first   second
"#]],
);

e2e_test!(
    bash_option_value_partial_completion,
    "bash",
    completest_pty::BashRuntimeBuilder,
    "exhaustive action --choice=f\t",
    snapbox::str!["exhaustive action --choice=f    % exhaustive action --choice=f"],
);

e2e_test!(
    bash_subcommand_option_completion,
    "bash",
    completest_pty::BashRuntimeBuilder,
    "exhaustive hint --\t\t",
    snapbox::str![[r#"
%
--choice    --unknown   --other     --path      --file      --dir       --exe       --cmd-name  --cmd       --user      --host      --url       --email     --help
"#]],
);

// =============================================================================
// Fish E2E tests
// =============================================================================

e2e_test!(
    fish_subcommand_completion,
    "fish",
    completest_pty::FishRuntimeBuilder,
    "exhaustive \t\t",
    snapbox::str![[r#"
% exhaustive empty
empty   quote   last   help  (Print this message or the help of the given subcommand(s))  --help  (Print help)
global  value   alias  --generate                                             (generate)
action  pacman  hint   --empty-choice
"#]],
);

e2e_test!(
    fish_long_option_completion,
    "fish",
    completest_pty::FishRuntimeBuilder,
    "exhaustive action --\t\t",
    snapbox::str![[r#"
% exhaustive action --set-true
--set-true  (bool)  --count  (number)  --help  (Print help (see more with '--help'))
--set      (value)  --choice    (enum)
"#]],
);

e2e_test!(
    fish_option_value_completion,
    "fish",
    completest_pty::FishRuntimeBuilder,
    "exhaustive action --choice=\t\t",
    snapbox::str![[r#"
% exhaustive action --choice=first
--choice=first  --choice=second
"#]],
);

e2e_test!(
    fish_option_value_partial_completion,
    "fish",
    completest_pty::FishRuntimeBuilder,
    "exhaustive action --choice=f\t",
    snapbox::str!["% exhaustive action --choice=first "],
);

e2e_test!(
    fish_subcommand_option_completion,
    "fish",
    completest_pty::FishRuntimeBuilder,
    "exhaustive hint --c\t\t",
    snapbox::str![[r#"
% exhaustive hint --choice
--choice  --cmd-name  --cmd  (CommandString)
"#]],
);

// =============================================================================
// Zsh E2E tests
// =============================================================================

e2e_test!(
    zsh_subcommand_completion,
    "zsh",
    completest_pty::ZshRuntimeBuilder,
    "exhaustive \t\t",
    snapbox::str![[r#"
% exhaustive
--generate      -- generate
--help          -- Print help
help            -- Print this message or the help of the given subcommand(s)
--empty-choice  alias           global          last            quote
action          empty           hint            pacman          value
"#]],
);

e2e_test!(
    zsh_long_option_completion,
    "zsh",
    completest_pty::ZshRuntimeBuilder,
    "exhaustive action --\t\t",
    snapbox::str![[r#"
% exhaustive action --
--choice    -- enum
--count     -- number
--help      -- Print help (see more with '--help')
--set       -- value
--set-true  -- bool
"#]],
);

e2e_test!(
    zsh_option_value_completion,
    "zsh",
    completest_pty::ZshRuntimeBuilder,
    "exhaustive action --choice=\t\t",
    snapbox::str![[r#"
% exhaustive action --choice=
--choice=first   --choice=second
"#]],
);

e2e_test!(
    zsh_option_value_partial_completion,
    "zsh",
    completest_pty::ZshRuntimeBuilder,
    "exhaustive action --choice=f\t\t",
    snapbox::str!["% exhaustive action --choice=first "],
);

e2e_test!(
    zsh_subcommand_option_completion,
    "zsh",
    completest_pty::ZshRuntimeBuilder,
    "exhaustive hint --c\t\t",
    snapbox::str![[r#"
% exhaustive hint --c
--choice    --cmd       --cmd-name
"#]],
);

// =============================================================================
// Elvish E2E tests
// =============================================================================

e2e_test!(
    elvish_subcommand_completion,
    "elvish",
    completest_pty::ElvishRuntimeBuilder,
    "exhaustive \t",
    snapbox::str![[r#"
% exhaustive --empty-choice
 COMPLETING argument
--empty-choice  --help  alias  global  hint  pacman  value
--generate      action  empty  help    last  quote
"#]],
);

e2e_test!(
    elvish_long_option_completion,
    "elvish",
    completest_pty::ElvishRuntimeBuilder,
    "exhaustive action --\t",
    snapbox::str![[r#"
% exhaustive action --choice
 COMPLETING argument
--choice  --count  --help  --set  --set-true
"#]],
);

e2e_test!(
    elvish_option_value_completion,
    "elvish",
    completest_pty::ElvishRuntimeBuilder,
    "exhaustive action --choice=\t",
    snapbox::str![[r#"
% exhaustive action '--choice=first'
 COMPLETING argument
--choice=first  --choice=second
"#]],
);

e2e_test!(
    elvish_option_value_partial_completion,
    "elvish",
    completest_pty::ElvishRuntimeBuilder,
    "exhaustive action --choice=f\t",
    snapbox::str![[r#"
% exhaustive action '--choice=first'
 COMPLETING argument
--choice=first
"#]],
);

e2e_test!(
    elvish_subcommand_option_completion,
    "elvish",
    completest_pty::ElvishRuntimeBuilder,
    "exhaustive hint --c\t",
    snapbox::str![[r#"
% exhaustive hint --choice
 COMPLETING argument
--choice    --cmd  --cmd-name
"#]],
);
