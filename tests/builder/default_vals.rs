use std::ffi::OsStr;
use std::ffi::OsString;

use clap::builder::ArgPredicate;
#[cfg(feature = "error-context")]
use clap::error::ErrorKind;
use clap::{arg, value_parser, Arg, ArgAction, Command};

#[cfg(feature = "error-context")]
use super::utils;

#[test]
fn opts() {
    let r = Command::new("df")
        .arg(arg!(o: -o <opt> "some opt").default_value("default"))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.value_source("o").unwrap(),
        clap::parser::ValueSource::DefaultValue
    );
    assert_eq!(
        m.get_one::<String>("o").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn default_has_index() {
    let r = Command::new("df")
        .arg(arg!(o: -o <opt> "some opt").default_value("default"))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.index_of("o"), Some(1));
}

#[test]
#[cfg(feature = "error-context")]
fn opt_without_value_fail() {
    let r = Command::new("df")
        .arg(
            arg!(o: -o <opt> "some opt")
                .default_value("default")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(vec!["", "-o"]);
    assert!(r.is_err());
    let err = r.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidValue);
    assert!(err
        .to_string()
        .contains("a value is required for '-o <opt>' but none was supplied"));
}

#[test]
fn opt_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg").default_value("default"))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.value_source("opt").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()).unwrap(),
        "value"
    );
}

#[test]
fn positionals() {
    let r = Command::new("df")
        .arg(arg!([arg] "some opt").default_value("default"))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.value_source("arg").unwrap(),
        clap::parser::ValueSource::DefaultValue
    );
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn positional_user_override() {
    let r = Command::new("df")
        .arg(arg!([arg] "some arg").default_value("default"))
        .try_get_matches_from(vec!["", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.value_source("arg").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "value"
    );
}

// OsStr Default Values

#[test]
fn osstr_opts() {
    use std::ffi::OsStr;
    let expected = OsStr::new("default");

    let r = Command::new("df")
        .arg(arg!(o: -o <opt> "some opt").default_value(expected))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_one::<String>("o").map(|v| v.as_str()).unwrap(),
        expected
    );
}

#[test]
fn osstr_opt_user_override() {
    use std::ffi::OsStr;
    let default = OsStr::new("default");

    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg").default_value(default))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("opt"));
    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()).unwrap(),
        "value"
    );
}

#[test]
fn osstr_positionals() {
    use std::ffi::OsStr;
    let expected = OsStr::new("default");

    let r = Command::new("df")
        .arg(arg!([arg] "some opt").default_value(expected))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        expected
    );
}

#[test]
fn osstr_positional_user_override() {
    use std::ffi::OsStr;
    let default = OsStr::new("default");

    let r = Command::new("df")
        .arg(arg!([arg] "some arg").default_value(default))
        .try_get_matches_from(vec!["", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "value"
    );
}

// --- Default if arg is present

#[test]
fn default_if_arg_present_no_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg").required(true))
        .arg(arg!([arg] "some arg").default_value_if(
            "opt",
            ArgPredicate::IsPresent,
            Some("default"),
        ))
        .try_get_matches_from(vec!["", "--opt", "some"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn default_if_arg_present_no_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!([arg] "some arg").default_value_if(
            "opt",
            ArgPredicate::IsPresent,
            Some("default"),
        ))
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn default_if_arg_present_no_arg_with_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", ArgPredicate::IsPresent, Some("default")),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "first"
    );
}

#[test]
fn default_if_arg_present_with_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", ArgPredicate::IsPresent, Some("default")),
        )
        .try_get_matches_from(vec!["", "--opt", "some"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn default_if_arg_present_with_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", ArgPredicate::IsPresent, Some("default")),
        )
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn default_if_arg_present_no_arg_with_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", ArgPredicate::IsPresent, Some("default")),
        )
        .try_get_matches_from(vec!["", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

// Conditional Default Values

#[test]
fn default_if_arg_present_with_value_no_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!([arg] "some arg").default_value_if("opt", "value", Some("default")))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn default_if_arg_present_with_value_no_default_fail() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!([arg] "some arg").default_value_if("opt", "value", Some("default")))
        .try_get_matches_from(vec!["", "--opt", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.contains_id("arg"));
    assert!(m.get_one::<String>("arg").map(|v| v.as_str()).is_none());
}

#[test]
fn default_if_arg_present_with_value_no_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!([arg] "some arg").default_value_if("opt", "some", Some("default")))
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn default_if_arg_present_with_value_no_arg_with_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", "some", Some("default")),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "first"
    );
}

#[test]
fn default_if_arg_present_with_value_no_arg_with_default_fail() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", "some", Some("default")),
        )
        .try_get_matches_from(vec!["", "--opt", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "first"
    );
}

