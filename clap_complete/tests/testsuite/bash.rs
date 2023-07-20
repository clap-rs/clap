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
    let name = "my-app";
    let executables = [name];
    let completer = name;
    let behavior = clap_complete::dynamic::bash::Behavior::Minimal;

    let mut buf = Vec::new();
    clap_complete::dynamic::bash::register(name, executables, completer, &behavior, &mut buf)
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
fn complete() {
    if !has_command("bash") {
        return;
    }

    let shell = "bash";

    let home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("clap_complete_{shell}_home"));
    let _ = std::fs::remove_dir_all(&home);

    let bin_path = snapbox::cmd::compile_example("test", []).unwrap();
    let bin_root = bin_path.parent().unwrap().to_owned();

    let registration = std::process::Command::new(&bin_path)
        .arg(format!("--generate={shell}"))
        .output()
        .unwrap();
    assert!(
        registration.status.success(),
        "{}",
        String::from_utf8_lossy(&registration.stderr)
    );
    let registration = std::str::from_utf8(&registration.stdout).unwrap();
    assert!(!registration.is_empty());
    snapbox::Assert::new()
        .action_env("SNAPSHOTS")
        .normalize_paths(false)
        .matches_path(format!("tests/snapshots/test.{shell}"), registration);

    let term = completest::Term::new();
    let runtime = completest::BashRuntime::new(bin_root, home).unwrap();

    runtime.register("test", registration).unwrap();

    let expected = r#"% 
-h          --global    --help      action      value       last        hint        
-V          --generate  --version   quote       pacman      alias       help        "#;
    let actual = runtime.complete("test \t\t", &term).unwrap();
    snapbox::assert_eq(expected, actual);
}

fn has_command(command: &str) -> bool {
    let output = match std::process::Command::new(command)
        .arg("--version")
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            // CI is expected to support all of the commands
            if is_ci() && cfg!(linux) {
                panic!(
                    "expected command `{}` to be somewhere in PATH: {}",
                    command, e
                );
            }
            return false;
        }
    };
    if !output.status.success() {
        panic!(
            "expected command `{}` to be runnable, got error {}:\n\
            stderr:{}\n\
            stdout:{}\n",
            command,
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!(
        "$ bash --version
{}",
        stdout
    );
    if cfg!(target_os = "macos") && stdout.starts_with("GNU bash, version 3") {
        return false;
    }

    true
}

/// Whether or not this running in a Continuous Integration environment.
fn is_ci() -> bool {
    // Consider using `tracked_env` instead of option_env! when it is stabilized.
    // `tracked_env` will handle changes, but not require rebuilding the macro
    // itself like option_env does.
    option_env!("CI").is_some() || option_env!("TF_BUILD").is_some()
}
