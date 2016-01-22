extern crate clap;

use clap::{App, Arg, ErrorKind};

#[test]
fn positional() {
    let m = App::new("positional")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from(vec!["myprog", "-f", "test"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional"), Some("test"));

    let m = App::new("positional")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from(vec!["myprog", "test", "--flag"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional"), Some("test"));
}

#[test]
fn positional_multiple() {
    let m = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .multiple(true)
            ])
        .get_matches_from(vec!["myprog", "-f", "test1", "test2", "test3"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.values_of("positional").unwrap().collect::<Vec<_>>(), vec!["test1", "test2", "test3"]);

    let m = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .multiple(true)
            ])
        .get_matches_from(vec!["myprog", "test1", "test2", "test3", "--flag"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.values_of("positional").unwrap().collect::<Vec<_>>(), vec!["test1", "test2", "test3"]);
}

#[test]
fn positional_multiple_2() {
    let result = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from_safe(vec!["myprog", "-f", "test1", "test2", "test3"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}
