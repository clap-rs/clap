extern crate clap;

use clap::{App, ClapErrorType};

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["", "-V"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::VersionDisplayed);
}

#[test]
fn version_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["", "--version"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::VersionDisplayed);
}
