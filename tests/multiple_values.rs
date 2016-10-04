extern crate clap;

use clap::{App, Arg, ErrorKind, SubCommand};

#[test]
fn multiple_values_of_option_long() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "--option", "val1",
            "--option", "val2",
            "--option", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_with_subcmd() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .subcommand(SubCommand::with_name("foo"))
        .get_matches_from_safe(vec![
            "",
            "--option", "val1",
            "val2", "foo"
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2"]);
    assert_eq!(m.subcommand_name(), Some("foo"));
}

#[test]
fn multiple_values_of_option_short() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_option_mixed() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "--option", "val2",
            "--option", "val3",
            "-o", "val4",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 4);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3", "val4"]);
}

#[test]
fn multiple_values_of_option_exact_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_option_exact_exact_not_mult() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1", "val2", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_option_exact_exact_mult() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1", "val2", "val3",
            "-o", "val4", "val5", "val6",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 2);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3", "val4", "val5", "val6"]);
}

#[test]
fn multiple_values_of_option_exact_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn multiple_values_of_option_exact_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
            "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn multiple_values_of_option_min_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .min_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_option_min_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .min_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooFewValues);
}

#[test]
fn option_short_min_more_mult_occurs() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("arg")
            .required(true))
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .min_values(3))
        .get_matches_from_safe(vec![
            "",
            "pos",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
            "-o", "val4",
        ]);

    let m = m.map_err(|e| println!("failed to unwrap err with error kind {:?}", e.kind)).unwrap();

    assert!(m.is_present("option"));
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("option"), 4);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3", "val4"]);
    assert_eq!(m.value_of("arg"), Some("pos"));
}

#[test]
fn option_short_min_more_single_occur() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("arg")
            .required(true))
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .min_values(3))
        .get_matches_from_safe(vec![
            "",
            "pos",
            "-o", "val1",
            "val2",
            "val3",
            "val4",
        ]);

    let m = m.map_err(|e| println!("failed to unwrap err with error kind {:#?}", e)).unwrap();

    assert!(m.is_present("option"));
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3", "val4"]);
    assert_eq!(m.value_of("arg"), Some("pos"));
}

#[test]
fn multiple_values_of_option_max_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .max_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_option_max_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .max_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 2);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2"]);
}

#[test]
fn multiple_values_of_option_max_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .max_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
            "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooManyValues);
}

#[test]
fn multiple_values_of_positional() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .multiple(true))
        .get_matches_from_safe(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_positional_exact_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .number_of_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_positional_exact_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .number_of_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn multiple_values_of_positional_exact_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .number_of_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
}

#[test]
fn multiple_values_of_positional_min_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .min_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_positional_min_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .min_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooFewValues);
}

#[test]
fn multiple_values_of_positional_min_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .min_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 4);
    assert_eq!(m.values_of("pos").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3", "val4"]);
}

#[test]
fn multiple_values_of_positional_max_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .max_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_of_positional_max_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .max_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 2);
    assert_eq!(m.values_of("pos").unwrap().collect::<Vec<_>>(), ["val1", "val2"]);
}

#[test]
fn multiple_values_of_positional_max_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .max_values(3))
        .get_matches_from_safe(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::TooManyValues);
}

