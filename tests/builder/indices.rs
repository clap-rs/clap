use clap::{Arg, ArgAction, Command};

#[test]
fn indices_mult_opts() {
    let m = Command::new("ind")
        .arg(
            Arg::new("exclude")
                .short('e')
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("include")
                .short('i')
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec!["ind", "-e", "A", "B", "-i", "B", "C", "-e", "C"])
        .unwrap();

    assert_eq!(
        m.indices_of("exclude").unwrap().collect::<Vec<_>>(),
        &[2, 3, 8]
    );
    assert_eq!(
        m.indices_of("include").unwrap().collect::<Vec<_>>(),
        &[5, 6]
    );
}

#[test]
fn index_mult_opts() {
    let m = Command::new("ind")
        .arg(
            Arg::new("exclude")
                .short('e')
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("include")
                .short('i')
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec!["ind", "-e", "A", "B", "-i", "B", "C", "-e", "C"])
        .unwrap();

    assert_eq!(m.index_of("exclude"), Some(2));
    assert_eq!(m.index_of("include"), Some(5));
}

#[test]
fn index_flag() {
    let m = Command::new("ind")
        .arg(Arg::new("exclude").short('e'))
        .arg(Arg::new("include").short('i'))
        .try_get_matches_from(vec!["ind", "-e", "-i"])
        .unwrap();

    assert_eq!(m.index_of("exclude"), Some(1));
    assert_eq!(m.index_of("include"), Some(2));
}

#[test]
fn index_flags() {
    let m = Command::new("ind")
        .arg(Arg::new("exclude").short('e').action(ArgAction::SetTrue))
        .arg(Arg::new("include").short('i').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["ind", "-e", "-i", "-e", "-e", "-i"])
        .unwrap();

    assert_eq!(m.index_of("exclude"), Some(4));
    assert_eq!(m.index_of("include"), Some(5));
}

#[test]
fn indices_mult_flags() {
    let m = Command::new("ind")
        .arg(Arg::new("exclude").short('e').action(ArgAction::SetTrue))
        .arg(Arg::new("include").short('i').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["ind", "-e", "-i", "-e", "-e", "-i"])
        .unwrap();

    assert_eq!(m.indices_of("exclude").unwrap().collect::<Vec<_>>(), &[4]);
    assert_eq!(m.indices_of("include").unwrap().collect::<Vec<_>>(), &[5]);
}

#[test]
fn indices_mult_flags_combined() {
    let m = Command::new("ind")
        .arg(Arg::new("exclude").short('e').action(ArgAction::SetTrue))
        .arg(Arg::new("include").short('i').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["ind", "-eieei"])
        .unwrap();

    assert_eq!(m.indices_of("exclude").unwrap().collect::<Vec<_>>(), &[4]);
    assert_eq!(m.indices_of("include").unwrap().collect::<Vec<_>>(), &[5]);
}

#[test]
fn indices_mult_flags_opt_combined() {
    let m = Command::new("ind")
        .arg(Arg::new("exclude").short('e').action(ArgAction::SetTrue))
        .arg(Arg::new("include").short('i').action(ArgAction::SetTrue))
        .arg(Arg::new("option").short('o').takes_value(true))
        .try_get_matches_from(vec!["ind", "-eieeio", "val"])
        .unwrap();

    assert_eq!(m.indices_of("exclude").unwrap().collect::<Vec<_>>(), &[4]);
    assert_eq!(m.indices_of("include").unwrap().collect::<Vec<_>>(), &[5]);
    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[7]);
}

#[test]
fn indices_mult_flags_opt_combined_eq() {
    let m = Command::new("ind")
        .arg(Arg::new("exclude").short('e').action(ArgAction::SetTrue))
        .arg(Arg::new("include").short('i').action(ArgAction::SetTrue))
        .arg(Arg::new("option").short('o').takes_value(true))
        .try_get_matches_from(vec!["ind", "-eieeio=val"])
        .unwrap();

    assert_eq!(m.indices_of("exclude").unwrap().collect::<Vec<_>>(), &[4]);
    assert_eq!(m.indices_of("include").unwrap().collect::<Vec<_>>(), &[5]);
    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[7]);
}

#[test]
fn indices_mult_opt_value_delim_eq() {
    let m = Command::new("myapp")
        .arg(
            Arg::new("option")
                .short('o')
                .takes_value(true)
                .use_value_delimiter(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec!["myapp", "-o=val1,val2,val3"])
        .unwrap();
    assert_eq!(
        m.indices_of("option").unwrap().collect::<Vec<_>>(),
        &[2, 3, 4]
    );
}

#[test]
fn indices_mult_opt_value_no_delim_eq() {
    let m = Command::new("myapp")
        .arg(
            Arg::new("option")
                .short('o')
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec!["myapp", "-o=val1,val2,val3"])
        .unwrap();
    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2]);
}

#[test]
fn indices_mult_opt_mult_flag() {
    let m = Command::new("myapp")
        .arg(Arg::new("option").short('o').action(ArgAction::Append))
        .arg(Arg::new("flag").short('f').action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["myapp", "-o", "val1", "-f", "-o", "val2", "-f"])
        .unwrap();

    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 5]);
    assert_eq!(m.indices_of("flag").unwrap().collect::<Vec<_>>(), &[6]);
}
