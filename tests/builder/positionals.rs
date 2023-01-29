use clap::{arg, error::ErrorKind, Arg, ArgAction, Command};

#[test]
fn only_pos_follow() {
    let r = Command::new("onlypos")
        .args([arg!(f: -f [flag] "some opt"), arg!([arg] "some arg")])
        .try_get_matches_from(vec!["", "--", "-f"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert!(!m.contains_id("f"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "-f"
    );
}

#[test]
fn issue_946() {
    let r = Command::new("compiletest")
        .arg(arg!(--exact    "filters match exactly").action(ArgAction::SetTrue))
        .arg(
            clap::Arg::new("filter")
                .index(1)
                .action(ArgAction::Set)
                .allow_hyphen_values(true)
                .help("filters to apply to output"),
        )
        .try_get_matches_from(vec!["compiletest", "--exact"]);
    assert!(r.is_ok(), "{:#?}", r);
    let matches = r.unwrap();

    assert!(*matches.get_one::<bool>("exact").expect("defaulted by clap"));
    assert!(matches
        .get_one::<String>("filter")
        .map(|v| v.as_str())
        .is_none());
}

#[test]
fn positional() {
    let r = Command::new("positional")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            Arg::new("positional").index(1),
        ])
        .try_get_matches_from(vec!["", "-f", "test"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.contains_id("positional"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert_eq!(
        m.get_one::<String>("positional")
            .map(|v| v.as_str())
            .unwrap(),
        "test"
    );

    let m = Command::new("positional")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            Arg::new("positional").index(1),
        ])
        .try_get_matches_from(vec!["", "test", "--flag"])
        .unwrap();
    assert!(m.contains_id("positional"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert_eq!(
        m.get_one::<String>("positional")
            .map(|v| v.as_str())
            .unwrap(),
        "test"
    );
}

#[test]
fn lots_o_vals() {
    let r = Command::new("opts")
        .arg(arg!(<opt>... "some pos"))
        .try_get_matches_from(vec![
            "", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
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
            "some",
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(m.get_many::<String>("opt").unwrap().count(), 297); // i.e. more than u8
}

#[test]
fn positional_multiple() {
    let r = Command::new("positional_multiple")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            Arg::new("positional")
                .index(1)
                .action(ArgAction::Set)
                .num_args(1..),
        ])
        .try_get_matches_from(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.contains_id("positional"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert_eq!(
        &*m.get_many::<String>("positional")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["test1", "test2", "test3"]
    );
}

#[test]
fn positional_multiple_3() {
    let r = Command::new("positional_multiple")
        .args([
            arg!(-f  --flag "some flag").action(ArgAction::SetTrue),
            Arg::new("positional")
                .index(1)
                .action(ArgAction::Set)
                .num_args(1..),
        ])
        .try_get_matches_from(vec!["", "test1", "test2", "test3", "--flag"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.contains_id("positional"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert_eq!(
        &*m.get_many::<String>("positional")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["test1", "test2", "test3"]
    );
}

#[test]
fn positional_multiple_2() {
    let result = Command::new("positional_multiple")
        .args([arg!(-f --flag "some flag"), Arg::new("positional").index(1)])
        .try_get_matches_from(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn positional_possible_values() {
    let r = Command::new("positional_possible_values")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            Arg::new("positional").index(1).value_parser(["test123"]),
        ])
        .try_get_matches_from(vec!["", "-f", "test123"]);
    assert!(r.is_ok(), "{r:#?}");
    let m = r.unwrap();
    assert!(m.contains_id("positional"));
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert_eq!(
        &*m.get_many::<String>("positional")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["test123"]
    );
}

#[test]
fn create_positional() {
    let _ = Command::new("test")
        .arg(Arg::new("test").index(1).help("testing testing"))
        .try_get_matches_from(vec![""])
        .unwrap();
}

#[test]
fn positional_hyphen_does_not_panic() {
    let _ = Command::new("test")
        .arg(Arg::new("dummy"))
        .try_get_matches_from(vec!["test", "-"])
        .unwrap();
}

#[test]
fn single_positional_usage_string() {
    let mut cmd = Command::new("test").arg(arg!([FILE] "some file"));
    crate::utils::assert_eq(cmd.render_usage().to_string(), "Usage: test [FILE]");
}

#[test]
fn single_positional_multiple_usage_string() {
    let mut cmd = Command::new("test").arg(arg!([FILE]... "some file"));
    crate::utils::assert_eq(cmd.render_usage().to_string(), "Usage: test [FILE]...");
}

#[test]
fn multiple_positional_usage_string() {
    let mut cmd = Command::new("test")
        .arg(arg!([FILE] "some file"))
        .arg(arg!([FILES]... "some file"));
    crate::utils::assert_eq(
        cmd.render_usage().to_string(),
        "\
Usage: test [FILE] [FILES]...",
    );
}

#[test]
fn multiple_positional_one_required_usage_string() {
    let mut cmd = Command::new("test")
        .arg(arg!(<FILE> "some file"))
        .arg(arg!([FILES]... "some file"));
    crate::utils::assert_eq(
        cmd.render_usage().to_string(),
        "Usage: test <FILE> [FILES]...",
    );
}

#[test]
fn single_positional_required_usage_string() {
    let mut cmd = Command::new("test").arg(arg!(<FILE> "some file"));
    crate::utils::assert_eq(cmd.render_usage().to_string(), "Usage: test <FILE>");
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Found non-required positional argument \
with a lower index than a required positional argument"]
fn missing_required() {
    let _ = Command::new("test")
        .arg(arg!([FILE1] "some file"))
        .arg(arg!(<FILE2> "some file"))
        .try_get_matches_from(vec![""]);
}

#[test]
fn missing_required_2() {
    let r = Command::new("test")
        .arg(arg!(<FILE1> "some file"))
        .arg(arg!(<FILE2> "some file"))
        .try_get_matches_from(vec!["test", "file"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn last_positional() {
    let r = Command::new("test")
        .arg(arg!(<TARGET> "some target"))
        .arg(arg!([CORPUS] "some corpus"))
        .arg(arg!([ARGS]... "some file").last(true))
        .try_get_matches_from(vec!["test", "tgt", "--", "arg"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(
        m.get_many::<String>("ARGS")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["arg"]
    );
}

#[test]
fn last_positional_no_double_dash() {
    let r = Command::new("test")
        .arg(arg!(<TARGET> "some target"))
        .arg(arg!([CORPUS] "some corpus"))
        .arg(arg!([ARGS]... "some file").last(true))
        .try_get_matches_from(vec!["test", "tgt", "crp", "arg"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn last_positional_second_to_last_mult() {
    let r = Command::new("test")
        .arg(arg!(<TARGET> "some target"))
        .arg(arg!([CORPUS]... "some corpus"))
        .arg(arg!([ARGS]... "some file").last(true))
        .try_get_matches_from(vec!["test", "tgt", "crp1", "crp2", "--", "arg"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument 'arg' is a positional argument and can't have short or long name versions"]
fn positional_arg_with_long() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(Arg::new("arg").index(1).long("arg"))
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument 'arg' is a positional argument and can't have short or long name versions"]
fn positional_arg_with_short() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(Arg::new("arg").index(1).short('a'))
        .try_get_matches();
}

#[test]
fn ignore_hyphen_values_on_last() {
    let cmd = clap::Command::new("foo")
        .arg(
            clap::Arg::new("cmd")
                .num_args(1..)
                .last(true)
                .allow_hyphen_values(true),
        )
        .arg(
            clap::Arg::new("name")
                .long("name")
                .short('n')
                .action(ArgAction::Set)
                .required(false),
        );

    let matches = cmd.try_get_matches_from(["test", "-n", "foo"]).unwrap();
    assert_eq!(
        matches.get_one::<String>("name").map(|v| v.as_str()),
        Some("foo")
    );
}
