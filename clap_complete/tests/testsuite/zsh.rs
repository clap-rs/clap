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

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive \t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive
Options
Commands
--generate      -- generate
--help          -- Print help
help    -- Print this message or the help of the given subcommand(s)
                action          empty           hint            pacman          value           
--empty-choice  alias           global          last            quote           
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
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

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive quote \t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive quote
Options
Commands
--backslash      -- Avoid '/n'
--backticks      -- For more information see `echo test`
--brackets       -- List packages [filter]
--double-quotes  -- Can be "always", "auto", or "never"
--expansions     -- Execute the shell command with $SHELL
--help           -- Print help (see more with '--help')
--single-quotes  -- Can be 'always', 'auto', or 'never'
cmd-backslash      -- Avoid '/n'
cmd-backticks      -- For more information see `echo test`
cmd-brackets       -- List packages [filter]
cmd-double-quotes  -- Can be "always", "auto", or "never"
cmd-expansions     -- Execute the shell command with $SHELL
cmd-single-quotes  -- Can be 'always', 'auto', or 'never'
escape-help        -- /tab      "'
help               -- Print this message or the help of the given subcommand(s)
          --choice
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
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

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive action --choice=\t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive action --choice=
choice <choice>
--choice=first   --choice=second
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive action --choice=f\t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive action --choice=first 
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
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

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive quote --choice \t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive quote --choice
choice <choice>
another shell  -- something with a space
bash           -- bash (shell)
fish           -- fish shell
zsh            -- zsh shell
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive quote --choice an\t\t",
    ]
    .join("\n");
    let expected = snapbox::str![
        "% zstyle ':completion:*:descriptions' format '%d'\n% exhaustive quote --choice another/ shell "
    ];
    let actual = runtime.complete(&input, &term).unwrap();
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

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive empty \t\t",
    ]
    .join("\n");
    let expected =
        snapbox::str!["% zstyle ':completion:*:descriptions' format '%d'\n% exhaustive empty "];
    let actual = runtime.complete(&input, &term).unwrap();
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

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive --empty=\t",
    ]
    .join("\n");
    let expected =
        snapbox::str!["% zstyle ':completion:*:descriptions' format '%d'\n% exhaustive --empty="];
    let actual = runtime.complete(&input, &term).unwrap();
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
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive quote  -\x1b[D\x1b[D\t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive quote  -
Options
Commands
--backslash      -- Avoid '/n'
--backticks      -- For more information see `echo test`
--brackets       -- List packages [filter]
--double-quotes  -- Can be "always", "auto", or "never"
--expansions     -- Execute the shell command with $SHELL
--help           -- Print help (see more with '--help')
--single-quotes  -- Can be 'always', 'auto', or 'never'
cmd-backslash      -- Avoid '/n'
cmd-backticks      -- For more information see `echo test`
cmd-brackets       -- List packages [filter]
cmd-double-quotes  -- Can be "always", "auto", or "never"
cmd-expansions     -- Execute the shell command with $SHELL
cmd-single-quotes  -- Can be 'always', 'auto', or 'never'
escape-help        -- /tab      "'
help               -- Print this message or the help of the given subcommand(s)
          --choice
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
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
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive hint --file tes\t\t",
    ]
    .join("\n");
    let expected = snapbox::str![
        "% zstyle ':completion:*:descriptions' format '%d'\n% exhaustive hint --file tests/"
    ];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);

    // Verify hitting tab again shows the directory contents.
    // This only works if there is no trailing space after the slash.
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive hint --file tests/\t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive hint --file tests/
file <file>
tests/examples.rs  tests/snapshots    tests/testsuite
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_tag_with_file_hint() {
    if !common::has_command(CMD) {
        //return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    // Regression: path completions were tagged with arg.to_string() ("--file <FILE>"),
    // which contains spaces and angle brackets. zsh's _describe uses the tag name in
    // zstyle context strings where those characters are invalid, causing "_describe: bad option" errors.
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive hint --file tests/\t\t",
    ]
    .join("\n");
    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive hint --file tests/
file <file>
tests/examples.rs  tests/snapshots    tests/testsuite
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_tagged_options() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive -\t\t",
    ]
    .join("\n");

    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive -
Options
--generate      -- generate
-h              -- Print help
--empty-choice
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_tagged_positionals_and_flags() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    // `alias` has both named flags and a positional with possible values,
    // so we can assert that positionals appear in their own tagged group alongside flags.
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive alias \t\t",
    ]
    .join("\n");

    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive alias
[positional]
Options
--flag    -- cmd flag
--help    -- Print help
--option  -- cmd option
      pos1  pos2
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_tagged_subcommands_and_flags() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    // `global` has both flags (--global) and subcommands (one, two),
    // to test that they appear in different tagged groups.
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive global \t\t",
    ]
    .join("\n");

    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive global
Options
Commands
--global   -- everywhere
--help     -- Print help
--version  -- Print version
help  -- Print this message or the help of the given subcommand(s)
     one  two
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_tagged_help_heading() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "exhaustive");

    // `action` has some flags under the default "Options" heading and some
    // under "Advanced", to tesst that help_headings end up in distinct tag groups.
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "exhaustive action -\t\t",
    ]
    .join("\n");

    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% exhaustive action -
Options
Advanced
--choice  -- enum
--count     -- number
--set     -- value
--set-true  -- bool
-h        -- Print help
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn register_dynamic_env_untagged() {
    common::register_example::<RuntimeBuilder>("dynamic-env", "untagged");
}

#[test]
#[cfg(all(unix, feature = "unstable-dynamic"))]
#[cfg(feature = "unstable-shell-tests")]
fn complete_dynamic_untagged_candidates() {
    if !common::has_command(CMD) {
        return;
    }

    let term = completest::Term::new();
    let mut runtime = common::load_runtime::<RuntimeBuilder>("dynamic-env", "untagged");

    // `ext` is a subcommand that returns candidates with no tag set,
    // to test how the completion code handles no-tags.
    let input = [
        "zstyle ':completion:*:descriptions' format '%d'",
        "untagged ext \t\t",
    ]
    .join("\n");

    let expected = snapbox::str![[r#"
% zstyle ':completion:*:descriptions' format '%d'
% untagged ext
Options
--help  -- Print help
"#]];
    let actual = runtime.complete(&input, &term).unwrap();
    assert_data_eq!(actual, expected);
}
