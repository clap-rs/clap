// Copyright ⓒ 2015-2016 Kevin B. Knapp and [`clap-rs` contributors](https://github.com/clap-rs/clap/graphs/contributors).
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/clap/3.0.0-beta.2")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]
//! https://github.com/clap-rs/clap
#![crate_type = "lib"]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]

#[cfg(not(feature = "std"))]
compile_error!("`std` feature is currently required to build `clap`");

pub use crate::{
    build::{App, AppSettings, Arg, ArgGroup, ArgSettings, ValueHint},
    parse::errors::{Error, ErrorKind, Result},
    parse::{ArgMatches, Indices, OsValues, Values},
};

#[cfg(feature = "derive")]
pub use crate::derive::{ArgEnum, Clap, FromArgMatches, IntoApp, Subcommand};

#[cfg(feature = "yaml")]
#[doc(hidden)]
pub use yaml_rust::YamlLoader;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use clap_derive::{self, *};

#[cfg(any(feature = "derive", feature = "cargo"))]
#[doc(hidden)]
pub use lazy_static;

#[macro_use]
#[allow(missing_docs)]
mod macros;

#[cfg(feature = "derive")]
mod derive;

#[cfg(feature = "regex")]
pub use crate::build::arg::RegexRef;

mod build;
mod mkeymap;
mod output;
mod parse;
mod util;

const INTERNAL_ERROR_MSG: &str = "Fatal internal error. Please consider filing a bug \
                                  report at https://github.com/clap-rs/clap/issues";
const INVALID_UTF8: &str = "unexpected invalid UTF-8 code point";
