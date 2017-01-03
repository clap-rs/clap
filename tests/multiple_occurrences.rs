extern crate clap;

use clap::{App, Arg};

#[test]
fn multiple_occurrences_of_flags_long() {
    let m = App::new("mo_flags_long")
                .arg(Arg::from_usage("--multflag 'allowed multiple flag'")
                    .multiple(true))
                .arg(Arg::from_usage("--flag 'disallowed multiple flag'"))
                .get_matches_from(vec![
                    "",
                    "--multflag",
                    "--flag",
                    "--multflag"
                    ]);
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1)
}

#[test]
fn multiple_occurrences_of_flags_short() {
    let m = App::new("mo_flags_short")
                .arg(Arg::from_usage("-m --multflag 'allowed multiple flag'")
                    .multiple(true))
                .arg(Arg::from_usage("-f --flag 'disallowed multiple flag'"))
                .get_matches_from(vec![
                    "",
                    "-m",
                    "-f",
                    "-m"
                    ]);
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn multiple_occurrences_of_flags_mixed() {
    let m = App::new("mo_flags_mixed")
                .arg(Arg::from_usage("-m, --multflag1 'allowed multiple flag'")
                    .multiple(true))
                .arg(Arg::from_usage("-n, --multflag2 'another allowed multiple flag'")
                    .multiple(true))
                .arg(Arg::from_usage("-f, --flag 'disallowed multiple flag'"))
                .get_matches_from(vec![
                    "",
                    "-m",
                    "-f",
                    "-n",
                    "--multflag1",
                    "-m",
                    "--multflag2"
                    ]);
    assert!(m.is_present("multflag1"));
    assert_eq!(m.occurrences_of("multflag1"), 3);
    assert!(m.is_present("multflag2"));
    assert_eq!(m.occurrences_of("multflag2"), 2);
    assert!(m.is_present("flag"));
    assert_eq!(m.occurrences_of("flag"), 1);
}

#[test]
fn multiple_occurrences_of_flags_large_quantity() {
    let args : Vec<&str> = vec![""].into_iter().chain(vec!["-m"; 1024].into_iter()).collect();
    let m = App::new("mo_flags_larg_qty")
                .arg(Arg::from_usage("-m --multflag 'allowed multiple flag'")
                    .multiple(true))
                .get_matches_from(args);
    assert!(m.is_present("multflag"));
    assert_eq!(m.occurrences_of("multflag"), 1024);
}
