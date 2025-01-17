use clap::{builder::PossibleValue, error::ErrorKind, Arg, ArgAction, Command};

#[cfg(feature = "error-context")]
use super::utils;

#[test]
fn possible_values_of_positional() {
    let m = Command::new("possible_values")
        .arg(Arg::new("positional").index(1).value_parser(["test123"]))
        .try_get_matches_from(vec!["myprog", "test123"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("positional"));
    assert_eq!(
        m.get_one::<String>("positional").map(|v| v.as_str()),
        Some("test123")
    );
}

#[test]
fn possible_value_arg_value() {
    let m = Command::new("possible_values")
        .arg(
            Arg::new("arg_value")
                .index(1)
                .value_parser([PossibleValue::new("test123")
                    .hide(false)
                    .help("It's just a test")]),
        )
        .try_get_matches_from(vec!["myprog", "test123"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("arg_value"));
    assert_eq!(
        m.get_one::<String>("arg_value").map(|v| v.as_str()),
        Some("test123")
    );
}

#[test]
fn possible_values_of_positional_fail() {
    let m = Command::new("possible_values")
        .arg(Arg::new("positional").index(1).value_parser(["test123"]))
        .try_get_matches_from(vec!["myprog", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn possible_values_of_positional_multiple() {
    let m = Command::new("possible_values")
        .arg(
            Arg::new("positional")
                .index(1)
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"])
                .num_args(1..),
        )
        .try_get_matches_from(vec!["myprog", "test123", "test321"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("positional"));
    assert_eq!(
        m.get_many::<String>("positional")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        vec!["test123", "test321"]
    );
}

#[test]
fn possible_values_of_positional_multiple_fail() {
    let m = Command::new("possible_values")
        .arg(
            Arg::new("positional")
                .index(1)
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"])
                .num_args(1..),
        )
        .try_get_matches_from(vec!["myprog", "test123", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn possible_values_of_option() {
    let m = Command::new("possible_values")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123"]),
        )
        .try_get_matches_from(vec!["myprog", "--option", "test123"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_one::<String>("option").map(|v| v.as_str()),
        Some("test123")
    );
}

#[test]
fn possible_values_of_option_fail() {
    let m = Command::new("possible_values")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123"]),
        )
        .try_get_matches_from(vec!["myprog", "--option", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn possible_values_of_option_multiple() {
    let m = Command::new("possible_values")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"])
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "--option", "test123", "--option", "test321"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.contains_id("option"));
    assert_eq!(
        m.get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        vec!["test123", "test321"]
    );
}

#[test]
fn possible_values_of_option_multiple_fail() {
    let m = Command::new("possible_values")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"])
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["", "--option", "test123", "--option", "notest"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
#[cfg(feature = "error-context")]
fn possible_values_output() {
    #[cfg(feature = "suggestions")]
    static PV_ERROR: &str = "\
error: invalid value 'slo' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

  tip: a similar value exists: 'slow'

For more information, try '--help'.
";

    #[cfg(not(feature = "suggestions"))]
    static PV_ERROR: &str = "\
error: invalid value 'slo' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

For more information, try '--help'.
";

    utils::assert_output(
        Command::new("test").arg(
            Arg::new("option")
                .short('O')
                .action(ArgAction::Set)
                .value_parser(["slow", "fast", "ludicrous speed"]),
        ),
        "clap-test -O slo",
        PV_ERROR,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn possible_values_alias_output() {
    #[cfg(feature = "suggestions")]
    static PV_ERROR: &str = "\
error: invalid value 'slo' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

  tip: a similar value exists: 'slow'

For more information, try '--help'.
";

    #[cfg(not(feature = "suggestions"))]
    static PV_ERROR: &str = "\
error: invalid value 'slo' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

For more information, try '--help'.
";

    utils::assert_output(
        Command::new("test").arg(
            Arg::new("option")
                .short('O')
                .action(ArgAction::Set)
                .value_parser([
                    "slow".into(),
                    PossibleValue::new("fast").alias("fost"),
                    PossibleValue::new("ludicrous speed").aliases(["ls", "lcs"]),
                ]),
        ),
        "clap-test -O slo",
        PV_ERROR,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn possible_values_hidden_output() {
    #[cfg(feature = "suggestions")]
    static PV_ERROR: &str = "\
error: invalid value 'slo' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

  tip: a similar value exists: 'slow'

For more information, try '--help'.
";

    #[cfg(not(feature = "suggestions"))]
    static PV_ERROR: &str = "\
error: invalid value 'slo' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

For more information, try '--help'.
";

    utils::assert_output(
        Command::new("test").arg(
            Arg::new("option")
                .short('O')
                .action(ArgAction::Set)
                .value_parser([
                    "slow".into(),
                    "fast".into(),
                    PossibleValue::new("ludicrous speed"),
                    PossibleValue::new("forbidden speed").hide(true),
                ]),
        ),
        "clap-test -O slo",
        PV_ERROR,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn escaped_possible_values_output() {
    #[cfg(feature = "suggestions")]
    static PV_ERROR_ESCAPED: &str = "\
error: invalid value 'ludicrous' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

  tip: a similar value exists: 'ludicrous speed'

For more information, try '--help'.
";

    #[cfg(not(feature = "suggestions"))]
    static PV_ERROR_ESCAPED: &str = "\
error: invalid value 'ludicrous' for '-O <option>'
  [possible values: slow, fast, \"ludicrous speed\"]

For more information, try '--help'.
";

    utils::assert_output(
        Command::new("test").arg(
            Arg::new("option")
                .short('O')
                .action(ArgAction::Set)
                .value_parser(["slow", "fast", "ludicrous speed"]),
        ),
        "clap-test -O ludicrous",
        PV_ERROR_ESCAPED,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn missing_possible_value_error() {
    static MISSING_PV_ERROR: &str = "\
error: a value is required for '-O <option>' but none was supplied
  [possible values: slow, fast, \"ludicrous speed\"]

For more information, try '--help'.
";

    utils::assert_output(
        Command::new("test").arg(
            Arg::new("option")
                .short('O')
                .action(ArgAction::Set)
                .value_parser([
                    "slow".into(),
                    PossibleValue::new("fast").alias("fost"),
                    PossibleValue::new("ludicrous speed"),
                    PossibleValue::new("forbidden speed").hide(true),
                ]),
        ),
        "clap-test -O",
        MISSING_PV_ERROR,
        true,
    );
}

#[test]
fn alias() {
    let m = Command::new("pv")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser([PossibleValue::new("test123").alias("123"), "test321".into()])
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "123"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m
        .unwrap()
        .get_one::<String>("option")
        .map(|v| v.as_str())
        .unwrap()
        .eq("123"));
}

#[test]
fn aliases() {
    let m = Command::new("pv")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser([
                    PossibleValue::new("test123").aliases(["1", "2", "3"]),
                    "test321".into(),
                ])
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "2"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m
        .unwrap()
        .get_one::<String>("option")
        .map(|v| v.as_str())
        .unwrap()
        .eq("2"));
}

#[test]
fn ignore_case() {
    let m = Command::new("pv")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"])
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "TeSt123"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m
        .unwrap()
        .get_one::<String>("option")
        .map(|v| v.as_str())
        .unwrap()
        .eq_ignore_ascii_case("test123"));
}

#[test]
fn ignore_case_fail() {
    let m = Command::new("pv")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"]),
        )
        .try_get_matches_from(vec!["pv", "--option", "TeSt123"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[test]
fn ignore_case_multiple() {
    let m = Command::new("pv")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"])
                .num_args(1..)
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["pv", "--option", "TeSt123", "teST123", "tESt321"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert_eq!(
        m.unwrap()
            .get_many::<String>("option")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["TeSt123", "teST123", "tESt321"]
    );
}

#[test]
fn ignore_case_multiple_fail() {
    let m = Command::new("pv")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .action(ArgAction::Set)
                .value_parser(["test123", "test321"])
                .num_args(1..),
        )
        .try_get_matches_from(vec!["pv", "--option", "test123", "teST123", "test321"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidValue);
}

#[cfg(feature = "string")]
mod expensive {
    use std::sync::{Arc, Mutex};

    use clap::{Arg, Command};
    use clap_builder::builder::{PossibleValue, PossibleValuesParser, TypedValueParser};

    #[cfg(feature = "error-context")]
    use super::utils;

    #[derive(Clone)]
    struct ExpensiveValues {
        iterated: Arc<Mutex<bool>>,
    }

    impl ExpensiveValues {
        pub(crate) fn new() -> Self {
            ExpensiveValues {
                iterated: Arc::new(Mutex::new(false)),
            }
        }
    }

    impl IntoIterator for ExpensiveValues {
        type Item = String;

        type IntoIter = ExpensiveValuesIntoIterator;

        fn into_iter(self) -> Self::IntoIter {
            ExpensiveValuesIntoIterator { me: self, index: 0 }
        }
    }

    struct ExpensiveValuesIntoIterator {
        me: ExpensiveValues,
        index: usize,
    }

    impl Iterator for ExpensiveValuesIntoIterator {
        type Item = String;
        fn next(&mut self) -> Option<String> {
            let mut guard = self
                .me
                .iterated
                .lock()
                .expect("not working across multiple threads");

            *guard = true;
            self.index += 1;

            if self.index < 3 {
                Some(format!("expensive-value-{}", self.index))
            } else {
                None
            }
        }
    }

    impl TypedValueParser for ExpensiveValues {
        type Value = String;

        fn parse_ref(
            &self,
            _cmd: &Command,
            _arg: Option<&Arg>,
            _value: &std::ffi::OsStr,
            _index: isize,
        ) -> Result<Self::Value, clap_builder::Error> {
            unimplemented!()
        }

        fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
            Some(Box::new(self.clone().into_iter().map(PossibleValue::from)))
        }
    }

    #[test]
    fn no_iterate_when_hidden() {
        static PV_EXPECTED: &str = "\
Usage: clap-test [some-cheap-option] [some-expensive-option]

Arguments:
  [some-cheap-option]      cheap [possible values: some, cheap, values]
  [some-expensive-option]  expensive

Options:
  -h, --help  Print help
";
        let expensive = ExpensiveValues::new();
        utils::assert_output(
            Command::new("test")
                .arg(
                    Arg::new("some-cheap-option")
                        .help("cheap")
                        .value_parser(PossibleValuesParser::new(["some", "cheap", "values"])),
                )
                .arg(
                    Arg::new("some-expensive-option")
                        .help("expensive")
                        .hide_possible_values(true)
                        .value_parser(expensive.clone()),
                ),
            "clap-test -h",
            PV_EXPECTED,
            false,
        );
        assert_eq!(*expensive.iterated.lock().unwrap(), false);
    }

    #[test]
    fn iterate_when_displayed() {
        static PV_EXPECTED: &str = "\
Usage: clap-test [some-cheap-option] [some-expensive-option]

Arguments:
  [some-cheap-option]      cheap [possible values: some, cheap, values]
  [some-expensive-option]  expensive [possible values: expensive-value-1, expensive-value-2]

Options:
  -h, --help  Print help
";
        let expensive = ExpensiveValues::new();
        utils::assert_output(
            Command::new("test")
                .arg(
                    Arg::new("some-cheap-option")
                        .help("cheap")
                        .value_parser(PossibleValuesParser::new(["some", "cheap", "values"])),
                )
                .arg(
                    Arg::new("some-expensive-option")
                        .help("expensive")
                        .hide_possible_values(false)
                        .value_parser(expensive.clone()),
                ),
            "clap-test -h",
            PV_EXPECTED,
            false,
        );
        assert_eq!(*expensive.iterated.lock().unwrap(), true);
    }
}
