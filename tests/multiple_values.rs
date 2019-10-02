extern crate clap;

use clap::{App, Arg, ErrorKind};

#[test]
fn option_long() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .help("multiple options")
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec![
            "", "--option", "val1", "--option", "val2", "--option", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_short() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_mixed() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "--option", "val2", "--option", "val3", "-o", "val4",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 4);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn option_exact_exact() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_exact_exact_not_mult() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_exact_exact_mult() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .number_of_values(3),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "val2", "val3", "-o", "val4", "val5", "val6",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 2);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4", "val5", "val6"]
    );
}

#[test]
fn option_exact_less() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn option_exact_more() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .number_of_values(3),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn option_min_exact() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .min_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_min_less() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .min_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooFewValues);
}

#[test]
fn option_short_min_more_mult_occurs() {
    let res = App::new("multiple_values")
        .arg(Arg::with_name("arg").required(true))
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .min_values(3),
        )
        .try_get_matches_from(vec![
            "", "pos", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);
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
    let res = App::new("multiple_values")
        .arg(Arg::with_name("arg").required(true))
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .min_values(3),
        )
        .try_get_matches_from(vec!["", "pos", "-o", "val1", "val2", "val3", "val4"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);
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
fn option_max_exact() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .max_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_max_less() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .max_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 2);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}

#[test]
fn option_max_more() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple(true)
                .max_values(3),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooManyValues);
}

#[test]
fn positional() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .multiple(true),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_exact_exact() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_exact_less() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn positional_exact_more() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn positional_min_exact() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .min_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_min_less() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .min_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooFewValues);
}

#[test]
fn positional_min_more() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .min_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 4);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn positional_max_exact() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .max_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_max_less() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .max_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 2);
    assert_eq!(
        m.values_of("pos").unwrap().collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}

#[test]
fn positional_max_more() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("pos")
                .help("multiple positionals")
                .max_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooManyValues);
}

