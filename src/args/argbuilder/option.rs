use std::collections::HashSet;
use std::collections::BTreeSet;
use std::fmt::{ Display, Formatter, Result };

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
    pub blacklist: Option<HashSet<&'n str>>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// A list of possible values for this argument
    pub possible_vals: Option<BTreeSet<&'n str>>,
    /// A list of names of other arguments that are *required* to be used when
    /// this flag is used
    pub requires: Option<HashSet<&'n str>>,
    pub num_vals: Option<u8>,
    pub min_vals: Option<u8>,
    pub max_vals: Option<u8>,
    pub val_names: Option<Vec<&'n str>>,
    pub empty_vals: bool,
    pub global: bool,
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
