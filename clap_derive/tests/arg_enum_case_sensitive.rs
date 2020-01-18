// // Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// // Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// // Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
// //
// // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// // option. This file may not be copied, modified, or distributed
// // except according to those terms.
// #[macro_use]
// extern crate clap;

// use clap::{App, Arg, ArgEnum};

// #[derive(ArgEnum, Debug, PartialEq)]
// #[case_sensitive]
// enum ArgChoice {
//     Foo,
//     Bar,
//     Baz,
// }

// #[test]
// fn when_lowercase() {
//     let matches = App::new(env!("CARGO_PKG_NAME"))
//         .arg(
//             Arg::with_name("arg")
//                 .required(true)
//                 .takes_value(true)
//                 .possible_values(&ArgChoice::variants()),
//         )
//         .try_get_matches_from(vec!["", "foo"]); // We expect this to fail.
//     assert!(matches.is_err());
//     assert_eq!(matches.unwrap_err().kind, clap::ErrorKind::InvalidValue);
// }

// #[test]
// fn when_capitalized() {
//     let matches = App::new(env!("CARGO_PKG_NAME"))
//         .arg(
//             Arg::with_name("arg")
//                 .required(true)
//                 .takes_value(true)
//                 .possible_values(&ArgChoice::variants()),
//         )
//         .try_get_matches_from(vec!["", "Foo"])
//         .unwrap();
//     let t = value_t!(matches.value_of("arg"), ArgChoice);
//     assert!(t.is_ok());
//     assert_eq!(t.unwrap(), ArgChoice::Foo);
// }
