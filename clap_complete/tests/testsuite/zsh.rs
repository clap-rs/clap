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
#[cfg(all(unix, feature = "unstable-dynamic", feature = "unstable-shell-tests"))]
fn dynamic_env_registration_survives_compinit() {
    use clap_complete::env::EnvCompleter as _;

    // This does not use a PTY, so it also works with macOS's system zsh.
    for bin in ["myprog", "my-prog", "custom-bin"] {
        for mode in ["autoload", "source"] {
            let scratch = snapbox::dir::DirRoot::mutable_temp().unwrap();
            let root = scratch.path().unwrap();
            let mut registration = Vec::new();
            clap_complete::env::Zsh
                .write_registration("COMPLETE", "myprog", bin, "completer", &mut registration)
                .unwrap();
            std::fs::write(root.join(format!("_{bin}")), registration).unwrap();

            let script = r#"
if [[ $2 != source ]]; then
    fpath=("$PWD" $fpath)
fi
autoload -Uz compinit
compinit -u -d "$PWD/.zcompdump"

# Exercise the generated adapter without needing an interactive completion widget.
bin_name=$1
completer() {
    [[ $COMPLETE == zsh && $_CLAP_COMPLETE_INDEX == 2 ]] || return 1
    [[ $# == 4 && $1 == -- && $2 == $bin_name && $3 == --format && -z $4 ]] || return 1
    print -r -- 'json:JSON'
}
_describe() { print -rl -- "${(@P)3}"; }
words=("$1" --format '')
CURRENT=3
if [[ $2 == source ]]; then
    # Sourcing must register the command even when its file is not on fpath.
    source "$PWD/_$1"
    fpath=("$PWD" $fpath)
fi
print -r -- "${_comps[$1]}"
${_comps[$1]}
${_comps[$1]}
print -r -- "${_comps[$1]}"

# A rescan must not cache a function that has no corresponding file in fpath.
rm -f "$PWD/.zcompdump"
compinit -u -d "$PWD/.zcompdump"
"#;
            // The second process must load the dump produced by the first, without sourcing.
            for startup in [mode, "cached"] {
                let output = std::process::Command::new(CMD)
                    .args(["-f", "-c", script, "zsh", bin, startup])
                    .env("ZDOTDIR", root)
                    .current_dir(root)
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{bin}: {mode}: {startup}");
                assert_data_eq!(String::from_utf8(output.stderr).unwrap(), "");
                assert_data_eq!(
                    String::from_utf8(output.stdout).unwrap(),
                    format!("_{bin}\njson:JSON\njson:JSON\n_{bin}\n")
                );
                assert!(root.join(".zcompdump").exists());
            }
            scratch.close().unwrap();
        }
    }
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
% exhaustive
help            -- Print this message or the help of the given subcommand(s)
--generate      -- generate
--help          -- Print help
empty           action          value           last            hint            
global          quote           pacman          alias           --empty-choice  
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

    let input = "exhaustive action --choice=\t";
    let expected = snapbox::str![[r#"
% exhaustive action --choice=
--choice=first   --choice=second
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

    let input = "exhaustive quote --choice \t";
    let expected = snapbox::str![[r#"
% exhaustive quote --choice
another shell  -- something with a space
bash           -- bash (shell)
fish           -- fish shell
zsh            -- zsh shell
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

    let input = "exhaustive empty \t";
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
    let input = "exhaustive quote  -\x1b[D\x1b[D\t";
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
    let input = "exhaustive hint --file tes\t";
    let expected = snapbox::str!["% exhaustive hint --file tests/"];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    // Verify hitting tab again shows the directory contents.
    // This only works if there is no trailing space after the slash.
    let input = "exhaustive hint --file tests/\t";
    let expected = snapbox::str![[r#"
% exhaustive hint --file tests/
tests/snapshots    tests/testsuite    tests/examples.rs
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}
