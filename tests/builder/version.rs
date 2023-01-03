use clap::{error::ErrorKind, ArgAction, Command};

use crate::utils;

fn common() -> Command {
    Command::new("foo").help_template(utils::FULL_TEMPLATE)
}

fn with_version() -> Command {
    common().version("3.0")
}

fn with_long_version() -> Command {
    common().long_version("3.0 (abcdefg)")
}

fn with_both() -> Command {
    common().version("3.0").long_version("3.0 (abcdefg)")
}

fn with_subcommand() -> Command {
    with_version().subcommand(Command::new("bar").subcommand(Command::new("baz")))
}

#[test]
fn version_short_flag_no_version() {
    let res = common().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
}

#[test]
fn version_long_flag_no_version() {
    let res = common().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
}

#[test]
fn version_short_flag_with_version() {
    let res = with_version().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0\n");
}

#[test]
fn version_long_flag_with_version() {
    let res = with_version().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0\n");
}

#[test]
fn version_short_flag_with_long_version() {
    let res = with_long_version().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0 (abcdefg)\n");
}

#[test]
fn version_long_flag_with_long_version() {
    let res = with_long_version().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0 (abcdefg)\n");
}

#[test]
fn version_short_flag_with_both() {
    let res = with_both().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0\n");
}

#[test]
fn version_long_flag_with_both() {
    let res = with_both().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0 (abcdefg)\n");
}

#[test]
fn help_short_flag_no_version() {
    static EXPECTED: &str = "\
foo 

Usage: foo

Options:
  -h, --help  Print help
";
    let cmd = common();
    utils::assert_output(cmd, "foo -h", EXPECTED, false);
}

#[test]
fn help_long_flag_no_version() {
    static EXPECTED: &str = "\
foo 

Usage: foo

Options:
  -h, --help  Print help
";
    let cmd = common();
    utils::assert_output(cmd, "foo --help", EXPECTED, false);
}

#[test]
fn help_short_flag_with_version() {
    static EXPECTED: &str = "\
foo 3.0

Usage: foo

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    let cmd = with_version();
    utils::assert_output(cmd, "foo -h", EXPECTED, false);
}

#[test]
fn help_long_flag_with_version() {
    static EXPECTED: &str = "\
foo 3.0

Usage: foo

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    let cmd = with_version();
    utils::assert_output(cmd, "foo --help", EXPECTED, false);
}

#[test]
fn help_short_flag_with_long_version() {
    static EXPECTED: &str = "\
foo 3.0 (abcdefg)

Usage: foo

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    let cmd = with_long_version();
    utils::assert_output(cmd, "foo -h", EXPECTED, false);
}

#[test]
fn help_long_flag_with_long_version() {
    static EXPECTED: &str = "\
foo 3.0 (abcdefg)

Usage: foo

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    let cmd = with_long_version();
    utils::assert_output(cmd, "foo --help", EXPECTED, false);
}

#[test]
fn help_short_flag_with_both() {
    static EXPECTED: &str = "\
foo 3.0

Usage: foo

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    let cmd = with_both();
    utils::assert_output(cmd, "foo -h", EXPECTED, false);
}

#[test]
fn help_long_flag_with_both() {
    static EXPECTED: &str = "\
foo 3.0

Usage: foo

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    let cmd = with_both();
    utils::assert_output(cmd, "foo --help", EXPECTED, false);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Command foo: Long option names must be unique for each argument, but '--version' is in use by both 'ver' and 'version' (call `cmd.disable_version_flag(true)` to remove the auto-generated `--version`)"]
fn override_version_long_with_user_flag() {
    with_version()
        .arg(
            clap::Arg::new("ver")
                .long("version")
                .action(ArgAction::SetTrue),
        )
        .debug_assert();
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Command foo: Short option names must be unique for each argument, but '-V' is in use by both 'ver' and 'version' (call `cmd.disable_version_flag(true)` to remove the auto-generated `--version`)"]
fn override_version_short_with_user_flag() {
    with_version()
        .arg(clap::Arg::new("ver").short('V').action(ArgAction::SetTrue))
        .debug_assert();
}

#[test]
fn no_propagation_by_default_long() {
    // Version Flag should not be propagated to subcommands
    let res = with_subcommand().try_get_matches_from("foo bar --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn no_propagation_by_default_short() {
    let res = with_subcommand().try_get_matches_from("foo bar -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn propagate_version_long() {
    let res = with_subcommand()
        .propagate_version(true)
        .try_get_matches_from("foo bar --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
}

#[test]
fn propagate_version_short() {
    let res = with_subcommand()
        .propagate_version(true)
        .try_get_matches_from("foo bar -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "`ArgAction::Version` used without providing Command::version or Command::long_version"]
fn version_required() {
    let _res = common()
        .arg(clap::arg!(--version).action(ArgAction::Version))
        .try_get_matches_from("foo -z".split(' '));
}

#[test]
#[should_panic = "Argument `version` is undefined"]
fn mut_arg_version_no_auto_version() {
    let _ = common().mut_arg("version", |v| v.short('z').action(ArgAction::SetTrue));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "No version information via Command::version or Command::long_version to propagate"]
fn propagate_version_no_version_info() {
    let _res = common()
        .propagate_version(true)
        .subcommand(Command::new("bar"))
        .try_get_matches_from("foo".split(' '));
}
