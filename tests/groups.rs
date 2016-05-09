extern crate clap;
extern crate clap_test;

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
    let app = App::new("req_group")
        .args_from_usage("[base] 'Base commit'
                          -d, --delete 'Remove the base commit information'")
        .group(ArgGroup::with_name("base_or_delete")
            .args(&["base", "delete"])
            .required(true));

    clap_test::check_err_output(app, "clap-test",
"error: The following required arguments were not provided:
    <base|--delete>

USAGE:
    clap-test [FLAGS] <base|--delete> [ARGS]

For more information try --help", true);

}

#[test]
fn req_group_with_conflict_usage_string() {
    let app = App::new("req_group")
        .arg(Arg::from_usage("[base] 'Base commit'").conflicts_with("delete"))
        .arg(Arg::from_usage("-d, --delete 'Remove the base commit information'"))
        .group(ArgGroup::with_name("base_or_delete")
            .args(&["base", "delete"])
            .required(true));

    clap_test::check_err_output(app, "clap-test --delete base",
"error: The argument '--delete' cannot be used with 'base'

USAGE:
    clap-test <base|--delete>

For more information try --help", true);

}

#[test]
fn req_group_with_conflict_rev_usage_string() {
    let app = App::new("req_group")
        .arg(Arg::from_usage("[base] 'Base commit'").conflicts_with("delete"))
        .arg(Arg::from_usage("-d, --delete 'Remove the base commit information'"))
        .group(ArgGroup::with_name("base_or_delete")
            .args(&["base", "delete"])
            .required(true));

    clap_test::check_err_output(app, "clap-test --delete base",
"error: The argument '--delete' cannot be used with 'base'

USAGE:
    clap-test <base|--delete>

For more information try --help", true);

}
