extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, ArgMatches, ErrorKind};

#[cfg(feature = "suggestions")]
static DYM: &'static str =
    "error: Found argument '--optio' which wasn't expected, or isn't valid in this context
\tDid you mean --option?

USAGE:
    clap-test --option <opt>...

For more information try --help";

#[test]
fn require_equals_fail() {
    let res = App::new("prog")
        .arg(
            Arg::with_name("cfg")
                .require_equals(true)
                .takes_value(true)
                .long("config"),
        )
        .get_matches_from_safe(vec!["prog", "--config", "file.conf"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn require_equals_min_values_zero() {
    let res = App::new("prog")
        .arg(
            Arg::with_name("cfg")
                .require_equals(true)
                .takes_value(true)
                .min_values(0)
                .long("config"),
        )
        .arg(Arg::with_name("cmd"))
        .get_matches_from_safe(vec!["prog", "--config", "cmd"]);
    assert!(res.is_ok());
    let m = res.unwrap();
    assert!(m.is_present("cfg"));
    assert_eq!(m.value_of("cmd"), Some("cmd"));
}

#[test]
fn double_hyphen_as_value() {
    let res = App::new("prog")
        .arg(
            Arg::with_name("cfg")
                .takes_value(true)
                .allow_hyphen_values(true)
                .long("config"),
        )
        .get_matches_from_safe(vec!["prog", "--config", "--"]);
    assert!(res.is_ok(), "{:?}", res);
    assert_eq!(res.unwrap().value_of("cfg"), Some("--"));
}

#[test]
fn require_equals_no_empty_values_fail() {
    let res = App::new("prog")
        .arg(
            Arg::with_name("cfg")
                .require_equals(true)
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::with_name("some"))
        .get_matches_from_safe(vec!["prog", "--config=", "file.conf"]);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn require_equals_empty_vals_pass() {
    let res = App::new("prog")
        .arg(
            Arg::with_name("cfg")
                .require_equals(true)
                .takes_value(true)
                .empty_values(true)
                .long("config"),
        )
        .get_matches_from_safe(vec!["prog", "--config="]);
    assert!(res.is_ok());
}

#[test]
fn require_equals_pass() {
    let res = App::new("prog")
        .arg(
            Arg::with_name("cfg")
                .require_equals(true)
                .takes_value(true)
                .long("config"),
        )
        .get_matches_from_safe(vec!["prog", "--config=file.conf"]);
    assert!(res.is_ok());
}

#[test]
fn stdin_char() {
    let r = App::new("opts")
        .arg(Arg::from_usage("-f [flag] 'some flag'"))
        .get_matches_from_safe(vec!["", "-f", "-"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "-");
}

#[test]
fn opts_using_short() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f [flag] 'some flag'"),
            Arg::from_usage("-c [color] 'some other flag'"),
        ])
        .get_matches_from_safe(vec!["", "-f", "some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "some");
    assert!(m.is_present("c"));
    assert_eq!(m.value_of("c").unwrap(), "other");
}

#[test]
fn lots_o_vals() {
    let r = App::new("opts")
        .arg(Arg::from_usage("-o [opt]... 'some opt'"))
        .get_matches_from_safe(vec![
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
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>().len(), 297); // i.e. more than u8
}

#[test]
fn opts_using_long_space() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'"),
        ])
        .get_matches_from_safe(vec!["", "--flag", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_equals() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'"),
        ])
        .get_matches_from_safe(vec!["", "--flag=some", "--color=other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'"),
        ])
        .get_matches_from_safe(vec!["", "-f", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed2() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'"),
        ])
        .get_matches_from_safe(vec!["", "--flag=some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn default_values_user_value() {
    let r = App::new("df")
        .arg(Arg::from_usage("-o [opt] 'some opt'").default_value("default"))
        .get_matches_from_safe(vec!["", "-o", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "value");
}

#[test]
fn multiple_vals_pos_arg_equals() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt]... 'some opt'"))
        .arg(Arg::from_usage("[file] 'some file'"))
        .get_matches_from_safe(vec!["", "-o=1", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.value_of("o").unwrap(), "1");
    assert!(m.is_present("file"));
    assert_eq!(m.value_of("file").unwrap(), "some");
}

#[test]
fn multiple_vals_pos_arg_delim() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt]... 'some opt'"))
        .arg(Arg::from_usage("[file] 'some file'"))
        .get_matches_from_safe(vec!["", "-o", "1,2", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["1", "2"]);
    assert!(m.is_present("file"));
    assert_eq!(m.value_of("file").unwrap(), "some");
}

#[test]
fn require_delims_no_delim() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt]... 'some opt'").require_delimiter(true))
        .arg(Arg::from_usage("[file] 'some file'"))
        .get_matches_from_safe(vec!["mvae", "-o", "1", "2", "some"]);
    assert!(r.is_err());
    let err = r.unwrap_err();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn require_delims() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt]... 'some opt'").require_delimiter(true))
        .arg(Arg::from_usage("[file] 'some file'"))
        .get_matches_from_safe(vec!["", "-o", "1,2", "some"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["1", "2"]);
    assert!(m.is_present("file"));
    assert_eq!(m.value_of("file").unwrap(), "some");
}

