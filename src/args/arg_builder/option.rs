use std::rc::Rc;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::io;

use args::{AnyArg, ArgMatcher, Arg};
use args::settings::{ArgFlags, ArgSettings};
use errors::{ClapResult, error_builder};
use app::App;

#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct OptBuilder<'n> {
    pub name: &'n str,
    /// The short version (i.e. single character) of the argument, no preceding `-`
    pub short: Option<char>,
    /// The long version of the flag (i.e. word) without the preceding `--`
    pub long: Option<&'n str>,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'n str>,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<Vec<&'n str>>,
    /// A list of possible values for this argument
    pub possible_vals: Option<Vec<&'n str>>,
    /// A list of names of other arguments that are *required* to be used when
    /// this flag is used
    pub requires: Option<Vec<&'n str>>,
    pub num_vals: Option<u8>,
    pub min_vals: Option<u8>,
    pub max_vals: Option<u8>,
    pub val_names: Option<BTreeSet<&'n str>>,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    /// A list of names for other arguments that *mutually override* this flag
    pub overrides: Option<Vec<&'n str>>,
    pub settings: ArgFlags,
}

impl<'n> OptBuilder<'n> {
    pub fn new(name: &'n str) -> Self {
        OptBuilder {
            name: name,
            ..Default::default()
        }
    }

    pub fn from_arg(a: &Arg<'n, 'n, 'n, 'n, 'n, 'n>, reqs: &mut Vec<&'n str>) -> Self {
        if a.short.is_none() && a.long.is_none() {
            panic!("Argument \"{}\" has takes_value(true), yet neither a short() or long() \
                was supplied",
                   a.name);
        }
        // No need to check for .index() as that is handled above
        let mut ob = OptBuilder {
            name: a.name,
            short: a.short,
            long: a.long,
            help: a.help,
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names.clone(),
            ..Default::default()
        };
        if a.multiple {
            ob.settings.set(&ArgSettings::Multiple);
        }
        if a.required {
            ob.settings.set(&ArgSettings::Required);
        }
        if a.global {
            ob.settings.set(&ArgSettings::Global);
        }
        if !a.empty_vals {
            ob.settings.unset(&ArgSettings::Global);
        }
        if a.hidden {
            ob.settings.set(&ArgSettings::Hidden);
        }
        if let Some(ref vec) = ob.val_names {
            ob.num_vals = Some(vec.len() as u8);
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
            ob.blacklist = Some(bhs);
        }
        if let Some(ref p) = a.validator {
            ob.validator = Some(p.clone());
        }
        // Check if there is anything in the requires list and add any values
        if let Some(ref r) = a.requires {
            let mut rhs = vec![];
            // without derefing n = &&str
            for n in r {
                rhs.push(*n);
                if a.required {
                    reqs.push(*n);
                }
            }
            ob.requires = Some(rhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            ob.overrides = Some(bhs);
        }
        // Check if there is anything in the possible values and add those as well
        if let Some(ref p) = a.possible_vals {
            let mut phs = vec![];
            // without derefing n = &&str
            for n in p {
                phs.push(*n);
            }
            ob.possible_vals = Some(phs);
        }

        ob
    }

    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize) -> io::Result<()> {
        // if it supports multiple we add '...' i.e. 3 to the name length
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
        }
        if let Some(ref vec) = self.val_names {
            for val in vec {
                try!(write!(w, " <{}>", val));
            }
        } else if let Some(num) = self.num_vals {
            for _ in 0..num {
                try!(write!(w, " <{}>", self.name));
            }
        } else {
            try!(write!(w,
                        " <{}>{}",
                        self.name,
                        if self.settings.is_set(&ArgSettings::Multiple) {
                            "..."
                        } else {
                            ""
                        }));
        }
        if self.long.is_some() {
            write_spaces!((longest + 4) - (self.to_string().len()), w);
        } else {
            // 8 = tab + '-a, '.len()
            write_spaces!((longest + 8) - (self.to_string().len()), w);
        }
        print_opt_help!(self, longest + 12, w);
        write!(w, "\n")
    }

    pub fn validate_value(&self,
                       val: &str,
                       matcher: &ArgMatcher,
                       app: &App)
                       -> ClapResult<()> {
        // Check the possible values
        if let Some(ref p_vals) = self.possible_vals {
            if !p_vals.contains(&val) {
                let usage = try!(app.create_current_usage(matcher));
                return Err(error_builder::InvalidValue(val, p_vals, &self.to_string(), &usage));
            }
        }

        // Check the required number of values
        if let Some(num) = self.num_vals {
            if let Some(ref ma) = matcher.get(self.name) {
                if let Some(ref vals) = ma.values {
                    if (vals.len() as u8) > num && !self.settings.is_set(&ArgSettings::Multiple) {
                        return Err(error_builder::TooManyValues(
                            val,
                            &self.to_string(),
                            &*try!(app.create_current_usage(matcher))));
                    }
                }
            }
        }

        // if it's an empty value, and we don't allow that, report the error
        if !self.settings.is_set(&ArgSettings::EmptyValues) &&
            matcher.contains(self.name) &&
            val.is_empty() {
            return Err(error_builder::EmptyValue(&*self.to_string(),
                                                 &*try!(app.create_current_usage(matcher))));
        }

        if let Some(ref vtor) = self.validator {
            if let Err(e) = vtor(val.to_owned()) {
                return Err(error_builder::ValueValidationError(&*e));
            }
        }

        Ok(())
    }
}

