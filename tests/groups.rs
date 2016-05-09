extern crate clap;
extern crate regex;

use std::str;

use regex::Regex;

use clap::{App, Arg, ArgGroup, ErrorKind};

#[test]
fn required_group_missing_arg() {
    let result = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color 'some other flag'")
        .group(ArgGroup::with_name("req")
            .args(&["flag", "color"])
            .required(true))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_single_value() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp")
            .args(&["flag", "color"]))
        .get_matches_from(vec!["", "-c", "blue"]);
    assert!(m.is_present("grp"));
    assert_eq!(m.value_of("grp").unwrap(), "blue");
}

#[test]
fn group_single_flag() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp")
            .args(&["flag", "color"]))
        .get_matches_from(vec!["", "-f"]);
    assert!(m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_empty() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp")
            .args(&["flag", "color"]))
        .get_matches_from(vec![""]);
    assert!(!m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_reqired_flags_empty() {
    let result = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color 'some option'")
        .group(ArgGroup::with_name("grp")
            .required(true)
            .args(&["flag", "color"]))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_multi_value_single_arg() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color]... 'some option'")
        .group(ArgGroup::with_name("grp")
            .args(&["flag", "color"]))
        .get_matches_from(vec!["", "-c", "blue", "red", "green"]);
    assert!(m.is_present("grp"));
    assert_eq!(&*m.values_of("grp").unwrap().collect::<Vec<_>>(), &["blue", "red", "green"]);
}

#[test]
fn empty_group() {
    let r = App::new("empty_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("vers")
            .required(true))
        .get_matches_from_safe(vec!["empty_prog"]);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn req_group_usage_string() {
    let re = Regex::new("\x1b[^m]*m").unwrap();
    let r = App::new("req_group")
        .args_from_usage("[base] 'Base commit'
                          -d, --delete 'Remove the base commit information'")
        .group(ArgGroup::with_name("base_or_delete")
            .args(&["base", "delete"])
            .required(true))
        .get_matches_from_safe(vec!["req_group"]);

    let mut w = vec![];
    let err = r.unwrap_err();
    err.write_to(&mut w).unwrap();
    let err_s = str::from_utf8(&w).unwrap();
    assert_eq!(re.replace_all(err_s, ""),
"error: The following required arguments were not provided:
    <base|--delete>

USAGE:
    req_group [FLAGS] <base|--delete> [ARGS]

For more information try --help")
}
