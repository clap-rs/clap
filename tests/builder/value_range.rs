use crate::utils;
use clap::{arg, Arg, Command, ErrorKind};

#[test]
fn value_range_opt_missing() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .takes_values(0..)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "auto");
    assert_eq!(m.occurrences_of("color"), 0);
}

#[test]
fn value_range_opt_present_with_missing_value() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .takes_values(0..)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "always");
    assert_eq!(m.occurrences_of("color"), 1);
}

#[test]
fn value_range_opt_present_with_value() {
    let r = Command::new("df")
        .arg(
            Arg::new("color")
                .long("color")
                .default_value("auto")
                .takes_values(0..)
                .require_equals(true)
                .default_missing_value("always"),
        )
        .try_get_matches_from(vec!["", "--color=never"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "never");
    assert_eq!(m.occurrences_of("color"), 1);
}

#[test]
fn value_range_issue_1374() {
    let cmd = Command::new("MyApp").arg(
        Arg::new("input")
            .takes_value(true)
            .long("input")
            .overrides_with("input")
            .takes_values_total(0..)
            .multiple_occurrences(true),
    );
    let matches = cmd
        .clone()
        .try_get_matches_from(&["MyApp", "--input", "a", "b", "c", "--input", "d"])
        .unwrap();
    let vs = matches.values_of("input").unwrap();
    assert_eq!(vs.collect::<Vec<_>>(), vec!["a", "b", "c", "d"]);
    let matches = cmd
        .clone()
        .try_get_matches_from(&["MyApp", "--input", "a", "b", "--input", "c", "d"])
        .unwrap();
    let vs = matches.values_of("input").unwrap();
    assert_eq!(vs.collect::<Vec<_>>(), vec!["a", "b", "c", "d"]);
}

#[test]
fn value_range_option_min_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(3..)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_range_opt_min_less_multi_occ() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(3..)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooFewValues);
}

#[test]
fn value_range_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values(3..),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooFewValues);
}

#[test]
fn value_range_option_short_min_more_mult_occurs() {
    let res = Command::new("multiple_values")
        .arg(Arg::new("arg").required(true))
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(3..)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec![
            "", "pos", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());
    let m = res.unwrap();

    assert!(m.is_present("option"));
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("option"), 4);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
    assert_eq!(m.value_of("arg"), Some("pos"));
}

#[test]
fn option_short_min_more_single_occur() {
    let res = Command::new("multiple_values")
        .arg(Arg::new("arg").required(true))
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values(3..),
        )
        .try_get_matches_from(vec!["", "pos", "-o", "val1", "val2", "val3", "val4"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());
    let m = res.unwrap();

    assert!(m.is_present("option"));
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
    assert_eq!(m.value_of("arg"), Some("pos"));
}

#[test]
fn positional_min_exact() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").min_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_range_positional_min_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_values(3..),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooFewValues);
}

#[test]
fn value_range_positional_min_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_values(3..),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 4);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn value_range_positional_min_less_total() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_values_total(3..),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooFewValues);
}

#[test]
fn value_range_positional_min_more_total() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_values_total(3..),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 4);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn value_range_require_equals_min_values_zero() {
    let res = Command::new("prog")
        .arg(
            Arg::new("cfg")
                .takes_values(0..)
                .require_equals(true)
                .long("config"),
        )
        .arg(Arg::new("cmd"))
        .try_get_matches_from(vec!["prog", "--config", "cmd"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("cfg"));
    assert_eq!(m.value_of("cmd"), Some("cmd"));
}

#[test]
fn value_range_issue_1047_min_zero_vals_default_val() {
    let m = Command::new("foo")
        .arg(
            Arg::new("del")
                .short('d')
                .long("del")
                .takes_values(0..)
                .require_equals(true)
                .default_missing_value("default"),
        )
        .try_get_matches_from(vec!["foo", "-d"])
        .unwrap();
    assert_eq!(m.occurrences_of("del"), 1);
    assert_eq!(m.value_of("del"), Some("default"));
}

#[test]
fn issue_1374_overrides_self_with_multiple_values() {
    let cmd = Command::new("test").arg(
        Arg::new("input")
            .long("input")
            .takes_values(0..)
            .overrides_with("input"),
    );
    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "--input", "a", "b", "c", "--input", "d"])
        .unwrap();
    assert_eq!(m.values_of("input").unwrap().collect::<Vec<_>>(), &["d"]);
    let m = cmd
        .clone()
        .try_get_matches_from(&["test", "--input", "a", "b", "--input", "c", "d"])
        .unwrap();
    assert_eq!(
        m.values_of("input").unwrap().collect::<Vec<_>>(),
        &["c", "d"]
    );
}

