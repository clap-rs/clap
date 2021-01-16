//! Test `parse(auto)`.

use clap::Clap;
use std::ffi::{OsStr, OsString};

// Define structs which implement various traits.
#[derive(Debug, Eq, PartialEq)]
struct HasTryFromOsStr(String);
#[derive(Debug, Eq, PartialEq)]
struct HasTraitFromStr(String);
#[derive(Debug, Eq, PartialEq)]
struct HasTryFromStr(String);
#[derive(Debug, Eq, PartialEq)]
struct HasFromOsStr(String);
#[derive(Debug, Eq, PartialEq)]
struct HasFromStr(String);
#[derive(Debug, Eq, PartialEq)]
struct HasNone(String);

impl std::convert::TryFrom<&std::ffi::OsStr> for HasTryFromOsStr {
    type Error = String;

    fn try_from(s: &std::ffi::OsStr) -> Result<HasTryFromOsStr, Self::Error> {
        if format!("{:?}", s).contains("!") {
            return Err(format!("failed HasTryFromOsStr on '{:?}'", s));
        }
        Ok(HasTryFromOsStr(format!("HasTryFromOsStr({:?})", s)))
    }
}

impl std::str::FromStr for HasTraitFromStr {
    type Err = String;

    fn from_str(s: &str) -> Result<HasTraitFromStr, Self::Err> {
        if format!("{:?}", s).contains("!") {
            return Err(format!("failed HasTraitFromStr on '{:?}'", s));
        }
        Ok(HasTraitFromStr(format!("HasTraitFromStr({:?})", s)))
    }
}

impl std::convert::TryFrom<&str> for HasTryFromStr {
    type Error = String;

    fn try_from(s: &str) -> Result<HasTryFromStr, Self::Error> {
        if format!("{:?}", s).contains("!") {
            return Err(format!("failed HasTryFromStr on '{:?}'", s));
        }
        Ok(HasTryFromStr(format!("HasTryFromStr({:?})", s)))
    }
}

impl From<&std::ffi::OsStr> for HasFromOsStr {
    fn from(s: &std::ffi::OsStr) -> HasFromOsStr {
        HasFromOsStr(format!("HasFromOsStr({:?})", s))
    }
}

impl From<&str> for HasFromStr {
    fn from(s: &str) -> HasFromStr {
        HasFromStr(format!("HasFromStr({:?})", s))
    }
}

/// Define a `Clap` type which contains one of each of the above structs.
#[derive(Clap, PartialEq, Debug)]
struct Opt {
    #[clap(long, parse(auto))]
    has_try_from_os_str: HasTryFromOsStr,

    #[clap(long, parse(auto))]
    has_trait_from_str: HasTraitFromStr,

    #[clap(long, parse(auto))]
    has_try_from_str: HasTryFromStr,

    #[clap(long, parse(auto))]
    has_from_os_str: HasFromOsStr,

    #[clap(long, parse(auto))]
    has_from_str: HasFromStr,
}

/// Test successful parsing with each trait.
#[test]
fn auto_basic() {
    assert_eq!(
        Opt {
            has_try_from_os_str: HasTryFromOsStr("HasTryFromOsStr(\"red\")".to_string()),
            has_trait_from_str: HasTraitFromStr("HasTraitFromStr(\"orange\")".to_string()),
            has_try_from_str: HasTryFromStr("HasTryFromStr(\"yellow\")".to_string()),
            has_from_os_str: HasFromOsStr("HasFromOsStr(\"green\")".to_string()),
            has_from_str: HasFromStr("HasFromStr(\"blue\")".to_string()),
        },
        Opt::try_parse_from(&[
            "test",
            "--has-try-from-os-str",
            "red",
            "--has-trait-from-str",
            "orange",
            "--has-try-from-str",
            "yellow",
            "--has-from-os-str",
            "green",
            "--has-from-str",
            "blue",
        ])
        .unwrap()
    );
}

// Test invalid UTF-8 with each trait which needs an `&OsStr`.

#[test]
fn auto_invalid_encodings_red() {
    assert_eq!(
        Opt {
            #[cfg(not(windows))]
            has_try_from_os_str: HasTryFromOsStr(
                "HasTryFromOsStr(\"\\xED\\xA0\\x80\")".to_string()
            ),
            #[cfg(windows)]
            has_try_from_os_str: HasTryFromOsStr("HasTryFromOsStr(\"\\u{d800}\")".to_string()),
            has_trait_from_str: HasTraitFromStr("HasTraitFromStr(\"orange\")".to_string()),
            has_try_from_str: HasTryFromStr("HasTryFromStr(\"yellow\")".to_string()),
            has_from_os_str: HasFromOsStr("HasFromOsStr(\"green\")".to_string()),
            has_from_str: HasFromStr("HasFromStr(\"blue\")".to_string()),
        },
        Opt::try_parse_from(&[
            to_os_string(b"test"),
            to_os_string(b"--has-try-from-os-str"),
            to_os_string(b"\xed\xa0\x80"),
            to_os_string(b"--has-trait-from-str"),
            to_os_string(b"orange"),
            to_os_string(b"--has-try-from-str"),
            to_os_string(b"yellow"),
            to_os_string(b"--has-from-os-str"),
            to_os_string(b"green"),
            to_os_string(b"--has-from-str"),
            to_os_string(b"blue"),
        ])
        .unwrap()
    );
}

