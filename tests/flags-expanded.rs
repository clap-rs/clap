#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

#[macro_use]
extern crate clap;

use clap::Clap;

#[test]
pub fn unique_flag() {
    struct Opt {
        #[clap(short = "a", long = "alice")]
        alice: bool,
    }
    #[allow(unused_variables)]
    impl ::clap::Clap for Opt {}
    impl ::clap::IntoApp for Opt {
        fn into_app<'a, 'b>() -> ::clap::App<'a, 'b> {
            let app =
                ::clap::App::new("clap_derive").version("0.3.0").about("Parse command line argument by defining a struct, derive crate.").author("Guillaume Pinot <texitoi@texitoi.eu>, Kevin K. <kbknapp@gmail.com>, hoverbear <andrew@hoverbear.org>");
            Self::augment_clap(app)
        }
    }
    impl Into<::clap::App> for Opt {
        fn into(self) -> ::clap::App { <Self as ::clap::IntoApp>::into_app() }
    }
    impl ::clap::FromArgMatches for Opt {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            Opt {
                alice: matches.is_present("alice"),
            }
        }
    }
    impl From<::clap::ArgMatches> for Opt {
        fn from(m: ::clap::ArgMatches) -> Self {
            <Self as ::clap::FromArgMatches>::from_argmatches(&m)
        }
    }
    #[allow(dead_code, unreachable_code)]
    #[doc(hidden)]
    impl Opt {
        pub fn augment_app<'a, 'b>(app: ::clap::App<'a, 'b>) -> ::clap::App<'a, 'b> {
            {
                let app = app.arg(
                    ::clap::Arg::with_name("alice")
                        .takes_value(false)
                        .multiple(false),
                );
                app
            }
        }
        pub fn is_subcommand() -> bool { false }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Opt {
        #[inline]
        fn eq(&self, other: &Opt) -> bool {
            match *other {
                Opt {
                    alice: ref __self_1_0,
                } => match *self {
                    Opt {
                        alice: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Opt) -> bool {
            match *other {
                Opt {
                    alice: ref __self_1_0,
                } => match *self {
                    Opt {
                        alice: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Opt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Opt {
                    alice: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Opt");
                    let _ = debug_trait_builder.field("alice", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }

    {
        match (
            &Opt { alice: false },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 28u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt { alice: true },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 32u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt { alice: true },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "--alice"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 36u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    if !Opt::into_app()
        .get_matches_from_safe(&["test", "-i"])
        .is_err()
    {
        {
            ::rt::begin_panic("assertion failed: Opt::into_app().get_matches_from_safe(&[\"test\", \"-i\"]).is_err()",
                              &("tests/flags.rs", 40u32, 5u32))
        }
    };
    if !Opt::into_app()
        .get_matches_from_safe(&["test", "-a", "foo"])
        .is_err()
    {
        {
            ::rt::begin_panic("assertion failed: Opt::into_app().get_matches_from_safe(&[\"test\", \"-a\", \"foo\"]).is_err()",
                              &("tests/flags.rs", 45u32, 5u32))
        }
    };
    if !Opt::into_app()
        .get_matches_from_safe(&["test", "-a", "-a"])
        .is_err()
    {
        {
            ::rt::begin_panic("assertion failed: Opt::into_app().get_matches_from_safe(&[\"test\", \"-a\", \"-a\"]).is_err()",
                              &("tests/flags.rs", 50u32, 5u32))
        }
    };
    if !Opt::into_app()
        .get_matches_from_safe(&["test", "-a", "--alice"])
        .is_err()
    {
        {
            ::rt::begin_panic("assertion failed: Opt::into_app().get_matches_from_safe(&[\"test\", \"-a\", \"--alice\"]).is_err()",
                              &("tests/flags.rs", 55u32, 5u32))
        }
    };
}
#[test]
pub fn multiple_flag() {
    struct Opt {
        #[clap(short = "a", long = "alice", parse(from_occurrences))]
        alice: u64,
        #[clap(short = "b", long = "bob", parse(from_occurrences))]
        bob: u8,
    }
    #[allow(unused_variables)]
    impl ::clap::Clap for Opt {}
    impl ::clap::IntoApp for Opt {
        fn into_app<'a, 'b>() -> ::clap::App<'a, 'b> {
            let app =
                ::clap::App::new("clap_derive").version("0.3.0").about("Parse command line argument by defining a struct, derive crate.").author("Guillaume Pinot <texitoi@texitoi.eu>, Kevin K. <kbknapp@gmail.com>, hoverbear <andrew@hoverbear.org>");
            Self::augment_clap(app)
        }
    }
    impl Into<::clap::App> for Opt {
        fn into(self) -> ::clap::App { <Self as ::clap::IntoApp>::into_app() }
    }
    impl ::clap::FromArgMatches for Opt {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            Opt {
                alice: matches
                    .value_of("alice")
                    .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                    .unwrap(),
                bob: matches
                    .value_of("bob")
                    .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                    .unwrap(),
            }
        }
    }
    impl From<::clap::ArgMatches> for Opt {
        fn from(m: ::clap::ArgMatches) -> Self {
            <Self as ::clap::FromArgMatches>::from_argmatches(&m)
        }
    }
    #[allow(dead_code, unreachable_code)]
    #[doc(hidden)]
    impl Opt {
        pub fn augment_app<'a, 'b>(app: ::clap::App<'a, 'b>) -> ::clap::App<'a, 'b> {
            {
                let app = app.arg(
                    ::clap::Arg::with_name("alice")
                        .takes_value(true)
                        .multiple(false)
                        .required(true)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(&s)
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        }),
                );
                let app = app.arg(
                    ::clap::Arg::with_name("bob")
                        .takes_value(true)
                        .multiple(false)
                        .required(true)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(&s)
                                .map(|_: u8| ())
                                .map_err(|e| e.to_string())
                        }),
                );
                app
            }
        }
        pub fn is_subcommand() -> bool { false }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Opt {
        #[inline]
        fn eq(&self, other: &Opt) -> bool {
            match *other {
                Opt {
                    alice: ref __self_1_0,
                    bob: ref __self_1_1,
                } => match *self {
                    Opt {
                        alice: ref __self_0_0,
                        bob: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Opt) -> bool {
            match *other {
                Opt {
                    alice: ref __self_1_0,
                    bob: ref __self_1_1,
                } => match *self {
                    Opt {
                        alice: ref __self_0_0,
                        bob: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Opt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Opt {
                    alice: ref __self_0_0,
                    bob: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Opt");
                    let _ = debug_trait_builder.field("alice", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("bob", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    {
        match (
            &Opt { alice: 0, bob: 0 },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 72u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt { alice: 1, bob: 0 },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 76u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt { alice: 2, bob: 0 },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a", "-a"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 80u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt { alice: 2, bob: 2 },
            &Opt::from_argmatches(
                &Opt::into_app().get_matches_from(&["test", "-a", "--alice", "-bb"]),
            ),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 84u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt { alice: 3, bob: 1 },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-aaa", "--bob"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 88u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    if !Opt::into_app()
        .get_matches_from_safe(&["test", "-i"])
        .is_err()
    {
        {
            ::rt::begin_panic("assertion failed: Opt::into_app().get_matches_from_safe(&[\"test\", \"-i\"]).is_err()",
                              &("tests/flags.rs", 92u32, 5u32))
        }
    };
    if !Opt::into_app()
        .get_matches_from_safe(&["test", "-a", "foo"])
        .is_err()
    {
        {
            ::rt::begin_panic("assertion failed: Opt::into_app().get_matches_from_safe(&[\"test\", \"-a\", \"foo\"]).is_err()",
                              &("tests/flags.rs", 97u32, 5u32))
        }
    };
}
#[test]
pub fn combined_flags() {
    struct Opt {
        #[clap(short = "a", long = "alice")]
        alice: bool,
        #[clap(short = "b", long = "bob", parse(from_occurrences))]
        bob: u64,
    }
    #[allow(unused_variables)]
    impl ::clap::Clap for Opt {}
    impl ::clap::IntoApp for Opt {
        fn into_app<'a, 'b>() -> ::clap::App<'a, 'b> {
            let app =
                ::clap::App::new("clap_derive").version("0.3.0").about("Parse command line argument by defining a struct, derive crate.").author("Guillaume Pinot <texitoi@texitoi.eu>, Kevin K. <kbknapp@gmail.com>, hoverbear <andrew@hoverbear.org>");
            Self::augment_clap(app)
        }
    }
    impl Into<::clap::App> for Opt {
        fn into(self) -> ::clap::App { <Self as ::clap::IntoApp>::into_app() }
    }
    impl ::clap::FromArgMatches for Opt {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            Opt {
                alice: matches.is_present("alice"),
                bob: matches
                    .value_of("bob")
                    .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                    .unwrap(),
            }
        }
    }
    impl From<::clap::ArgMatches> for Opt {
        fn from(m: ::clap::ArgMatches) -> Self {
            <Self as ::clap::FromArgMatches>::from_argmatches(&m)
        }
    }
    #[allow(dead_code, unreachable_code)]
    #[doc(hidden)]
    impl Opt {
        pub fn augment_app<'a, 'b>(app: ::clap::App<'a, 'b>) -> ::clap::App<'a, 'b> {
            {
                let app = app.arg(
                    ::clap::Arg::with_name("alice")
                        .takes_value(false)
                        .multiple(false),
                );
                let app = app.arg(
                    ::clap::Arg::with_name("bob")
                        .takes_value(true)
                        .multiple(false)
                        .required(true)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(&s)
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        }),
                );
                app
            }
        }
        pub fn is_subcommand() -> bool { false }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Opt {
        #[inline]
        fn eq(&self, other: &Opt) -> bool {
            match *other {
                Opt {
                    alice: ref __self_1_0,
                    bob: ref __self_1_1,
                } => match *self {
                    Opt {
                        alice: ref __self_0_0,
                        bob: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Opt) -> bool {
            match *other {
                Opt {
                    alice: ref __self_1_0,
                    bob: ref __self_1_1,
                } => match *self {
                    Opt {
                        alice: ref __self_0_0,
                        bob: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Opt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Opt {
                    alice: ref __self_0_0,
                    bob: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Opt");
                    let _ = debug_trait_builder.field("alice", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("bob", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    {
        match (
            &Opt {
                alice: false,
                bob: 0,
            },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 114u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt {
                alice: true,
                bob: 0,
            },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 121u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt {
                alice: true,
                bob: 0,
            },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-a"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 128u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt {
                alice: false,
                bob: 1,
            },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-b"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 135u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt {
                alice: true,
                bob: 1,
            },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "--alice", "--bob"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 142u32, 5u32),
                        )
                    }
                }
            }
        }
    };
    {
        match (
            &Opt {
                alice: true,
                bob: 4,
            },
            &Opt::from_argmatches(&Opt::into_app().get_matches_from(&["test", "-bb", "-a", "-bb"])),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::rt::begin_panic_fmt(
                            &::std::fmt::Arguments::new_v1_formatted(
                                &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ],
                                &match (&left_val, &right_val) {
                                    (arg0, arg1) => [
                                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt),
                                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                                    ],
                                },
                                &[
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(0usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::std::fmt::rt::v1::Argument {
                                        position: ::std::fmt::rt::v1::Position::At(1usize),
                                        format: ::std::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::std::fmt::rt::v1::Count::Implied,
                                            width: ::std::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                            ),
                            &("tests/flags.rs", 149u32, 5u32),
                        )
                    }
                }
            }
        }
    };
}
pub mod __test_reexports {
    pub use super::combined_flags;
    pub use super::multiple_flag;
    pub use super::unique_flag;
}
pub mod __test {
    extern crate test;
    #[main]
    pub fn main() -> () { test::test_main_static(TESTS) }
    const TESTS: &'static [self::test::TestDescAndFn] = &[
        self::test::TestDescAndFn {
            desc: self::test::TestDesc {
                name: self::test::StaticTestName("unique_flag"),
                ignore: false,
                should_panic: self::test::ShouldPanic::No,
                allow_fail: false,
            },
            testfn: self::test::StaticTestFn(|| {
                self::test::assert_test_result(::__test_reexports::unique_flag())
            }),
        },
        self::test::TestDescAndFn {
            desc: self::test::TestDesc {
                name: self::test::StaticTestName("multiple_flag"),
                ignore: false,
                should_panic: self::test::ShouldPanic::No,
                allow_fail: false,
            },
            testfn: self::test::StaticTestFn(|| {
                self::test::assert_test_result(::__test_reexports::multiple_flag())
            }),
        },
        self::test::TestDescAndFn {
            desc: self::test::TestDesc {
                name: self::test::StaticTestName("combined_flags"),
                ignore: false,
                should_panic: self::test::ShouldPanic::No,
                allow_fail: false,
            },
            testfn: self::test::StaticTestFn(|| {
                self::test::assert_test_result(::__test_reexports::combined_flags())
            }),
        },
    ];
}
