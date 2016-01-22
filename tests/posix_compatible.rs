extern crate clap;

use clap::{App, Arg, ErrorKind};

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
                .get_matches_from(vec!["myprog", "--color", "--flag"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_short() {
    let m = App::new("posix")
                .arg(Arg::from_usage("-f, --flag  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c, --color 'some other flag'"))
                .get_matches_from(vec!["myprog", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("-f, --flag  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c, --color 'some other flag'"))
                .get_matches_from(vec!["myprog", "-c", "-f"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long() {
    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["myprog", "--flag", "some" ,"--color", "other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["myprog", "--color", "some" ,"--flag", "other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_long_equals() {
    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["myprog", "--flag=some" ,"--color=other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("--color [color] 'some other flag'"))
                .get_matches_from(vec!["myprog", "--color=some" ,"--flag=other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_short() {
    let m = App::new("posix")
                .arg(Arg::from_usage("-f [flag]  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c [color] 'some other flag'"))
                .get_matches_from(vec!["myprog", "-f", "some", "-c", "other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));

    let m = App::new("posix")
                .arg(Arg::from_usage("-f [flag]  'some flag'").mutually_overrides_with("color"))
                .arg(Arg::from_usage("-c [color] 'some other flag'"))
                .get_matches_from(vec!["myprog", "-c", "some", "-f", "other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn conflict_overriden() {
    let m = App::new("conflict_overriden")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("debug"))
        .arg(Arg::from_usage("-d, --debug 'other flag'"))
        .arg(Arg::from_usage("-c, --color 'third flag'")
            .mutually_overrides_with("flag"))
        .get_matches_from(vec!["myprog", "-f", "-c", "-d"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(m.is_present("debug"));
}

#[test]
fn conflict_overriden_2() {
    let result = App::new("conflict_overriden")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("debug"))
        .arg(Arg::from_usage("-d, --debug 'other flag'"))
        .arg(Arg::from_usage("-c, --color 'third flag'")
            .mutually_overrides_with("flag"))
        .get_matches_from_safe(vec!["myprog", "-f", "-d", "-c"]);
    assert!(result.is_ok());
    let m = result.unwrap();
    assert!(m.is_present("color"));
    assert!(m.is_present("debug"));
    assert!(!m.is_present("flag"));
}

#[test]
fn conflict_overriden_3() {
    let result = App::new("conflict_overriden")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("debug"))
        .arg(Arg::from_usage("-d, --debug 'other flag'"))
        .arg(Arg::from_usage("-c, --color 'third flag'")
            .mutually_overrides_with("flag"))
        .get_matches_from_safe(vec!["myprog", "-d", "-c", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn conflict_overriden_4() {
    let m = App::new("conflict_overriden")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .conflicts_with("debug"))
        .arg(Arg::from_usage("-d, --debug 'other flag'"))
        .arg(Arg::from_usage("-c, --color 'third flag'")
            .mutually_overrides_with("flag"))
        .get_matches_from(vec!["myprog", "-d", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(m.is_present("debug"));
}

#[test]
fn pos_required_overridden_by_flag() {
    let result = App::new("require_overriden")
        .arg(Arg::with_name("pos")
            .index(1)
            .required(true))
        .arg(Arg::from_usage("-c, --color 'some flag'")
            .mutually_overrides_with("pos"))
        .get_matches_from_safe(vec!["myprog", "test", "-c"]);
    assert!(result.is_ok(), "{:?}", result.unwrap_err());
}

#[test]
fn require_overriden_2() {
    let m = App::new("require_overriden")
        .arg(Arg::with_name("flag")
            .index(1)
            .required(true))
        .arg(Arg::from_usage("-c, --color 'other flag'")
            .mutually_overrides_with("flag"))
        .get_matches_from(vec!["myprog", "-c", "flag"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn require_overriden_3() {
    let m = App::new("require_overriden")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("debug"))
        .arg(Arg::from_usage("-d, --debug 'other flag'"))
        .arg(Arg::from_usage("-c, --color 'third flag'")
            .mutually_overrides_with("flag"))
        .get_matches_from(vec!["myprog", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(!m.is_present("debug"));
}

#[test]
fn require_overriden_4() {
    let result = App::new("require_overriden")
        .arg(Arg::from_usage("-f, --flag 'some flag'")
            .requires("debug"))
        .arg(Arg::from_usage("-d, --debug 'other flag'"))
        .arg(Arg::from_usage("-c, --color 'third flag'")
            .mutually_overrides_with("flag"))
        .get_matches_from_safe(vec!["myprog", "-c", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}
