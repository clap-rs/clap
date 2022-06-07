#![cfg(feature = "env")]

use std::env;
use std::ffi::OsStr;

use clap::{arg, Arg, Command};

#[test]
fn env() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = Command::new("df")
        .arg(arg!([arg] "some opt").env("CLP_TEST_ENV").takes_value(true))
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn env_bool_literal() {
    env::set_var("CLP_TEST_FLAG_TRUE", "On");
    env::set_var("CLP_TEST_FLAG_FALSE", "nO");

    let r = Command::new("df")
        .arg(Arg::new("present").short('p').env("CLP_TEST_FLAG_TRUE"))
        .arg(Arg::new("negated").short('n').env("CLP_TEST_FLAG_FALSE"))
        .arg(Arg::new("absent").short('a').env("CLP_TEST_FLAG_ABSENT"))
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("present"));
    assert_eq!(m.occurrences_of("present"), 0);
    assert_eq!(m.value_of("present"), None);
    assert!(!m.is_present("negated"));
    assert!(!m.is_present("absent"));
}

#[test]
fn env_os() {
    env::set_var("CLP_TEST_ENV_OS", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env_os(OsStr::new("CLP_TEST_ENV_OS"))
                .takes_value(true),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
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

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_NONE")
                .takes_value(true),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg"), None);
}

#[test]
fn no_env_no_takes_value() {
    // All the other tests use the presence of the Environment variable...
    // we need another variable just in case one of the others is running at the same time...
    env::remove_var("CLP_TEST_ENV_NONE");

    let r = Command::new("df")
        .arg(arg!([arg] "some opt").env("CLP_TEST_ENV_NONE"))
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg"), None);
}

#[test]
fn with_default() {
    env::set_var("CLP_TEST_ENV_WD", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_WD")
                .takes_value(true)
                .default_value("default"),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn opt_user_override() {
    env::set_var("CLP_TEST_ENV_OR", "env");

    let r = Command::new("df")
        .arg(
            arg!(--arg [FILE] "some arg")
                .env("CLP_TEST_ENV_OR")
                .takes_value(true),
        )
        .try_get_matches_from(vec!["", "--arg", "opt"]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 1);
    assert_eq!(m.value_of("arg").unwrap(), "opt");

    // see https://github.com/clap-rs/clap/issues/1835
    let values: Vec<_> = m.values_of("arg").unwrap().collect();
    assert_eq!(values, vec!["opt"]);
}

#[test]
fn positionals() {
    env::set_var("CLP_TEST_ENV_P", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_P")
                .takes_value(true),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn positionals_user_override() {
    env::set_var("CLP_TEST_ENV_POR", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_POR")
                .takes_value(true),
        )
        .try_get_matches_from(vec!["", "opt"]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 1);
    assert_eq!(m.value_of("arg").unwrap(), "opt");

    // see https://github.com/clap-rs/clap/issues/1835
    let values: Vec<_> = m.values_of("arg").unwrap().collect();
    assert_eq!(values, vec!["opt"]);
}

#[test]
fn multiple_one() {
    env::set_var("CLP_TEST_ENV_MO", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_MO")
                .takes_value(true)
                .use_value_delimiter(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.values_of("arg").unwrap().collect::<Vec<_>>(), vec!["env"]);
}

#[test]
fn multiple_three() {
    env::set_var("CLP_TEST_ENV_MULTI1", "env1,env2,env3");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_MULTI1")
                .takes_value(true)
                .use_value_delimiter(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
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

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_MULTI2")
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
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

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_PV")
                .takes_value(true)
                .possible_value("env"),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn not_possible_value() {
    env::set_var("CLP_TEST_ENV_NPV", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_NPV")
                .takes_value(true)
                .possible_value("never"),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_err());
}

#[test]
fn validator() {
    env::set_var("CLP_TEST_ENV_VDOR", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_VDOR")
                .takes_value(true)
                .validator(|s| {
                    if s == "env" {
                        Ok(())
                    } else {
                        Err("not equal".to_string())
                    }
                }),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(m.occurrences_of("arg"), 0);
    assert_eq!(m.value_of("arg").unwrap(), "env");
}

#[test]
fn validator_output() {
    env::set_var("CLP_TEST_ENV_VO", "42");

    let m = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_VO")
                .takes_value(true)
                .validator(|s| s.parse::<i32>()),
        )
        .try_get_matches_from(vec![""])
        .unwrap();

    assert_eq!(m.value_of("arg").unwrap().parse(), Ok(42));
}

#[test]
fn validator_invalid() {
    env::set_var("CLP_TEST_ENV_IV", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_IV")
                .takes_value(true)
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
