use clap::{error::ErrorKind, Arg, ArgAction, Command};

#[test]
fn option_long() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .help("multiple options")
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "", "--option", "val1", "--option", "val2", "--option", "val3",
        ]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_short() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_mixed() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .short('o')
                .help("multiple options")
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "--option", "val2", "--option", "val3", "-o", "val4",
        ]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn option_exact_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .number_of_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_exact_exact_not_mult() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_exact_exact_mult() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .number_of_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "val2", "val3", "-o", "val4", "val5", "val6",
        ]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4", "val5", "val6"]
    );
}

#[test]
fn option_exact_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .number_of_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn option_exact_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .number_of_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn option_min_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .min_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_min_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .min_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooFewValues);
}

#[test]
fn option_short_min_more_mult_occurs() {
    let res = Command::new("multiple_values")
        .arg(Arg::new("arg").required(true))
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .min_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "", "pos", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());
    let m = res.unwrap();

    assert!(m.contains_id("option"));
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
    assert_eq!(m.get_one::<String>("arg").map(|v| v.as_str()), Some("pos"));
}

#[test]
fn option_short_min_more_single_occur() {
    let res = Command::new("multiple_values")
        .arg(Arg::new("arg").required(true))
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .min_values(3),
        )
        .try_get_matches_from(vec!["", "pos", "-o", "val1", "val2", "val3", "val4"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());
    let m = res.unwrap();

    assert!(m.contains_id("option"));
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
    assert_eq!(m.get_one::<String>("arg").map(|v| v.as_str()), Some("pos"));
}

#[test]
fn option_max_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .max_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2", "-o", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn option_max_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .max_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}

#[test]
fn option_max_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .max_values(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyValues);
}

#[test]
fn positional() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("pos"));
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_exact_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("pos"));
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_exact_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn positional_exact_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .number_of_values(3),
        )
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn positional_min_exact() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").min_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("pos"));
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_min_less() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").min_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooFewValues);
}

#[test]
fn positional_min_more() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").min_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("pos"));
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn positional_max_exact() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").max_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("pos"));
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn positional_max_less() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").max_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("pos"));
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}

#[test]
fn positional_max_more() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").max_values(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyValues);
}

#[test]
fn sep_long_equals() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .help("multiple options")
                .use_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--option=val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_long_space() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .help("multiple options")
                .use_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--option", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_short_equals() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .use_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "-o=val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_short_space() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .use_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_short_no_space() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .use_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "-oval1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn sep_positional() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .help("multiple options")
                .use_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn different_sep() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .help("multiple options")
                .value_delimiter(';'),
        )
        .try_get_matches_from(vec!["", "--option=val1;val2;val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn different_sep_positional() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .help("multiple options")
                .value_delimiter(';'),
        )
        .try_get_matches_from(vec!["", "val1;val2;val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn no_sep() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .help("multiple options")
                .takes_value(true)
                .use_value_delimiter(false),
        )
        .try_get_matches_from(vec!["", "--option=val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()).unwrap(),
        "val1,val2,val3"
    );
}

#[test]
fn no_sep_positional() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .help("multiple options")
                .takes_value(true)
                .use_value_delimiter(false),
        )
        .try_get_matches_from(vec!["", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()).unwrap(),
        "val1,val2,val3"
    );
}

