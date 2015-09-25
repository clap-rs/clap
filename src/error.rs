use std::error::Error;
use std::fmt;

// ClapError
#[derive(Debug)]
pub enum ClapError<'a> {
    TooManyInstances(&'a str),     // -v #3; -vvvv; 4 > 3
    ExpectedValue(&'a str),        // --config <config>; --config; missing <conf>; iterator starved
    UnexpectedLong(String) ,       // unknown long provided
    UnexpectedShort(char),         // unknown short provided
    UnexpectedPositional(String),  // unknown positional provided
    UnexpectedValue(String),       // unexpected value provided
}

impl<'a> fmt::Display for ClapError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<'a> Error for ClapError<'a> {
    fn description(&self) -> &str {
        use ClapError::*;
        match *self {
            TooManyInstances(_) => "too many instances of argument",
            ExpectedValue(_) => "expected a value but is missing",
            UnexpectedLong(_) => "unexpected long argument provided",
            UnexpectedShort(_) => "unexpected short argument provided",
            UnexpectedPositional(_) => "unexpected positional provided",
            UnexpectedValue(_) => "unexpected value provided",
        }
    }
}
