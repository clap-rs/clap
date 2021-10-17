mod utils;

use clap::{App, Arg, ColorChoice, Error, ErrorKind};

fn compare_error(
    err: Error,
    expected_kind: ErrorKind,
    expected_output: &str,
    stderr: bool,
) -> bool {
    let actual_output = err.to_string();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    assert_eq!(expected_kind, err.kind);
    utils::compare(expected_output, actual_output)
}

#[test]
fn app_error() {
    static MESSAGE: &str = "error: Failed for mysterious reasons

USAGE:
    test [OPTIONS] --all

For more information try --help
";
    let app = App::new("test")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .required(true)
                .about("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .about("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .about("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .about("Do not push generated commit and tags to git remote"),
        );
    #[cfg(feature = "color")]
    let app = app.color(ColorChoice::Never);
    let mut app = app;
    let expected_kind = ErrorKind::InvalidValue;
    let err = app.error(expected_kind, "Failed for mysterious reasons");
    assert!(compare_error(err, expected_kind, MESSAGE, true));
}
