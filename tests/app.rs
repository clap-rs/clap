extern crate clap;

use clap::{App, Rule, ClapError, CollectionMatcher};

#[test]
fn unknown_long_config() {
    let ref mut sample = vec!["-vvv", "--config", "config.conf"].into_iter();
    let ac = App::with_rules(vec![
            Rule::with_name("verbose").short('v').max_occurrences(3)
        ]);
    match ac.get_matches(sample) {
        Err(ClapError::UnexpectedLong(_config)) => (),
            // Found unexpected argument: "config"
        _ => unreachable!(),
    }
}

#[test]
fn flag_verbose_found_3_times() {
    let ref mut sample = vec!["-vv", "--verbose"].into_iter();
    let ac = App::with_rules(vec![
            Rule::with_name("verbose").short('v').long("verbose").max_occurrences(3),
        ]);
    let matches = ac.get_matches(sample).unwrap();
    assert_eq!(matches.get("verbose").unwrap().get_occurrences(), 3)
}

#[test]
fn collect_short_values() {
    let ref mut sample = vec!["-babcb", "1", "2", "3"].into_iter();
    let ac = App::with_rules(vec![
            Rule::with_name("a").short('a').takes_value_unnamed_n_times(1),
            Rule::with_name("b").short('b').multiple(),
            Rule::with_name("c").short('c').takes_value_unnamed_n_times(2),
        ]);
    let matches = ac.get_matches(sample).unwrap();
    assert_eq!(&*matches.get("a").unwrap().get_vec(), &["1"]);
    assert_eq!(matches.get("b").unwrap().get_occurrences(), 3);
    assert_eq!(&*matches.get("c").unwrap().get_vec(), &["2", "3"]);
}

#[test]
fn positionals() {
    let ref mut sample = vec!["-vvv", "pos1", "pos2"].into_iter();
    let ac = App::with_rules(vec![
            Rule::with_name("verbose").short('v').max_occurrences(3),
            Rule::with_name("pos1").required().takes_value_unnamed(),
            Rule::with_name("pos2").takes_value_unnamed(),
        ]);
    let matches = ac.get_matches(sample).unwrap();
    assert_eq!(matches.get("verbose").unwrap().get_occurrences(), 3);
    assert_eq!(&*matches.get("pos1").unwrap().get_vec(), &["pos1"]);
    assert_eq!(&*matches.get("pos2").unwrap().get_vec(), &["pos2"]);
}
