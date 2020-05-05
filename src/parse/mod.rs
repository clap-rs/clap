pub mod errors;
pub mod features;

mod arg_matcher;
pub mod matches;
mod parser;
mod validator;

pub(crate) use self::{
    arg_matcher::ArgMatcher,
    matches::{MatchedArg, SubCommand, ValueType},
    parser::{Input, ParseResult, Parser},
    validator::Validator,
};

pub use self::matches::ArgMatches;
pub use self::matches::{OsValues, Values};
