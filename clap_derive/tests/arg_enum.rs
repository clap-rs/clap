// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use clap::{ArgEnum, Parser, PossibleValue};

#[test]
fn basic() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::try_parse_from(&["", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Bar
        },
        Opt::try_parse_from(&["", "bar"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "fOo"]).is_err());
}

#[test]
fn default_value() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    impl Default for ArgChoice {
        fn default() -> Self {
            Self::Bar
        }
    }

    impl std::fmt::Display for ArgChoice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(self.to_possible_value().unwrap().get_name(), f)
        }
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, default_value_t)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::try_parse_from(&["", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Bar
        },
        Opt::try_parse_from(&["", "bar"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Bar
        },
        Opt::try_parse_from(&[""]).unwrap()
    );
}

#[test]
fn multi_word_is_renamed_kebab() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    #[allow(non_camel_case_types)]
    enum ArgChoice {
        FooBar,
        BAR_BAZ,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::try_parse_from(&["", "foo-bar"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::BAR_BAZ
        },
        Opt::try_parse_from(&["", "bar-baz"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
}

#[test]
fn variant_with_defined_casing() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        #[clap(rename_all = "screaming_snake")]
        FooBar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::try_parse_from(&["", "FOO_BAR"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
}

#[test]
fn casing_is_propagated_from_parent() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        FooBar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::try_parse_from(&["", "FOO_BAR"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
}

#[test]
fn casing_propagation_is_overridden() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        #[clap(rename_all = "camel")]
        FooBar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::try_parse_from(&["", "fooBar"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
    assert!(Opt::try_parse_from(&["", "FOO_BAR"]).is_err());
}

#[test]
fn case_insensitive() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(true))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::try_parse_from(&["", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::try_parse_from(&["", "fOo"]).unwrap()
    );
}

#[test]
fn case_insensitive_set_to_false() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(false))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::try_parse_from(&["", "foo"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "fOo"]).is_err());
}

#[test]
fn alias() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        #[clap(alias = "TOTP")]
        TOTP,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(false))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::try_parse_from(&["", "totp"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::try_parse_from(&["", "TOTP"]).unwrap()
    );
}

#[test]
fn multiple_alias() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        #[clap(alias = "TOTP", alias = "t")]
        TOTP,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(false))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::try_parse_from(&["", "totp"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::try_parse_from(&["", "TOTP"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::try_parse_from(&["", "t"]).unwrap()
    );
}

#[test]
fn skip_variant() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    #[allow(dead_code)] // silence warning about `Baz` being unused
    enum ArgChoice {
        Foo,
        Bar,
        #[clap(skip)]
        Baz,
    }

    assert_eq!(
        ArgChoice::value_variants()
            .iter()
            .map(ArgEnum::to_possible_value)
            .map(Option::unwrap)
            .collect::<Vec<_>>(),
        vec![PossibleValue::new("foo"), PossibleValue::new("bar")]
    );
    assert!(ArgChoice::from_str("foo", true).is_ok());
    assert!(ArgChoice::from_str("bar", true).is_ok());
    assert!(ArgChoice::from_str("baz", true).is_err());
}

#[test]
fn from_str_invalid() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
    }

    assert!(ArgChoice::from_str("bar", true).is_err());
}

#[test]
fn option_type() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: Option<ArgChoice>,
    }

    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&[""]).unwrap());
    assert_eq!(
        Opt {
            arg: Some(ArgChoice::Foo)
        },
        Opt::try_parse_from(&["", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: Some(ArgChoice::Bar)
        },
        Opt::try_parse_from(&["", "bar"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "fOo"]).is_err());
}

#[test]
fn option_option_type() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, long)]
        arg: Option<Option<ArgChoice>>,
    }

    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&[""]).unwrap());
    assert_eq!(
        Opt { arg: Some(None) },
        Opt::try_parse_from(&["", "--arg"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: Some(Some(ArgChoice::Foo))
        },
        Opt::try_parse_from(&["", "--arg", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: Some(Some(ArgChoice::Bar))
        },
        Opt::try_parse_from(&["", "--arg", "bar"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "--arg", "fOo"]).is_err());
}

#[test]
fn vec_type() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, short, long)]
        arg: Vec<ArgChoice>,
    }

    assert_eq!(Opt { arg: vec![] }, Opt::try_parse_from(&[""]).unwrap());
    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo]
        },
        Opt::try_parse_from(&["", "-a", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&["", "-a", "foo", "-a", "bar"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "-a", "fOo"]).is_err());
}

#[test]
fn option_vec_type() {
    #[derive(ArgEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, short, long)]
        arg: Option<Vec<ArgChoice>>,
    }

    assert_eq!(Opt { arg: None }, Opt::try_parse_from(&[""]).unwrap());
    assert_eq!(
        Opt {
            arg: Some(vec![ArgChoice::Foo])
        },
        Opt::try_parse_from(&["", "-a", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: Some(vec![ArgChoice::Foo, ArgChoice::Bar])
        },
        Opt::try_parse_from(&["", "-a", "foo", "-a", "bar"]).unwrap()
    );
    assert!(Opt::try_parse_from(&["", "-a", "fOo"]).is_err());
}