#[test]
fn multiple_values_sep_long_equals() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .use_delimiter(true)
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "--option=val1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_sep_long_space() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .use_delimiter(true)
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "--option",
            "val1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_sep_short_equals() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .use_delimiter(true)
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "-o=val1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_sep_short_space() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .use_delimiter(true)
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "-o",
            "val1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_sep_short_no_space() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .use_delimiter(true)
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "-oval1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_sep_positional() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .help("multiple options")
            .use_delimiter(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "val1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_different_sep() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .help("multiple options")
            .takes_value(true)
            .value_delimiter(";"))
        .get_matches_from_safe(vec![
            "",
            "--option=val1;val2;val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_different_sep_positional() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .help("multiple options")
            .value_delimiter(";"))
        .get_matches_from_safe(vec![
            "",
            "val1;val2;val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
}

#[test]
fn multiple_values_no_sep() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .help("multiple options")
            .takes_value(true)
            .use_delimiter(false))
        .get_matches_from_safe(vec![
            "",
            "--option=val1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn multiple_values_no_sep_positional() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .help("multiple options")
            .use_delimiter(false))
        .get_matches_from_safe(vec![
            "",
            "val1,val2,val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn multiple_values_req_delimiter_long() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .multiple(true)
            .use_delimiter(true)
            .require_delimiter(true)
            .takes_value(true))
        .arg(Arg::with_name("args")
            .multiple(true)
            .index(1))
        .get_matches_from_safe(vec![
            "",
            "--option", "val1", "val2", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), &["val1"]);
    assert_eq!(m.values_of("args").unwrap().collect::<Vec<_>>(), &["val2", "val3"]);
}

#[test]
fn multiple_values_req_delimiter_long_with_equal() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .multiple(true)
            .use_delimiter(true)
            .require_delimiter(true)
            .takes_value(true))
        .arg(Arg::with_name("args")
            .multiple(true)
            .index(1))
        .get_matches_from_safe(vec![
            "",
            "--option=val1", "val2", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), &["val1"]);
    assert_eq!(m.values_of("args").unwrap().collect::<Vec<_>>(), &["val2", "val3"]);
}

#[test]
fn multiple_values_req_delimiter_short_with_space() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .multiple(true)
            .use_delimiter(true)
            .require_delimiter(true)
            .takes_value(true))
        .arg(Arg::with_name("args")
            .multiple(true)
            .index(1))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1", "val2", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), &["val1"]);
    assert_eq!(m.values_of("args").unwrap().collect::<Vec<_>>(), &["val2", "val3"]);
}

#[test]
fn multiple_values_req_delimiter_short_with_no_space() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .multiple(true)
            .use_delimiter(true)
            .require_delimiter(true)
            .takes_value(true))
        .arg(Arg::with_name("args")
            .multiple(true)
            .index(1))
        .get_matches_from_safe(vec![
            "",
            "-oval1", "val2", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), &["val1"]);
    assert_eq!(m.values_of("args").unwrap().collect::<Vec<_>>(), &["val2", "val3"]);
}

#[test]
fn multiple_values_req_delimiter_short_with_equal() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("option")
            .multiple(true)
            .use_delimiter(true)
            .require_delimiter(true)
            .takes_value(true))
        .arg(Arg::with_name("args")
            .multiple(true)
            .index(1))
        .get_matches_from_safe(vec![
            "",
            "-o=val1", "val2", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(), &["val1"]);
    assert_eq!(m.values_of("args").unwrap().collect::<Vec<_>>(), &["val2", "val3"]);
}

#[test]
fn multiple_values_req_delimiter_complex() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .short("o")
            .multiple(true)
            .use_delimiter(true)
            .require_delimiter(true)
            .takes_value(true))
        .arg(Arg::with_name("args")
            .multiple(true)
            .index(1))
        .get_matches_from_safe(vec![
            "",
            "val1",
            "-oval2", "val3",
            "-o", "val4", "val5",
            "-o=val6", "val7",
            "--option=val8", "val9",
            "--option", "val10", "val11",
            "-oval12,val13", "val14",
            "-o", "val15,val16", "val17",
            "-o=val18,val19", "val20",
            "--option=val21,val22", "val23",
            "--option", "val24,val25", "val26"
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 10);
    assert_eq!(m.values_of("option").unwrap().collect::<Vec<_>>(),
        &["val2", "val4", "val6", "val8", "val10", "val12", "val13", "val15",
          "val16", "val18", "val19", "val21", "val22", "val24", "val25"]);
    assert_eq!(m.values_of("args").unwrap().collect::<Vec<_>>(),
        &["val1", "val3", "val5", "val7", "val9", "val11", "val14", "val17",
          "val20", "val23", "val26"]);
}
