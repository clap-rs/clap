use snapbox::assert_data_eq;

use crate::common;

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

#[cfg(feature = "unstable-dynamic")]
#[test]
fn register_minimal() {
    use clap_complete::dynamic::Completer;

    let name = "my-app";
    let bin = name;
    let completer = name;

    let mut buf = Vec::new();
    clap_complete::dynamic::shells::Bash
        .write_registration(name, bin, completer, &mut buf)
        .unwrap();
    snapbox::Assert::new()
        .action_env("SNAPSHOTS")
        .eq(buf, snapbox::file!["../snapshots/register_minimal.bash"]);
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
fn register_completion() {
    common::register_example::<completest_pty::BashRuntimeBuilder>("static", "exhaustive");
}

#[test]
#[cfg(unix)]
fn complete() {
    if !common::has_command("bash") {
        return;
    }

    let term = completest::Term::new();
    let mut runtime =
        common::load_runtime::<completest_pty::BashRuntimeBuilder>("static", "exhaustive");

    let input = "exhaustive \t\t";
    let expected = r#"% 
-h          --global    --help      action      value       last        hint        help        
-V          --generate  --version   quote       pacman      alias       complete    "#;
    let actual = runtime.complete(input, &term).unwrap();
    assert_data_eq!(actual, expected);

    // Issue 5239 (https://github.com/clap-rs/clap/issues/5239)
    let input = "exhaustive hint --file test\t";
    let expected = "exhaustive hint --file test     % exhaustive hint --file tests/";
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
            "Actual output:\n{}",
            actual
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
            "Actual output:\n{}",
            actual
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
#[cfg(unix)]
fn register_dynamic_completion() {
    common::register_example::<completest_pty::BashRuntimeBuilder>("dynamic", "exhaustive");
}
