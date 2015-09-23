//! clap2

mod arg;
mod app;
mod macros;

pub use arg::{Accumulator, Rule};
pub use app::{App, AppBuilder, CollectionMatcher};

// ClapError
#[derive(Debug)]
pub enum ClapError<'a> {
    TooManyInstances(&'a str),     // -v #3; -vvvv; 4 > 3
    ExpectedValue(&'a str),        // --config <config>; --config; missing <conf>; iterator starved
    UnexpectedLong(String) ,       // unknown long provided
    UnexpectedShort(char),         // unknown short provided
    UnexpectedPositional(String),  // unknown positional provided
}
