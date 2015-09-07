use std::rc::Rc;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;

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
        };

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }
}
