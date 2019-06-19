extern crate clap;

use clap::{App, Arg, ErrorKind};

#[test]
fn only_pos_follow() {
    let r = App::new("onlypos")
        .args(&[
            Arg::from_usage("-f [flag] 'some opt'"),
            Arg::from_usage("[arg] 'some arg'"),
        ])
        .get_matches_from_safe(vec!["", "--", "-f"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.value_of("arg").unwrap(), "-f");
}

#[test]
fn issue_946() {
    let r = App::new("compiletest")
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .args_from_usage("--exact    'filters match exactly'")
        .arg(
            clap::Arg::with_name("filter")
                .index(1)
                .takes_value(true)
                .help("filters to apply to output"),
        )
        .get_matches_from_safe(vec!["compiletest", "--exact"]);
    assert!(r.is_ok(), "{:#?}", r);
    let matches = r.unwrap();

    assert!(matches.is_present("exact"));
    assert!(matches.value_of("filter").is_none());
}

#[test]
fn positional() {
    let r = App::new("positional")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional").index(1),
        ])
        .get_matches_from_safe(vec!["", "-f", "test"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");

    let m = App::new("positional")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional").index(1),
        ])
        .get_matches_from(vec!["", "test", "--flag"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");
}

#[test]
fn lots_o_vals() {
    let r = App::new("opts")
        .arg(Arg::from_usage("[opt]... 'some pos'"))
        .get_matches_from_safe(vec![
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
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>().len(), 297); // i.e. more than u8
}

#[test]
fn positional_multiple() {
    let r = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional").index(1).multiple(true),
        ])
        .get_matches_from_safe(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(
        &*m.values_of("positional").unwrap().collect::<Vec<_>>(),
        &["test1", "test2", "test3"]
    );
}

#[test]
fn positional_multiple_3() {
    let r = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional").index(1).multiple(true),
        ])
        .get_matches_from_safe(vec!["", "test1", "test2", "test3", "--flag"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(
        &*m.values_of("positional").unwrap().collect::<Vec<_>>(),
        &["test1", "test2", "test3"]
    );
}

#[test]
fn positional_multiple_2() {
    let result = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional").index(1),
        ])
        .get_matches_from_safe(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn positional_possible_values() {
    let r = App::new("positional_possible_values")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .possible_value("test123"),
        ])
        .get_matches_from_safe(vec!["", "-f", "test123"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(
        &*m.values_of("positional").unwrap().collect::<Vec<_>>(),
        &["test123"]
    );
}

#[test]
fn create_positional() {
    let _ = App::new("test")
        .arg(Arg::with_name("test").index(1).help("testing testing"))
        .get_matches_from(vec![""]);
}

#[test]
fn positional_hyphen_does_not_panic() {
    let _ = App::new("test")
        .arg(Arg::with_name("dummy"))
        .get_matches_from(vec!["test", "-"]);
}

#[test]
fn single_positional_usage_string() {
    let m = App::new("test")
        .arg_from_usage("[FILE] 'some file'")
        .get_matches_from(vec!["test"]);
    assert_eq!(m.usage(), "USAGE:\n    test [FILE]");
}

#[test]
fn single_positional_multiple_usage_string() {
    let m = App::new("test")
        .arg_from_usage("[FILE]... 'some file'")
        .get_matches_from(vec!["test"]);
    assert_eq!(m.usage(), "USAGE:\n    test [FILE]...");
}

#[test]
fn multiple_positional_usage_string() {
    let m = App::new("test")
        .arg_from_usage("[FILE] 'some file'")
        .arg_from_usage("[FILES]... 'some file'")
        .get_matches_from(vec!["test"]);
    assert_eq!(m.usage(), "USAGE:\n    test [ARGS]");
}

#[test]
fn multiple_positional_one_required_usage_string() {
    let m = App::new("test")
        .arg_from_usage("<FILE> 'some file'")
        .arg_from_usage("[FILES]... 'some file'")
        .get_matches_from(vec!["test", "file"]);
    assert_eq!(m.usage(), "USAGE:\n    test <FILE> [FILES]...");
}

#[test]
fn single_positional_required_usage_string() {
    let m = App::new("test")
        .arg_from_usage("<FILE> 'some file'")
        .get_matches_from(vec!["test", "file"]);
    assert_eq!(m.usage(), "USAGE:\n    test <FILE>");
}

#[test]
#[should_panic]
fn missing_required() {
    let r = App::new("test")
        .arg_from_usage("[FILE1] 'some file'")
        .arg_from_usage("<FILE2> 'some file'")
        .get_matches_from_safe(vec!["test", "file"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn missing_required_2() {
    let r = App::new("test")
        .arg_from_usage("<FILE1> 'some file'")
        .arg_from_usage("<FILE2> 'some file'")
        .get_matches_from_safe(vec!["test", "file"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn last_positional() {
    let r = App::new("test")
        .arg_from_usage("<TARGET> 'some target'")
        .arg_from_usage("[CORPUS] 'some corpus'")
        .arg(Arg::from_usage("[ARGS]... 'some file'").last(true))
        .get_matches_from_safe(vec!["test", "tgt", "--", "arg"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.values_of("ARGS").unwrap().collect::<Vec<_>>(), &["arg"]);
}

#[test]
fn last_positional_no_double_dash() {
    let r = App::new("test")
        .arg_from_usage("<TARGET> 'some target'")
        .arg_from_usage("[CORPUS] 'some corpus'")
        .arg(Arg::from_usage("[ARGS]... 'some file'").last(true))
        .get_matches_from_safe(vec!["test", "tgt", "crp", "arg"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn last_positional_second_to_last_mult() {
    let r = App::new("test")
        .arg_from_usage("<TARGET> 'some target'")
        .arg_from_usage("[CORPUS]... 'some corpus'")
        .arg(Arg::from_usage("[ARGS]... 'some file'").last(true))
        .get_matches_from_safe(vec!["test", "tgt", "crp1", "crp2", "--", "arg"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind);
}
