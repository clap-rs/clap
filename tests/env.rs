extern crate clap;

use std::env;
use std::ffi::OsStr;

use clap::{App, Arg};

#[test]
fn env() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = App::new("df")
        .arg(Arg::from("[arg] 'some opt'").env("CLP_TEST_ENV"))
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn env_os() {
    env::set_var("CLP_TEST_ENV_OS", "env");

    let r = App::new("df")
        .arg(Arg::from("[arg] 'some opt'").env_os(OsStr::new("CLP_TEST_ENV_OS")))
        .try_get_matches_from(vec![""]);

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
        .arg(Arg::from("[arg] 'some opt'").env("CLP_TEST_ENV_NONE"))
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(!m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg"), None);
}

#[test]
fn with_default() {
    env::set_var("CLP_TEST_ENV_WD", "env");

    let r = App::new("df")
        .arg(
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_WD")
                .default_value("default"),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn opt_user_override() {
    env::set_var("CLP_TEST_ENV_OR", "env");

    let r = App::new("df")
        .arg(Arg::from("--arg [FILE] 'some arg'").env("CLP_TEST_ENV_OR"))
        .try_get_matches_from(vec!["", "--arg", "opt"]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 1);
    assert_eq!(m.value_of("arg").unwrap(), "opt");
}

#[test]
fn positionals() {
    env::set_var("CLP_TEST_ENV_P", "env");

    let r = App::new("df")
        .arg(Arg::from("[arg] 'some opt'").env("CLP_TEST_ENV_P"))
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn positionals_user_override() {
    env::set_var("CLP_TEST_ENV_POR", "env");

    let r = App::new("df")
        .arg(Arg::from("[arg] 'some opt'").env("CLP_TEST_ENV_POR"))
        .try_get_matches_from(vec!["", "opt"]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 1);
    assert_eq!(m.value_of("arg").unwrap(), "opt");
}

#[test]
fn multiple_one() {
    env::set_var("CLP_TEST_ENV_MO", "env");

    let r = App::new("df")
        .arg(
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_MO")
                .use_delimiter(true)
                .multiple(true),
        )
        .try_get_matches_from(vec![""]);

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
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_MULTI1")
                .use_delimiter(true)
                .multiple(true),
        )
        .try_get_matches_from(vec![""]);

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
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_MULTI2")
                .multiple(true),
        )
        .try_get_matches_from(vec![""]);

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
    env::set_var("CLP_TEST_ENV_PV", "env");

    let r = App::new("df")
        .arg(
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_PV")
                .possible_value("env"),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn not_possible_value() {
    env::set_var("CLP_TEST_ENV_NPV", "env");

    let r = App::new("df")
        .arg(
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_NPV")
                .possible_value("never"),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_err());
}

#[test]
fn validator() {
    env::set_var("CLP_TEST_ENV_VDOR", "env");

    let r = App::new("df")
        .arg(
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_VDOR")
                .validator(|s| {
                    if s == "env" {
                        Ok(())
                    } else {
                        Err("not equal".to_string())
                    }
                }),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn validator_output() {
    env::set_var("CLP_TEST_ENV_VO", "42");

    let m = App::new("df")
        .arg(
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_VO")
                .validator(|s| s.parse::<i32>()),
        )
        .get_matches_from(vec![""]);

    assert_eq!(m.value_of("arg").unwrap().parse(), Ok(42));
}

#[test]
fn validator_invalid() {
    env::set_var("CLP_TEST_ENV_IV", "env");

    let r = App::new("df")
        .arg(
            Arg::from("[arg] 'some opt'")
                .env("CLP_TEST_ENV_IV")
                .validator(|s| {
                    if s != "env" {
                        Ok(())
                    } else {
                        Err("is equal".to_string())
                    }
                }),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_err());
}
