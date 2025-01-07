use clap::{error::ErrorKind, Arg, ArgAction, Command};
use snapbox::assert_data_eq;
use snapbox::str;

use crate::utils;

#[test]
fn option_long() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .help("multiple options")
                .action(ArgAction::Set)
                .num_args(1..)
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
                .action(ArgAction::Set)
                .num_args(1..)
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
                .action(ArgAction::Set)
                .num_args(1..)
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
                .num_args(3)
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
fn option_exact_exact_not_mult() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(3),
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
                .num_args(3)
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
#[cfg(feature = "error-context")]
fn option_exact_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "-o", "val2"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::WrongNumberOfValues, str![[r#"
error: 3 values required for '-o <option> <option> <option>' but 1 was provided

Usage: multiple_values [OPTIONS]

For more information, try '--help'.

"#]], true);
}

#[test]
#[cfg(feature = "error-context")]
fn option_exact_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(3)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "", "-o", "val1", "-o", "val2", "-o", "val3", "-o", "val4",
        ]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::WrongNumberOfValues, str![[r#"
error: 3 values required for '-o <option> <option> <option>' but 1 was provided

Usage: multiple_values [OPTIONS]

For more information, try '--help'.

"#]], true);
}

#[test]
fn option_min_exact() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(3..)
                .action(ArgAction::Set),
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
#[cfg(feature = "error-context")]
fn option_min_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(3..)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::TooFewValues, str![[r#"
error: 3 values required by '-o <option> <option> <option>...'; only 2 were provided

Usage: multiple_values [OPTIONS]

For more information, try '--help'.

"#]], true);
}

#[test]
fn option_short_min_more_mult_occurs() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("arg").required(true))
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(3..)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "pos", "-o", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

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
    let m = Command::new("multiple_values")
        .arg(Arg::new("arg").required(true))
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(3..),
        )
        .try_get_matches_from(vec!["", "pos", "-o", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

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
                .num_args(1..=3)
                .action(ArgAction::Set),
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
fn option_max_less() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(1..=3)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2"]);

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
#[cfg(feature = "error-context")]
fn option_max_zero() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(1..=3)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "-o"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::InvalidValue, str![[r#"
error: a value is required for '-o <option>...' but none was supplied

For more information, try '--help'.

"#]], true);
}

#[test]
fn option_max_zero_eq() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(1..=3)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "-o="]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        [""]
    );
}