#[test]
fn leading_hyphen_pass() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt]... 'some opt'").allow_hyphen_values(true))
        .get_matches_from_safe(vec!["", "-o", "-2", "3"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2", "3"]);
}

#[test]
fn leading_hyphen_fail() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt] 'some opt'"))
        .get_matches_from_safe(vec!["", "-o", "-2"]);
    assert!(r.is_err());
    let m = r.unwrap_err();
    assert_eq!(m.kind, ErrorKind::UnknownArgument);
}

#[test]
fn leading_hyphen_with_flag_after() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt]... 'some opt'").allow_hyphen_values(true))
        .arg_from_usage("-f 'some flag'")
        .get_matches_from_safe(vec!["", "-o", "-2", "-f"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2", "-f"]);
    assert!(!m.is_present("f"));
}

#[test]
fn leading_hyphen_with_flag_before() {
    let r = App::new("mvae")
        .arg(Arg::from_usage("-o [opt]... 'some opt'").allow_hyphen_values(true))
        .arg_from_usage("-f 'some flag'")
        .get_matches_from_safe(vec!["", "-f", "-o", "-2"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2"]);
    assert!(m.is_present("f"));
}

#[test]
fn leading_hyphen_with_only_pos_follows() {
    let r = App::new("mvae")
        .arg(
            Arg::from_usage("-o [opt]... 'some opt'")
                .number_of_values(1)
                .allow_hyphen_values(true),
        )
        .arg_from_usage("[arg] 'some arg'")
        .get_matches_from_safe(vec!["", "-o", "-2", "--", "val"]);
    assert!(r.is_ok(), "{:?}", r);
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2"]);
    assert_eq!(m.value_of("arg"), Some("val"));
}

#[test]
#[cfg(feature = "suggestions")]
fn did_you_mean() {
    assert!(test::compare_output(
        test::complex_app(),
        "clap-test --optio=foo",
        DYM,
        true
    ));
}

#[test]
fn issue_665() {
    let res = App::new("tester")
        .arg_from_usage("-v, --reroll-count=[N] 'Mark the patch series as PATCH vN'")
        .arg(Arg::from_usage(
"--subject-prefix [Subject-Prefix] 'Use [Subject-Prefix] instead of the standard [PATCH] prefix'")
            .empty_values(false))
        .get_matches_from_safe(vec!["test", "--subject-prefix", "-v", "2"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn issue_1047_min_zero_vals_default_val() {
    let m = App::new("foo")
        .arg(
            Arg::with_name("del")
                .short("d")
                .long("del")
                .takes_value(true)
                .require_equals(true)
                .min_values(0)
                .default_value("default"),
        )
        .get_matches_from(vec!["foo", "-d"]);
    assert_eq!(m.occurrences_of("del"), 1);
    assert_eq!(m.value_of("del"), Some("default"));
}

fn issue_1105_setup(argv: Vec<&'static str>) -> Result<ArgMatches<'static>, clap::Error> {
    App::new("opts")
        .arg_from_usage("-o, --option [opt] 'some option'")
        .arg_from_usage("--flag 'some flag'")
        .get_matches_from_safe(argv)
}

#[test]
fn issue_1105_empty_value_long_fail() {
    let r = issue_1105_setup(vec!["app", "--option", "--flag"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn issue_1105_empty_value_long_explicit() {
    let r = issue_1105_setup(vec!["app", "--option", ""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_long_equals() {
    let r = issue_1105_setup(vec!["app", "--option="]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_short_fail() {
    let r = issue_1105_setup(vec!["app", "-o", "--flag"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::EmptyValue);
}

#[test]
fn issue_1105_empty_value_short_explicit() {
    let r = issue_1105_setup(vec!["app", "-o", ""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_short_equals() {
    let r = issue_1105_setup(vec!["app", "-o="]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}

#[test]
fn issue_1105_empty_value_short_explicit_no_space() {
    let r = issue_1105_setup(vec!["app", "-o", ""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert_eq!(m.value_of("option"), Some(""));
}
