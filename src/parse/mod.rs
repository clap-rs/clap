pub mod errors;
pub mod features;

mod arg_matcher;
pub mod matches;
mod parser;
#[cfg(feature = "validators")]
mod validator;

pub(crate) use self::{
    arg_matcher::ArgMatcher,
    matches::{MatchedArg, SubCommand, ValueType},
    parser::{Input, ParseResult, Parser},
};

#[cfg(feature = "validators")]
pub(crate) use self::validator::Validator;

pub use self::matches::ArgMatches;
pub use self::matches::{OsValues, Values};