#[test]
fn value_range_required_unless_present_with_optional_value() {
    let res = Command::new("unlesstest")
        .arg(Arg::new("opt").long("opt").takes_values(0..=1))
        .arg(
            Arg::new("cfg")
                .required_unless_present("dbg")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .try_get_matches_from(vec!["unlesstest", "--opt"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn value_range_short_flag_require_equals_with_minvals_zero() {
    let m = Command::new("foo")
        .arg(
            Arg::new("check")
                .short('c')
                .takes_values(0..)
                .require_equals(true),
        )
        .arg(Arg::new("unique").short('u'))
        .try_get_matches_from(&["foo", "-cu"])
        .unwrap();
    assert!(m.is_present("check"));
    assert!(m.is_present("unique"));
}

#[test]
fn value_range_issue_2624() {
    let matches = Command::new("foo")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .require_equals(true)
                .takes_values(0..)
                .possible_values(["silent", "quiet", "diagnose-first"]),
        )
        .arg(Arg::new("unique").short('u').long("unique"))
        .try_get_matches_from(&["foo", "-cu"])
        .unwrap();
    assert!(matches.is_present("check"));
    assert!(matches.is_present("unique"));
}

#[test]
fn value_range_option_max_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(..=3)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_range_option_max_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(..=3)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 2);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}

#[test]
fn value_range_option_max_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(..=3)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyValues);
}

#[test]
fn value_range_positional_max_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_values(..=3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_range_positional_max_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_values(..=3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 2);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}

#[test]
fn value_range_positional_max_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_values(..=3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyValues);
}

