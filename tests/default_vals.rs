extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, ErrorKind};

#[test]
fn opts() {
    let r = App::new("df")
        .arg(Arg::from("-o [opt] 'some opt'").default_value("default"))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "default");
}

#[test]
fn opt_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'").default_value("default"))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.value_of("opt").unwrap(), "value");
}

#[test]
fn positionals() {
    let r = App::new("df")
        .arg(Arg::from("[arg] 'some opt'").default_value("default"))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "default");
}

#[test]
fn positional_user_override() {
    let r = App::new("df")
        .arg(Arg::from("[arg] 'some arg'").default_value("default"))
        .try_get_matches_from(vec!["", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "value");
}

// OsStr Default Values

#[test]
fn osstr_opts() {
    use std::ffi::OsStr;
    let expected = OsStr::new("default");

    let r = App::new("df")
        .arg(Arg::from("-o [opt] 'some opt'").default_value_os(expected))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), expected);
}

#[test]
fn osstr_opt_user_override() {
    use std::ffi::OsStr;
    let default = OsStr::new("default");

    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'").default_value_os(default))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.value_of("opt").unwrap(), "value");
}

#[test]
fn osstr_positionals() {
    use std::ffi::OsStr;
    let expected = OsStr::new("default");

    let r = App::new("df")
        .arg(Arg::from("[arg] 'some opt'").default_value_os(expected))
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), expected);
}

#[test]
fn osstr_positional_user_override() {
    use std::ffi::OsStr;
    let default = OsStr::new("default");

    let r = App::new("df")
        .arg(Arg::from("[arg] 'some arg'").default_value_os(default))
        .try_get_matches_from(vec!["", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "value");
}

// --- Default if arg is present

#[test]
fn default_if_arg_present_no_default() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("[arg] 'some arg'").default_value_if("opt", None, "default"))
        .try_get_matches_from(vec!["", "--opt", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "default");
}

#[test]
fn default_if_arg_present_no_default_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("[arg] 'some arg'").default_value_if("opt", None, "default"))
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "other");
}

#[test]
fn default_if_arg_present_no_arg_with_default() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", None, "default"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "first");
}

#[test]
fn default_if_arg_present_with_default() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", None, "default"),
        )
        .try_get_matches_from(vec!["", "--opt", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "default");
}

#[test]
fn default_if_arg_present_with_default_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", None, "default"),
        )
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "other");
}

#[test]
fn default_if_arg_present_no_arg_with_default_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", None, "default"),
        )
        .try_get_matches_from(vec!["", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "other");
}

// Conditional Default Values

#[test]
fn default_if_arg_present_with_value_no_default() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("[arg] 'some arg'").default_value_if("opt", Some("value"), "default"))
        .try_get_matches_from(vec!["", "--opt", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "default");
}

#[test]
fn default_if_arg_present_with_value_no_default_fail() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("[arg] 'some arg'").default_value_if("opt", Some("value"), "default"))
        .try_get_matches_from(vec!["", "--opt", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(!m.is_present("arg"));
}

#[test]
fn default_if_arg_present_with_value_no_default_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("[arg] 'some arg'").default_value_if("opt", Some("some"), "default"))
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "other");
}

#[test]
fn default_if_arg_present_with_value_no_arg_with_default() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", Some("some"), "default"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "first");
}

#[test]
fn default_if_arg_present_with_value_no_arg_with_default_fail() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", Some("some"), "default"),
        )
        .try_get_matches_from(vec!["", "--opt", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "first");
}

#[test]
fn default_if_arg_present_with_value_with_default() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", Some("some"), "default"),
        )
        .try_get_matches_from(vec!["", "--opt", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "default");
}

#[test]
fn default_if_arg_present_with_value_with_default_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", Some("some"), "default"),
        )
        .try_get_matches_from(vec!["", "--opt", "some", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "other");
}

#[test]
fn default_if_arg_present_no_arg_with_value_with_default_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", Some("some"), "default"),
        )
        .try_get_matches_from(vec!["", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "other");
}

#[test]
fn default_if_arg_present_no_arg_with_value_with_default_user_override_fail() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_if("opt", Some("some"), "default"),
        )
        .try_get_matches_from(vec!["", "--opt", "value", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "other");
}

// Multiple conditions

#[test]
fn default_ifs_arg_present() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("--flag 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_ifs(&[("opt", Some("some"), "default"), ("flag", None, "flg")]),
        )
        .try_get_matches_from(vec!["", "--flag"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "flg");
}

#[test]
fn default_ifs_arg_present_user_override() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("--flag 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_ifs(&[("opt", Some("some"), "default"), ("flag", None, "flg")]),
        )
        .try_get_matches_from(vec!["", "--flag", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "value");
}

#[test]
fn default_ifs_arg_present_order() {
    let r = App::new("df")
        .arg(Arg::from("--opt [FILE] 'some arg'"))
        .arg(Arg::from("--flag 'some arg'"))
        .arg(
            Arg::from("[arg] 'some arg'")
                .default_value("first")
                .default_value_ifs(&[("opt", Some("some"), "default"), ("flag", None, "flg")]),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--flag"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "default");
}

#[test]
fn conditional_reqs_fail() {
    let m = App::new("Test app")
        .version("1.0")
        .author("F0x06")
        .about("Arg test")
        .arg(
            Arg::with_name("target")
                .takes_value(true)
                .default_value("file")
                .possible_values(&["file", "stdout"])
                .long("target"),
        )
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::with_name("output")
                .takes_value(true)
                .required_if("target", "file")
                .long("output"),
        )
        .try_get_matches_from(vec!["test", "--input", "some"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn conditional_reqs_pass() {
    let m = App::new("Test app")
        .version("1.0")
        .author("F0x06")
        .about("Arg test")
        .arg(
            Arg::with_name("target")
                .takes_value(true)
                .default_value("file")
                .possible_values(&["file", "stdout"])
                .long("target"),
        )
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::with_name("output")
                .takes_value(true)
                .required_if("target", "file")
                .long("output"),
        )
        .try_get_matches_from(vec!["test", "--input", "some", "--output", "other"]);

    assert!(m.is_ok());
    let m = m.unwrap();
    assert_eq!(m.value_of("output"), Some("other"));
    assert_eq!(m.value_of("input"), Some("some"));
}

#[test]
fn issue_1050_num_vals_and_defaults() {
    let res = App::new("hello")
        .arg(
            Arg::with_name("exit-code")
                .long("exit-code")
                .required(true)
                .takes_value(true)
                .number_of_values(1)
                .default_value("0"),
        )
        .try_get_matches_from(vec!["hello", "--exit-code=1"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert_eq!(m.value_of("exit-code"), Some("1"));
}
