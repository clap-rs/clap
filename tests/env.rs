extern crate clap;

use std::env;
use std::ffi::OsStr;

use clap::{App, Arg};

#[test]
fn env() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(Arg::from_usage("[arg] 'some opt'").env("CLP_TEST_ENV"))
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn env_os() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(Arg::from_usage("[arg] 'some opt'").env_os(OsStr::new("CLP_TEST_ENV")))
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn no_env() {
    // All the other tests use the presence of the Environment variable...
    // we need another variable just in case one of the others is running at the same time...
    env::remove_var("CLP_TEST_ENV_NONE");

    let r = App::new("df")
        .arg(Arg::from_usage("[arg] 'some opt'").env("CLP_TEST_ENV_NONE"))
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(!m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg"), None);
}

#[test]
fn with_default() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV")
                .default_value("default"),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn opt_user_override() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(Arg::from_usage("--arg [FILE] 'some arg'").env("CLP_TEST_ENV"))
        .get_matches_from_safe(vec!["", "--arg", "opt"]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 1);
    assert_eq!(m.value_of("arg").unwrap(), "opt");
}

#[test]
fn positionals() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(Arg::from_usage("[arg] 'some opt'").env("CLP_TEST_ENV"))
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn positionals_user_override() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(Arg::from_usage("[arg] 'some opt'").env("CLP_TEST_ENV"))
        .get_matches_from_safe(vec!["", "opt"]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 1);
    assert_eq!(m.value_of("arg").unwrap(), "opt");
}

#[test]
fn multiple_one() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV")
                .use_delimiter(true)
                .multiple(true),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(), vec!["env"]);
}

#[test]
fn multiple_three() {
    env::set_var("CLP_TEST_ENV_MULTI1", "env1,env2,env3");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV_MULTI1")
                .use_delimiter(true)
                .multiple(true),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(
        m.values_of("arg").unwrap().collect::<Vec<_>>(),
        vec!["env1", "env2", "env3"]
    );
}

#[test]
fn multiple_no_delimiter() {
    env::set_var("CLP_TEST_ENV_MULTI2", "env1 env2 env3");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV_MULTI2")
                .multiple(true),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(
        m.values_of("arg").unwrap().collect::<Vec<_>>(),
        vec!["env1 env2 env3"]
    );
}

#[test]
fn possible_value() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV")
                .possible_value("env"),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn not_possible_value() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV")
                .possible_value("never"),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_err());
}

#[test]
fn validator() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV")
                .validator(|s| {
                    if s == "env" {
                        Ok(())
                    } else {
                        Err("not equal".to_string())
                    }
                }),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn validator_output() {
    env::set_var("CLP_TEST_ENV", "42");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV")
                .validator(|s| s.parse::<i32>())
        )
        .get_matches_from_safe(vec![""]);

    assert_eq!(r.unwrap().value_of("arg").unwrap().parse(), Ok(42));
}

#[test]
fn validator_invalid() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(
            Arg::from_usage("[arg] 'some opt'")
                .env("CLP_TEST_ENV")
                .validator(|s| {
                    if s != "env" {
                        Ok(())
                    } else {
                        Err("is equal".to_string())
                    }
                }),
        )
        .get_matches_from_safe(vec![""]);

    assert!(r.is_err());
}