impl<'n> Display for OptBuilder<'n> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write the name such --long or -l
        if let Some(l) = self.long {
            try!(write!(f, "--{}", l));
        } else {
            try!(write!(f, "-{}", self.short.unwrap()));
        }

        // Write the values such as <name1> <name2>
        if let Some(ref vec) = self.val_names {
            for n in vec.iter() {
                try!(write!(f, " <{}>", n));
            }
        } else {
            let num = self.num_vals.unwrap_or(1);
            for _ in 0..num {
                try!(write!(f, " <{}>", self.name));
            }
            if self.settings.is_set(&ArgSettings::Multiple) && num == 1 {
                try!(write!(f, "..."));
            }
        }

        Ok(())
    }
}

impl<'n> AnyArg<'n> for OptBuilder<'n> {
    fn name(&self) -> &'n str {
        self.name
    }

    fn overrides(&self) -> Option<&[&'n str]> {
        self.overrides.as_ref().map(|o| &o[..])
    }

    fn requires(&self) -> Option<&[&'n str]> {
        self.requires.as_ref().map(|o| &o[..])
    }

    fn blacklist(&self) -> Option<&[&'n str]> {
        self.blacklist.as_ref().map(|o| &o[..])
    }

    fn is_set(&self, s: ArgSettings) -> bool {
        self.settings.is_set(&s)
    }

    fn has_switch(&self) -> bool {
        true
    }

    fn set(&mut self, s: ArgSettings) {
        self.settings.set(&s)
    }

    fn max_vals(&self) -> Option<u8> {
        self.max_vals
    }
    fn num_vals(&self) -> Option<u8> {
        self.num_vals
    }
    fn possible_vals(&self) -> Option<&[&'n str]> {
        self.possible_vals.as_ref().map(|o| &o[..])
    }

    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.validator.as_ref()
    }

    fn min_vals(&self) -> Option<u8> {
        self.min_vals
    }
}

#[cfg(test)]
mod test {
    use super::OptBuilder;
    use std::collections::BTreeSet;
    use args::settings::ArgSettings;

    #[test]
    fn optbuilder_display() {
        let mut o = OptBuilder::new("opt");
        o.long = Some("option");
        o.settings.set(&ArgSettings::Multiple);

        assert_eq!(&*format!("{}", o), "--option <opt>...");

        let mut v_names = BTreeSet::new();
        v_names.insert("file");
        v_names.insert("name");

        let mut o2 = OptBuilder::new("opt");
        o2.short = Some('o');
        o2.val_names = Some(v_names);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }
}
