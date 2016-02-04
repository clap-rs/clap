// use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::convert::From;
use std::io;
use std::rc::Rc;
use std::result::Result as StdResult;

use Arg;
use args::AnyArg;
use args::settings::{ArgFlags, ArgSettings};

#[derive(Debug)]
#[doc(hidden)]
pub struct FlagBuilder<'n, 'e> {
    pub name: &'n str,
    pub long: Option<&'e str>,
    pub help: Option<&'e str>,
    pub blacklist: Option<Vec<&'e str>>,
    pub requires: Option<Vec<&'e str>>,
    pub short: Option<char>,
    pub overrides: Option<Vec<&'e str>>,
    pub settings: ArgFlags,
}

impl<'n, 'e> Default for FlagBuilder<'n, 'e> {
    fn default() -> Self {
        FlagBuilder {
            name: "",
            long: None,
            help: None,
            blacklist: None,
            requires: None,
            short: None,
            overrides: None,
            settings: ArgFlags::new(),
        }
    }
}

impl<'n, 'e> FlagBuilder<'n, 'e> {
    pub fn new(name: &'n str) -> Self {
        FlagBuilder {
            name: name,
            ..Default::default()
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

impl<'a, 'b, 'z> From<&'z Arg<'a, 'b>> for FlagBuilder<'a, 'b> {
    fn from(a: &'z Arg<'a, 'b>) -> Self {
        assert!(a.validator.is_none(),
            format!("The argument '{}' has a validator set, yet was parsed as a flag. Ensure \
                .takes_value(true) or .index(u64) is set.", a.name));
        assert!(a.possible_vals.is_none(),
            format!("The argument '{}' cannot have a specific value set because it doesn't \
                have takes_value(true) set",
                   a.name));
        assert!(!a.is_set(ArgSettings::Required),
            format!("The argument '{}' cannot be required because it's a flag, perhaps you forgot \
                takes_value(true)?", a.name));
        // No need to check for index() or takes_value() as that is handled above

        FlagBuilder {
            name: a.name,
            short: a.short,
            long: a.long,
            help: a.help,
            blacklist: a.blacklist.clone(),
            overrides: a.overrides.clone(),
            requires: a.requires.clone(),
            settings: a.settings.clone(),
        }
    }
}

impl<'n, 'e> Display for FlagBuilder<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(l) = self.long {
            write!(f, "--{}", l)
        } else {
            write!(f, "-{}", self.short.unwrap())
        }
    }
}

impl<'n, 'e> AnyArg<'n, 'e> for FlagBuilder<'n, 'e> {
    fn name(&self) -> &'n str { self.name }
    fn overrides(&self) -> Option<&[&'e str]> { self.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[&'e str]> { self.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.blacklist.as_ref().map(|o| &o[..]) }
    fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
    fn has_switch(&self) -> bool { true }
    fn set(&mut self, s: ArgSettings) { self.settings.set(s) }
    fn max_vals(&self) -> Option<u64> { None }
    fn num_vals(&self) -> Option<u64> { None }
    fn possible_vals(&self) -> Option<&[&'e str]> { None }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> { None }
    fn min_vals(&self) -> Option<u64> { None }
    fn short(&self) -> Option<char> { self.short }
    fn long(&self) -> Option<&'e str> { self.long }
    fn val_delim(&self) -> Option<char> { None }
}

#[cfg(test)]
mod test {
    use super::FlagBuilder;
    use args::settings::ArgSettings;

    #[test]
    fn flagbuilder_display() {
        let mut f = FlagBuilder::new("flg");
        f.settings.set(ArgSettings::Multiple);
        f.long = Some("flag");

        assert_eq!(&*format!("{}", f), "--flag");

        let mut f2 = FlagBuilder::new("flg");
        f2.short = Some('f');

        assert_eq!(&*format!("{}", f2), "-f");
    }
}
