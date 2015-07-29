use std::collections::HashSet;
use std::fmt::{ Display, Formatter, Result };

pub struct FlagBuilder<'n> {
    pub name: &'n str,
    /// The long version of the flag (i.e. word)
    /// without the preceding `--`
    pub long: Option<&'n str>,
    /// The string of text that will displayed to
    /// the user when the application's `help`
    /// text is displayed
    pub help: Option<&'n str>,
    /// Determines if multiple instances of the same
    /// flag are allowed
    /// I.e. `-v -v -v` or `-vvv`
    pub multiple: bool,
    /// A list of names for other arguments that
    /// *may not* be used with this flag
    pub blacklist: Option<HashSet<&'n str>>,
    /// A list of names of other arguments that
    /// are *required* to be used when this
    /// flag is used
    pub requires: Option<HashSet<&'n str>>,
    /// The short version (i.e. single character)
    /// of the argument, no preceding `-`
    pub short: Option<char>,
    pub global: bool,
}

impl<'n> Display for FlagBuilder<'n> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(l) = self.long {
            write!(f, "--{}", l)
        } else {
            write!(f, "-{}", self.short.unwrap())
        }
    }
}
