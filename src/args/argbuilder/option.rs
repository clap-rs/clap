use std::rc::Rc;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;

use Arg;

pub struct OptBuilder<'n> {
    pub name: &'n str,
    /// The short version (i.e. single character) of the argument, no preceding `-`
    pub short: Option<char>,
    /// The long version of the flag (i.e. word) without the preceding `--`
    pub long: Option<&'n str>,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'n str>,
    /// Allow multiple occurrences of an option argument such as "-c some -c other"
    pub multiple: bool,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<Vec<&'n str>>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// A list of possible values for this argument
    pub possible_vals: Option<Vec<&'n str>>,
    /// A list of names of other arguments that are *required* to be used when
    /// this flag is used
    pub requires: Option<Vec<&'n str>>,
    pub num_vals: Option<u8>,
    pub min_vals: Option<u8>,
    pub max_vals: Option<u8>,
    pub val_names: Option<BTreeSet<&'n str>>,
    pub empty_vals: bool,
    pub global: bool,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    /// A list of names for other arguments that *mutually override* this flag
    pub overrides: Option<Vec<&'n str>>,
    pub hidden: bool
}

impl<'n> OptBuilder<'n> {
    pub fn from_arg(a: &Arg<'n, 'n, 'n, 'n, 'n,'n>,
                    reqs: &mut Vec<&'n str>) -> Self {
        if a.short.is_none() && a.long.is_none() {
            panic!("Argument \"{}\" has take_value(true), yet neither a short() or long() \
                were supplied", a.name);
        }
        // No need to check for .index() as that is handled above
        let mut ob = OptBuilder {
            name: a.name,
            short: a.short,
            long: a.long,
            multiple: a.multiple,
            blacklist: None,
            help: a.help,
            global: a.global,
            possible_vals: None,
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names.clone(),
            requires: None,
            required: a.required,
            empty_vals: a.empty_vals,
            validator: None,
            overrides: None,
            hidden: a.hidden
        };
        if let Some(ref vec) = ob.val_names {
            ob.num_vals = Some(vec.len() as u8);
        }
        if ob.min_vals.is_some() && !ob.multiple {
            panic!("Argument \"{}\" does not allow multiple values, yet it is expecting {} \
                values", ob.name, ob.num_vals.unwrap());
        }
        if ob.max_vals.is_some() && !ob.multiple {
            panic!("Argument \"{}\" does not allow multiple values, yet it is expecting {} \
                values", ob.name, ob.num_vals.unwrap());
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
                if ob.required {
                    reqs.push(*n);
                }
            }
            rhs.dedup();
            ob.requires = Some(rhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            bhs.dedup();
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
}

impl<'n> Display for OptBuilder<'n> {
    fn fmt(&self,
           f: &mut Formatter) 
           -> Result {
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
            for _ in (0..num) {
                try!(write!(f, " <{}>", self.name));
            }
            if self.multiple && num == 1 {
                try!(write!(f, "..."));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::OptBuilder;
    use std::collections::BTreeSet;

    #[test]
    fn optbuilder_display() {
        let o = OptBuilder {
            name: "opt",
            short: None,
            long: Some("option"),
            help: None,
            multiple: true,
            blacklist: None,
            required: false,
            possible_vals: None,
            requires: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            val_names: None,
            empty_vals: true,
            global: false,
            validator: None,
            overrides: None,
            hidden: false,
        };

        assert_eq!(&*format!("{}", o), "--option <opt>...");

        let mut v_names = BTreeSet::new();
        v_names.insert("file");
        v_names.insert("name");

        let o2 = OptBuilder {
            name: "opt",
            short: Some('o'),
            long: None,
            help: None,
            multiple: false,
            blacklist: None,
            required: false,
            possible_vals: None,
            requires: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            val_names: Some(v_names),
            empty_vals: true,
            global: false,
            validator: None,
            overrides: None,
            hidden: false,
        };

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }
}
