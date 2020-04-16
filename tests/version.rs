mod utils;

use std::str;

use clap::{App, AppSettings, ErrorKind};

static VERSION: &str = "clap-test v1.4.8";

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .try_get_matches_from(vec!["myprog", "-V"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::VersionDisplayed);
    assert_eq!(err.to_string(), "test 1.3");
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
    assert_eq!(err.kind, ErrorKind::VersionDisplayed);
    assert_eq!(err.to_string(), "test 1.3");
}

#[test]
fn complex_version_output() {
    let mut a = App::new("clap-test").version("v1.4.8");
    let _ = a.try_get_matches_from_mut(vec![""]);

    // Now we check the output of print_version()
    let mut ver = vec![];
    a.write_version(&mut ver).unwrap();
    assert_eq!(str::from_utf8(&ver).unwrap(), VERSION);
}

#[test]
fn override_ver() {
    let m = App::new("test")
        .setting(AppSettings::NoAutoVersion)
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .mut_arg("version", |a| {
            a.short('v').long("version").help("some version")
        })
        .try_get_matches_from(vec!["test", "--version"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    assert!(m.unwrap().is_present("version"));
}