#[test]
fn default_if_arg_present_with_value_with_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", "some", Some("default")),
        )
        .try_get_matches_from(vec!["", "--opt", "some"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn default_if_arg_present_with_value_with_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", "some", Some("default")),
        )
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn default_if_arg_present_no_arg_with_value_with_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", "some", Some("default")),
        )
        .try_get_matches_from(vec!["", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn default_if_arg_present_no_arg_with_value_with_default_user_override_fail() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", "some", Some("default")),
        )
        .try_get_matches_from(vec!["", "--opt", "value", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

// Unsetting the default

#[test]
fn no_default_if_arg_present_with_value_no_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!([arg] "some arg").default_value_if("opt", "value", None))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.contains_id("arg"));
}

#[test]
fn no_default_if_arg_present_with_value_with_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("default")
                .default_value_if("opt", "value", None),
        )
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.contains_id("arg"));
    assert!(m.get_one::<String>("arg").map(|v| v.as_str()).is_none());
}

#[test]
fn no_default_if_arg_present_with_value_with_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("default")
                .default_value_if("opt", "value", None),
        )
        .try_get_matches_from(vec!["", "--opt", "value", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "other"
    );
}

#[test]
fn no_default_if_arg_present_no_arg_with_value_with_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("default")
                .default_value_if("opt", "value", None),
        )
        .try_get_matches_from(vec!["", "--opt", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

// Multiple conditions

#[test]
fn default_ifs_arg_present() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs([
                    ("opt", ArgPredicate::from("some"), Some("default")),
                    ("flag", ArgPredicate::IsPresent, Some("flg")),
                ]),
        )
        .try_get_matches_from(vec!["", "--flag"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "flg"
    );
}

#[test]
fn no_default_ifs_arg_present() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs([
                    ("opt", ArgPredicate::from("some"), Some("default")),
                    ("flag", ArgPredicate::IsPresent, None),
                ]),
        )
        .try_get_matches_from(vec!["", "--flag"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.contains_id("arg"));
    assert!(m.get_one::<String>("arg").map(|v| v.as_str()).is_none());
}

#[test]
fn default_ifs_arg_present_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs([
                    ("opt", ArgPredicate::from("some"), Some("default")),
                    ("flag", ArgPredicate::IsPresent, Some("flg")),
                ]),
        )
        .try_get_matches_from(vec!["", "--flag", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "value"
    );
}

