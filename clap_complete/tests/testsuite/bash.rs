#[allow(unused_imports)]
use snapbox::assert_data_eq;

use crate::common;

#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
const CMD: &str = "bash";
#[cfg(unix)]
#[cfg(feature = "unstable-shell-tests")]
type RuntimeBuilder = completest_pty::BashRuntimeBuilder;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/basic.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/feature_sample.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/special_commands.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/quoting.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/aliases.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/sub_subcommands.bash"],
        clap_complete::shells::Bash,
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
        snapbox::file!["../snapshots/custom_bin_name.bash"],
        clap_complete::shells::Bash,
        cmd,
        bin_name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_hint.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/value_terminator.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches(
        snapbox::file!["../snapshots/two_multi_valued_arguments.bash"],
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches(
        snapbox::file!["../snapshots/subcommand_last.bash"],
        clap_complete::shells::Bash,
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

    let input = "exhaustive \t\t";
    let expected = snapbox::str![[r#"
% 
-h              --empty-choice  empty           action          value           last            hint
--generate      --help          global          quote           pacman          alias           help
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive empty \t";
    let expected = snapbox::str!["exhaustive empty        % exhaustive empty "];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive --empty=\t";
    let expected = snapbox::str!["exhaustive --empty=     % exhaustive --empty="];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    // Issue 5239 (https://github.com/clap-rs/clap/issues/5239)
    let input = "exhaustive hint --file test\t";
    let expected = snapbox::str!["exhaustive hint --file test     % exhaustive hint --file tests/"];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    {
        use std::fs::File;
        use std::path::Path;

        let testdir = snapbox::dir::DirRoot::mutable_temp().unwrap();
        let testdir_path = testdir.path().unwrap();

        File::create(Path::new(testdir_path).join("a_file")).unwrap();
        File::create(Path::new(testdir_path).join("b_file")).unwrap();
        std::fs::create_dir_all(Path::new(testdir_path).join("c_dir")).unwrap();
        std::fs::create_dir_all(Path::new(testdir_path).join("d_dir")).unwrap();

        let input = format!(
            "exhaustive hint --file {}/\t\t",
            testdir_path.to_string_lossy()
        );
        let actual = runtime.complete(input.as_str(), &term).unwrap();
        assert!(
            actual.contains("a_file")
                && actual.contains("b_file")
                && actual.contains("c_dir")
                && actual.contains("d_dir"),
            "Actual output:\n{actual}"
        );

        let input = format!(
            "exhaustive hint --dir {}/\t\t",
            testdir_path.to_string_lossy()
        );
        let actual = runtime.complete(input.as_str(), &term).unwrap();
        assert!(
            !actual.contains("a_file")
                && !actual.contains("b_file")
                && actual.contains("c_dir")
                && actual.contains("d_dir"),
            "Actual output:\n{actual}"
        );
    }

    {
        use std::fs::File;
        use std::path::Path;

        let testdir = snapbox::dir::DirRoot::mutable_temp().unwrap();
        let testdir_path = testdir.path().unwrap();

        File::create(Path::new(testdir_path).join("foo bar.txt")).unwrap();
        File::create(Path::new(testdir_path).join("baz\tqux.txt")).unwrap();

        let input = format!(
            "exhaustive hint --file {}/b\t",
            testdir_path.to_string_lossy()
        );
        let actual = runtime.complete(input.as_str(), &term).unwrap();
        assert!(!actual.contains("foo"), "Actual output:\n{actual}");
    }

    let input = "exhaustive hint --other \t";
    let expected = snapbox::str!["exhaustive hint --other         % exhaustive hint --other "];
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
% 
empty           action          value           last            hint            --generate      --help
global          quote           pacman          alias           help            --empty-choice  
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
% 
cmd-single-quotes  cmd-backslash      escape-help        --double-quotes    --brackets         --help
cmd-double-quotes  cmd-brackets       help               --backticks        --expansions       
cmd-backticks      cmd-expansions     --single-quotes    --backslash        --choice           
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
    let expected = snapbox::str!["% "];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive action --choice=f\t";
    let expected = snapbox::str!["exhaustive action --choice=f    % exhaustive action --choice=f"];
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
% 
another shell  bash           fish           zsh            
"#]];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    let input = "exhaustive quote --choice an\t";
    let expected =
        snapbox::str!["exhaustive quote --choice an    % exhaustive quote --choice another shell "];
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
    let expected = snapbox::str!["exhaustive empty        % exhaustive empty "];
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
    let expected = snapbox::str!["exhaustive --empty=     % exhaustive --empty="];
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);
}
