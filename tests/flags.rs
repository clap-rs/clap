extern crate clap;

use clap::{App, Arg};

#[test]
fn flag_using_short() {
    let m = App::new("flag")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn flag_using_long() {
    let m = App::new("flag")
        .args(vec![
            Arg::from_usage("--flag 'some flag'"),
            Arg::from_usage("--color 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag", "--color"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn flag_using_mixed() {
    let m = App::new("flag")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "--color"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));

    let m = App::new("flag")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag", "-c"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn multiple_flags_in_single() {
    let m = App::new("multe_flags")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'"),
            Arg::from_usage("-d, --debug 'another other flag'")
            ])
        .get_matches_from(vec!["", "-fcd"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
    assert!(m.is_present("debug"));
}

#[test]
#[should_panic]
fn short_flag_misspel() {
    App::new("short_flag")
        .arg(Arg::from_usage("-f1, --flag 'some flag'"));
}

#[test]
#[should_panic]
fn short_flag_name_missing() {
    App::new("short_flag")
        .arg(Arg::from_usage("-f 'some flag'"));
}