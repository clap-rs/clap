#[macro_use]
mod macros;

pub mod app;
pub mod arg;

mod app_settings;
mod arg_group;
mod usage_parser;

#[cfg(debug_assertions)]
mod debug_asserts;

#[cfg(test)]
mod app_tests;

pub use self::app::App;
pub use self::app_settings::{AppFlags, AppSettings};
pub use self::arg::{Arg, ArgFlags, ArgSettings, PossibleValue, ValueHint};
pub use self::arg_group::ArgGroup;
pub(crate) use arg::ArgPredicate;
