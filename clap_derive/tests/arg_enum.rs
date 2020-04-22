// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use clap::Clap;

#[test]
fn basic() {
    #[derive(Clap, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
        Bar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    };

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
fn multi_word_is_renamed_kebab() {
    #[derive(Clap, PartialEq, Debug)]
    #[allow(non_camel_case_types)]
    enum ArgChoice {
        FooBar,
        BAR_BAZ,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    };

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
    #[derive(Clap, PartialEq, Debug)]
    enum ArgChoice {
        #[clap(rename_all = "screaming_snake")]
        FooBar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    };

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
    #[derive(Clap, PartialEq, Debug)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        FooBar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    };

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
    #[derive(Clap, PartialEq, Debug)]
    #[clap(rename_all = "screaming_snake")]
    enum ArgChoice {
        #[clap(rename_all = "camel")]
        FooBar,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum)]
        arg: ArgChoice,
    };

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
    #[derive(Clap, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(true))]
        arg: ArgChoice,
    };

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
    #[derive(Clap, PartialEq, Debug)]
    enum ArgChoice {
        Foo,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(arg_enum, case_insensitive(false))]
        arg: ArgChoice,
    };

    assert_eq!(
        Opt {
            arg: ArgChoice::Foo
        },
        Opt::parse_from(&["", "foo"])
    );
    assert!(Opt::try_parse_from(&["", "fOo"]).is_err());
}