#[test]
fn value_range_issue_1480_max_values_consumes_extra_arg_1() {
    let res = Command::new("prog")
        .arg(Arg::new("field").takes_values(..=1).long("field"))
        .arg(Arg::new("positional").required(true).index(1))
        .try_get_matches_from(vec!["prog", "--field", "1", "file"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn value_range_issue_1480_max_values_consumes_extra_arg_2() {
    let res = Command::new("prog")
        .arg(Arg::new("field").takes_values(..=1).long("field"))
        .try_get_matches_from(vec!["prog", "--field", "1", "2"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn value_range_issue_1480_max_values_consumes_extra_arg_3() {
    let res = Command::new("prog")
        .arg(Arg::new("field").takes_values(..=1).long("field"))
        .try_get_matches_from(vec!["prog", "--field", "1", "2", "3"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn value_range_aaos_opts_mult() {
    // opts with multiple
    let res = Command::new("posix")
        .args_override_self(true)
        .arg(
            arg!(--opt <val> ... "some option")
                .takes_values_total(4)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 3);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["some", "other", "one", "two"]
    );
}

#[test]
fn value_range_multiple_defaults() {
    let r = Command::new("diff")
        .arg(
            Arg::new("files")
                .long("files")
                .takes_values(2)
                .allow_invalid_utf8(true)
                .default_values(&["old", "new"]),
        )
        .try_get_matches_from(vec![""]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("files"));
    assert_eq!(m.values_of_lossy("files").unwrap(), vec!["old", "new"]);
}

#[test]
fn multiple_defaults_override() {
    let r = Command::new("diff")
        .arg(
            Arg::new("files")
                .long("files")
                .takes_values(2)
                .allow_invalid_utf8(true)
                .default_values(&["old", "new"]),
        )
        .try_get_matches_from(vec!["", "--files", "other", "mine"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("files"));
    assert_eq!(m.values_of_lossy("files").unwrap(), vec!["other", "mine"]);
}

#[test]
fn value_range_issue_1050_num_vals_and_defaults() {
    let res = Command::new("hello")
        .arg(
            Arg::new("exit-code")
                .long("exit-code")
                .takes_value(true)
                .takes_values(1)
                .default_value("0"),
        )
        .try_get_matches_from(vec!["hello", "--exit-code=1"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert_eq!(m.value_of("exit-code"), Some("1"));
}

#[test]
fn value_range_issue_760() {
    // Using takes_values(1) with multiple_values(true) misaligns help message
    static ISSUE_760: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -o, --option <option>    tests options
    -O, --opt <opt>          tests options
    -V, --version            Print version information
";

    let cmd = Command::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("option")
                .help("tests options")
                .short('o')
                .long("option")
                .takes_values(1)
                .multiple_values(true),
        )
        .arg(
            Arg::new("opt")
                .help("tests options")
                .short('O')
                .long("opt")
                .takes_value(true),
        );
    utils::assert_output(cmd, "ctest --help", ISSUE_760, false);
}

#[test]
fn value_range_issue_1571() {
    let cmd = Command::new("hello").arg(
        Arg::new("name")
            .long("package")
            .short('p')
            .takes_values(1)
            .multiple_values(true),
    );
    utils::assert_output(
        cmd,
        "hello --help",
        "hello 

USAGE:
    hello [OPTIONS]

OPTIONS:
    -h, --help              Print help information
    -p, --package <name>    
",
        false,
    );
}

#[test]
fn value_range_issue_2508_number_of_values_with_single_value_name() {
    let cmd = Command::new("my_app")
        .arg(Arg::new("some_arg").long("some_arg").takes_values(2))
        .arg(
            Arg::new("some_arg_issue")
                .long("some_arg_issue")
                .takes_values(2)
                .value_name("ARG"),
        );
    utils::assert_output(
        cmd,
        "my_app --help",
        "my_app 

USAGE:
    my_app [OPTIONS]

OPTIONS:
    -h, --help                              Print help information
        --some_arg <some_arg> <some_arg>    
        --some_arg_issue <ARG> <ARG>        
",
        false,
    );
}

#[test]
fn value_range_option_exact_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(3)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_range_option_exact_exact_not_mult() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_range_option_exact_exact_mult() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values(3)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "val2", "val3", "-o", "val4", "val5", "val6",
        ]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 2);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4", "val5", "val6"]
    );
}

#[test]
fn value_range_option_exact_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(3)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn value_range_option_exact_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_values_total(3)
                .multiple_occurrences(true),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn value_range_positional_exact_exact() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").takes_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_range_positional_exact_less() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").takes_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn value_range_positional_exact_more() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").takes_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn value_range_issue_2229() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").takes_values(3))
        .try_get_matches_from(vec![
            "myprog", "val1", "val2", "val3", "val4", "val5", "val6",
        ]);

    assert!(m.is_err()); // This panics, because `m.is_err() == false`.
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn value_range_number_of_values_preferred_over_value_names() {
    let m = Command::new("test")
        .arg(
            Arg::new("pos")
                .long("pos")
                .takes_values(4)
                .value_names(&["who", "what", "why"]),
        )
        .try_get_matches_from(vec!["myprog", "--pos", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn value_range_values_per_occurrence_named() {
    let mut a = Command::new("test").arg(
        Arg::new("pos")
            .long("pos")
            .takes_values(2)
            .multiple_occurrences(true),
    );

    let m = a.try_get_matches_from_mut(vec!["myprog", "--pos", "val1", "val2"]);
    let m = match m {
        Ok(m) => m,
        Err(err) => panic!("{}", err),
    };
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2"]
    );

    let m = a.try_get_matches_from_mut(vec![
        "myprog", "--pos", "val1", "val2", "--pos", "val3", "val4",
    ]);
    let m = match m {
        Ok(m) => m,
        Err(err) => panic!("{}", err),
    };
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn value_range_leading_hyphen_with_only_pos_follows() {
    let r = Command::new("mvae")
        .arg(
            arg!(o: -o [opt] ... "some opt")
                .takes_values(1)
                .allow_hyphen_values(true),
        )
        .arg(arg!([arg] "some arg"))
        .try_get_matches_from(vec!["", "-o", "-2", "--", "val"]);
    assert!(r.is_ok(), "{:?}", r);
    let m = r.unwrap();
    assert!(m.is_present("o"));
    assert_eq!(m.values_of("o").unwrap().collect::<Vec<_>>(), &["-2"]);
    assert_eq!(m.value_of("arg"), Some("val"));
}

#[test]
fn value_range_mult_option_require_delim_overrides_itself() {
    let res = Command::new("posix")
        .arg(
            arg!(--opt <val> ... "some option")
                .required(false)
                .overrides_with("opt")
                .takes_values_total(4)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=some", "--opt=other", "--opt=one,two"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.occurrences_of("opt"), 3);
    assert_eq!(
        m.values_of("opt").unwrap().collect::<Vec<_>>(),
        &["some", "other", "one", "two"]
    );
}
