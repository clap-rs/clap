use clap::{arg, error::ErrorKind, Arg, Command};

#[test]
fn flag_overrides_itself() {
    let res = Command::new("posix")
        .arg(
            arg!(--flag  "some flag"
            )
            .overrides_with("flag"),
        )
        .try_get_matches_from(vec!["", "--flag", "--flag"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn mult_flag_overrides_itself() {
    let res = Command::new("posix")
        .arg(arg!(--flag ...  "some flag").overrides_with("flag"))
        .try_get_matches_from(vec!["", "--flag", "--flag", "--flag", "--flag"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 4);
}

#[test]
fn option_overrides_itself() {
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> "some option")
                .required(false)
                .overrides_with("opt"),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn mult_option_require_delim_overrides_itself() {
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> ... "some option")
                .required(false)
                .overrides_with("opt")
                .number_of_values(1)
                .takes_value(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 3);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["some", "other", "one", "two"]
    );
}

#[test]
fn mult_option_overrides_itself() {
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> ... "some option")
                .required(false)
                .multiple_values(true)
                .overrides_with("opt"),
        )
        .try_get_matches_from(vec![
            "",
            "--opt",
            "first",
            "overrides",
            "--opt",
            "some",
            "other",
            "val",
        ]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 2);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["first", "overrides", "some", "other", "val"]
    );
}

#[test]
fn option_use_delim_false_override_itself() {
    let m = Command::new("posix")
        .arg(
            arg!(--opt <val> "some option")
                .required(false)
                .overrides_with("opt"),
        )
        .try_get_matches_from(vec!["", "--opt=some,other", "--opt=one,two"])
        .unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["one,two"]
    );
}

