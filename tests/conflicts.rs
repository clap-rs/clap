mod utils;

use clap::{App, Arg, ArgGroup, ErrorKind};

static CONFLICT_ERR: &str = "error: The argument '-F' cannot be used with '--flag'

USAGE:
    clap-test --flag --long-option-2 <option2> <positional> <positional2>

For more information try --help";

static CONFLICT_ERR_REV: &str = "error: The argument '--flag' cannot be used with '-F'

USAGE:
    clap-test -F --long-option-2 <option2> <positional> <positional2>

For more information try --help";

static CONFLICT_ERR_THREE: &str = "error: The argument '--two' cannot be used with '--one'

USAGE:
    three_conflicting_arguments --one

For more information try --help";

#[test]
fn flag_conflict() {
    let result = App::new("flag_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("other"))
        .arg(Arg::from("-o, --other 'some flag'"))
        .try_get_matches_from(vec!["myprog", "-f", "-o"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn flag_conflict_2() {
    let result = App::new("flag_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("other"))
        .arg(Arg::from("-o, --other 'some flag'"))
        .try_get_matches_from(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn flag_conflict_with_all() {
    let result = App::new("flag_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with_all(&["other"]))
        .arg(Arg::from("-o, --other 'some flag'"))
        .try_get_matches_from(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn flag_conflict_with_everything() {
    let result = App::new("flag_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").exclusive(true))
        .arg(Arg::from("-o, --other 'some flag'"))
        .try_get_matches_from(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_conflict() {
    let result = App::new("group_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("gr"))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(Arg::from("--some 'some arg'"))
        .arg(Arg::from("--other 'other arg'"))
        .try_get_matches_from(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_conflict_2() {
    let result = App::new("group_conflict")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("gr"))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(Arg::from("--some 'some arg'"))
        .arg(Arg::from("--other 'other arg'"))
        .try_get_matches_from(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn conflict_output() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test val1 fa --flag --long-option-2 val2 -F",
        CONFLICT_ERR,
        true,
    ));
}

#[test]
fn conflict_output_rev() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test val1 fa -F --long-option-2 val2 --flag",
        CONFLICT_ERR_REV,
        true,
    ));
}

#[test]
fn conflict_output_with_required() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test val1 --flag --long-option-2 val2 -F",
        CONFLICT_ERR,
        true,
    ));
}

#[test]
fn conflict_output_rev_with_required() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test val1 -F --long-option-2 val2 --flag",
        CONFLICT_ERR_REV,
        true,
    ));
}

#[test]
fn conflict_output_three_conflicting() {
    let app = App::new("three_conflicting_arguments")
        .arg(
            Arg::new("one")
                .long("one")
                .conflicts_with_all(&["two", "three"]),
        )
        .arg(
            Arg::new("two")
                .long("two")
                .conflicts_with_all(&["one", "three"]),
        )
        .arg(
            Arg::new("three")
                .long("three")
                .conflicts_with_all(&["one", "two"]),
        );
    assert!(utils::compare_output(
        app,
        "three_conflicting_arguments --one --two --three",
        CONFLICT_ERR_THREE,
        true,
    ));
}

#[test]
fn conflict_with_unused_default_value() {
    let result = App::new("conflict")
        .arg(Arg::from("-o, --opt=[opt] 'some opt'").default_value("default"))
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("opt"))
        .try_get_matches_from(vec!["myprog", "-f"]);
    assert!(result.is_ok());
    let m = result.unwrap();
    assert_eq!(m.value_of("opt"), Some("default"));
    assert!(m.is_present("flag"));
}

#[test]
fn two_conflicting_arguments() {
    let a = App::new("two_conflicting_arguments")
        .arg(
            Arg::new("develop")
                .long("develop")
                .conflicts_with("production"),
        )
        .arg(
            Arg::new("production")
                .long("production")
                .conflicts_with("develop"),
        )
        .try_get_matches_from(vec!["", "--develop", "--production"]);

    assert!(a.is_err());
    let a = a.unwrap_err();
    assert!(
        a.to_string()
            .contains("The argument \'--production\' cannot be used with \'--develop\'"),
        "{}",
        a
    );
}

#[test]
fn three_conflicting_arguments() {
    let a = App::new("three_conflicting_arguments")
        .arg(
            Arg::new("one")
                .long("one")
                .conflicts_with_all(&["two", "three"]),
        )
        .arg(
            Arg::new("two")
                .long("two")
                .conflicts_with_all(&["one", "three"]),
        )
        .arg(
            Arg::new("three")
                .long("three")
                .conflicts_with_all(&["one", "two"]),
        )
        .try_get_matches_from(vec!["", "--one", "--two", "--three"]);

    assert!(a.is_err());
    let a = a.unwrap_err();
    assert!(
        a.to_string()
            .contains("The argument \'--two\' cannot be used with \'--one\'"),
        "{}",
        a
    );
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument 'config' cannot conflict with itself"]
fn self_conflicting_arg() {
    let _ = App::new("prog")
        .arg(Arg::new("config").long("config").conflicts_with("config"))
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument or group 'extra' specified in 'conflicts_with*' for 'config' does not exist"]
fn conflicts_with_invalid_arg() {
    let _ = App::new("prog")
        .arg(Arg::new("config").long("config").conflicts_with("extra"))
        .try_get_matches_from(vec!["", "--config"]);
}

#[test]
fn conflicts_with_default() {
    let result = App::new("conflict")
        .arg(
            Arg::from("-o, --opt=[opt] 'some opt'")
                .default_value("default")
                .conflicts_with("flag"),
        )
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("opt"))
        .try_get_matches_from(vec!["myprog", "-f"]);

    assert!(result.is_ok(), "{:?}", result.unwrap());
    let m = result.unwrap();

    assert_eq!(m.value_of("opt"), Some("default"));
    assert!(m.is_present("flag"));
}
