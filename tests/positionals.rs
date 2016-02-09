extern crate clap;

use clap::{App, Arg, ErrorKind};

#[test]
fn only_pos_follow() {
    let r = App::new("onlypos")
        .args(&[Arg::from_usage("-f [flag] 'some opt'"),
                Arg::from_usage("[arg] 'some arg'")])
        .get_matches_from_safe(vec!["", "--", "-f"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.value_of("arg").unwrap(), "-f");
}

#[test]
fn positional() {
    let m = App::new("positional")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from(vec!["", "-f", "test"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");

    let m = App::new("positional")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from(vec!["", "test", "--flag"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");
}

#[test]
fn lots_o_vals() {
    let r = App::new("opts")
        .arg(
            Arg::from_usage("[opt]... 'some pos'"),
            )
        .get_matches_from_safe(vec!["",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            ]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().collect::<Vec<_>>().len(), 297); // i.e. more than u8
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
        .get_matches_from(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(&*m.values_of("positional").unwrap().collect::<Vec<_>>(), &["test1", "test2", "test3"]);

    let m = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .multiple(true)
            ])
        .get_matches_from(vec!["", "test1", "test2", "test3", "--flag"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(&*m.values_of("positional").unwrap().collect::<Vec<_>>(), &["test1", "test2", "test3"]);
}

#[test]
fn positional_multiple_2() {
    let result = App::new("positional_multiple")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from_safe(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn positional_possible_values() {
    let m = App::new("positional_possible_values")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .possible_value("test123")
            ])
        .get_matches_from(vec!["", "-f", "test123"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(&*m.values_of("positional").unwrap().collect::<Vec<_>>(), &["test123"]);
}

#[test]
fn create_positional() {
    let _ = App::new("test")
                .arg(Arg::with_name("test")
                            .index(1)
                            .help("testing testing"))
                .get_matches();
}

#[test]
fn positional_hyphen_does_not_panic() {
    let _ = App::new("test")
        .arg(Arg::with_name("dummy"))
        .get_matches_from(vec!["test", "-"]);
}

#[test]
fn default_values_default() {
    let r = App::new("df")
        .arg( Arg::from_usage("[arg] 'some opt'")
            .default_value("default"))
        .get_matches_from_safe(vec![""]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "default");
}

#[test]
fn default_values_user_value() {
    let r = App::new("df")
        .arg( Arg::from_usage("[arg] 'some arg'")
            .default_value("default"))
        .get_matches_from_safe(vec!["", "value"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.value_of("arg").unwrap(), "value");
}