#[test]
fn pos_mult_overrides_itself() {
    // opts with multiple
    let res = Command::new("posix")
        .arg(arg!([val] ... "some pos").overrides_with("val"))
        .try_get_matches_from(vec!["", "some", "other", "value"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("val"));
    assert_eq!(m.occurrences_of("val"), 3);
    assert_eq!(
        m.values_of("val").unwrap().collect::<Vec<_>>(),
        &["some", "other", "value"]
    );
}
#[test]
fn posix_compatible_flags_long() {
    let m = Command::new("posix")
        .arg(arg!(--flag  "some flag").overrides_with("color"))
        .arg(arg!(--color "some other flag"))
        .try_get_matches_from(vec!["", "--flag", "--color"])
        .unwrap();
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_long_rev() {
    let m = Command::new("posix")
        .arg(arg!(--flag  "some flag").overrides_with("color"))
        .arg(arg!(--color "some other flag"))
        .try_get_matches_from(vec!["", "--color", "--flag"])
        .unwrap();
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_short() {
    let m = Command::new("posix")
        .arg(arg!(-f --flag  "some flag").overrides_with("color"))
        .arg(arg!(-c --color "some other flag"))
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_short_rev() {
    let m = Command::new("posix")
        .arg(arg!(-f --flag  "some flag").overrides_with("color"))
        .arg(arg!(-c --color "some other flag"))
        .try_get_matches_from(vec!["", "-c", "-f"])
        .unwrap();
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long() {
    let m = Command::new("posix")
        .arg(
            arg!(--flag <flag> "some flag")
                .required(false)
                .overrides_with("color"),
        )
        .arg(arg!(--color <color> "some other flag").required(false))
        .try_get_matches_from(vec!["", "--flag", "some", "--color", "other"])
        .unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long_rev() {
    let m = Command::new("posix")
        .arg(
            arg!(--flag <flag> "some flag")
                .required(false)
                .overrides_with("color"),
        )
        .arg(arg!(--color <color> "some other flag").required(false))
        .try_get_matches_from(vec!["", "--color", "some", "--flag", "other"])
        .unwrap();
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_long_equals() {
    let m = Command::new("posix")
        .arg(
            arg!(--flag <flag> "some flag")
                .required(false)
                .overrides_with("color"),
        )
        .arg(arg!(--color <color> "some other flag").required(false))
        .try_get_matches_from(vec!["", "--flag=some", "--color=other"])
        .unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long_equals_rev() {
    let m = Command::new("posix")
        .arg(
            arg!(--flag <flag> "some flag")
                .required(false)
                .overrides_with("color"),
        )
        .arg(arg!(--color <color> "some other flag").required(false))
        .try_get_matches_from(vec!["", "--color=some", "--flag=other"])
        .unwrap();
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_short() {
    let m = Command::new("posix")
        .arg(
            arg!(f: -f <flag>  "some flag")
                .required(false)
                .overrides_with("c"),
        )
        .arg(arg!(c: -c <color> "some other flag").required(false))
        .try_get_matches_from(vec!["", "-f", "some", "-c", "other"])
        .unwrap();
    assert!(m.is_present("c"));
    assert_eq!(m.value_of("c").unwrap(), "other");
    assert!(!m.is_present("f"));
}

#[test]
fn posix_compatible_opts_short_rev() {
    let m = Command::new("posix")
        .arg(
            arg!(f: -f <flag>  "some flag")
                .required(false)
                .overrides_with("c"),
        )
        .arg(arg!(c: -c <color> "some other flag").required(false))
        .try_get_matches_from(vec!["", "-c", "some", "-f", "other"])
        .unwrap();
    assert!(!m.is_present("c"));
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "other");
}

#[test]
fn conflict_overridden() {
    let m = Command::new("conflict_overridden")
        .arg(arg!(-f --flag "some flag").conflicts_with("debug"))
        .arg(arg!(-d --debug "other flag"))
        .arg(arg!(-c --color "third flag").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-f", "-c", "-d"])
        .unwrap();
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(m.is_present("debug"));
}

#[test]
fn conflict_overridden_2() {
    let result = Command::new("conflict_overridden")
        .arg(arg!(-f --flag "some flag").conflicts_with("debug"))
        .arg(arg!(-d --debug "other flag"))
        .arg(arg!(-c --color "third flag").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-f", "-d", "-c"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.is_present("color"));
    assert!(m.is_present("debug"));
    assert!(!m.is_present("flag"));
}

#[test]
fn conflict_overridden_3() {
    let result = Command::new("conflict_overridden")
        .arg(arg!(-f --flag "some flag").conflicts_with("debug"))
        .arg(arg!(-d --debug "other flag"))
        .arg(arg!(-c --color "third flag").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-d", "-c", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn conflict_overridden_4() {
    let m = Command::new("conflict_overridden")
        .arg(arg!(-f --flag "some flag").conflicts_with("debug"))
        .arg(arg!(-d --debug "other flag"))
        .arg(arg!(-c --color "third flag").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-d", "-f", "-c"])
        .unwrap();
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(m.is_present("debug"));
}

#[test]
fn pos_required_overridden_by_flag() {
    let result = Command::new("require_overridden")
        .arg(Arg::new("pos").index(1).required(true))
        .arg(arg!(-c --color "some flag").overrides_with("pos"))
        .try_get_matches_from(vec!["", "test", "-c"]);
    assert!(result.is_ok(), "{:?}", result.unwrap_err());
}

#[test]
fn require_overridden_2() {
    let m = Command::new("require_overridden")
        .arg(Arg::new("req_pos").required(true))
        .arg(arg!(-c --color "other flag").overrides_with("req_pos"))
        .try_get_matches_from(vec!["", "-c", "req_pos"])
        .unwrap();
    assert!(!m.is_present("color"));
    assert!(m.is_present("req_pos"));
}

#[test]
fn require_overridden_3() {
    let m = Command::new("require_overridden")
        .arg(arg!(-f --flag "some flag").requires("debug"))
        .arg(arg!(-d --debug "other flag"))
        .arg(arg!(-c --color "third flag").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(!m.is_present("debug"));
}

#[test]
fn require_overridden_4() {
    let result = Command::new("require_overridden")
        .arg(arg!(-f --flag "some flag").requires("debug"))
        .arg(arg!(-d --debug "other flag"))
        .arg(arg!(-c --color "third flag").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-c", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn issue_1374_overrides_self_with_multiple_values() {
    let cmd = Command::new("test").arg(
        Arg::new("input")
            .long("input")
            .takes_value(true)
            .overrides_with("input")
            .min_values(0),
    );
    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "--input", "a", "b", "c", "--input", "d"])
        .unwrap();
    assert_eq!(m.values_of("input").unwrap().collect::<Vec<_>>(), &["d"]);
    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "--input", "a", "b", "--input", "c", "d"])
        .unwrap();
    assert_eq!(
        m.values_of("input").unwrap().collect::<Vec<_>>(),
        &["c", "d"]
    );
}

#[test]
fn incremental_override() {
    let mut cmd = Command::new("test")
        .arg(arg!(--name <NAME>).multiple_occurrences(true))
        .arg(arg!(--"no-name").overrides_with("name"));
    let m = cmd
        .try_get_matches_from_mut(&["test", "--name=ahmed", "--no-name", "--name=ali"])
        .unwrap();
    assert_eq!(m.values_of("name").unwrap().collect::<Vec<_>>(), &["ali"]);
    assert!(!m.is_present("no-name"));
}
