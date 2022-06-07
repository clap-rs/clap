use super::utils;

use clap::{arg, error::ErrorKind, value_parser, Arg, Command, Error};

fn assert_error(err: Error, expected_kind: ErrorKind, expected_output: &str, stderr: bool) {
    let actual_output = err.to_string();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    assert_eq!(expected_kind, err.kind());
    utils::assert_eq(expected_output, actual_output)
}

#[test]
fn app_error() {
    static MESSAGE: &str = "error: Failed for mysterious reasons

USAGE:
    test [OPTIONS] --all

For more information try --help
";
    let cmd = Command::new("test")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .required(true)
                .help("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .help("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .help("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .help("Do not push generated commit and tags to git remote"),
        );
    let mut cmd = cmd;
    let expected_kind = ErrorKind::InvalidValue;
    let err = cmd.error(expected_kind, "Failed for mysterious reasons");
    assert_error(err, expected_kind, MESSAGE, true);
}

#[test]
fn value_validation_has_newline() {
    let res = Command::new("test")
        .arg(
            arg!(<PORT>)
                .value_parser(value_parser!(usize))
                .help("Network port to use"),
        )
        .try_get_matches_from(["test", "foo"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(
        err.to_string().ends_with('\n'),
        "Errors should have a trailing newline, got {:?}",
        err.to_string()
    );
}
