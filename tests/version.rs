extern crate clap;

use clap::{App, ErrorKind};

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "-V"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::VersionDisplayed);
}

#[test]
fn version_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "--version"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::VersionDisplayed);
}
