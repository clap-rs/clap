use std::ffi::OsStr;
use std::ffi::OsString;

use super::utils;
use clap::{arg, error::ErrorKind, value_parser, Arg, Command};

#[test]
fn opts() {
    let r = Command::new("df")
        .arg(
            arg!(o: -o <opt> "some opt")
                .required(false)
                .default_value("default"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert_eq!(
        m.get_one::<String>("o").map(|v| v.as_str()).unwrap(),
        "default"
    );
}

#[test]
fn default_has_index() {
    let r = Command::new("df")
        .arg(
            arg!(o: -o <opt> "some opt")
                .required(false)
                .default_value("default"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.index_of("o"), Some(1));
}

#[test]
fn opt_without_value_fail() {
    let r = Command::new("df")
        .arg(
            arg!(o: -o <opt> "some opt")
                .required(false)
                .default_value("default")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .try_get_matches_from(vec!["", "-o"]);
    assert!(r.is_err());
    let err = r.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::EmptyValue);
    assert!(err
        .to_string()
        .contains("The argument '-o <opt>' requires a value but none was supplied"));
}

#[test]
fn opt_user_override() {
    let r = Command::new("df")
        .arg(
            arg!(--opt <FILE> "some arg")
                .required(false)
                .default_value("default"),
        )
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
fn positionals() {
    let r = Command::new("df")
        .arg(arg!([arg] "some opt").default_value("default"))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
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
        .arg(
            arg!(o: -o <opt> "some opt")
                .required(false)
                .default_value_os(expected),
        )
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
        .arg(
            arg!(--opt <FILE> "some arg")
                .required(false)
                .default_value_os(default),
        )
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
        .arg(arg!([arg] "some opt").default_value_os(expected))
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
        .arg(arg!([arg] "some arg").default_value_os(default))
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
        .arg(arg!([arg] "some arg").default_value_if("opt", None, Some("default")))
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!([arg] "some arg").default_value_if("opt", None, Some("default")))
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", None, Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", None, Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", None, Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", None, Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!([arg] "some arg").default_value_if("opt", Some("value"), Some("default")))
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!([arg] "some arg").default_value_if("opt", Some("value"), Some("default")))
        .try_get_matches_from(vec!["", "--opt", "other"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.contains_id("arg"));
    assert!(m.get_one::<String>("arg").map(|v| v.as_str()).is_none());
}

#[test]
fn default_if_arg_present_with_value_no_default_user_override() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!([arg] "some arg").default_value_if("opt", Some("some"), Some("default")))
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", Some("some"), Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", Some("some"), Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", Some("some"), Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", Some("some"), Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", Some("some"), Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_if("opt", Some("some"), Some("default")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!([arg] "some arg").default_value_if("opt", Some("value"), None))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.contains_id("arg"));
}

#[test]
fn no_default_if_arg_present_with_value_with_default() {
    let r = Command::new("df")
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("default")
                .default_value_if("opt", Some("value"), None),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("default")
                .default_value_if("opt", Some("value"), None),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(
            arg!([arg] "some arg")
                .default_value("default")
                .default_value_if("opt", Some("value"), None),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs(&[
                    ("opt", Some("some"), Some("default")),
                    ("flag", None, Some("flg")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs(&[("opt", Some("some"), Some("default")), ("flag", None, None)]),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs(&[
                    ("opt", Some("some"), Some("default")),
                    ("flag", None, Some("flg")),
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
        .arg(arg!(--opt <FILE> "some arg").required(false))
        .arg(arg!(--flag "some arg"))
        .arg(
            arg!([arg] "some arg")
                .default_value("first")
                .default_value_ifs(&[
                    ("opt", Some("some"), Some("default")),
                    ("flag", None, Some("flg")),
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
                .takes_value(true),
        )
        .arg(
            Arg::new("other")
                .long("other")
                .value_parser(value_parser!(OsString))
                .default_value_ifs_os(&[(
                    "flag",
                    Some(OsStr::new("标记2")),
                    Some(OsStr::new("flag=标记2")),
                )]),
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
                .takes_value(true)
                .default_value("file")
                .long("target"),
        )
        .arg(
            Arg::new("input")
                .takes_value(true)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::new("output")
                .takes_value(true)
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
                .number_of_values(2)
                .default_values(&["old", "new"]),
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
                .number_of_values(2)
                .default_values(&["old", "new"]),
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
fn default_vals_donnot_show_in_smart_usage() {
    let cmd = Command::new("bug")
        .arg(
            Arg::new("foo")
                .long("config")
                .takes_value(true)
                .default_value("bar"),
        )
        .arg(Arg::new("input").required(true));

    utils::assert_output(
        cmd,
        "bug",
        "error: The following required arguments were not provided:
    <input>

USAGE:
    bug [OPTIONS] <input>

For more information try --help
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
                .takes_value(true)
                .number_of_values(1)
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
        .group(ArgGroup::new("group").args(&["arg"]).required(true));

    let result = cmd.clone().try_get_matches_from(&["test"]);
    assert!(result.is_err());

    let result = cmd.clone().try_get_matches_from(&["test", "value"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.contains_id("arg"));
    assert!(m.contains_id("group"));
}

#[test]
fn required_args_with_default_values() {
    use clap::{Arg, Command};

    let cmd = Command::new("test").arg(Arg::new("arg").required(true).default_value("value"));

    let result = cmd.clone().try_get_matches_from(&["test"]);
    assert!(result.is_err());

    let result = cmd.clone().try_get_matches_from(&["test", "value"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.contains_id("arg"));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument `arg`'s default_value=\"value\" failed validation: error: \"value\" isn't a valid value for '<arg>'"]
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
#[should_panic = "Argument `arg`'s default_value=\"one\" failed validation: error: Invalid value \"one\" for '<arg>"]
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

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(clap::value_parser!(u32))
                .use_value_delimiter(true)
                .require_value_delimiter(true)
                .default_value("1,2,3"),
        )
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument `arg`'s default_value=\"one\" failed validation: error: Invalid value \"one\" for '<arg>"]
fn invalid_delimited_default_values() {
    use clap::{Arg, Command};

    let _ = Command::new("test")
        .arg(
            Arg::new("arg")
                .value_parser(clap::value_parser!(u32))
                .use_value_delimiter(true)
                .require_value_delimiter(true)
                .default_value("one,two"),
        )
        .try_get_matches();
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
            .default_missing_values(&["value1;value2;value3", "value4;value5"]),
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
