mod utils;
use clap::{App, Arg};

const USE_FLAG_AS_ARGUMENT: &str =
    "error: Found argument '--another-flag' which wasn't expected, or isn't valid in this context

\tIf you tried to supply `--another-flag` as a value rather than a flag, use `-- --another-flag`

USAGE:
    mycat [FLAGS] [filename]

For more information try --help";

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
fn issue_1284_argument_in_flag_style() {
    let app = App::new("mycat")
        .arg(Arg::new("filename"))
        .arg(Arg::new("a-flag").long("a-flag"));

    let m = app
        .clone()
        .get_matches_from(vec!["", "--", "--another-flag"]);
    assert_eq!(m.value_of("filename"), Some("--another-flag"));

    let m = app.clone().get_matches_from(vec!["", "--a-flag"]);
    assert!(m.is_present("a-flag"));

    let m = app.clone().get_matches_from(vec!["", "--", "--a-flag"]);
    assert_eq!(m.value_of("filename"), Some("--a-flag"));

    assert!(utils::compare_output(
        app,
        "mycat --another-flag",
        USE_FLAG_AS_ARGUMENT,
        true
    ));
}

#[test]
fn issue_2308_multiple_dashes() {
    static MULTIPLE_DASHES: &str =
        "error: Found argument '-----' which wasn't expected, or isn't valid in this context

	If you tried to supply `-----` as a value rather than a flag, use `-- -----`

USAGE:
    test <arg>

For more information try --help";
    let app = App::new("test").arg(Arg::new("arg").takes_value(true).required(true));

    assert!(utils::compare_output(
        app,
        "test -----",
        MULTIPLE_DASHES,
        true
    ));
}
