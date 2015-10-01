use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::rc::Rc;

use Arg;
use args::settings::{ArgFlags, ArgSettings};

pub struct PosBuilder<'n> {
    pub name: &'n str,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'n str>,
    /// A list of names of other arguments that are *required* to be used when
    /// this flag is used
    pub requires: Option<Vec<&'n str>>,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<Vec<&'n str>>,
    /// A list of possible values for this argument
    pub possible_vals: Option<Vec<&'n str>>,
    /// The index of the argument
    pub index: u8,
    pub num_vals: Option<u8>,
    pub max_vals: Option<u8>,
    pub min_vals: Option<u8>,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    /// A list of names for other arguments that *mutually override* this flag
    pub overrides: Option<Vec<&'n str>>,
    pub settings: ArgFlags,
}

impl<'n> PosBuilder<'n> {
    pub fn new(name: &'n str, idx: u8) -> Self {
        PosBuilder {
            name: name,
            index: idx,
            help: None,
            blacklist: None,
            possible_vals: None,
            requires: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            validator: None,
            overrides: None,
            settings: ArgFlags::new()
        }
    }

    pub fn from_arg(a: &Arg<'n, 'n, 'n, 'n, 'n, 'n>,
                idx: u8,
                reqs: &mut Vec<&'n str>) -> Self {
        if a.short.is_some() || a.long.is_some() {
            panic!("Argument \"{}\" has conflicting requirements, both index() and short(), \
                or long(), were supplied", a.name);
        }

        if a.takes_value {
            panic!("Argument \"{}\" has conflicting requirements, both index() and \
                takes_value(true) were supplied\n\n\tArguments with an index automatically \
                take a value, you do not need to specify it manually", a.name);
        }

        if a.val_names.is_some() {
            panic!("Positional arguments (\"{}\") do not support named values, instead \
                consider multiple positional arguments", a.name);
        }

        // Create the Positional Arguemnt Builder with each HashSet = None to only allocate
        // those that require it
        let mut pb = PosBuilder {
            name: a.name,
            index: idx,
            blacklist: None,
            requires: None,
            possible_vals: None,
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            help: a.help,
            validator: None,
            overrides: None,
            settings: ArgFlags::new()
        };
        if a.multiple {
            pb.settings.set(&ArgSettings::Multiple);
        }
        if a.required {
            pb.settings.set(&ArgSettings::Required);
        }
        if a.global {
            pb.settings.set(&ArgSettings::Global);
        }
        if a.hidden {
            pb.settings.set(&ArgSettings::Hidden);
        }
        if pb.min_vals.is_some() && !a.multiple {
            panic!("Argument \"{}\" does not allow multiple values, yet it is expecting {} \
                values", pb.name, pb.num_vals.unwrap());
        }
        if pb.max_vals.is_some() && !a.multiple {
            panic!("Argument \"{}\" does not allow multiple values, yet it is expecting {} \
                values", pb.name, pb.num_vals.unwrap());
        }
        // Check if there is anything in the blacklist (mutually excludes list) and add any
        // values
        if let Some(ref bl) = a.blacklist {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in bl {
                bhs.push(*n);
            }
            pb.blacklist = Some(bhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            pb.overrides = Some(bhs);
        }
        // Check if there is anything in the possible values and add those as well
        if let Some(ref p) = a.possible_vals {
            let mut phs = vec![];
            // without derefing n = &&str
            for n in p {
                phs.push(*n);
            }
            pb.possible_vals = Some(phs);
        }
        if let Some(ref p) = a.validator {
            pb.validator = Some(p.clone());
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
            pb.requires = Some(rhs);
        }

        pb
    }
}

impl<'n> Display for PosBuilder<'n> {
    fn fmt(&self,
           f: &mut Formatter)
           -> Result {
        if self.settings.is_set(&ArgSettings::Required) {
            try!(write!(f, "<{}>", self.name));
        } else {
            try!(write!(f, "[{}]", self.name));
        }
        if self.settings.is_set(&ArgSettings::Multiple) {
            try!(write!(f, "..."));
        }

        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::PosBuilder;
    use args::settings::ArgSettings;

    #[test]
    fn posbuilder_display() {
        let mut p = PosBuilder::new("pos", 1);
        p.settings.set(&ArgSettings::Multiple);

        assert_eq!(&*format!("{}", p), "[pos]...");

        let mut p2 = PosBuilder::new("pos", 1);
        p2.settings.set(&ArgSettings::Required);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }
}