#[test]
fn auto_invalid_encodings_green() {
    assert_eq!(
        Opt {
            has_try_from_os_str: HasTryFromOsStr("HasTryFromOsStr(\"red\")".to_string()),
            has_trait_from_str: HasTraitFromStr("HasTraitFromStr(\"orange\")".to_string()),
            has_try_from_str: HasTryFromStr("HasTryFromStr(\"yellow\")".to_string()),
            #[cfg(not(windows))]
            has_from_os_str: HasFromOsStr("HasFromOsStr(\"\\xED\\xA0\\x80\")".to_string()),
            #[cfg(windows)]
            has_from_os_str: HasFromOsStr("HasFromOsStr(\"\\u{d800}\")".to_string()),
            has_from_str: HasFromStr("HasFromStr(\"blue\")".to_string()),
        },
        Opt::try_parse_from(&[
            to_os_string(b"test"),
            to_os_string(b"--has-try-from-os-str"),
            to_os_string(b"red"),
            to_os_string(b"--has-trait-from-str"),
            to_os_string(b"orange"),
            to_os_string(b"--has-try-from-str"),
            to_os_string(b"yellow"),
            to_os_string(b"--has-from-os-str"),
            to_os_string(b"\xed\xa0\x80"),
            to_os_string(b"--has-from-str"),
            to_os_string(b"blue"),
        ])
        .unwrap()
    );
}

// Test invalid UTF-8 with each trait which needs a `&str`.

#[test]
fn auto_invalid_encodings_orange() {
    assert_eq!(
        format!(
            "error: Invalid value for \'has-trait-from-str\': \
                 The argument \'{}\' isn\'t a valid encoding for \'has-trait-from-str\'\n\n\
                 For more information try --help\n",
            to_os_string(b"\xed\xa0\x80").to_string_lossy()
        ),
        Opt::try_parse_from(&[
            to_os_string(b"test"),
            to_os_string(b"--has-try-from-os-str"),
            to_os_string(b"red"),
            to_os_string(b"--has-trait-from-str"),
            to_os_string(b"\xed\xa0\x80"),
            to_os_string(b"--has-try-from-str"),
            to_os_string(b"yellow"),
            to_os_string(b"--has-from-os-str"),
            to_os_string(b"green"),
            to_os_string(b"--has-from-str"),
            to_os_string(b"blue"),
        ])
        .unwrap_err()
        .to_string()
    );
}

#[test]
fn auto_invalid_encodings_yellow() {
    assert_eq!(
        format!(
            "error: Invalid value for \'has-try-from-str\': \
                 The argument \'{}\' isn\'t a valid encoding for \'has-try-from-str\'\n\n\
                 For more information try --help\n",
            to_os_string(b"\xed\xa0\x80").to_string_lossy()
        ),
        Opt::try_parse_from(&[
            to_os_string(b"test"),
            to_os_string(b"--has-try-from-os-str"),
            to_os_string(b"red"),
            to_os_string(b"--has-trait-from-str"),
            to_os_string(b"orange"),
            to_os_string(b"--has-try-from-str"),
            to_os_string(b"\xed\xa0\x80"),
            to_os_string(b"--has-from-os-str"),
            to_os_string(b"green"),
            to_os_string(b"--has-from-str"),
            to_os_string(b"blue"),
        ])
        .unwrap_err()
        .to_string()
    );
}

#[test]
fn auto_invalid_encodings_blue() {
    assert_eq!(
        format!(
            "error: Invalid value for \'has-from-str\': \
                 The argument \'{}\' isn\'t a valid encoding for \'has-from-str\'\n\n\
                 For more information try --help\n",
            to_os_string(b"\xed\xa0\x80").to_string_lossy()
        ),
        Opt::try_parse_from(&[
            to_os_string(b"test"),
            to_os_string(b"--has-try-from-os-str"),
            to_os_string(b"red"),
            to_os_string(b"--has-trait-from-str"),
            to_os_string(b"orange"),
            to_os_string(b"--has-try-from-str"),
            to_os_string(b"yellow"),
            to_os_string(b"--has-from-os-str"),
            to_os_string(b"green"),
            to_os_string(b"--has-from-str"),
            to_os_string(b"\xed\xa0\x80"),
        ])
        .unwrap_err()
        .to_string()
    );
}

