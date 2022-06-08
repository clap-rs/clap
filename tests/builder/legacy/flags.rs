use super::utils;
use clap::{arg, Arg, Command};

const USE_FLAG_AS_ARGUMENT: &str =
    "error: Found argument '--another-flag' which wasn't expected, or isn't valid in this context

\tIf you tried to supply `--another-flag` as a value rather than a flag, use `-- --another-flag`

USAGE:
    mycat [OPTIONS] [filename]

For more information try --help
";

#[test]
fn flag_using_short() {
    let m = Command::new("flag")
        .args(&[
            arg!(-f --flag "some flag"),
            arg!(-c --color "some other flag"),
        ])
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn lots_o_flags_sep() {
    let r = Command::new("opts")
        .arg(arg!(o: -o ... "some flag"))
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
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.occurrences_of("o"), 297); // i.e. more than u8
}

#[test]
fn lots_o_flags_combined() {
    let r = Command::new("opts")
        .arg(arg!(o: -o ... "some flag"))
        .try_get_matches_from(vec![
            "",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-ooooooooooooooooooooooooooooooooooooooooo",
        ]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.occurrences_of("o"), 297); // i.e. more than u8
}

#[test]
fn flag_using_long() {
    let m = Command::new("flag")
        .args(&[arg!(--flag "some flag"), arg!(--color "some other flag")])
        .try_get_matches_from(vec!["", "--flag", "--color"])
        .unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn flag_using_long_with_literals() {
    use clap::error::ErrorKind;

    let m = Command::new("flag")
        .arg(Arg::new("rainbow").long("rainbow"))
        .try_get_matches_from(vec!["", "--rainbow=false"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyValues);
}

#[test]
fn flag_using_mixed() {
    let m = Command::new("flag")
        .args(&[
            arg!(-f --flag "some flag"),
            arg!(-c --color "some other flag"),
        ])
        .try_get_matches_from(vec!["", "-f", "--color"])
        .unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));

    let m = Command::new("flag")
        .args(&[
            arg!(-f --flag "some flag"),
            arg!(-c --color "some other flag"),
        ])
        .try_get_matches_from(vec!["", "--flag", "-c"])
        .unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn multiple_flags_in_single() {
    let m = Command::new("multe_flags")
        .args(&[
            arg!(-f --flag "some flag"),
            arg!(-c --color "some other flag"),
            arg!(-d --debug "another other flag"),
        ])
        .try_get_matches_from(vec!["", "-fcd"])
        .unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
    assert!(m.is_present("debug"));
}

#[test]
fn issue_1284_argument_in_flag_style() {
    let cmd = Command::new("mycat")
        .arg(Arg::new("filename"))
        .arg(Arg::new("a-flag").long("a-flag"));

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["", "--", "--another-flag"])
        .unwrap();
    assert_eq!(m.value_of("filename"), Some("--another-flag"));

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["", "--a-flag"])
        .unwrap();
    assert!(m.is_present("a-flag"));

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["", "--", "--a-flag"])
        .unwrap();
    assert_eq!(m.value_of("filename"), Some("--a-flag"));

    utils::assert_output(cmd, "mycat --another-flag", USE_FLAG_AS_ARGUMENT, true);
}

#[test]
fn issue_2308_multiple_dashes() {
    static MULTIPLE_DASHES: &str =
        "error: Found argument '-----' which wasn't expected, or isn't valid in this context

	If you tried to supply `-----` as a value rather than a flag, use `-- -----`

USAGE:
    test <arg>

For more information try --help
";
    let cmd = Command::new("test").arg(Arg::new("arg").takes_value(true).required(true));

    utils::assert_output(cmd, "test -----", MULTIPLE_DASHES, true);
}

#[test]
#[cfg(not(feature = "unstable-v4"))]
fn leading_dash_stripped() {
    let cmd = Command::new("mycat").arg(Arg::new("filename").long("--filename"));
    let matches = cmd.try_get_matches_from(["mycat", "--filename"]).unwrap();
    assert!(matches.is_present("filename"));
}

#[test]
#[cfg(feature = "unstable-v4")]
#[cfg(debug_assertions)]
#[should_panic = "Argument filename: long \"--filename\" must not start with a `-`, that will be handled by the parser"]
fn leading_dash_stripped() {
    let cmd = Command::new("mycat").arg(Arg::new("filename").long("--filename"));
    cmd.debug_assert();
}