#[test]
#[cfg(feature = "error-context")]
fn option_max_more() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .help("multiple options")
                .num_args(1..=3)
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "-o", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    // Can end up being TooManyValues or UnknownArgument
    utils::assert_error(err, ErrorKind::UnknownArgument, str![[r#"
error: unexpected argument 'val4' found

Usage: multiple_values [OPTIONS]

For more information, try '--help'.

"#]], true);
}

#[test]
fn optional_value() {
    let mut cmd = Command::new("test").args_override_self(true).arg(
        Arg::new("port")
            .short('p')
            .value_name("NUM")
            .num_args(0..=1),
    );

    let r = cmd.try_get_matches_from_mut(["test", "-p42"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("port"));
    assert_eq!(m.get_one::<String>("port").unwrap(), "42");

    let r = cmd.try_get_matches_from_mut(["test", "-p"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("port"));
    assert!(m.get_one::<String>("port").is_none());

    let r = cmd.try_get_matches_from_mut(["test", "-p", "24", "-p", "42"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("port"));
    assert_eq!(m.get_one::<String>("port").unwrap(), "42");

    let help = cmd.render_help().to_string();
    assert_data_eq!(
        help,
        str![[r#"
Usage: test [OPTIONS]

Options:
  -p [<NUM>]      
  -h, --help      Print help

"#]]
    );
}

#[test]
fn positional() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .action(ArgAction::Set)
                .num_args(1..),
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
        .arg(Arg::new("pos").help("multiple positionals").num_args(3))
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
#[cfg(feature = "error-context")]
fn positional_exact_less() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").num_args(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::WrongNumberOfValues, str![[r#"
error: 3 values required for '[pos] [pos] [pos]' but 2 were provided

Usage: myprog [pos] [pos] [pos]

For more information, try '--help'.

"#]], true);
}

#[test]
#[cfg(feature = "error-context")]
fn positional_exact_more() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").num_args(3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::WrongNumberOfValues, str![[r#"
error: 3 values required for '[pos] [pos] [pos]' but 4 were provided

Usage: myprog [pos] [pos] [pos]

For more information, try '--help'.

"#]], true);
}

#[test]
fn positional_min_exact() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").num_args(3..))
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
#[cfg(feature = "error-context")]
fn positional_min_less() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").num_args(3..))
        .try_get_matches_from(vec!["myprog", "val1", "val2"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::TooFewValues, str![[r#"
error: 3 values required by '[pos] [pos] [pos]...'; only 2 were provided

Usage: myprog [pos] [pos] [pos]...

For more information, try '--help'.

"#]], true);
}

#[test]
fn positional_min_more() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").num_args(3..))
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
        .arg(Arg::new("pos").help("multiple positionals").num_args(1..=3))
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
        .arg(Arg::new("pos").help("multiple positionals").num_args(1..=3))
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
#[cfg(feature = "error-context")]
fn positional_max_more() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").num_args(1..=3))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::TooManyValues, str![[r#"
error: unexpected value 'val4' for '[pos]...' found; no more were expected

Usage: myprog [pos]...

For more information, try '--help'.

"#]], true);
}

#[test]
fn sep_long_equals() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .help("multiple options")
                .value_delimiter(','),
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
                .value_delimiter(','),
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
                .value_delimiter(','),
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
                .value_delimiter(','),
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
                .value_delimiter(','),
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
                .value_delimiter(','),
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
                .action(ArgAction::Set),
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
                .action(ArgAction::Set),
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
                .num_args(1)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Set)
                .num_args(1..)
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
        ["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val2", "val3"]
    );
}

#[test]
fn req_delimiter_long_with_equal() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .num_args(1)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Set)
                .num_args(1..)
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
        ["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_space() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .num_args(1)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Set)
                .num_args(1..)
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
        ["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_no_space() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .num_args(1)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Set)
                .num_args(1..)
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
        ["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val2", "val3"]
    );
}

#[test]
fn req_delimiter_short_with_equal() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .short('o')
                .num_args(1)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Set)
                .num_args(1..)
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
        ["val1"]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val2", "val3"]
    );
}

