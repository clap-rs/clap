use clap::{arg, error::ErrorKind, Arg, ArgAction, Command};

#[test]
fn flag_overrides_itself() {
    let res = Command::new("posix")
        .arg(
            arg!(--flag  "some flag"
            )
            .action(ArgAction::SetTrue)
            .overrides_with("flag"),
        )
        .try_get_matches_from(vec!["", "--flag", "--flag"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn mult_flag_overrides_itself() {
    let res = Command::new("posix")
        .arg(
            arg!(--flag ...  "some flag")
                .overrides_with("flag")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "--flag", "--flag", "--flag", "--flag"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
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
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("other")
    );
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
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
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
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
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
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_many::<String>("opt")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
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
    assert!(m.contains_id("val"));
    assert_eq!(
        m.get_many::<String>("val")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["some", "other", "value"]
    );
}
#[test]
fn posix_compatible_flags_long() {
    let m = Command::new("posix")
        .arg(
            arg!(--flag  "some flag")
                .overrides_with("color")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(--color "some other flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "--flag", "--color"])
        .unwrap();
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn posix_compatible_flags_long_rev() {
    let m = Command::new("posix")
        .arg(
            arg!(--flag  "some flag")
                .overrides_with("color")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(--color "some other flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "--color", "--flag"])
        .unwrap();
    assert!(!*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn posix_compatible_flags_short() {
    let m = Command::new("posix")
        .arg(
            arg!(-f --flag  "some flag")
                .overrides_with("color")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-c --color "some other flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn posix_compatible_flags_short_rev() {
    let m = Command::new("posix")
        .arg(
            arg!(-f --flag  "some flag")
                .overrides_with("color")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-c --color "some other flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-c", "-f"])
        .unwrap();
    assert!(!*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
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
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "other"
    );
    assert!(!m.contains_id("flag"));
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
    assert!(!m.contains_id("color"));
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()).unwrap(),
        "other"
    );
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
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "other"
    );
    assert!(!m.contains_id("flag"));
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
    assert!(!m.contains_id("color"));
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()).unwrap(),
        "other"
    );
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
    assert!(m.contains_id("c"));
    assert_eq!(
        m.get_one::<String>("c").map(|v| v.as_str()).unwrap(),
        "other"
    );
    assert!(!m.contains_id("f"));
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
    assert!(!m.contains_id("c"));
    assert!(m.contains_id("f"));
    assert_eq!(
        m.get_one::<String>("f").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn conflict_overridden() {
    let m = Command::new("conflict_overridden")
        .arg(
            arg!(-f --flag "some flag")
                .conflicts_with("debug")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-d --debug "other flag").action(ArgAction::SetTrue))
        .arg(
            arg!(-c --color "third flag")
                .overrides_with("flag")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "-f", "-c", "-d"])
        .unwrap();
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("debug").expect("defaulted by clap"));
}

#[test]
fn conflict_overridden_2() {
    let result = Command::new("conflict_overridden")
        .arg(
            arg!(-f --flag "some flag")
                .conflicts_with("debug")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-d --debug "other flag").action(ArgAction::SetTrue))
        .arg(
            arg!(-c --color "third flag")
                .overrides_with("flag")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "-f", "-d", "-c"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("debug").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
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
        .arg(
            arg!(-f --flag "some flag")
                .conflicts_with("debug")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-d --debug "other flag").action(ArgAction::SetTrue))
        .arg(
            arg!(-c --color "third flag")
                .overrides_with("flag")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "-d", "-f", "-c"])
        .unwrap();
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("debug").expect("defaulted by clap"));
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
        .arg(
            arg!(-c --color "other flag")
                .overrides_with("req_pos")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "-c", "req_pos"])
        .unwrap();
    assert!(!*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(m.contains_id("req_pos"));
}

#[test]
fn require_overridden_3() {
    let m = Command::new("require_overridden")
        .arg(
            arg!(-f --flag "some flag")
                .requires("debug")
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-d --debug "other flag").action(ArgAction::SetTrue))
        .arg(
            arg!(-c --color "third flag")
                .overrides_with("flag")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("debug").expect("defaulted by clap"));
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
    assert_eq!(
        m.get_many::<String>("input")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["d"]
    );
    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "--input", "a", "b", "--input", "c", "d"])
        .unwrap();
    assert_eq!(
        m.get_many::<String>("input")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["c", "d"]
    );
}

#[test]
fn incremental_override() {
    let mut cmd = Command::new("test")
        .arg(arg!(--name <NAME>).action(ArgAction::Append))
        .arg(
            arg!(--"no-name")
                .overrides_with("name")
                .action(ArgAction::SetTrue),
        );
    let m = cmd
        .try_get_matches_from_mut(&["test", "--name=ahmed", "--no-name", "--name=ali"])
        .unwrap();
    assert_eq!(
        m.get_many::<String>("name")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["ali"]
    );
    assert!(!*m.get_one::<bool>("no-name").expect("defaulted by clap"));
}
