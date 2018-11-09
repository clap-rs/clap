extern crate clap;

use clap::{App, Arg, ArgSettings};

#[test]
fn flag_using_short() {
    let m = App::new("flag")
        .args(&[
            Arg::from("-f, --flag 'some flag'"),
            Arg::from("-c, --color 'some other flag'"),
        ])
        .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn lots_o_flags_sep() {
    let r = App::new("opts")
        .arg(Arg::from("-o... 'some flag'"))
        .try_get_matches_from(vec![
            "", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o",
        ]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind);
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.occurrences_of("o"), 297); // i.e. more than u8
}

#[test]
fn lots_o_flags_combined() {
    let r = App::new("opts")
        .arg(Arg::from("-o... 'some flag'"))
        .try_get_matches_from(vec![
            "",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-ooooooooooooooooooooooooooooooooooooooooo",
        ]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind);
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.occurrences_of("o"), 297); // i.e. more than u8
}

#[test]
fn flag_using_long() {
    let m = App::new("flag")
        .args(&[
            Arg::from("--flag 'some flag'"),
            Arg::from("--color 'some other flag'"),
        ])
        .get_matches_from(vec!["", "--flag", "--color"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn flag_using_mixed() {
    let m = App::new("flag")
        .args(&[
            Arg::from("-f, --flag 'some flag'"),
            Arg::from("-c, --color 'some other flag'"),
        ])
        .get_matches_from(vec!["", "-f", "--color"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));

    let m = App::new("flag")
        .args(&[
            Arg::from("-f, --flag 'some flag'"),
            Arg::from("-c, --color 'some other flag'"),
        ])
        .get_matches_from(vec!["", "--flag", "-c"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn multiple_flags_in_single() {
    let m = App::new("multe_flags")
        .args(&[
            Arg::from("-f, --flag 'some flag'"),
            Arg::from("-c, --color 'some other flag'"),
            Arg::from("-d, --debug 'another other flag'"),
        ])
        .get_matches_from(vec!["", "-fcd"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
    assert!(m.is_present("debug"));
}

#[test]
fn short_flag_misspel() {
    let a = Arg::from("-f1, --flag 'some flag'");
    assert_eq!(a.name, "flag");
    assert_eq!(a.short.unwrap(), 'f');
    assert_eq!(a.long.unwrap(), "flag");
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::MultipleOccurrences));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());
}

#[test]
fn short_flag_name_missing() {
    let a = Arg::from("-f 'some flag'");
    assert_eq!(a.name, "f");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::MultipleOccurrences));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());
}
