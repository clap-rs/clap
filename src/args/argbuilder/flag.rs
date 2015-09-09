// use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

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
    pub blacklist: Option<Vec<&'n str>>,
    /// A list of names of other arguments that
    /// are *required* to be used when this
    /// flag is used
    pub requires: Option<Vec<&'n str>>,
    /// The short version (i.e. single character)
    /// of the argument, no preceding `-`
    pub short: Option<char>,
    pub global: bool,
    /// A list of names for other arguments that *mutually override* this flag
    pub overrides: Option<Vec<&'n str>>,
    pub hidden: bool
}

impl<'n> Display for FlagBuilder<'n> {
    fn fmt(&self,
           f: &mut Formatter) 
           -> Result {
        if let Some(l) = self.long {
            write!(f, "--{}", l)
        } else {
            write!(f, "-{}", self.short.unwrap())
        }
    }
}
#[cfg(test)]
mod test {
    use super::FlagBuilder;

    #[test]
    fn flagbuilder_display() {
        let f = FlagBuilder {
            name: "flg",
            short: None,
            long: Some("flag"),
            help: None,
            multiple: true,
            blacklist: None,
            requires: None,
            global: false,
            overrides: None,
        };

        assert_eq!(&*format!("{}", f), "--flag");

        let f2 = FlagBuilder {
            name: "flg",
            short: Some('f'),
            long: None,
            help: None,
            multiple: false,
            blacklist: None,
            requires: None,
            global: false,
            overrides: None,
        };

        assert_eq!(&*format!("{}", f2), "-f");
    }
}
