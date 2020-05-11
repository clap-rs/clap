// Copyright ⓒ 2015-2016 Kevin B. Knapp and [`clap-rs` contributors](https://github.com/clap-rs/clap/graphs/contributors).
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/clap/3.0.0-beta.1")]
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

#[cfg(not(feature = "std"))]
compile_error!("`std` feature is currently required to build `clap`");

pub use crate::{
    build::{App, AppSettings, Arg, ArgGroup, ArgSettings},
    derive::{ArgEnum, Clap, FromArgMatches, IntoApp, Subcommand},
    parse::errors::{Error, ErrorKind, Result},
    parse::{ArgMatches, OsValues, Values},
};

#[cfg(feature = "yaml")]
pub use yaml_rust::YamlLoader;

#[cfg(feature = "derive")]
#[cfg_attr(feature = "derive", doc(hidden))]
pub use clap_derive::{self, *};

#[cfg(feature = "derive")]
#[cfg_attr(feature = "derive", doc(hidden))]
pub use lazy_static;

#[macro_use]
#[allow(missing_docs)]
pub mod macros;

pub mod derive;

mod build;
mod mkeymap;
mod output;
mod parse;
mod util;

const INTERNAL_ERROR_MSG: &str = "Fatal internal error. Please consider filing a bug \
                                  report at https://github.com/clap-rs/clap/issues";
const INVALID_UTF8: &str = "unexpected invalid UTF-8 code point";
