extern crate clap;

use clap::{App, Arg, ArgSettings};

#[test]
fn flag_using_short() {
    let m = App::new("flag")
        .args(&[
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
        .args(&[
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
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "--color"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));

    let m = App::new("flag")
        .args(&[
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
        .args(&[
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
fn short_flag_misspel() {
    let a = Arg::from_usage("-f1, --flag 'some flag'");
    assert_eq!(a.name, "flag");
    assert_eq!(a.short.unwrap(), 'f');
    assert_eq!(a.long.unwrap(), "flag");
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());
}

#[test]
fn short_flag_name_missing() {
    let a = Arg::from_usage("-f 'some flag'");
    assert_eq!(a.name, "f");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

}
