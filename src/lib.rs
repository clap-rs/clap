// Copyright ⓒ 2015-2016 Kevin B. Knapp and [`clap-rs` contributors](https://github.com/clap-rs/clap/graphs/contributors).
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

#![cfg_attr(feature = "doc", doc = include_str!("../README.md"))]
//! <https://github.com/clap-rs/clap>
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]
// TODO: https://github.com/rust-lang/rust-clippy/issues/7290
#![allow(clippy::single_component_path_imports)]
#![allow(clippy::branches_sharing_code)]

#[cfg(not(feature = "std"))]
compile_error!("`std` feature is currently required to build `clap`");

#[cfg(feature = "color")]
pub use crate::util::color::ColorChoice;
pub use crate::{
    build::{
        App, AppFlags, AppSettings, Arg, ArgFlags, ArgGroup, ArgSettings, PossibleValue, ValueHint,
    },
    parse::errors::{Error, ErrorKind, Result},
    parse::{ArgMatches, Indices, OsValues, Values},
};

pub use crate::derive::{ArgEnum, Args, FromArgMatches, IntoApp, Parser, Subcommand};

#[cfg(feature = "yaml")]
#[doc(hidden)]
#[deprecated(
    since = "3.0.0",
    note = "Deprecated in Issue #9, maybe clap::Parser would fit your use case?"
)]
pub use yaml_rust::YamlLoader;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use clap_derive::{self, *};

/// Deprecated, replaced with [`Parser`]
#[deprecated(since = "3.0.0", note = "Replaced with `Parser`")]
pub use Parser as StructOpt;

#[cfg(any(feature = "derive", feature = "cargo"))]
#[doc(hidden)]
pub use lazy_static;

#[macro_use]
#[allow(missing_docs)]
mod macros;

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

/// Deprecated, replaced with [`App`]
#[deprecated(since = "3.0.0", note = "Replaced with `App`")]
#[derive(Debug, Copy, Clone)]
pub struct SubCommand {}

#[allow(deprecated)]
impl SubCommand {
    /// Deprecated, replaced with [`App::new`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::new`")]
    pub fn with_name<'help>(name: &str) -> App<'help> {
        App::new(name)
    }

    /// Deprecated in [Issue #9](https://github.com/epage/clapng/issues/9), maybe [`clap::Parser`][crate::Parser] would fit your use case?
    #[cfg(feature = "yaml")]
    #[deprecated(
        since = "3.0.0",
        note = "Deprecated in Issue #9, maybe clap::Parser would fit your use case?"
    )]
    pub fn from_yaml(yaml: &yaml_rust::Yaml) -> App {
        #![allow(deprecated)]
        App::from_yaml(yaml)
    }
}
