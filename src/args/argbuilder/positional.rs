use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::rc::Rc;

use Arg;

pub struct PosBuilder<'n> {
    pub name: &'n str,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'n str>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// Allow multiple occurrences of an option argument such as "-c some -c other"
    pub multiple: bool,
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
    pub empty_vals: bool,
    pub global: bool,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    /// A list of names for other arguments that *mutually override* this flag
    pub overrides: Option<Vec<&'n str>>,
    pub hidden: bool
}

impl<'n> PosBuilder<'n> {
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
            required: a.required,
            multiple: a.multiple,
            blacklist: None,
            requires: None,
            possible_vals: None,
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            help: a.help,
            global: a.global,
            empty_vals: a.empty_vals,
            validator: None,
            overrides: None,
            hidden: a.hidden
        };
        if pb.min_vals.is_some() && !pb.multiple {
            panic!("Argument \"{}\" does not allow multiple values, yet it is expecting {} \
                values", pb.name, pb.num_vals.unwrap());
        }
        if pb.max_vals.is_some() && !pb.multiple {
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
            bhs.dedup();
            pb.blacklist = Some(bhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            bhs.dedup();
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
                if pb.required {
                    reqs.push(*n);
                }
            }
            rhs.dedup();
            pb.requires = Some(rhs);
        }

        pb
    }
}

impl<'n> Display for PosBuilder<'n> {
    fn fmt(&self,
           f: &mut Formatter) 
           -> Result {
        if self.required {
            try!(write!(f, "<{}>", self.name));
        } else {
            try!(write!(f, "[{}]", self.name));
        }
        if self.multiple {
            try!(write!(f, "..."));
        }

        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::PosBuilder;

    #[test]
    fn posbuilder_display() {
        let p = PosBuilder {
            name: "pos",
            help: None,
            multiple: true,
            blacklist: None,
            required: false,
            possible_vals: None,
            requires: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            index: 1,
            empty_vals: true,
            global: false,
            validator: None,
            overrides: None,
            hidden: false,
        };

        assert_eq!(&*format!("{}", p), "[pos]...");

        let p2 = PosBuilder {
            name: "pos",
            help: None,
            multiple: false,
            blacklist: None,
            required: true,
            possible_vals: None,
            requires: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            index: 1,
            empty_vals: true,
            global: false,
            validator: None,
            overrides: None,
            hidden: false,
        };

        assert_eq!(&*format!("{}", p2), "<pos>");
    }
}
