extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg};

#[test]
fn indices_mult_opts() {
    let m = App::new("ind")
        .arg(
            Arg::with_name("exclude")
                .short('e')
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .takes_value(true)
                .multiple(true),
        )
        .get_matches_from(vec!["ind", "-e", "A", "B", "-i", "B", "C", "-e", "C"]);

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
    let m = App::new("ind")
        .arg(
            Arg::with_name("exclude")
                .short('e')
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .takes_value(true)
                .multiple(true),
        )
        .get_matches_from(vec!["ind", "-e", "A", "B", "-i", "B", "C", "-e", "C"]);

    assert_eq!(m.index_of("exclude"), Some(2));
    assert_eq!(m.index_of("include"), Some(5));
}

#[test]
fn index_flag() {
    let m = App::new("ind")
        .arg(Arg::with_name("exclude").short('e'))
        .arg(Arg::with_name("include").short('i'))
        .get_matches_from(vec!["ind", "-e", "-i"]);

    assert_eq!(m.index_of("exclude"), Some(1));
    assert_eq!(m.index_of("include"), Some(2));
}

#[test]
fn index_flags() {
    let m = App::new("ind")
        .arg(
            Arg::with_name("exclude")
                .short('e')
                .multiple_occurrences(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .multiple_occurrences(true),
        )
        .get_matches_from(vec!["ind", "-e", "-i", "-e", "-e", "-i"]);

    assert_eq!(m.index_of("exclude"), Some(1));
    assert_eq!(m.index_of("include"), Some(2));
}

#[test]
fn indices_mult_flags() {
    let m = App::new("ind")
        .arg(
            Arg::with_name("exclude")
                .short('e')
                .multiple_occurrences(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .multiple_occurrences(true),
        )
        .get_matches_from(vec!["ind", "-e", "-i", "-e", "-e", "-i"]);

    assert_eq!(
        m.indices_of("exclude").unwrap().collect::<Vec<_>>(),
        &[1, 3, 4]
    );
    assert_eq!(
        m.indices_of("include").unwrap().collect::<Vec<_>>(),
        &[2, 5]
    );
}

#[test]
fn indices_mult_flags_combined() {
    let m = App::new("ind")
        .arg(
            Arg::with_name("exclude")
                .short('e')
                .multiple_occurrences(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .multiple_occurrences(true),
        )
        .get_matches_from(vec!["ind", "-eieei"]);

    assert_eq!(
        m.indices_of("exclude").unwrap().collect::<Vec<_>>(),
        &[1, 3, 4]
    );
    assert_eq!(
        m.indices_of("include").unwrap().collect::<Vec<_>>(),
        &[2, 5]
    );
}

#[test]
fn indices_mult_flags_opt_combined() {
    let m = App::new("ind")
        .arg(
            Arg::with_name("exclude")
                .short('e')
                .multiple_occurrences(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .multiple_occurrences(true),
        )
        .arg(Arg::with_name("option").short('o').takes_value(true))
        .get_matches_from(vec!["ind", "-eieeio", "val"]);

    assert_eq!(
        m.indices_of("exclude").unwrap().collect::<Vec<_>>(),
        &[1, 3, 4]
    );
    assert_eq!(
        m.indices_of("include").unwrap().collect::<Vec<_>>(),
        &[2, 5]
    );
    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[7]);
}

#[test]
fn indices_mult_flags_opt_combined_eq() {
    let m = App::new("ind")
        .arg(
            Arg::with_name("exclude")
                .short('e')
                .multiple_occurrences(true),
        )
        .arg(
            Arg::with_name("include")
                .short('i')
                .multiple_occurrences(true),
        )
        .arg(Arg::with_name("option").short('o').takes_value(true))
        .get_matches_from(vec!["ind", "-eieeio=val"]);

    assert_eq!(
        m.indices_of("exclude").unwrap().collect::<Vec<_>>(),
        &[1, 3, 4]
    );
    assert_eq!(
        m.indices_of("include").unwrap().collect::<Vec<_>>(),
        &[2, 5]
    );
    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[7]);
}

#[test]
fn indices_mult_opt_value_delim_eq() {
    let m = App::new("myapp")
        .arg(
            Arg::with_name("option")
                .short('o')
                .takes_value(true)
                .use_delimiter(true)
                .multiple(true),
        )
        .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);
    assert_eq!(
        m.indices_of("option").unwrap().collect::<Vec<_>>(),
        &[2, 3, 4]
    );
}

#[test]
fn indices_mult_opt_value_no_delim_eq() {
    let m = App::new("myapp")
        .arg(
            Arg::with_name("option")
                .short('o')
                .takes_value(true)
                .multiple(true),
        )
        .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);
    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2]);
}

#[test]
fn indices_mult_opt_mult_flag() {
    let m = App::new("myapp")
        .arg(
            Arg::with_name("option")
                .short('o')
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(Arg::with_name("flag").short('f').multiple_occurrences(true))
        .get_matches_from(vec!["myapp", "-o", "val1", "-f", "-o", "val2", "-f"]);

    assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 5]);
    assert_eq!(m.indices_of("flag").unwrap().collect::<Vec<_>>(), &[3, 6]);
}
