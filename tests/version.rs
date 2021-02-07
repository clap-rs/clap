mod utils;

use std::str;

use clap::{App, AppSettings, Arg, ErrorKind};

static VERSION: &str = "clap-test v1.4.8\n";

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .try_get_matches_from(vec!["myprog", "-V"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");
}

#[test]
fn version_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .try_get_matches_from(vec!["myprog", "--version"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");
}

#[test]
fn complex_version_output() {
    let mut a = App::new("clap-test").version("v1.4.8");
    let _ = a.try_get_matches_from_mut(vec![""]);

    // Now we check the output of print_version()
    assert_eq!(a.render_version(), VERSION);
}

fn prefer_user_app() -> App<'static> {
    App::new("test")
        .version("1.3")
        .arg(
            Arg::new("version1")
                .long("version")
                .short('V')
                .about("some version"),
        )
        .subcommand(
            App::new("foo").arg(
                Arg::new("version1")
                    .long("version")
                    .short('V')
                    .about("some version"),
            ),
        )
}

#[test]
fn prefer_user_version_long() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "--version"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("version1"));
}

#[test]
fn prefer_user_version_short() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "-V"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("version1"));
}

#[test]
fn prefer_user_subcmd_version_long() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "foo", "--version"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("version1"));
}

#[test]
fn prefer_user_subcmd_version_short() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "foo", "-V"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("version1"));
}

#[test]
fn override_ver() {
    let m = App::new("test")
        .setting(AppSettings::NoAutoVersion)
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .mut_arg("version", |a| {
            a.short('v').long("version").about("some version")
        })
        .try_get_matches_from(vec!["test", "--version"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert!(m.unwrap().is_present("version"));
}
