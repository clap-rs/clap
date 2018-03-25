#![cfg(feature="iter_matches")]

extern crate clap;

use clap::{App, Arg};

#[test]
fn iter_matches_empty() {
    let res = App::new("prog")
        .arg(Arg::with_name("cfg")
            .takes_value(true)
            .long("config"))
        .get_matches_from_safe(vec!["prog"]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().args.iter().count(), 0);
}