// Test parse errors with traits that can have errors.

#[test]
fn auto_parse_errors_red() {
    assert_eq!(
        "error: Invalid value for \'has-try-from-os-str\': \
         The argument \'red!\' isn\'t a valid value for \'has-try-from-os-str\': failed HasTryFromOsStr on \'\"red!\"\'\n\n\
         For more information try --help\n",
        Opt::try_parse_from(&[
            "test",
            "--has-try-from-os-str", "red!",
            "--has-trait-from-str", "orange",
            "--has-try-from-str", "yellow",
            "--has-from-os-str", "green",
            "--has-from-str", "blue",
        ]).unwrap_err().to_string()
    );
}

#[test]
fn auto_parse_errors_orange() {
    assert_eq!(
        "error: Invalid value for \'has-trait-from-str\': \
         The argument \'orange!\' isn\'t a valid value for \'has-trait-from-str\': failed HasTraitFromStr on \'\"orange!\"\'\n\n\
         For more information try --help\n",
        Opt::try_parse_from(&[
            "test",
            "--has-try-from-os-str", "red",
            "--has-trait-from-str", "orange!",
            "--has-try-from-str", "yellow",
            "--has-from-os-str", "green",
            "--has-from-str", "blue",
        ]).unwrap_err().to_string()
    );
}

#[test]
fn auto_parse_errors_yellow() {
    assert_eq!(
        "error: Invalid value for \'has-try-from-str\': \
         The argument \'yellow!\' isn\'t a valid value for \'has-try-from-str\': failed HasTryFromStr on \'\"yellow!\"\'\n\n\
         For more information try --help\n",
        Opt::try_parse_from(&[
            "test",
            "--has-try-from-os-str", "red",
            "--has-trait-from-str", "orange",
            "--has-try-from-str", "yellow!",
            "--has-from-os-str", "green",
            "--has-from-str", "blue",
        ]).unwrap_err().to_string()
    );
}

// Test a type which doesn't implement any parsing traits, to ensure that we
// emit a friendly error message.

#[derive(Clap, PartialEq, Debug)]
struct NoneOpt {
    #[clap(long, parse(auto))]
    has_none: HasNone,
}

#[test]
fn auto_none() {
    assert_eq!(
        "error: Invalid value for \'has-none\': \
         The argument \'colorless\' isn\'t a valid value for \'has-none\': \
         Type `HasNone` does not implement any of the parsing traits: `clap::ArgEnum`, `TryFrom<&OsStr>`, `FromStr`, `TryFrom<&str>`, `From<&OsStr>`, or `From<&str>`\n\n\
         For more information try --help\n".to_string(),
        NoneOpt::try_parse_from(&[
            "test",
            "--has-none", "colorless",
        ]).unwrap_err().to_string()
    );
}

// Test that Vec-of-T parsing works with `auto`.
#[derive(Clap, PartialEq, Debug)]
struct VecOpt {
    #[clap(long, parse(auto))]
    has_try_from_os_str: Vec<HasTryFromOsStr>,

    #[clap(long, parse(auto))]
    has_trait_from_str: Vec<HasTraitFromStr>,

    #[clap(long, parse(auto))]
    has_try_from_str: Vec<HasTryFromStr>,

    #[clap(long, parse(auto))]
    has_from_os_str: Vec<HasFromOsStr>,

    #[clap(long, parse(auto))]
    has_from_str: Vec<HasFromStr>,
}

#[test]
fn auto_vec() {
    assert_eq!(
        VecOpt {
            has_try_from_os_str: vec![HasTryFromOsStr("HasTryFromOsStr(\"red\")".to_string())],
            has_trait_from_str: vec![HasTraitFromStr("HasTraitFromStr(\"orange\")".to_string())],
            has_try_from_str: vec![HasTryFromStr("HasTryFromStr(\"yellow\")".to_string())],
            has_from_os_str: vec![HasFromOsStr("HasFromOsStr(\"green\")".to_string())],
            has_from_str: vec![HasFromStr("HasFromStr(\"blue\")".to_string())],
        },
        VecOpt::try_parse_from(&[
            "test",
            "--has-try-from-os-str",
            "red",
            "--has-trait-from-str",
            "orange",
            "--has-try-from-str",
            "yellow",
            "--has-from-os-str",
            "green",
            "--has-from-str",
            "blue",
        ])
        .unwrap()
    );
}

/// Convert from bytes to `&OsStr`.
#[inline(never)]
fn to_os_string(bytes: &[u8]) -> OsString {
    use os_str_bytes::{EncodingError, OsStrBytes};
    use std::borrow::Cow;

    let t: Result<Cow<OsStr>, EncodingError> = OsStrBytes::from_bytes(bytes);
    t.unwrap().into_owned()
}