#[test]
fn req_delimiter_complex() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("option")
                .long("option")
                .short('o')
                .num_args(1)
                .action(ArgAction::Append)
                .value_delimiter(','),
        )
        .arg(Arg::new("args").num_args(1..).index(1))
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
        [
            "val2", "val4", "val6", "val8", "val10", "val12", "val13", "val15", "val16", "val18",
            "val19", "val21", "val22", "val24", "val25",
        ]
    );
    assert_eq!(
        m.get_many::<String>("args")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        [
            "val1", "val3", "val5", "val7", "val9", "val11", "val14", "val17", "val20", "val23",
            "val26",
        ]
    );
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Positional argument `[target]` *must* have `required(true)` or `last(true)` set \
because a prior positional argument (`<files>...`) has `num_args(1..)`"]
fn low_index_positional_not_required() {
    let _ = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .action(ArgAction::Set)
                .required(true)
                .num_args(1..),
        )
        .arg(Arg::new("target").index(2))
        .try_get_matches_from(vec![""]);
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Only one positional argument with `.num_args(1..)` \
set is allowed per command, unless the second one also has .last(true) set"]
fn low_index_positional_last_multiple_too() {
    let _ = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .action(ArgAction::Set)
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("target")
                .index(2)
                .action(ArgAction::Set)
                .required(true)
                .num_args(1..),
        )
        .try_get_matches_from(vec![""]);
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Only the last positional argument, or second to \
last positional argument may be set to `.num_args(1..)`"]
fn low_index_positional_too_far_back() {
    let _ = Command::new("lip")
        .arg(
            Arg::new("files")
                .index(1)
                .action(ArgAction::Set)
                .required(true)
                .num_args(1..),
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
                .action(ArgAction::Set)
                .required(true)
                .num_args(1..),
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
                        .action(ArgAction::Set)
                        .required(true)
                        .num_args(1..),
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
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .arg(Arg::new("target").index(2).required(true))
        .arg(Arg::new("opt").long("option").action(ArgAction::Set))
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
                .action(ArgAction::Set)
                .required(true)
                .num_args(1..),
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
fn low_index_positional_with_extra_flags() {
    let cmd = Command::new("test")
        .arg(Arg::new("yes").long("yes").action(ArgAction::SetTrue))
        .arg(Arg::new("one").long("one").action(ArgAction::Set))
        .arg(Arg::new("two").long("two").action(ArgAction::Set))
        .arg(Arg::new("input").num_args(1..).required(true))
        .arg(Arg::new("output").required(true));
    let m = cmd.try_get_matches_from([
        "test", "--one", "1", "--two", "2", "3", "4", "5", "6", "7", "8",
    ]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert_eq!(
        m.get_many::<String>("input")
            .unwrap()
            .map(String::from)
            .collect::<Vec<_>>(),
        vec![
            "3".to_owned(),
            "4".to_owned(),
            "5".to_owned(),
            "6".to_owned(),
            "7".to_owned()
        ],
    );
    assert_eq!(m.get_one::<String>("output").unwrap(), "8");
    assert_eq!(m.get_one::<String>("one").unwrap(), "1");
    assert_eq!(m.get_one::<String>("two").unwrap(), "2");
    assert!(!m.get_flag("yes"));
}

#[test]
fn multiple_value_terminator_option() {
    let m = Command::new("lip")
        .arg(
            Arg::new("files")
                .short('f')
                .value_terminator(";")
                .action(ArgAction::Set)
                .num_args(1..),
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
                .action(ArgAction::Set)
                .num_args(1..),
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
fn all_multiple_value_terminator() {
    let m = Command::new("lip")
        .arg(
            Arg::new("files")
                .value_terminator(";")
                .action(ArgAction::Set)
                .num_args(0..),
        )
        .arg(Arg::new("other").num_args(0..))
        .try_get_matches_from(vec!["test", "value", ";"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err().kind());
    let m = m.unwrap();

    assert!(m.contains_id("files"));
    assert!(!m.contains_id("other"));
    assert_eq!(
        m.get_many::<String>("files")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["value".to_owned()],
    );
}

#[test]
fn multiple_vals_with_hyphen() {
    let res = Command::new("do")
        .arg(
            Arg::new("cmds")
                .action(ArgAction::Set)
                .num_args(1..)
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
fn multiple_positional_multiple_values() {
    let res = Command::new("do")
        .arg(
            Arg::new("cmd1")
                .action(ArgAction::Set)
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";"),
        )
        .arg(
            Arg::new("cmd2")
                .action(ArgAction::Set)
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";"),
        )
        .try_get_matches_from(vec![
            "do",
            "find",
            "-type",
            "f",
            "-name",
            "special",
            ";",
            "/home/clap",
            "foo",
        ]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());

    let m = res.unwrap();
    let cmd1: Vec<_> = m
        .get_many::<String>("cmd1")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(&cmd1, &["find", "-type", "f", "-name", "special"]);
    let cmd2: Vec<_> = m
        .get_many::<String>("cmd2")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(&cmd2, &["/home/clap", "foo"]);
}

#[test]
fn value_terminator_has_higher_precedence_than_allow_hyphen_values() {
    let res = Command::new("do")
        .arg(
            Arg::new("cmd1")
                .action(ArgAction::Set)
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator("--foo"),
        )
        .arg(
            Arg::new("cmd2")
                .action(ArgAction::Set)
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";"),
        )
        .try_get_matches_from(vec![
            "do",
            "find",
            "-type",
            "f",
            "-name",
            "special",
            "--foo",
            "/home/clap",
            "foo",
        ]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());

    let m = res.unwrap();
    let cmd1: Vec<_> = m
        .get_many::<String>("cmd1")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(&cmd1, &["find", "-type", "f", "-name", "special"]);
    let cmd2: Vec<_> = m
        .get_many::<String>("cmd2")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(&cmd2, &["/home/clap", "foo"]);
}

#[test]
fn value_terminator_restores_escaping_disabled_by_allow_hyphen_values() {
    let res = Command::new("do")
        .arg(
            Arg::new("cmd1")
                .action(ArgAction::Set)
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator("--"),
        )
        .arg(
            Arg::new("cmd2")
                .action(ArgAction::Set)
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";"),
        )
        .try_get_matches_from(vec![
            "do",
            "find",
            "-type",
            "f",
            "-name",
            "special",
            "--",
            "/home/clap",
            "foo",
        ]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());

    let m = res.unwrap();
    let cmd1: Vec<_> = m
        .get_many::<String>("cmd1")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(&cmd1, &["find", "-type", "f", "-name", "special"]);
    let cmd2: Vec<_> = m
        .get_many::<String>("cmd2")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(&cmd2, &["/home/clap", "foo"]);
}

#[test]
fn issue_1480_max_values_consumes_extra_arg_1() {
    let res = Command::new("prog")
        .arg(Arg::new("field").num_args(..=1).long("field"))
        .arg(Arg::new("positional").required(true).index(1))
        .try_get_matches_from(vec!["prog", "--field", "1", "file"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
#[cfg(feature = "error-context")]
fn issue_1480_max_values_consumes_extra_arg_2() {
    let m = Command::new("prog")
        .arg(Arg::new("field").num_args(..=1).long("field"))
        .try_get_matches_from(vec!["prog", "--field", "1", "2"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::UnknownArgument, str![[r#"
error: unexpected argument '2' found

Usage: prog [OPTIONS]

For more information, try '--help'.

"#]], true);
}

#[test]
#[cfg(feature = "error-context")]
fn issue_1480_max_values_consumes_extra_arg_3() {
    let m = Command::new("prog")
        .arg(Arg::new("field").num_args(..=1).long("field"))
        .try_get_matches_from(vec!["prog", "--field", "1", "2", "3"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::UnknownArgument, str![[r#"
error: unexpected argument '2' found

Usage: prog [OPTIONS]

For more information, try '--help'.

"#]], true);
}

#[test]
fn value_names_building_num_vals() {
    let m = Command::new("test")
        .arg(
            Arg::new("pos")
                .long("pos")
                .value_names(["who", "what", "why"]),
        )
        .try_get_matches_from(vec!["myprog", "--pos", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
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
        .arg(Arg::new("pos").value_names(["who", "what", "why"]))
        .try_get_matches_from(vec!["myprog", "val1", "val2", "val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
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
fn num_args_preferred_over_value_names() {
    let m = Command::new("test")
        .arg(
            Arg::new("pos")
                .long("pos")
                .num_args(4)
                .value_names(["who", "what", "why"]),
        )
        .try_get_matches_from(vec!["myprog", "--pos", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
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
            .num_args(2)
            .action(ArgAction::Append),
    );

    let m = a.try_get_matches_from_mut(vec!["myprog", "--pos", "val1", "val2"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
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
        Err(err) => panic!("{err}"),
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
    let mut a = Command::new("test").arg(Arg::new("pos").num_args(2).action(ArgAction::Append));

    let m = a.try_get_matches_from_mut(vec!["myprog", "val1", "val2"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert_eq!(
        m.get_many::<String>("pos")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["val1", "val2"]
    );
}

#[test]
#[cfg(feature = "error-context")]
fn issue_2229() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos").help("multiple positionals").num_args(3))
        .try_get_matches_from(vec![
            "myprog", "val1", "val2", "val3", "val4", "val5", "val6",
        ]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    utils::assert_error(err, ErrorKind::WrongNumberOfValues, str![[r#"
error: 3 values required for '[pos] [pos] [pos]' but 6 were provided

Usage: myprog [pos] [pos] [pos]

For more information, try '--help'.

"#]], true);
}

#[test]
#[should_panic = "Argument 'pos` is positional and it must take a value but action is SetTrue"]
fn disallow_positionals_without_values() {
    let cmd = Command::new("test").arg(Arg::new("pos").num_args(0));
    cmd.debug_assert();
}
