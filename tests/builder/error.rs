use clap::{arg, builder::ArgAction, error::ErrorKind, value_parser, Arg, Command};
use snapbox::str;

use crate::utils::assert_error;

#[test]
fn app_error() {
    let message = str![[r#"
error: failed for mysterious reasons

Usage: test [OPTIONS] --all

For more information, try '--help'.

"#]];
    let cmd = Command::new("test")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .required(true)
                .action(ArgAction::SetTrue)
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
    let err = cmd.error(expected_kind, "failed for mysterious reasons");
    assert_error(err, expected_kind, message, true);
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

#[test]
fn kind_prints_help() {
    let cmd = Command::new("test");
    let res = cmd
        .try_get_matches_from(["test", "--help"])
        .map_err(|e| e.apply::<clap::error::KindFormatter>());
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::DisplayHelp;
    let message = str![[r#"
Usage: test

Options:
  -h, --help  Print help

"#]];
    assert_error(err, expected_kind, message, false);
}

#[test]
fn kind_formats_validation_error() {
    let cmd = Command::new("test");
    let res = cmd
        .try_get_matches_from(["test", "unused"])
        .map_err(|e| e.apply::<clap::error::KindFormatter>());
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument found

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
fn rich_formats_validation_error() {
    let cmd = Command::new("test");
    let res = cmd.try_get_matches_from(["test", "unused"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument 'unused' found

Usage: test

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
fn suggest_trailing() {
    let cmd = Command::new("rg").arg(arg!([PATTERN]));

    let res = cmd.try_get_matches_from(["rg", "--foo"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument '--foo' found

  tip: to pass '--foo' as a value, use '-- --foo'

Usage: rg [PATTERN]

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
fn suggest_trailing_last() {
    let cmd = Command::new("cargo")
        .arg(arg!([TESTNAME]).last(true))
        .arg(arg!(--"ignore-rust-version"));

    let res = cmd.try_get_matches_from(["cargo", "--ignored"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument '--ignored' found

  tip: a similar argument exists: '--ignore-rust-version'
  tip: to pass '--ignored' as a value, use '-- --ignored'

Usage: cargo --ignore-rust-version [-- <TESTNAME>]

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
fn trailing_already_in_use() {
    let cmd = Command::new("rg").arg(arg!([PATTERN]));

    let res = cmd.try_get_matches_from(["rg", "--", "--foo", "--foo"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument '--foo' found

Usage: rg [PATTERN]

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
fn cant_use_trailing() {
    let cmd = Command::new("test");

    let res = cmd.try_get_matches_from(["test", "--foo"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument '--foo' found

Usage: test

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
#[cfg(feature = "suggestions")]
fn cant_use_trailing_subcommand() {
    let cmd = Command::new("test").subcommand(Command::new("bar"));

    let res = cmd.try_get_matches_from(["test", "baz"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::InvalidSubcommand;
    let message = str![[r#"
error: unrecognized subcommand 'baz'

  tip: a similar subcommand exists: 'bar'

Usage: test [COMMAND]

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
#[cfg(feature = "suggestions")]
fn unknown_argument_option() {
    let cmd = Command::new("test").args([
        Arg::new("current-dir").short('C'),
        Arg::new("current-dir-unknown")
            .long("cwd")
            .aliases(["current-dir", "directory", "working-directory", "root"])
            .value_parser(
                clap::builder::UnknownArgumentValueParser::suggest_arg("-C")
                    .and_suggest("not much else to say"),
            )
            .hide(true),
    ]);

    let res = cmd.clone().try_get_matches_from(["test"]);
    assert!(res.is_ok());

    let res = cmd.try_get_matches_from(["test", "--cwd", ".."]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument '--cwd <current-dir-unknown>' found

  tip: a similar argument exists: '-C'
  tip: not much else to say

Usage: test [OPTIONS]

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}

#[test]
#[cfg(feature = "error-context")]
#[cfg(feature = "suggestions")]
fn unknown_argument_flag() {
    let cmd = Command::new("test").args([
        Arg::new("ignore-rust-version").long("ignore-rust-version"),
        Arg::new("libtest-ignore")
            .long("ignored")
            .action(ArgAction::SetTrue)
            .value_parser(
                clap::builder::UnknownArgumentValueParser::suggest_arg("-- --ignored")
                    .and_suggest("not much else to say"),
            )
            .hide(true),
    ]);

    let res = cmd.clone().try_get_matches_from(["test"]);
    assert!(res.is_ok());

    let res = cmd.try_get_matches_from(["test", "--ignored"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    let expected_kind = ErrorKind::UnknownArgument;
    let message = str![[r#"
error: unexpected argument '--ignored' found

  tip: a similar argument exists: '-- --ignored'
  tip: not much else to say

Usage: test [OPTIONS]

For more information, try '--help'.

"#]];
    assert_error(err, expected_kind, message, true);
}
