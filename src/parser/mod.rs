//! [`Command`][crate::Command] line argument parser

mod arg_matcher;
mod error;
mod matches;
#[allow(clippy::module_inception)]
mod parser;
mod validator;

pub(crate) mod features;

pub(crate) use self::arg_matcher::ArgMatcher;
pub(crate) use self::matches::AnyValue;
pub(crate) use self::matches::AnyValueId;
pub(crate) use self::matches::{MatchedArg, SubCommand};
pub(crate) use self::parser::{ParseState, Parser};
pub(crate) use self::validator::Validator;

pub use self::matches::{ArgMatches, Indices, OsValues, ValueSource, Values};
pub use error::MatchesError;
