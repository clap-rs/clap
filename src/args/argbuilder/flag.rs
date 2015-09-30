// use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::convert::From;

use Arg;

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

impl<'n, 'a> From<&'a Arg<'n, 'n, 'n, 'n, 'n, 'n>> for FlagBuilder<'n> {
    fn from(a: &Arg<'n, 'n, 'n, 'n, 'n, 'n>) -> Self {
        if a.validator.is_some() {
            panic!("The argument '{}' has a validator set, yet was parsed as a flag. Ensure \
                .takes_value(true) or .index(u8) is set.")
        }
        if !a.empty_vals {
            // Empty vals defaults to true, so if it's false it was manually set
            panic!("The argument '{}' cannot have empty_values() set because it is a flag. \
                Perhaps you mean't to set takes_value(true) as well?", a.name);
        }
        if a.required {
            panic!("The argument '{}' cannot be required(true) because it has no index() or \
                takes_value(true)", a.name);
        }
        if a.possible_vals.is_some() {
            panic!("The argument '{}' cannot have a specific value set because it doesn't \
                have takes_value(true) set", a.name);
        }
        // No need to check for index() or takes_value() as that is handled above

        let mut fb = FlagBuilder {
            name: a.name,
            short: a.short,
            long: a.long,
            help: a.help,
            blacklist: None,
            global: a.global,
            multiple: a.multiple,
            requires: None,
            overrides: None,
            hidden: a.hidden
        };
        // Check if there is anything in the blacklist (mutually excludes list) and add any
        // values
        if let Some(ref bl) = a.blacklist {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in bl {
                bhs.push(*n);
            }
            bhs.dedup();
            fb.blacklist = Some(bhs);
        }
        // Check if there is anything in the requires list and add any values
        if let Some(ref r) = a.requires {
            let mut rhs = vec![];
            // without derefing n = &&str
            for n in r {
                rhs.push(*n);
            }
            rhs.dedup();
            fb.requires = Some(rhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            bhs.dedup();
            fb.overrides = Some(bhs);
        }

        fb
    }
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
            hidden: false,
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
            hidden: false,
        };

        assert_eq!(&*format!("{}", f2), "-f");
    }
}
