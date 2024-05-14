use std::path::PathBuf;

use clap::{CommandFactory, Parser};

use crate::utils;

#[test]
fn default_value() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(default_value = "3")]
        arg: i32,
    }
    assert_eq!(Opt { arg: 3 }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(["test", "1"]).unwrap());

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}

#[test]
fn default_value_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(default_value_t = 3)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 3 }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(["test", "1"]).unwrap());

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: 3]"));
}

#[test]
fn auto_default_value_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(default_value_t)]
        arg: i32,
    }
    assert_eq!(Opt { arg: 0 }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(Opt { arg: 1 }, Opt::try_parse_from(["test", "1"]).unwrap());

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: 0]"));
}

#[test]
fn default_values_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(default_values_t = vec![1, 2, 3])]
        arg1: Vec<i32>,

        #[arg(long, default_values_t = [4, 5, 6])]
        arg2: Vec<i32>,

        #[arg(long, default_values_t = [7, 8, 9])]
        arg3: Vec<i32>,

        #[arg(long, default_values_t = 10..=12)]
        arg4: Vec<i32>,

        #[arg(long, default_values_t = vec!["hello".to_string(), "world".to_string()])]
        arg5: Vec<String>,

        #[arg(long, default_values_t = &vec!["foo".to_string(), "bar".to_string()])]
        arg6: Vec<String>,
    }
    assert_eq!(
        Opt {
            arg1: vec![1, 2, 3],
            arg2: vec![4, 5, 6],
            arg3: vec![7, 8, 9],
            arg4: vec![10, 11, 12],
            arg5: vec!["hello".to_string(), "world".to_string()],
            arg6: vec!["foo".to_string(), "bar".to_string()],
        },
        Opt::try_parse_from(["test"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg1: vec![1],
            arg2: vec![4, 5, 6],
            arg3: vec![7, 8, 9],
            arg4: vec![10, 11, 12],
            arg5: vec!["hello".to_string(), "world".to_string()],
            arg6: vec!["foo".to_string(), "bar".to_string()],
        },
        Opt::try_parse_from(["test", "1"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg1: vec![1, 2, 3],
            arg2: vec![4, 5, 6],
            arg3: vec![7, 8, 9],
            arg4: vec![42, 15],
            arg5: vec!["baz".to_string()],
            arg6: vec!["foo".to_string(), "bar".to_string()],
        },
        Opt::try_parse_from(["test", "--arg4", "42", "--arg4", "15", "--arg5", "baz"]).unwrap()
    );

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: 1 2 3]"));
}

#[test]
fn default_value_os_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(default_value_os_t = PathBuf::from("abc.def"))]
        arg: PathBuf,
    }
    assert_eq!(
        Opt {
            arg: PathBuf::from("abc.def")
        },
        Opt::try_parse_from(["test"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: PathBuf::from("ghi")
        },
        Opt::try_parse_from(["test", "ghi"]).unwrap()
    );

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: abc.def]"));
}

#[test]
fn default_values_os_t() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(
            default_values_os_t = vec![PathBuf::from("abc.def"), PathBuf::from("123.foo")]
        )]
        arg1: Vec<PathBuf>,

        #[arg(
            long,
            default_values_os_t = [PathBuf::from("bar.baz")]
        )]
        arg2: Vec<PathBuf>,
    }
    assert_eq!(
        Opt {
            arg1: vec![PathBuf::from("abc.def"), PathBuf::from("123.foo")],
            arg2: vec![PathBuf::from("bar.baz")]
        },
        Opt::try_parse_from(["test"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg1: vec![PathBuf::from("ghi")],
            arg2: vec![PathBuf::from("baz.bar"), PathBuf::from("foo.bar")]
        },
        Opt::try_parse_from(["test", "ghi", "--arg2", "baz.bar", "--arg2", "foo.bar"]).unwrap()
    );

    let help = utils::get_long_help::<Opt>();
    assert!(help.contains("[default: abc.def 123.foo]"));
}

#[test]
fn detect_os_variant() {
    #![allow(deprecated)]

    #[derive(clap::Parser)]
    pub(crate) struct Options {
        #[arg(default_value_os = "123")]
        x: String,
    }
    Options::command().debug_assert();
}
