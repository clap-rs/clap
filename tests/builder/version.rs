use super::utils;

use std::str;

use clap::{error::ErrorKind, Arg, ArgAction, Command};

fn common() -> Command<'static> {
    Command::new("foo")
}

fn with_version() -> Command<'static> {
    common().version("3.0")
}

fn with_long_version() -> Command<'static> {
    common().long_version("3.0 (abcdefg)")
}

fn with_subcommand() -> Command<'static> {
    with_version().subcommand(Command::new("bar").subcommand(Command::new("baz")))
}

#[test]
fn no_version_flag_short() {
    let res = common().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
}

#[test]
fn no_version_flag_long() {
    let res = common().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
}

#[test]
fn version_flag_from_version_short() {
    let res = with_version().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0\n");
}

#[test]
fn version_flag_from_version_long() {
    let res = with_version().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0\n");
}

#[test]
fn version_flag_from_long_version_short() {
    let res = with_long_version().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0 (abcdefg)\n");
}

#[test]
fn version_flag_from_long_version_long() {
    let res = with_long_version().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0 (abcdefg)\n");
}

#[test]
fn override_version_long_with_user_flag() {
    let res = with_version()
        .arg(Arg::new("ver").long("version").action(ArgAction::SetTrue))
        .try_get_matches_from("foo --version".split(' '));

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("ver").expect("defaulted by clap"));
}

#[test]
fn override_version_long_with_user_flag_no_version_flag() {
    let res = with_version()
        .arg(Arg::new("ver").long("version"))
        .try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn override_version_short_with_user_flag() {
    let res = with_version()
        .arg(Arg::new("ver").short('V').action(ArgAction::SetTrue))
        .try_get_matches_from("foo -V".split(' '));

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("ver").expect("defaulted by clap"));
}

#[test]
fn override_version_short_with_user_flag_long_still_works() {
    let res = with_version()
        .arg(Arg::new("ver").short('V'))
        .try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
}

#[test]
fn mut_version_short() {
    let res = with_version()
        .mut_arg("version", |a| a.short('z'))
        .try_get_matches_from("foo -z".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
}

#[test]
fn mut_version_long() {
    let res = with_version()
        .mut_arg("version", |a| a.long("qux"))
        .try_get_matches_from("foo --qux".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
}

static VERSION_ABOUT_MULTI_SC: &str = "foo-bar-baz 3.0

USAGE:
    foo bar baz

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print custom version about text
";

#[test]
fn version_about_multi_subcmd() {
    let cmd = with_subcommand()
        .mut_arg("version", |a| a.help("Print custom version about text"))
        .propagate_version(true);

    utils::assert_output(cmd, "foo bar baz -h", VERSION_ABOUT_MULTI_SC, false);
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
fn mut_arg_version_panic() {
    let _res = common()
        .mut_arg("version", |v| v.short('z'))
        .try_get_matches_from("foo -z".split(' '));
}

#[test]
fn mut_arg_version_no_auto_version() {
    let res = common()
        .mut_arg("version", |v| v.short('z').action(ArgAction::SetTrue))
        .try_get_matches_from("foo -z".split(' '));

    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().get_one::<bool>("version").copied(), Some(true));
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
