// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use clap::{ArgEnum, ArgValue, Clap};

#[test]
fn basic() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::parse_from(&["", "foo"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Bar
        },
        Opt::parse_from(&["", "bar"])
    );
    assert!(Opt::try_parse_from(&["", "fOo"]).is_err());
}

#[test]
fn default_value() {
    #[derive(ArgEnum, PartialEq, Debug)]
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
            std::fmt::Display::fmt(self.as_arg().unwrap(), f)
        }
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, default_value_t)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::parse_from(&["", "foo"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Bar
        },
        Opt::parse_from(&["", "bar"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Bar
        },
        Opt::parse_from(&[""])
    );
}

#[test]
fn multi_word_is_renamed_kebab() {
    #[derive(ArgEnum, PartialEq, Debug)]
    #[allow(non_camel_case_types)]
    enum ArgChoice {
        FooBar,
        BAR_BAZ,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::parse_from(&["", "foo-bar"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::BAR_BAZ
        },
        Opt::parse_from(&["", "bar-baz"])
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
}

#[test]
fn variant_with_defined_casing() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        #[clap(rename_all = "screaming_snake")]
        FooBar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::parse_from(&["", "FOO_BAR"])
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
}

#[test]
fn casing_is_propogated_from_parent() {
    #[derive(ArgEnum, PartialEq, Debug)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        FooBar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::parse_from(&["", "FOO_BAR"])
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
}

#[test]
fn casing_propogation_is_overridden() {
    #[derive(ArgEnum, PartialEq, Debug)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        #[clap(rename_all = "camel")]
        FooBar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::FooBar
        },
        Opt::parse_from(&["", "fooBar"])
    );
    assert!(Opt::try_parse_from(&["", "FooBar"]).is_err());
    assert!(Opt::try_parse_from(&["", "FOO_BAR"]).is_err());
}

#[test]
fn case_insensitive() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(true))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::parse_from(&["", "foo"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::parse_from(&["", "fOo"])
    );
}

#[test]
fn case_insensitive_set_to_false() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(false))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::parse_from(&["", "foo"])
    );
    assert!(Opt::try_parse_from(&["", "fOo"]).is_err());
}

#[test]
fn alias() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        #[clap(alias = "TOTP")]
        TOTP,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(false))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::parse_from(&["", "totp"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::parse_from(&["", "TOTP"])
    );
}

#[test]
fn multiple_alias() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        #[clap(alias = "TOTP", alias = "t")]
        TOTP,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(false))]
        arg: ArgChoice,
    }

    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::parse_from(&["", "totp"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::parse_from(&["", "TOTP"])
    );
    assert_eq!(
        Opt {
            arg: ArgChoice::TOTP
        },
        Opt::parse_from(&["", "t"])
    );
}

#[test]
fn option() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: Option<ArgChoice>,
    }

    assert_eq!(Opt { arg: None }, Opt::parse_from(&[""]));
    assert_eq!(
        Opt {
            arg: Some(ArgChoice::Foo)
        },
        Opt::parse_from(&["", "foo"])
    );
    assert_eq!(
        Opt {
            arg: Some(ArgChoice::Bar)
        },
        Opt::parse_from(&["", "bar"])
    );
    assert!(Opt::try_parse_from(&["", "fOo"]).is_err());
}

#[test]
fn vector() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, short, long)]
        arg: Vec<ArgChoice>,
    }

    assert_eq!(Opt { arg: vec![] }, Opt::parse_from(&[""]));
    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo]
        },
        Opt::parse_from(&["", "-a", "foo"])
    );
    assert_eq!(
        Opt {
            arg: vec![ArgChoice::Foo, ArgChoice::Bar]
        },
        Opt::parse_from(&["", "-a", "foo", "bar"])
    );
    assert!(Opt::try_parse_from(&["", "-a", "fOo"]).is_err());
}

#[test]
fn skip_variant() {
    #[derive(ArgEnum, PartialEq, Debug)]
    #[allow(dead_code)] // silence warning about `Baz` being unused
    enum ArgChoice {
        Foo,
        Bar,
        #[clap(skip)]
        Baz,
    }

    assert_eq!(
        ArgChoice::VARIANTS,
        [ArgValue::new("foo"), ArgValue::new("bar")]
    );
    assert!(ArgChoice::from_str("foo", true).is_ok());
    assert!(ArgChoice::from_str("bar", true).is_ok());
    assert!(ArgChoice::from_str("baz", true).is_err());
}

#[test]
fn from_str_invalid() {
    #[derive(ArgEnum, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
    }

    assert!(ArgChoice::from_str("bar", true).is_err());
}
