use clap::{arg, error::ErrorKind, Arg, ArgAction, ArgMatches, Command};

#[cfg(feature = "error-context")]
use super::utils;

#[test]
fn require_equals_fail() {
    let res = Command::new("prog")
        .arg(
            Arg::new("cfg")
                .require_equals(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .action(ArgAction::Set)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config", "file.conf"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::NoEquals);
}

#[test]
#[cfg(feature = "error-context")]
fn require_equals_fail_message() {
    static NO_EQUALS: &str = "error: equal sign is needed when assigning values to '--config=<cfg>'

Usage: prog [OPTIONS]

For more information, try '--help'.
";
    let cmd = Command::new("prog").arg(
        Arg::new("cfg")
            .require_equals(true)
            .action(ArgAction::Set)
            .long("config"),
    );
    utils::assert_output(cmd, "prog --config file.conf", NO_EQUALS, true);
}

#[test]
fn require_equals_min_values_zero() {
    let res = Command::new("prog")
        .arg(
            Arg::new("cfg")
                .action(ArgAction::Set)
                .require_equals(true)
                .num_args(0..)
                .long("config"),
        )
        .arg(Arg::new("cmd"))
        .try_get_matches_from(vec!["prog", "--config", "cmd"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.contains_id("cfg"));
    assert_eq!(m.get_one::<String>("cmd").map(|v| v.as_str()), Some("cmd"));
}

#[test]
fn double_hyphen_as_value() {
    let res = Command::new("prog")
        .arg(
            Arg::new("cfg")
                .action(ArgAction::Set)
                .allow_hyphen_values(true)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config", "--"]);
    assert!(res.is_ok(), "{:?}", res);
    assert_eq!(
        res.unwrap().get_one::<String>("cfg").map(|v| v.as_str()),
        Some("--")
    );
}

#[test]
fn require_equals_no_empty_values_fail() {
    let res = Command::new("prog")
        .arg(
            Arg::new("cfg")
                .action(ArgAction::Set)
                .require_equals(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .long("config"),
        )
        .arg(Arg::new("some"))
        .try_get_matches_from(vec!["prog", "--config=", "file.conf"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn require_equals_empty_vals_pass() {
    let res = Command::new("prog")
        .arg(
            Arg::new("cfg")
                .action(ArgAction::Set)
                .require_equals(true)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config="]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn require_equals_pass() {
    let res = Command::new("prog")
        .arg(
            Arg::new("cfg")
                .action(ArgAction::Set)
                .require_equals(true)
                .long("config"),
        )
        .try_get_matches_from(vec!["prog", "--config=file.conf"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn stdin_char() {
    let r = Command::new("opts")
        .arg(arg!(f: -f [flag] "some flag"))
        .try_get_matches_from(vec!["", "-f", "-"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("f"));
    assert_eq!(m.get_one::<String>("f").map(|v| v.as_str()).unwrap(), "-");
}

#[test]
fn opts_using_short() {
    let r = Command::new("opts")
        .args([
            arg!(f: -f [flag] "some flag"),
            arg!(c: -c [color] "some other flag"),
        ])
        .try_get_matches_from(vec!["", "-f", "some", "-c", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("f"));
    assert_eq!(
        m.get_one::<String>("f").map(|v| v.as_str()).unwrap(),
        "some"
    );
    assert!(m.contains_id("c"));
    assert_eq!(
        m.get_one::<String>("c").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn lots_o_vals() {
    let r = Command::new("opts")
        .arg(arg!(o: -o <opt> "some opt").num_args(1..).required(true))
        .try_get_matches_from(vec![
            "", "-o", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some",
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(m.get_many::<String>("o").unwrap().count(), 297); // i.e. more than u8
}

#[test]
fn opts_using_long_space() {
    let r = Command::new("opts")
        .args([
            arg!(--flag [flag] "some flag"),
            arg!(--color [color] "some other flag"),
        ])
        .try_get_matches_from(vec!["", "--flag", "some", "--color", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()).unwrap(),
        "some"
    );
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn opts_using_long_equals() {
    let r = Command::new("opts")
        .args([
            arg!(--flag [flag] "some flag"),
            arg!(--color [color] "some other flag"),
        ])
        .try_get_matches_from(vec!["", "--flag=some", "--color=other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()).unwrap(),
        "some"
    );
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn opts_using_mixed() {
    let r = Command::new("opts")
        .args([
            arg!(-f --flag [flag] "some flag"),
            arg!(-c --color [color] "some other flag"),
        ])
        .try_get_matches_from(vec!["", "-f", "some", "--color", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()).unwrap(),
        "some"
    );
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn opts_using_mixed2() {
    let r = Command::new("opts")
        .args([
            arg!(-f --flag [flag] "some flag"),
            arg!(-c --color [color] "some other flag"),
        ])
        .try_get_matches_from(vec!["", "--flag=some", "-c", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("flag"));
    assert_eq!(
        m.get_one::<String>("flag").map(|v| v.as_str()).unwrap(),
        "some"
    );
    assert!(m.contains_id("color"));
    assert_eq!(
        m.get_one::<String>("color").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn default_values_user_value() {
    let r = Command::new("df")
        .arg(arg!(o: -o [opt] "some opt").default_value("default"))
        .try_get_matches_from(vec!["", "-o", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_one::<String>("o").map(|v| v.as_str()).unwrap(),
        "value"
    );
}

#[test]
fn multiple_vals_pos_arg_equals() {
    let r = Command::new("mvae")
        .arg(arg!(o: -o [opt] ... "some opt"))
        .arg(arg!([file] "some file"))
        .try_get_matches_from(vec!["", "-o=1", "some"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(m.get_one::<String>("o").map(|v| v.as_str()).unwrap(), "1");
    assert!(m.contains_id("file"));
    assert_eq!(
        m.get_one::<String>("file").map(|v| v.as_str()).unwrap(),
        "some"
    );
}

#[test]
fn require_delims_no_delim() {
    let r = Command::new("mvae")
        .arg(arg!(o: -o [opt] ... "some opt").value_delimiter(','))
        .arg(arg!([file] "some file"))
        .try_get_matches_from(vec!["mvae", "-o", "1", "2", "some"]);
    assert!(r.is_err());
    let err = r.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn require_delims() {
    let r = Command::new("mvae")
        .arg(
            arg!(o: -o <opt> "some opt")
                .value_delimiter(',')
                .required(true),
        )
        .arg(arg!([file] "some file"))
        .try_get_matches_from(vec!["", "-o", "1,2", "some"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_many::<String>("o")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["1", "2"]
    );
    assert!(m.contains_id("file"));
    assert_eq!(
        m.get_one::<String>("file").map(|v| v.as_str()).unwrap(),
        "some"
    );
}

#[test]
fn leading_hyphen_pass() {
    let r = Command::new("mvae")
        .arg(
            arg!(o: -o <opt> "some opt")
                .required(true)
                .num_args(1..)
                .allow_hyphen_values(true),
        )
        .try_get_matches_from(vec!["", "-o", "-2", "3"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_many::<String>("o")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["-2", "3"]
    );
}

#[test]
fn leading_hyphen_fail() {
    let r = Command::new("mvae")
        .arg(arg!(o: -o <opt> "some opt").required(true))
        .try_get_matches_from(vec!["", "-o", "-2"]);
    assert!(r.is_err());
    let m = r.unwrap_err();
    assert_eq!(m.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn leading_hyphen_with_flag_after() {
    let r = Command::new("mvae")
        .arg(
            arg!(o: -o <opt> "some opt")
                .required(true)
                .num_args(1..)
                .allow_hyphen_values(true),
        )
        .arg(arg!(f: -f "some flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-o", "-2", "-f"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_many::<String>("o")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["-2", "-f"]
    );
    assert!(!*m.get_one::<bool>("f").expect("defaulted by clap"));
}

#[test]
fn leading_hyphen_with_flag_before() {
    let r = Command::new("mvae")
        .arg(arg!(o: -o [opt] ... "some opt").allow_hyphen_values(true))
        .arg(arg!(f: -f "some flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["", "-f", "-o", "-2"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_many::<String>("o")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["-2"]
    );
    assert!(*m.get_one::<bool>("f").expect("defaulted by clap"));
}

#[test]
fn leading_hyphen_with_only_pos_follows() {
    let r = Command::new("mvae")
        .arg(
            arg!(o: -o [opt] ... "some opt")
                .action(ArgAction::Set)
                .allow_hyphen_values(true),
        )
        .arg(arg!([arg] "some arg"))
        .try_get_matches_from(vec!["", "-o", "-2", "--", "val"]);
    assert!(r.is_ok(), "{:?}", r);
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_many::<String>("o")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["-2"]
    );
    assert_eq!(m.get_one::<String>("arg").map(|v| v.as_str()), Some("val"));
}

#[test]
#[cfg(feature = "suggestions")]
#[cfg(feature = "error-context")]
fn did_you_mean() {
    static DYM: &str = "\
error: unexpected argument '--optio'

  note: argument '--option' exists

Usage: clap-test --option <opt>... [positional] [positional2] [positional3]...

For more information, try '--help'.
";

    utils::assert_output(utils::complex_app(), "clap-test --optio=foo", DYM, true);
}

#[test]
fn issue_1047_min_zero_vals_default_val() {
    let m = Command::new("foo")
        .arg(
            Arg::new("del")
                .short('d')
                .long("del")
                .action(ArgAction::Set)
                .require_equals(true)
                .num_args(0..)
                .default_missing_value("default"),
        )
        .try_get_matches_from(vec!["foo", "-d"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("del").map(|v| v.as_str()),
        Some("default")
    );
}

fn issue_1105_setup(argv: Vec<&'static str>) -> Result<ArgMatches, clap::Error> {
    Command::new("opts")
        .arg(arg!(-o --option <opt> "some option").required(true))
        .arg(arg!(--flag "some flag"))
        .try_get_matches_from(argv)
}

#[test]
fn issue_1105_empty_value_long_fail() {
    let r = issue_1105_setup(vec!["cmd", "--option", "--flag"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn issue_1105_empty_value_long_explicit() {
    let r = issue_1105_setup(vec!["cmd", "--option", ""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.get_one::<String>("option").map(|v| v.as_str()), Some(""));
}

#[test]
fn issue_1105_empty_value_long_equals() {
    let r = issue_1105_setup(vec!["cmd", "--option="]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.get_one::<String>("option").map(|v| v.as_str()), Some(""));
}

#[test]
fn issue_1105_empty_value_short_fail() {
    let r = issue_1105_setup(vec!["cmd", "-o", "--flag"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn issue_1105_empty_value_short_explicit() {
    let r = issue_1105_setup(vec!["cmd", "-o", ""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.get_one::<String>("option").map(|v| v.as_str()), Some(""));
}

#[test]
fn issue_1105_empty_value_short_equals() {
    let r = issue_1105_setup(vec!["cmd", "-o="]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.get_one::<String>("option").map(|v| v.as_str()), Some(""));
}

#[test]
fn issue_1105_empty_value_short_explicit_no_space() {
    let r = issue_1105_setup(vec!["cmd", "-o", ""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.get_one::<String>("option").map(|v| v.as_str()), Some(""));
}

#[test]
#[cfg(feature = "suggestions")]
#[cfg(feature = "error-context")]
fn issue_1073_suboptimal_flag_suggestion() {
    static DYM_ISSUE_1073: &str = "\
error: unexpected argument '--files-without-matches'

  note: argument '--files-without-match' exists

Usage: ripgrep-616 --files-without-match

For more information, try '--help'.
";

    let cmd = Command::new("ripgrep-616")
        .arg(
            Arg::new("files-with-matches")
                .long("files-with-matches")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files-without-match")
                .long("files-without-match")
                .action(ArgAction::SetTrue),
        );
    utils::assert_output(
        cmd,
        "ripgrep-616 --files-without-matches",
        DYM_ISSUE_1073,
        true,
    );
}

#[test]
fn short_non_ascii_no_space() {
    let matches = Command::new("cmd")
        .arg(arg!(opt: -'磨' <opt>).required(true))
        .try_get_matches_from(["test", "-磨VALUE"])
        .unwrap();

    assert_eq!(
        "VALUE",
        matches
            .get_one::<String>("opt")
            .map(|v| v.as_str())
            .unwrap()
    );
}

#[test]
fn short_eq_val_starts_with_eq() {
    let matches = Command::new("cmd")
        .arg(arg!(opt: -f <opt>).required(true))
        .try_get_matches_from(["test", "-f==value"])
        .unwrap();

    assert_eq!(
        "=value",
        matches
            .get_one::<String>("opt")
            .map(|v| v.as_str())
            .unwrap()
    );
}

#[test]
fn long_eq_val_starts_with_eq() {
    let matches = Command::new("cmd")
        .arg(arg!(opt: --foo <opt>).required(true))
        .try_get_matches_from(["test", "--foo==value"])
        .unwrap();

    assert_eq!(
        "=value",
        matches
            .get_one::<String>("opt")
            .map(|v| v.as_str())
            .unwrap()
    );
}

#[test]
fn issue_2022_get_flags_misuse() {
    let cmd = Command::new("test")
        .next_help_heading(Some("test"))
        .arg(Arg::new("a").long("a").default_value("32"));
    let matches = cmd.try_get_matches_from([""]).unwrap();
    assert!(matches.get_one::<String>("a").map(|v| v.as_str()).is_some())
}

#[test]
fn issue_2279() {
    let before_help_heading = Command::new("cmd")
        .arg(Arg::new("foo").short('f').default_value("bar"))
        .next_help_heading(Some("This causes default_value to be ignored"))
        .try_get_matches_from([""])
        .unwrap();

    assert_eq!(
        before_help_heading
            .get_one::<String>("foo")
            .map(|v| v.as_str()),
        Some("bar")
    );

    let after_help_heading = Command::new("cmd")
        .next_help_heading(Some("This causes default_value to be ignored"))
        .arg(Arg::new("foo").short('f').default_value("bar"))
        .try_get_matches_from([""])
        .unwrap();

    assert_eq!(
        after_help_heading
            .get_one::<String>("foo")
            .map(|v| v.as_str()),
        Some("bar")
    );
}

#[test]
fn infer_long_arg() {
    let cmd = Command::new("test")
        .infer_long_args(true)
        .arg(
            Arg::new("racetrack")
                .long("racetrack")
                .alias("autobahn")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("racecar").long("racecar").action(ArgAction::Set));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--racec=hello"])
        .unwrap();
    assert!(!*matches
        .get_one::<bool>("racetrack")
        .expect("defaulted by clap"));
    assert_eq!(
        matches.get_one::<String>("racecar").map(|v| v.as_str()),
        Some("hello")
    );

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--racet"])
        .unwrap();
    assert!(*matches
        .get_one::<bool>("racetrack")
        .expect("defaulted by clap"));
    assert_eq!(
        matches.get_one::<String>("racecar").map(|v| v.as_str()),
        None
    );

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--auto"])
        .unwrap();
    assert!(*matches
        .get_one::<bool>("racetrack")
        .expect("defaulted by clap"));
    assert_eq!(
        matches.get_one::<String>("racecar").map(|v| v.as_str()),
        None
    );

    let cmd = Command::new("test")
        .infer_long_args(true)
        .arg(Arg::new("arg").long("arg").action(ArgAction::SetTrue));

    let matches = cmd.clone().try_get_matches_from(["test", "--"]).unwrap();
    assert!(!*matches.get_one::<bool>("arg").expect("defaulted by clap"));

    let matches = cmd.clone().try_get_matches_from(["test", "--a"]).unwrap();
    assert!(*matches.get_one::<bool>("arg").expect("defaulted by clap"));
}
