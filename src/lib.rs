#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
//#![cfg_attr(feature = "lints", plugin(clippy))]
//#![cfg_attr(feature = "lints", allow(option_unwrap_used))]
//#![cfg_attr(feature = "lints", allow(explicit_iter_loop))]
//#![cfg_attr(feature = "lints", deny(warnings))]
// Fix until clippy on crates.io is updated to include needless_lifetimes lint
//#![cfg_attr(feature = "lints", allow(unknown_lints))]

// DOCS

#[cfg(feature = "suggestions")]
extern crate strsim;
#[cfg(feature = "color")]
extern crate ansi_term;

pub use args::{Arg, SubCommand, ArgMatches, ArgGroup};
pub use app::{App, AppSettings};
pub use fmt::Format;

#[macro_use]
mod macros;
mod app;
mod args;
mod usageparser;
mod fmt;

#[cfg(test)]
mod tests;