#[test]
fn req_delimiter_long() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .multiple_values(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .arg(
            Arg::new("args")
                .takes_value(true)
                .multiple_values(true)
                .index(1),
        )
        .try_get_matches_from(vec!["", "--option", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_long_with_equal() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .multiple_values(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .arg(
            Arg::new("args")
                .takes_value(true)
                .multiple_values(true)
                .index(1),
        )
        .try_get_matches_from(vec!["", "--option=val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_space() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .multiple_values(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .arg(
            Arg::new("args")
                .takes_value(true)
                .multiple_values(true)
                .index(1),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_no_space() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .multiple_values(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .arg(
            Arg::new("args")
                .takes_value(true)
                .multiple_values(true)
                .index(1),
        )
        .try_get_matches_from(vec!["", "-oval1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_equal() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .multiple_values(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .arg(
            Arg::new("args")
                .takes_value(true)
                .multiple_values(true)
                .index(1),
        )
        .try_get_matches_from(vec!["", "-o=val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val2", "val3"]
    );
}

#[test]
fn req_delimiter_complex() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .short('o')
                .multiple_values(true)
                .action(ArgAction::Append)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
        )
        .arg(
            Arg::new("args")
                .takes_value(true)
                .multiple_values(true)
                .index(1),
        )
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

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &[
            "val2", "val4", "val6", "val8", "val10", "val12", "val13", "val15", "val16", "val18",
            "val19", "val21", "val22", "val24", "val25",
        ]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &[
            "val1", "val3", "val5", "val7", "val9", "val11", "val14", "val17", "val20", "val23",
            "val26",
        ]
    );
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "When using a positional argument with \
.multiple_values(true) that is *not the last* positional argument, the last \
positional argument (i.e. the one with the highest index) *must* have \
.required(true) or .last(true) set."]
fn low_index_positional_not_required() {
    let _ = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .arg(Arg::new("target").index(2))
        .try_get_matches_from(vec![""]);
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Only one positional argument with .multiple_values(true) \
set is allowed per command, unless the second one also has .last(true) set"]
fn low_index_positional_last_multiple_too() {
    let _ = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .arg(
            Arg::new("target")
                .index(2)
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec![""]);
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Only the last positional argument, or second to \
last positional argument may be set to .multiple_values(true)"]
fn low_index_positional_too_far_back() {
    let _ = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .arg(Arg::new("target").required(true).index(2))
        .arg(Arg::new("target2").required(true).index(3))
        .try_get_matches_from(vec![""]);
}

#[test]
fn low_index_positional() {
    let m = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .arg(Arg::new("target").index(2).required(true))
        .try_get_matches_from(vec!["lip", "file1", "file2", "file3", "target"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert!(m.contains_id("files"));
    assert_eq!(
        m.get_many::<String>("files")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert!(m.contains_id("target"));
    assert_eq!(
        m.get_one::<String>("target").map(|v| v.as_str()).unwrap(),
        "target"
    );
}

#[test]
fn low_index_positional_in_subcmd() {
    let m = Command::new("lip")
        .subcommand(
            Command::new("test")
                .arg(
                    Arg::new("files")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .multiple_values(true),
                )
                .arg(Arg::new("target").index(2).required(true)),
        )
        .try_get_matches_from(vec!["lip", "test", "file1", "file2", "file3", "target"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();
    let sm = m.subcommand_matches("test").unwrap();

    assert!(sm.contains_id("files"));
    assert_eq!(
        sm.get_many::<String>("files")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert!(sm.contains_id("target"));
    assert_eq!(
        sm.get_one::<String>("target").map(|v| v.as_str()).unwrap(),
        "target"
    );
}

#[test]
fn low_index_positional_with_option() {
    let m = Command::new("lip")
        .arg(
            Arg::new("files")
                .required(true)
                .index(1)
                .takes_value(true)
                .multiple_values(true),
        )
        .arg(Arg::new("target").index(2).required(true))
        .arg(Arg::new("opt").long("option").takes_value(true))
        .try_get_matches_from(vec![
            "lip", "file1", "file2", "file3", "target", "--option", "test",
        ]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert!(m.contains_id("files"));
    assert_eq!(
        m.get_many::<String>("files")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert!(m.contains_id("target"));
    assert_eq!(
        m.get_one::<String>("target").map(|v| v.as_str()).unwrap(),
        "target"
    );
    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()).unwrap(),
        "test"
    );
}

#[test]
fn low_index_positional_with_flag() {
    let m = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .arg(Arg::new("target").index(2).required(true))
        .arg(Arg::new("flg").long("flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["lip", "file1", "file2", "file3", "target", "--flag"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert!(m.contains_id("files"));
    assert_eq!(
        m.get_many::<String>("files")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["file1", "file2", "file3"]
    );
    assert!(m.contains_id("target"));
    assert_eq!(
        m.get_one::<String>("target").map(|v| v.as_str()).unwrap(),
        "target"
    );
    assert!(*m.get_one::<bool>("flg").expect("defaulted by clap"));
}

#[test]
fn multiple_value_terminator_option() {
    let m = Command::new("lip")
        .arg(
            Arg::new("files")
                .short('f')
                .value_terminator(";")
                .takes_value(true)
                .multiple_values(true),
        )
        .arg(Arg::new("other"))
        .try_get_matches_from(vec!["lip", "-f", "val1", "val2", ";", "otherval"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert!(m.contains_id("other"));
    assert!(m.contains_id("files"));
    assert_eq!(
        m.get_many::<String>("files")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2"]
    );
    assert_eq!(
        m.get_one::<String>("other").map(|v| v.as_str()),
        Some("otherval")
    );
}

#[test]
fn multiple_value_terminator_option_other_arg() {
    let m = Command::new("lip")
        .arg(
            Arg::new("files")
                .short('f')
                .value_terminator(";")
                .takes_value(true)
                .multiple_values(true),
        )
        .arg(Arg::new("other"))
        .arg(Arg::new("flag").short('F').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["lip", "-f", "val1", "val2", "-F", "otherval"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert!(m.contains_id("other"));
    assert!(m.contains_id("files"));
    assert_eq!(
        m.get_many::<String>("files")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2"]
    );
    assert_eq!(
        m.get_one::<String>("other").map(|v| v.as_str()),
        Some("otherval")
    );
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn multiple_vals_with_hyphen() {
    let res = Command::new("do")
        .arg(
            Arg::new("cmds")
                .takes_value(true)
                .multiple_values(true)
                .allow_hyphen_values(true)
                .value_terminator(";"),
        )
        .arg(Arg::new("location"))
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
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());

    let m = res.unwrap();
    let cmds: Vec<_> = m
        .get_many::<String>("cmds")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(&cmds, &["find", "-type", "f", "-name", "special"]);
    assert_eq!(
        m.get_one::<String>("location").map(|v| v.as_str()),
        Some("/home/clap")
    );
}

#[test]
fn issue_1480_max_values_consumes_extra_arg_1() {
    let res = Command::new("prog")
        .arg(Arg::new("field").max_values(1).long("field"))
        .arg(Arg::new("positional").required(true).index(1))
        .try_get_matches_from(vec!["prog", "--field", "1", "file"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn issue_1480_max_values_consumes_extra_arg_2() {
    let res = Command::new("prog")
        .arg(Arg::new("field").max_values(1).long("field"))
        .try_get_matches_from(vec!["prog", "--field", "1", "2"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn issue_1480_max_values_consumes_extra_arg_3() {
    let res = Command::new("prog")
        .arg(Arg::new("field").max_values(1).long("field"))
        .try_get_matches_from(vec!["prog", "--field", "1", "2", "3"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn issue_2229() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .number_of_values(3),
        )
        .try_get_matches_from(vec![
            "myprog", "val1", "val2", "val3", "val4", "val5", "val6",
        ]);

    assert!(m.is_err()); // This panics, because `m.is_err() == false`.
    assert_eq!(m.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
}

#[test]
fn value_names_building_num_vals() {
    let m = Command::new("test")
        .arg(
            Arg::new("pos")
                .long("pos")
                .value_names(&["who", "what", "why"]),
        )
        .try_get_matches_from(vec!["myprog", "--pos", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn value_names_building_num_vals_for_positional() {
    let m = Command::new("test")
        .arg(Arg::new("pos").value_names(&["who", "what", "why"]))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3"]
    );
}

#[test]
fn number_of_values_preferred_over_value_names() {
    let m = Command::new("test")
        .arg(
            Arg::new("pos")
                .long("pos")
                .number_of_values(4)
                .value_names(&["who", "what", "why"]),
        )
        .try_get_matches_from(vec!["myprog", "--pos", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn values_per_occurrence_named() {
    let mut a = Command::new("test").arg(
        Arg::new("pos")
            .long("pos")
            .number_of_values(2)
            .action(ArgAction::Append),
    );

    let m = a.try_get_matches_from_mut(vec!["myprog", "--pos", "val1", "val2"]);
    let m = match m {
        Ok(m) => m,
        Err(err) => panic!("{}", err),
    };
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
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
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2", "val3", "val4"]
    );
}

#[test]
fn values_per_occurrence_positional() {
    let mut a = Command::new("test").arg(
        Arg::new("pos")
            .number_of_values(2)
            .action(ArgAction::Append),
    );

    let m = a.try_get_matches_from_mut(vec!["myprog", "val1", "val2"]);
    let m = match m {
        Ok(m) => m,
        Err(err) => panic!("{}", err),
    };
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}
