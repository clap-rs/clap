mod utils;

use std::str;

use clap::{App, AppSettings, Arg, ErrorKind};

fn common() -> App<'static> {
    App::new("foo")
}

fn with_version() -> App<'static> {
    common().version("3.0")
}

fn with_long_version() -> App<'static> {
    common().long_version("3.0 (abcdefg)")
}

fn with_subcommand() -> App<'static> {
    with_version().subcommand(App::new("bar").subcommand(App::new("baz")))
}

#[test]
fn no_version_flag_short() {
    let res = common().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, clap::ErrorKind::UnknownArgument);
    assert_eq!(err.info, ["-V"]);
}

#[test]
fn no_version_flag_long() {
    let res = common().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, clap::ErrorKind::UnknownArgument);
    assert_eq!(err.info, ["--version"]);
}

#[test]
fn version_flag_from_version_short() {
    let res = with_version().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0\n");
}

#[test]
fn version_flag_from_version_long() {
    let res = with_version().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0\n");
}

#[test]
fn version_flag_from_long_version_short() {
    let res = with_long_version().try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0 (abcdefg)\n");
}

#[test]
fn version_flag_from_long_version_long() {
    let res = with_long_version().try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "foo 3.0 (abcdefg)\n");
}

#[test]
fn override_version_long_with_user_flag() {
    let res = with_version()
        .arg(Arg::new("ver").long("version"))
        .try_get_matches_from("foo --version".split(' '));

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("ver"));
}

#[test]
fn override_version_long_with_user_flag_no_version_flag() {
    let res = with_version()
        .arg(Arg::new("ver").long("version"))
        .try_get_matches_from("foo -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn override_version_short_with_user_flag() {
    let res = with_version()
        .arg(Arg::new("ver").short('V'))
        .try_get_matches_from("foo -V".split(' '));

    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("ver"));
}

#[test]
fn override_version_short_with_user_flag_long_still_works() {
    let res = with_version()
        .arg(Arg::new("ver").short('V'))
        .try_get_matches_from("foo --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
}

#[test]
fn mut_version_short() {
    let res = with_version()
        .mut_arg("version", |a| a.short('z'))
        .try_get_matches_from("foo -z".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
}

#[test]
fn mut_version_long() {
    let res = with_version()
        .mut_arg("version", |a| a.long("qux"))
        .try_get_matches_from("foo --qux".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
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
    let app = with_subcommand()
        .mut_arg("version", |a| a.help("Print custom version about text"))
        .global_setting(AppSettings::PropagateVersion);

    assert!(utils::compare_output(
        app,
        "foo bar baz -h",
        VERSION_ABOUT_MULTI_SC,
        false
    ));
}

#[test]
fn no_propagation_by_default_long() {
    // Version Flag should not be propagated to subcommands
    let res = with_subcommand().try_get_matches_from("foo bar --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
    assert_eq!(err.info, &["--version"]);
}

#[test]
fn no_propagation_by_default_short() {
    let res = with_subcommand().try_get_matches_from("foo bar -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
    assert_eq!(err.info, &["-V"]);
}

#[test]
fn propagate_version_long() {
    let res = with_subcommand()
        .setting(AppSettings::PropagateVersion)
        .try_get_matches_from("foo bar --version".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
}

#[test]
fn propagate_version_short() {
    let res = with_subcommand()
        .setting(AppSettings::PropagateVersion)
        .try_get_matches_from("foo bar -V".split(' '));

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Used App::mut_arg(\"version\", ..) without providing App::version, App::long_version or using AppSettings::NoAutoVersion"]
fn mut_arg_version_panic() {
    let _res = common()
        .mut_arg("version", |v| v.short('z'))
        .try_get_matches_from("foo -z".split(' '));
}

#[test]
fn mut_arg_version_no_auto_version() {
    let res = common()
        .mut_arg("version", |v| v.short('z'))
        .setting(AppSettings::NoAutoVersion)
        .try_get_matches_from("foo -z".split(' '));

    assert!(res.is_ok());
    assert!(res.unwrap().is_present("version"));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "No version information via App::version or App::long_version to propagate"]
fn propagate_version_no_version_info() {
    let _res = common()
        .setting(AppSettings::PropagateVersion)
        .subcommand(App::new("bar"))
        .try_get_matches_from("foo".split(' '));
}
