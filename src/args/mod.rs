pub use self::arg::Arg;
pub use self::arg_matcher::ArgMatcher;
pub use self::arg_matches::{ArgMatches, OsValues, Values};
pub use self::group::ArgGroup;
pub use self::matched_arg::MatchedArg;
pub use self::settings::{ArgFlags, ArgSettings};
pub use self::subcommand::SubCommand;

#[macro_use]
mod macros;
mod arg;
mod arg_matches;
mod arg_matcher;
mod subcommand;
mod matched_arg;
mod group;
pub mod settings;

pub trait DispOrder {
    fn disp_ord(&self) -> usize;
}