#[test]
fn sep_long_equals() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .use_delimiter(true)
                .help("multiple options")
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "--option=val1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_long_space() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .use_delimiter(true)
                .help("multiple options")
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "--option", "val1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_short_equals() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .use_delimiter(true)
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "-o=val1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_short_space() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .use_delimiter(true)
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_short_no_space() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .help("multiple options")
                .use_delimiter(true)
                .takes_value(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "-oval1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_positional() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .help("multiple options")
                .use_delimiter(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["", "val1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn different_sep() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .help("multiple options")
                .takes_value(true)
                .value_delimiter(";"),
        )
        .try_get_matches_from(vec!["", "--option=val1;val2;val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn different_sep_positional() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .help("multiple options")
                .value_delimiter(";"),
        )
        .try_get_matches_from(vec!["", "val1;val2;val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn no_sep() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .help("multiple options")
                .takes_value(true)
                .use_delimiter(false),
        )
        .try_get_matches_from(vec!["", "--option=val1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn no_sep_positional() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .help("multiple options")
                .use_delimiter(false),
        )
        .try_get_matches_from(vec!["", "val1,val2,val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn req_delimiter_long() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .multiple(true)
                .use_delimiter(true)
                .require_delimiter(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("args").multiple(true).index(1))
        .try_get_matches_from(vec!["", "--option", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.values_of("args").unwrap().collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_long_with_equal() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .multiple(true)
                .use_delimiter(true)
                .require_delimiter(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("args").multiple(true).index(1))
        .try_get_matches_from(vec!["", "--option=val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.values_of("args").unwrap().collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_space() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .multiple(true)
                .use_delimiter(true)
                .require_delimiter(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("args").multiple(true).index(1))
        .try_get_matches_from(vec!["", "-o", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.values_of("args").unwrap().collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_no_space() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .multiple(true)
                .use_delimiter(true)
                .require_delimiter(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("args").multiple(true).index(1))
        .try_get_matches_from(vec!["", "-oval1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.values_of("args").unwrap().collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_equal() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .short('o')
                .multiple(true)
                .use_delimiter(true)
                .require_delimiter(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("args").multiple(true).index(1))
        .try_get_matches_from(vec!["", "-o=val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.values_of("args").unwrap().collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_complex() {
    let m = App::new("multiple_values")
        .arg(
            Arg::with_name("option")
                .long("option")
                .short('o')
                .multiple(true)
                .use_delimiter(true)
                .require_delimiter(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("args").multiple(true).index(1))
        .try_get_matches_from(vec![
            "",
            "val1",
            "-oval2",
            "val3",
            "-o",
            "val4",
            "val5",
            "-o=val6",
            "val7",
            "--option=val8",
            "val9",
            "--option",
            "val10",
            "val11",
            "-oval12,val13",
            "val14",
            "-o",
            "val15,val16",
            "val17",
            "-o=val18,val19",
            "val20",
            "--option=val21,val22",
            "val23",
            "--option",
            "val24,val25",
            "val26",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 10);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        &[
            "val2", "val4", "val6", "val8", "val10", "val12", "val13", "val15", "val16", "val18",
            "val19", "val21", "val22", "val24", "val25",
        ]
    );
    assert_eq!(
        m.values_of("args").unwrap().collect::<Vec<_>>(),
        &[
            "val1", "val3", "val5", "val7", "val9", "val11", "val14", "val17", "val20", "val23",
            "val26",
        ]
    );
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "When using a positional argument with \
.multiple(true) that is *not the last* positional argument, the last \
positional argument (i.e the one with the highest index) *must* have \
.required(true) or .last(true) set.")]
fn low_index_positional_not_required() {
    let _ = App::new("lip")
        .arg(
            Arg::with_name("files")
                .index(1)
                .required(true)
                .multiple(true),
        )
        .arg(Arg::with_name("target").index(2))
        .try_get_matches_from(vec![""]);
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Only one positional argument with .multiple(true) \
set is allowed per command, unless the second one also has .last(true) set")]
fn low_index_positional_last_multiple_too() {
    let _ = App::new("lip")
        .arg(
            Arg::with_name("files")
                .index(1)
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("target")
                .index(2)
                .required(true)
                .multiple(true),
        )
        .try_get_matches_from(vec![""]);
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Only the last positional argument, or second to \
last positional argument may be set to .multiple(true)")]
fn low_index_positional_too_far_back() {
    let _ = App::new("lip")
        .arg(
            Arg::with_name("files")
                .index(1)
                .required(true)
                .multiple(true),
        )
        .arg(Arg::with_name("target").required(true).index(2))
        .arg(Arg::with_name("target2").required(true).index(3))
        .try_get_matches_from(vec![""]);
}

#[test]
fn low_index_positional() {
    let m = App::new("lip")
        .arg(
            Arg::with_name("files")
                .index(1)
                .required(true)
                .multiple(true),
        )
        .arg(Arg::with_name("target").index(2).required(true))
        .try_get_matches_from(vec!["lip", "file1", "file2", "file3", "target"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();

    assert!(m.is_present("files"));
    assert_eq!(m.occurrences_of("files"), 3);
    assert!(m.is_present("target"));
    assert_eq!(m.occurrences_of("target"), 1);
    assert_eq!(
        m.values_of("files").unwrap().collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert_eq!(m.value_of("target").unwrap(), "target");
}

#[test]
fn low_index_positional_in_subcmd() {
    let m = App::new("lip")
        .subcommand(
            App::new("test")
                .arg(
                    Arg::with_name("files")
                        .index(1)
                        .required(true)
                        .multiple(true),
                )
                .arg(Arg::with_name("target").index(2).required(true)),
        )
        .try_get_matches_from(vec!["lip", "test", "file1", "file2", "file3", "target"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();
    let sm = m.subcommand_matches("test").unwrap();

    assert!(sm.is_present("files"));
    assert_eq!(sm.occurrences_of("files"), 3);
    assert!(sm.is_present("target"));
    assert_eq!(sm.occurrences_of("target"), 1);
    assert_eq!(
        sm.values_of("files").unwrap().collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert_eq!(sm.value_of("target").unwrap(), "target");
}

#[test]
fn low_index_positional_with_option() {
    let m = App::new("lip")
        .arg(
            Arg::with_name("files")
                .required(true)
                .index(1)
                .multiple(true),
        )
        .arg(Arg::with_name("target").index(2).required(true))
        .arg(Arg::with_name("opt").long("option").takes_value(true))
        .try_get_matches_from(vec![
            "lip", "file1", "file2", "file3", "target", "--option", "test",
        ]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();

    assert!(m.is_present("files"));
    assert_eq!(m.occurrences_of("files"), 3);
    assert!(m.is_present("target"));
    assert_eq!(m.occurrences_of("target"), 1);
    assert_eq!(
        m.values_of("files").unwrap().collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert_eq!(m.value_of("target").unwrap(), "target");
    assert_eq!(m.value_of("opt").unwrap(), "test");
}

#[test]
fn low_index_positional_with_flag() {
    let m = App::new("lip")
        .arg(
            Arg::with_name("files")
                .index(1)
                .required(true)
                .multiple(true),
        )
        .arg(Arg::with_name("target").index(2).required(true))
        .arg(Arg::with_name("flg").long("flag"))
        .try_get_matches_from(vec!["lip", "file1", "file2", "file3", "target", "--flag"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();

    assert!(m.is_present("files"));
    assert_eq!(m.occurrences_of("files"), 3);
    assert!(m.is_present("target"));
    assert_eq!(m.occurrences_of("target"), 1);
    assert_eq!(
        m.values_of("files").unwrap().collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert_eq!(m.value_of("target").unwrap(), "target");
    assert!(m.is_present("flg"));
}

#[test]
fn multiple_value_terminator_option() {
    let m = App::new("lip")
        .arg(
            Arg::with_name("files")
                .short('f')
                .value_terminator(";")
                .multiple(true),
        )
        .arg(Arg::with_name("other"))
        .try_get_matches_from(vec!["lip", "-f", "val1", "val2", ";", "otherval"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();

    assert!(m.is_present("other"));
    assert_eq!(m.occurrences_of("other"), 1);
    assert!(m.is_present("files"));
    assert_eq!(
        m.values_of("files").unwrap().collect::<Vec<_>>(),
        ["val1", "val2"]
    );
    assert_eq!(m.value_of("other"), Some("otherval"));
}

#[test]
fn multiple_value_terminator_option_other_arg() {
    let m = App::new("lip")
        .arg(
            Arg::with_name("files")
                .short('f')
                .value_terminator(";")
                .multiple(true),
        )
        .arg(Arg::with_name("other"))
        .arg(Arg::with_name("flag").short('F'))
        .try_get_matches_from(vec!["lip", "-f", "val1", "val2", "-F", "otherval"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind);
    let m = m.unwrap();

    assert!(m.is_present("other"));
    assert!(m.is_present("files"));
    assert_eq!(
        m.values_of("files").unwrap().collect::<Vec<_>>(),
        ["val1", "val2"]
    );
    assert_eq!(m.value_of("other"), Some("otherval"));
    assert!(m.is_present("flag"));
}

#[test]
fn multiple_vals_with_hyphen() {
    let res = App::new("do")
        .arg(
            Arg::with_name("cmds")
                .multiple(true)
                .allow_hyphen_values(true)
                .value_terminator(";"),
        )
        .arg(Arg::with_name("location"))
        .try_get_matches_from(vec![
            "do",
            "find",
            "-type",
            "f",
            "-name",
            "special",
            ";",
            "/home/clap",
        ]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);

    let m = res.unwrap();
    let cmds: Vec<_> = m.values_of("cmds").unwrap().collect();
    assert_eq!(&cmds, &["find", "-type", "f", "-name", "special"]);
    assert_eq!(m.value_of("location"), Some("/home/clap"));
}
