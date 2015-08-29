extern crate clap;

use clap::{App, Arg};

#[test]
fn posix_compatible_flags_long() {
    let m = App::new("posix")
                .arg(Arg::from_usage("--flag  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color 'some other flag'"))
                .get_matches_from(vec!["", "--flag", "--color"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("--flag  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color 'some other flag'"))
                .get_matches_from(vec!["", "--color", "--flag"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_short() {
    let m = App::new("posix")
                .arg(Arg::from_usage("-f, --flag  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c, --color 'some other flag'"))
                .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("-f, --flag  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c, --color 'some other flag'"))
                .get_matches_from(vec!["", "-c", "-f"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long() {
    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["", "--flag", "some" ,"--color", "other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["", "--color", "some" ,"--flag", "other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_long_equals() {
    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["", "--flag=some" ,"--color=other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["", "--color=some" ,"--flag=other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_short() {
    let m = App::new("posix")
                .arg(Arg::from_usage("-f [flag]  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c [color] 'some other flag'"))
                .get_matches_from(vec!["", "-f", "some", "-c", "other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("-f [flag]  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c [color] 'some other flag'"))
                .get_matches_from(vec!["", "-c", "some", "-f", "other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}