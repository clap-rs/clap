use clap::{Arg, ArgAction, Command};

#[test]
fn opt_default_no_delim() {
    let m = Command::new("no_delim")
        .arg(Arg::new("option").long("option").action(ArgAction::Set))
        .try_get_matches_from(vec!["", "--option", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()).unwrap(),
        "val1,val2,val3"
    );
}

#[test]
fn opt_eq_no_delim() {
    let m = Command::new("no_delim")
        .arg(Arg::new("option").long("option").action(ArgAction::Set))
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
fn opt_s_eq_no_delim() {
    let m = Command::new("no_delim")
        .arg(Arg::new("option").short('o').action(ArgAction::Set))
        .try_get_matches_from(vec!["", "-o=val1,val2,val3"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()).unwrap(),
        "val1,val2,val3"
    );
}

#[test]
fn opt_s_default_no_delim() {
    let m = Command::new("no_delim")
        .arg(Arg::new("option").short('o').action(ArgAction::Set))
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()).unwrap(),
        "val1,val2,val3"
    );
}

#[test]
fn opt_s_no_space_no_delim() {
    let m = Command::new("no_delim")
        .arg(Arg::new("option").short('o').action(ArgAction::Set))
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()).unwrap(),
        "val1,val2,val3"
    );
}

#[test]
fn opt_s_no_space_mult_no_delim() {
    let m = Command::new("no_delim")
        .arg(
            Arg::new("option")
                .short('o')
                .action(ArgAction::Set)
                .number_of_values(1..),
        )
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()).unwrap(),
        "val1,val2,val3"
    );
}

#[test]
fn opt_eq_mult_def_delim() {
    let m = Command::new("no_delim")
        .arg(
            Arg::new("option")
                .long("opt")
                .action(ArgAction::Set)
                .number_of_values(1..)
                .use_value_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        &["val1", "val2", "val3"]
    );
}