#[test]
fn default_ifs_arg_present_order() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg"))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs([
                    ("opt", ArgPredicate::from("some"), Some("default")),
                    ("flag", ArgPredicate::IsPresent, Some("flg")),
                ]),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--flag"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn default_value_ifs_os() {
    let cmd = Command::new("my_cargo")
        .arg(
            Arg::new("flag")
                .long("flag")
                .value_parser(value_parser!(OsString))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("other")
                .long("other")
                .value_parser(value_parser!(OsString))
                .default_value_ifs([("flag", "标记2", OsStr::new("flag=标记2"))]),
        );
    let result = cmd.try_get_matches_from(["my_cargo", "--flag", "标记2"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert_eq!(
        m.get_one::<OsString>("flag").map(OsString::as_os_str),
        Some(OsStr::new("标记2"))
    );
    assert_eq!(
        m.get_one::<OsString>("other").map(OsString::as_os_str),
        Some(OsStr::new("flag=标记2")),
    );
}

// Interaction with requires

#[test]
fn conditional_reqs_pass() {
    let m = Command::new("Test cmd")
        .arg(
            Arg::new("target")
                .action(ArgAction::Set)
                .default_value("file")
                .long("target"),
        )
        .arg(
            Arg::new("input")
                .action(ArgAction::Set)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::new("output")
                .action(ArgAction::Set)
                .required_if_eq("target", "file")
                .long("output"),
        )
        .try_get_matches_from(vec!["test", "--input", "some", "--output", "other"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(
        m.get_one::<String>("output").map(|v| v.as_str()),
        Some("other")
    );
    assert_eq!(
        m.get_one::<String>("input").map(|v| v.as_str()),
        Some("some")
    );
}

#[test]
fn multiple_defaults() {
    let r = Command::new("diff")
        .arg(
            Arg::new("files")
                .long("files")
                .num_args(2)
                .default_values(["old", "new"]),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("files"));
    assert_eq!(
        m.get_many::<String>("files").unwrap().collect::<Vec<_>>(),
        vec!["old", "new"]
    );
}

#[test]
fn multiple_defaults_override() {
    let r = Command::new("diff")
        .arg(
            Arg::new("files")
                .long("files")
                .num_args(2)
                .default_values(["old", "new"]),
        )
        .try_get_matches_from(vec!["", "--files", "other", "mine"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("files"));
    assert_eq!(
        m.get_many::<String>("files").unwrap().collect::<Vec<_>>(),
        vec!["other", "mine"]
    );
}

#[test]
#[cfg(feature = "error-context")]
fn default_vals_donnot_show_in_smart_usage() {
    let cmd = Command::new("bug")
        .arg(
            Arg::new("foo")
                .long("config")
                .action(ArgAction::Set)
                .default_value("bar"),
        )
        .arg(Arg::new("input").required(true));

    utils::assert_output(
        cmd,
        "bug",
        "\
error: the following required arguments were not provided:
  <input>

Usage: bug <input>

For more information, try '--help'.
",
        true,
    );
}

#[test]
fn issue_1050_num_vals_and_defaults() {
    let res = Command::new("hello")
        .arg(
            Arg::new("exit-code")
                .long("exit-code")
                .action(ArgAction::Set)
                .num_args(1)
                .default_value("0"),
        )
        .try_get_matches_from(vec!["hello", "--exit-code=1"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert_eq!(
        m.get_one::<String>("exit-code").map(|v| v.as_str()),
        Some("1")
    );
}

#[test]
fn required_groups_with_default_values() {
    use clap::{Arg, ArgGroup, Command};

    let cmd = Command::new("test")
        .arg(Arg::new("arg").default_value("value"))
        .group(ArgGroup::new("group").args(["arg"]).required(true));

    let result = cmd.clone().try_get_matches_from(["test"]);
    assert!(result.is_err());

    let result = cmd.clone().try_get_matches_from(["test", "value"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.contains_id("arg"));
    assert!(m.contains_id("group"));
}

#[test]
fn required_args_with_default_values() {
    use clap::{Arg, Command};

    let cmd = Command::new("test").arg(Arg::new("arg").required(true).default_value("value"));

    let result = cmd.clone().try_get_matches_from(["test"]);
    assert!(result.is_err());

    let result = cmd.clone().try_get_matches_from(["test", "value"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.contains_id("arg"));
}

#[cfg(debug_assertions)]
#[test]
#[cfg(feature = "error-context")]
#[should_panic = "Argument `arg`'s default_value=\"value\" failed validation: error: invalid value 'value' for '[arg]'"]
fn default_values_are_possible_values() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(["one", "two"])
                .default_value("value"),
        )
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[cfg(feature = "error-context")]
#[should_panic = "Argument `arg`'s default_value=\"one\" failed validation: error: invalid value 'one' for '[arg]"]
fn invalid_default_values() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(clap::value_parser!(u32))
                .default_value("one"),
        )
        .try_get_matches();
}

#[test]
fn valid_delimited_default_values() {
    use clap::{Arg, Command};

    Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(clap::value_parser!(u32))
                .value_delimiter(',')
                .default_value("1,2,3"),
        )
        .debug_assert();
}

#[cfg(debug_assertions)]
#[test]
#[cfg(feature = "error-context")]
#[should_panic = "Argument `arg`'s default_value=\"one\" failed validation: error: invalid value 'one' for '[arg]"]
fn invalid_delimited_default_values() {
    use clap::{Arg, Command};

    Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(clap::value_parser!(u32))
                .value_delimiter(',')
                .default_value("one,two"),
        )
        .debug_assert();
}

#[test]
fn with_value_delimiter() {
    let cmd = Command::new("multiple_values").arg(
        Arg::new("option")
            .long("option")
            .help("multiple options")
            .value_delimiter(';')
            .default_value("first;second"),
    );

    let matches = cmd.try_get_matches_from(vec![""]).unwrap();

    assert_eq!(
        matches
            .get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["first", "second"]
    );
}

#[test]
fn missing_with_value_delimiter() {
    let cmd = Command::new("program").arg(
        Arg::new("option")
            .long("option")
            .value_delimiter(';')
            .num_args(0..=1)
            .default_missing_values(["value1;value2;value3", "value4;value5"]),
    );

    let matches = cmd
        .try_get_matches_from(vec!["program", "--option"])
        .unwrap();

    assert_eq!(
        matches
            .get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["value1", "value2", "value3", "value4", "value5"]
    );
}

#[test]
fn default_independent_of_trailing() {
    let cmd = Command::new("test")
        .dont_delimit_trailing_values(true)
        .arg(Arg::new("pos").required(true))
        .arg(
            Arg::new("flag")
                .long("flag")
                .default_value("one,two")
                .value_delimiter(','),
        );

    // Baseline behavior
    let m = cmd
        .clone()
        .try_get_matches_from(vec!["program", "here"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("pos").map(|v| v.as_str()).unwrap(),
        "here"
    );
    assert_eq!(
        m.get_many::<String>("flag")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["one", "two"]
    );

    // Trailing-values behavior should match the baseline
    let m = cmd
        .try_get_matches_from(vec!["program", "--", "here"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("pos").map(|v| v.as_str()).unwrap(),
        "here"
    );
    assert_eq!(
        m.get_many::<String>("flag")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["one", "two"]
    );
}
