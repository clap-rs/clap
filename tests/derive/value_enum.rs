// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use clap::Parser;

#[test]
fn basic() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    impl Default for ArgChoice {
        fn default() -> Self {
            Self::Bar
        }
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser, default_value_t)]
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
fn vec_for_default_values_t() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, default_values_t = vec![ArgChoice::Foo, ArgChoice::Bar])]
        arg1: Vec<ArgChoice>,

        #[clap(
            long,
            value_enum,
            default_values_t = clap::ValueEnum::value_variants()
        )]
        arg2: Vec<ArgChoice>,
    }

    assert_eq!(
        Opt {
            arg1: vec![ArgChoice::Foo],
            arg2: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&["", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg1: vec![ArgChoice::Bar],
            arg2: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&["", "bar"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg1: vec![ArgChoice::Foo, ArgChoice::Bar],
            arg2: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&[""]).unwrap()
    );
    assert_eq!(
        Opt {
            arg1: vec![ArgChoice::Foo, ArgChoice::Bar],
            arg2: vec![ArgChoice::Foo]
        },
        Opt::try_parse_from(&["", "--arg2", "foo"]).unwrap()
    );
}

#[test]
fn vec_for_default_values_os_t() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, default_values_os_t = vec![ArgChoice::Foo, ArgChoice::Bar])]
        arg: Vec<ArgChoice>,

        #[clap(
            long,
            value_enum,
            default_values_os_t = clap::ValueEnum::value_variants()
        )]
        arg2: Vec<ArgChoice>,
    }

    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo],
            arg2: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&["", "foo"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Bar],
            arg2: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&["", "bar"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo, ArgChoice::Bar],
            arg2: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&[""]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo, ArgChoice::Bar],
            arg2: vec![ArgChoice::Foo]
        },
        Opt::try_parse_from(&["", "--arg2", "foo"]).unwrap()
    );
}

#[test]
fn multi_word_is_renamed_kebab() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    #[allow(non_camel_case_types)]
    enum ArgChoice {
        FooBar,
        BAR_BAZ,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        #[clap(rename_all = "screaming_snake")]
        FooBar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        FooBar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        #[clap(rename_all = "camel")]
        FooBar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser)]
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
fn ignore_case() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser, ignore_case(true))]
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
fn ignore_case_set_to_false() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, ignore_case(false), value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        #[clap(alias = "TOTP")]
        Totp,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, ignore_case(false), value_parser)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Totp
        },
        Opt::try_parse_from(&["", "totp"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Totp
        },
        Opt::try_parse_from(&["", "TOTP"]).unwrap()
    );
}

#[test]
fn multiple_alias() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        #[clap(alias = "TOTP", alias = "t")]
        Totp,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, ignore_case(false), value_parser)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Totp
        },
        Opt::try_parse_from(&["", "totp"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Totp
        },
        Opt::try_parse_from(&["", "TOTP"]).unwrap()
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Totp
        },
        Opt::try_parse_from(&["", "t"]).unwrap()
    );
}

#[test]
fn skip_variant() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    #[allow(dead_code)] // silence warning about `Baz` being unused
    enum ArgChoice {
        Foo,
        Bar,
        #[clap(skip)]
        Baz,
    }

    assert_eq!(
        <ArgChoice as clap::ValueEnum>::value_variants()
            .iter()
            .map(clap::ValueEnum::to_possible_value)
            .map(Option::unwrap)
            .collect::<Vec<_>>(),
        vec![
            clap::PossibleValue::new("foo"),
            clap::PossibleValue::new("bar")
        ]
    );

    {
        use clap::ValueEnum;
        assert!(ArgChoice::from_str("foo", true).is_ok());
        assert!(ArgChoice::from_str("bar", true).is_ok());
        assert!(ArgChoice::from_str("baz", true).is_err());
    }
}

#[test]
fn skip_non_unit_variant() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    #[allow(dead_code)] // silence warning about `Baz` being unused
    enum ArgChoice {
        Foo,
        Bar,
        #[clap(skip)]
        Baz(usize),
    }

    assert_eq!(
        <ArgChoice as clap::ValueEnum>::value_variants()
            .iter()
            .map(clap::ValueEnum::to_possible_value)
            .map(Option::unwrap)
            .collect::<Vec<_>>(),
        vec![
            clap::PossibleValue::new("foo"),
            clap::PossibleValue::new("bar")
        ]
    );

    {
        use clap::ValueEnum;
        assert!(ArgChoice::from_str("foo", true).is_ok());
        assert!(ArgChoice::from_str("bar", true).is_ok());
        assert!(ArgChoice::from_str("baz", true).is_err());
    }
}

#[test]
fn from_str_invalid() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
    }

    {
        use clap::ValueEnum;
        assert!(ArgChoice::from_str("bar", true).is_err());
    }
}

#[test]
fn option_type() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, long, value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, short, long, value_parser)]
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
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(value_enum, short, long, value_parser)]
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

#[test]
fn vec_type_default_value() {
    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        Foo,
        Bar,
        Baz,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[clap(
            value_enum,
            short,
            long,
            default_value = "foo,bar",
            value_delimiter = ',',
            value_parser
        )]
        arg: Vec<ArgChoice>,
    }

    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::try_parse_from(&[""]).unwrap()
    );

    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo, ArgChoice::Baz]
        },
        Opt::try_parse_from(&["", "-a", "foo,baz"]).unwrap()
    );
}
