// use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::convert::From;
use std::io;

use Arg;
use args::AnyArg;
use args::settings::{ArgFlags, ArgSettings};

#[derive(Debug)]
pub struct FlagBuilder<'n> {
    pub name: &'n str,
    /// The long version of the flag (i.e. word)
    /// without the preceding `--`
    pub long: Option<&'n str>,
    /// The string of text that will displayed to
    /// the user when the application's `help`
    /// text is displayed
    pub help: Option<&'n str>,
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
    /// A list of names for other arguments that *mutually override* this flag
    pub overrides: Option<Vec<&'n str>>,
    pub settings: ArgFlags,
}

impl<'n> FlagBuilder<'n> {
    pub fn new(name: &'n str) -> Self {
        FlagBuilder {
            name: name,
            short: None,
            long: None,
            help: None,
            blacklist: None,
            requires: None,
            overrides: None,
            settings: ArgFlags::new(),
        }
    }

    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize) -> io::Result<()> {
        try!(write!(w, "{}", tab));
        if let Some(s) = self.short {
            try!(write!(w, "-{}", s));
        } else {
            try!(write!(w, "{}", tab));
        }
        if let Some(l) = self.long {
            try!(write!(w,
                        "{}--{}",
                        if self.short.is_some() {
                            ", "
                        } else {
                            ""
                        },
                        l));
            write_spaces!((longest + 4) - (l.len() + 2), w);
        } else {
            // 6 is tab (4) + -- (2)
            write_spaces!((longest + 6), w);
        }
        if let Some(h) = self.help {
            if h.contains("{n}") {
                let mut hel = h.split("{n}");
                while let Some(part) = hel.next() {
                    try!(write!(w, "{}\n", part));
                    write_spaces!((longest + 12), w);
                    try!(write!(w, "{}", hel.next().unwrap_or("")));
                }
            } else {
                try!(write!(w, "{}", h));
            }
        }
        write!(w, "\n")
    }
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
                Perhaps you mean't to set takes_value(true) as well?",
                   a.name);
        }
        if a.required {
            panic!("The argument '{}' cannot be required(true) because it has no index() or \
                takes_value(true)",
                   a.name);
        }
        if a.possible_vals.is_some() {
            panic!("The argument '{}' cannot have a specific value set because it doesn't \
                have takes_value(true) set",
                   a.name);
        }
        // No need to check for index() or takes_value() as that is handled above

        let mut fb = FlagBuilder {
            name: a.name,
            short: a.short,
            long: a.long,
            help: a.help,
            blacklist: None,
            requires: None,
            overrides: None,
            settings: ArgFlags::new(),
        };
        if a.multiple {
            fb.settings.set(&ArgSettings::Multiple);
        }
        if a.global {
            fb.settings.set(&ArgSettings::Global);
        }
        if a.hidden {
            fb.settings.set(&ArgSettings::Hidden);
        }
        // Check if there is anything in the blacklist (mutually excludes list) and add
        // any
        // values
        if let Some(ref bl) = a.blacklist {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in bl {
                bhs.push(*n);
            }
            fb.blacklist = Some(bhs);
        }
        // Check if there is anything in the requires list and add any values
        if let Some(ref r) = a.requires {
            let mut rhs = vec![];
            // without derefing n = &&str
            for n in r {
                rhs.push(*n);
            }
            fb.requires = Some(rhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            fb.overrides = Some(bhs);
        }

        fb
    }
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

impl<'n> AnyArg<'n> for FlagBuilder<'n> {
    fn name(&self) -> &'n str {
        self.name
    }

    fn overrides(&self) -> Option<&[&'n str]> {
        self.overrides.as_ref().map(|o| &o[..])
    }

    fn is_set(&self, s: &ArgSettings) -> bool {
        self.settings.is_set(s)
    }

    fn set(&mut self, s: &ArgSettings) {
        self.settings.set(s)
    }
}

#[cfg(test)]
mod test {
    use super::FlagBuilder;
    use args::settings::ArgSettings;

    #[test]
    fn flagbuilder_display() {
        let mut f = FlagBuilder::new("flg");
        f.settings.set(&ArgSettings::Multiple);
        f.long = Some("flag");

        assert_eq!(&*format!("{}", f), "--flag");

        let mut f2 = FlagBuilder::new("flg");
        f2.short = Some('f');

        assert_eq!(&*format!("{}", f2), "-f");
    }
}
