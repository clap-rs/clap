use crate::common;

#[test]
fn basic() {
    let name = "my-app";
    let cmd = common::basic_command(name);
    common::assert_matches_path(
        "tests/snapshots/basic.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn feature_sample() {
    let name = "my-app";
    let cmd = common::feature_sample_command(name);
    common::assert_matches_path(
        "tests/snapshots/feature_sample.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn special_commands() {
    let name = "my-app";
    let cmd = common::special_commands_command(name);
    common::assert_matches_path(
        "tests/snapshots/special_commands.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn quoting() {
    let name = "my-app";
    let cmd = common::quoting_command(name);
    common::assert_matches_path(
        "tests/snapshots/quoting.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn aliases() {
    let name = "my-app";
    let cmd = common::aliases_command(name);
    common::assert_matches_path(
        "tests/snapshots/aliases.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn sub_subcommands() {
    let name = "my-app";
    let cmd = common::sub_subcommands_command(name);
    common::assert_matches_path(
        "tests/snapshots/sub_subcommands.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn value_hint() {
    let name = "my-app";
    let cmd = common::value_hint_command(name);
    common::assert_matches_path(
        "tests/snapshots/value_hint.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn value_terminator() {
    let name = "my-app";
    let cmd = common::value_terminator_command(name);
    common::assert_matches_path(
        "tests/snapshots/value_terminator.bash",
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
        .matches_path("tests/snapshots/register_minimal.bash", buf);
}

#[test]
fn two_multi_valued_arguments() {
    let name = "my-app";
    let cmd = common::two_multi_valued_arguments_command(name);
    common::assert_matches_path(
        "tests/snapshots/two_multi_valued_arguments.bash",
        clap_complete::shells::Bash,
        cmd,
        name,
    );
}

#[test]
fn subcommand_last() {
    let name = "my-app";
    let cmd = common::subcommand_last(name);
    common::assert_matches_path(
        "tests/snapshots/subcommand_last.bash",
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
    snapbox::assert_eq(expected, actual);

    // Issue 5239 (https://github.com/clap-rs/clap/issues/5239)
    let input = "exhaustive hint --file test\t";
    let expected = "exhaustive hint --file test     % exhaustive hint --file tests/";
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);

    {
        use std::fs::File;
        use std::path::Path;

        let testdir = snapbox::path::PathFixture::mutable_temp().unwrap();
        let testdir_path = testdir.path().unwrap();

        File::create(Path::new(testdir_path).join("a_file")).unwrap();
        File::create(Path::new(testdir_path).join("b_file")).unwrap();
        std::fs::create_dir(Path::new(testdir_path).join("c_dir")).unwrap();
        std::fs::create_dir(Path::new(testdir_path).join("d_dir")).unwrap();

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
            actual.contains("a_file")
                && actual.contains("b_file")
                && actual.contains("c_dir")
                && actual.contains("d_dir"),
            "Actual output:\n{}",
            actual
        );
    }

    let input = "exhaustive hint --other \t";
    let expected = "exhaustive hint --other         % exhaustive hint --other  ";
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);
}

#[test]
#[cfg(unix)]
fn register_dynamic_completion() {
    common::register_example::<completest_pty::BashRuntimeBuilder>("dynamic", "exhaustive");
}
