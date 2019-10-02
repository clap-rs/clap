extern crate clap;

use clap::{App, Arg, ErrorKind};
#[test]
fn flag_overrides_itself() {
    let res = App::new("posix")
        .arg(Arg::from("--flag  'some flag'").overrides_with("flag"))
        .try_get_matches_from(vec!["", "--flag", "--flag"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn mult_flag_overrides_itself() {
    let res = App::new("posix")
        .arg(Arg::from("--flag...  'some flag'").overrides_with("flag"))
        .try_get_matches_from(vec!["", "--flag", "--flag", "--flag", "--flag"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 4);
}

#[test]
fn option_overrides_itself() {
    let res = App::new("posix")
        .arg(Arg::from("--opt [val] 'some option'").overrides_with("opt"))
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 1);
    assert_eq!(m.value_of("opt"), Some("other"));
}

#[test]
fn mult_option_require_delim_overrides_itself() {
    let res = App::new("posix")
        .arg(
            Arg::from("--opt [val]... 'some option'")
                .overrides_with("opt")
                .number_of_values(1)
                .require_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
    assert!(res.is_ok());
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
    let res = App::new("posix")
        .arg(Arg::from("--opt [val]... 'some option'").overrides_with("opt"))
        .try_get_matches_from(vec![
            "", "--opt", "first", "overides", "--opt", "some", "other", "val",
        ]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 2);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["first", "overides", "some", "other", "val"]
    );
}

#[test]
fn option_use_delim_false_override_itself() {
    let m = App::new("posix")
        .arg(Arg::from("--opt [val] 'some option'").overrides_with("opt"))
        .get_matches_from(vec!["", "--opt=some,other", "--opt=one,two"]);
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
    let res = App::new("posix")
        .arg(Arg::from("[val]... 'some pos'").overrides_with("val"))
        .try_get_matches_from(vec!["", "some", "other", "value"]);
    assert!(res.is_ok());
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
    let m = App::new("posix")
        .arg(Arg::from("--flag  'some flag'").overrides_with("color"))
        .arg(Arg::from("--color 'some other flag'"))
        .get_matches_from(vec!["", "--flag", "--color"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_long_rev() {
    let m = App::new("posix")
        .arg(Arg::from("--flag  'some flag'").overrides_with("color"))
        .arg(Arg::from("--color 'some other flag'"))
        .get_matches_from(vec!["", "--color", "--flag"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_short() {
    let m = App::new("posix")
        .arg(Arg::from("-f, --flag  'some flag'").overrides_with("color"))
        .arg(Arg::from("-c, --color 'some other flag'"))
        .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_flags_short_rev() {
    let m = App::new("posix")
        .arg(Arg::from("-f, --flag  'some flag'").overrides_with("color"))
        .arg(Arg::from("-c, --color 'some other flag'"))
        .get_matches_from(vec!["", "-c", "-f"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long() {
    let m = App::new("posix")
        .arg(Arg::from("--flag [flag] 'some flag'").overrides_with("color"))
        .arg(Arg::from("--color [color] 'some other flag'"))
        .get_matches_from(vec!["", "--flag", "some", "--color", "other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long_rev() {
    let m = App::new("posix")
        .arg(Arg::from("--flag [flag] 'some flag'").overrides_with("color"))
        .arg(Arg::from("--color [color] 'some other flag'"))
        .get_matches_from(vec!["", "--color", "some", "--flag", "other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_long_equals() {
    let m = App::new("posix")
        .arg(Arg::from("--flag [flag] 'some flag'").overrides_with("color"))
        .arg(Arg::from("--color [color] 'some other flag'"))
        .get_matches_from(vec!["", "--flag=some", "--color=other"]);
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
    assert!(!m.is_present("flag"));
}

#[test]
fn posix_compatible_opts_long_equals_rev() {
    let m = App::new("posix")
        .arg(Arg::from("--flag [flag] 'some flag'").overrides_with("color"))
        .arg(Arg::from("--color [color] 'some other flag'"))
        .get_matches_from(vec!["", "--color=some", "--flag=other"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "other");
}

#[test]
fn posix_compatible_opts_short() {
    let m = App::new("posix")
        .arg(Arg::from("-f [flag]  'some flag'").overrides_with("c"))
        .arg(Arg::from("-c [color] 'some other flag'"))
        .get_matches_from(vec!["", "-f", "some", "-c", "other"]);
    assert!(m.is_present("c"));
    assert_eq!(m.value_of("c").unwrap(), "other");
    assert!(!m.is_present("f"));
}

#[test]
fn posix_compatible_opts_short_rev() {
    let m = App::new("posix")
        .arg(Arg::from("-f [flag]  'some flag'").overrides_with("c"))
        .arg(Arg::from("-c [color] 'some other flag'"))
        .get_matches_from(vec!["", "-c", "some", "-f", "other"]);
    assert!(!m.is_present("c"));
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "other");
}

#[test]
fn conflict_overriden() {
    let m = App::new("conflict_overriden")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("debug"))
        .arg(Arg::from("-d, --debug 'other flag'"))
        .arg(Arg::from("-c, --color 'third flag'").overrides_with("flag"))
        .get_matches_from(vec!["", "-f", "-c", "-d"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(m.is_present("debug"));
}

#[test]
fn conflict_overriden_2() {
    let result = App::new("conflict_overriden")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("debug"))
        .arg(Arg::from("-d, --debug 'other flag'"))
        .arg(Arg::from("-c, --color 'third flag'").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-f", "-d", "-c"]);
    assert!(result.is_ok());
    let m = result.unwrap();
    assert!(m.is_present("color"));
    assert!(m.is_present("debug"));
    assert!(!m.is_present("flag"));
}

#[test]
fn conflict_overriden_3() {
    let result = App::new("conflict_overriden")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("debug"))
        .arg(Arg::from("-d, --debug 'other flag'"))
        .arg(Arg::from("-c, --color 'third flag'").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-d", "-c", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn conflict_overriden_4() {
    let m = App::new("conflict_overriden")
        .arg(Arg::from("-f, --flag 'some flag'").conflicts_with("debug"))
        .arg(Arg::from("-d, --debug 'other flag'"))
        .arg(Arg::from("-c, --color 'third flag'").overrides_with("flag"))
        .get_matches_from(vec!["", "-d", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(m.is_present("debug"));
}

#[test]
fn pos_required_overridden_by_flag() {
    let result = App::new("require_overriden")
        .arg(Arg::with_name("pos").index(1).required(true))
        .arg(Arg::from("-c, --color 'some flag'").overrides_with("pos"))
        .try_get_matches_from(vec!["", "test", "-c"]);
    assert!(result.is_ok(), "{:?}", result.unwrap_err());
}

#[test]
fn require_overriden_2() {
    let m = App::new("require_overriden")
        .arg(Arg::with_name("req_pos").required(true))
        .arg(Arg::from("-c, --color 'other flag'").overrides_with("req_pos"))
        .get_matches_from(vec!["", "-c", "req_pos"]);
    assert!(!m.is_present("color"));
    assert!(m.is_present("req_pos"));
}

#[test]
fn require_overriden_3() {
    let m = App::new("require_overriden")
        .arg(Arg::from("-f, --flag 'some flag'").requires("debug"))
        .arg(Arg::from("-d, --debug 'other flag'"))
        .arg(Arg::from("-c, --color 'third flag'").overrides_with("flag"))
        .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("color"));
    assert!(!m.is_present("flag"));
    assert!(!m.is_present("debug"));
}

#[test]
fn require_overriden_4() {
    let result = App::new("require_overriden")
        .arg(Arg::from("-f, --flag 'some flag'").requires("debug"))
        .arg(Arg::from("-d, --debug 'other flag'"))
        .arg(Arg::from("-c, --color 'third flag'").overrides_with("flag"))
        .try_get_matches_from(vec!["", "-c", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}
