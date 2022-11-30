#![cfg(feature = "env")]

use std::env;
use std::ffi::OsStr;

use clap::{arg, builder::FalseyValueParser, Arg, ArgAction, Command};

#[test]
fn env() {
    env::set_var("CLP_TEST_ENV", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV")
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.value_source("arg").unwrap(),
        clap::parser::ValueSource::EnvVariable
    );
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "env"
    );
}

#[test]
fn env_bool_literal() {
    env::set_var("CLP_TEST_FLAG_TRUE", "On");
    env::set_var("CLP_TEST_FLAG_FALSE", "nO");

    let r = Command::new("df")
        .arg(
            Arg::new("present")
                .short('p')
                .env("CLP_TEST_FLAG_TRUE")
                .action(ArgAction::SetTrue)
                .value_parser(FalseyValueParser::new()),
        )
        .arg(
            Arg::new("negated")
                .short('n')
                .env("CLP_TEST_FLAG_FALSE")
                .action(ArgAction::SetTrue)
                .value_parser(FalseyValueParser::new()),
        )
        .arg(
            Arg::new("absent")
                .short('a')
                .env("CLP_TEST_FLAG_ABSENT")
                .action(ArgAction::SetTrue)
                .value_parser(FalseyValueParser::new()),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(*m.get_one::<bool>("present").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("negated").expect("defaulted by clap"));
    assert!(!*m.get_one::<bool>("absent").expect("defaulted by clap"));
}

#[test]
fn env_os() {
    env::set_var("CLP_TEST_ENV_OS", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env(OsStr::new("CLP_TEST_ENV_OS"))
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "env"
    );
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
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(!m.contains_id("arg"));
    assert_eq!(m.value_source("arg"), None);
    assert_eq!(m.get_one::<String>("arg").map(|v| v.as_str()), None);
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
    assert!(!m.contains_id("arg"));
    assert_eq!(m.value_source("arg"), None);
    assert_eq!(m.get_one::<String>("arg").map(|v| v.as_str()), None);
}

#[test]
fn with_default() {
    env::set_var("CLP_TEST_ENV_WD", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_WD")
                .action(ArgAction::Set)
                .default_value("default"),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.value_source("arg").unwrap(),
        clap::parser::ValueSource::EnvVariable
    );
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "env"
    );
}

#[test]
fn opt_user_override() {
    env::set_var("CLP_TEST_ENV_OR", "env");

    let r = Command::new("df")
        .arg(
            arg!(--arg [FILE] "some arg")
                .env("CLP_TEST_ENV_OR")
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "--arg", "opt"]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.value_source("arg").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "opt"
    );

    // see https://github.com/clap-rs/clap/issues/1835
    let values: Vec<_> = m
        .get_many::<String>("arg")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(values, vec!["opt"]);
}

#[test]
fn positionals() {
    env::set_var("CLP_TEST_ENV_P", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_P")
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.value_source("arg").unwrap(),
        clap::parser::ValueSource::EnvVariable
    );
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "env"
    );
}

#[test]
fn positionals_user_override() {
    env::set_var("CLP_TEST_ENV_POR", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_POR")
                .action(ArgAction::Set),
        )
        .try_get_matches_from(vec!["", "opt"]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.value_source("arg").unwrap(),
        clap::parser::ValueSource::CommandLine
    );
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "opt"
    );

    // see https://github.com/clap-rs/clap/issues/1835
    let values: Vec<_> = m
        .get_many::<String>("arg")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
    assert_eq!(values, vec!["opt"]);
}

#[test]
fn multiple_one() {
    env::set_var("CLP_TEST_ENV_MO", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_MO")
                .action(ArgAction::Set)
                .value_delimiter(',')
                .num_args(1..),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_many::<String>("arg")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        vec!["env"]
    );
}

#[test]
fn multiple_three() {
    env::set_var("CLP_TEST_ENV_MULTI1", "env1,env2,env3");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_MULTI1")
                .action(ArgAction::Set)
                .value_delimiter(',')
                .num_args(1..),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_many::<String>("arg")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
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
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_many::<String>("arg")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
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
                .action(ArgAction::Set)
                .value_parser(["env"]),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "env"
    );
}

#[test]
fn not_possible_value() {
    env::set_var("CLP_TEST_ENV_NPV", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_NPV")
                .action(ArgAction::Set)
                .value_parser(["never"]),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_err());
}

#[test]
fn value_parser() {
    env::set_var("CLP_TEST_ENV_VDOR", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_VDOR")
                .action(ArgAction::Set)
                .value_parser(|s: &str| -> Result<String, String> {
                    if s == "env" {
                        Ok(s.to_owned())
                    } else {
                        Err("not equal".to_owned())
                    }
                }),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.contains_id("arg"));
    assert_eq!(
        m.get_one::<String>("arg").map(|v| v.as_str()).unwrap(),
        "env"
    );
}

#[test]
fn value_parser_output() {
    env::set_var("CLP_TEST_ENV_VO", "42");

    let m = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_VO")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(i32)),
        )
        .try_get_matches_from(vec![""])
        .unwrap();

    assert_eq!(*m.get_one::<i32>("arg").unwrap(), 42);
}

#[test]
fn value_parser_invalid() {
    env::set_var("CLP_TEST_ENV_IV", "env");

    let r = Command::new("df")
        .arg(
            arg!([arg] "some opt")
                .env("CLP_TEST_ENV_IV")
                .action(ArgAction::Set)
                .value_parser(|s: &str| -> Result<String, String> {
                    if s != "env" {
                        Ok(s.to_owned())
                    } else {
                        Err("is equal".to_string())
                    }
                }),
        )
        .try_get_matches_from(vec![""]);

    assert!(r.is_err());
}
