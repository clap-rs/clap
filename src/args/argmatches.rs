use std::collections::HashMap;

use args::{ FlagArg, OptArg, PosArg, SubCommand };

/// Used to get information about the arguments that
/// where supplied to the program at runtime.
///
///
/// Fields of `ArgMatches` aren't designed to be used directly, only 
/// the methods in order to query information.
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg};
/// let matches = App::new("MyApp")
/// // adding of arguments and configuration goes here...
/// #                    .arg(Arg::new("config")
/// #                               .long("config")
/// #                               .required(true)
/// #                               .takes_value(true))
/// #                    .arg(Arg::new("debug")
/// #                                   .short("d")
/// #                                   .multiple(true))
///                     .get_matches();
/// // if you had an argument named "output" that takes a value 
/// if let Some(o) = matches.value_of("output") {
///     println!("Value for output: {}", o);
/// }
///
/// // Although not advised, if you have a required argument
/// // you can call .unwrap() because the program will exit long before
/// // here at noticing the user didn't supply a required argument...
/// // use at your own risk ;)
/// println!("Config file: {}", matches.value_of("config").unwrap());
///
/// // You can check the present of an argument
/// if matches.is_present("debug") {
///     // Checking if "debug" was present was necessary,
///     // as occurrences returns 0 if a flag isn't found
///     // but we can check how many times "debug" was found
///     // if we allow multiple (if multiple isn't allowed it always be 1 or 0)
///     if matches.occurrences_of("debug") > 1 {
///         println!("Debug mode is REALLY on");
///     } else {
///         println!("Debug mode kind of on");
///     }
/// }
///
/// // You can get the sub-matches of a particular subcommand (in this case "test")
/// // If "test" had it's own "-l" flag you could check for it's presence accordingly
/// if let Some(ref matches) = matches.subcommand_matches("test") {
///     if matches.is_present("list") {
///         println!("Printing testing lists...");
///     } else {
///         println!("Not printing testing lists...");
///     }
/// }
pub struct ArgMatches {
    pub matches_of: &'static str,
    pub flags: HashMap<&'static str, FlagArg>,
    pub opts: HashMap<&'static str, OptArg>,
    pub positionals: HashMap<&'static str, PosArg>,
    pub subcommand: Option<(&'static str, Box<SubCommand>)>
}

impl ArgMatches {
    /// Creates a new instance of `ArgMatches`. This ins't called directly, but
    /// through the `.get_matches()` method of `App`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog").get_matches();
    /// ```
    pub fn new(name: &'static str) -> ArgMatches {
        ArgMatches {
            matches_of: name,
            flags: HashMap::new(),
            opts: HashMap::new(),
            positionals: HashMap::new(),
            subcommand: None
        }
    }

    /// Gets the value of a specific option or positional argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`. 
    ///
    /// *NOTE:* If getting a value for an option argument that allows multiples, prefer `values_of()`
    /// as `value_of()` will only return the _*first*_ value.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::new("output").takes_value(true)).get_matches();
    /// if let Some(o) = matches.value_of("output") {
    ///        println!("Value for output: {}", o);
    /// }
    /// ```
    pub fn value_of(&self, name: &'static str) -> Option<&String> {
        if let Some(ref opt) = self.opts.get(name) {
            if ! opt.values.is_empty() {
                if let Some(ref s) = opt.values.iter().nth(0) {
                    return Some(s);
                }
            } 
        }
        if let Some(ref pos) = self.positionals.get(name) {
            if let Some(ref v) = pos.value {
                return Some(v);
            }  
        }
        None
    }

    /// Gets the values of a specific option in a vector (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::new("output").takes_value(true)).get_matches();
    /// // If the program had option "-c" that took a value and was run
    /// // via "myapp -o some -o other -o file"
    /// // values_of() would return a [&str; 3] ("some", "other", "file")
    /// if let Some(os) = matches.values_of("output") {
    ///        for o in os {
    ///            println!("A value for output: {}", o);
    ///        }
    /// }
    /// ```
    pub fn values_of(&self, name: &'static str) -> Option<Vec<&str>> {
        if let Some(ref opt) = self.opts.get(name) {
            if opt.values.is_empty() { return None; } 

            return Some(opt.values.iter().map(|s| &s[..]).collect::<Vec<_>>());
        }
        None
    }

    /// Checks if a flag was argument was supplied at runtime. **DOES NOT** work for
    /// option or positional arguments (use `.value_of()` instead)
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::new("output").takes_value(true)).get_matches();
    /// if matches.is_present("output") {
    ///        println!("The output argument was used!");
    /// }
    /// ```
    pub fn is_present(&self, name: &'static str) -> bool {
        if let Some((sc_name, _ )) = self.subcommand {
            if sc_name == name { return true; } 
        }
        if self.flags.contains_key(name) ||
             self.opts.contains_key(name) ||
              self.positionals.contains_key(name) {
                return true;
              }
        false
    }

    /// Checks the number of occurrences of an option or flag at runtime. 
    /// If an option or flag isn't present it will return `0`, if the option or flag doesn't 
    /// allow multiple occurrences, it will return `1` no matter how many times it occurred 
    /// (unless it wasn't prsent) at all.
    ///
    /// *NOTE:* This _*DOES NOT*_ work for positional arguments (use `.value_of()` instead). 
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::new("output").takes_value(true)).get_matches();
    /// if matches.occurrences_of("debug") > 1 {
    ///     println!("Debug mode is REALLY on");
    /// } else {
    ///     println!("Debug mode kind of on");
    /// }
    /// ```
    pub fn occurrences_of(&self, name: &'static str) -> u8 {
        if let Some(ref f) = self.flags.get(name) {
            return f.occurrences;
        }
        if let Some(ref o) = self.opts.get(name) {
            return o.occurrences;
        }
        0
    }

    /// If a subcommand was found, returns the ArgMatches struct associated with it's matches
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp").subcommand(SubCommand::new("test")).get_matches();
    /// if let Some(matches) = app_matches.subcommand_matches("test") {
    ///     // Use matches as normal
    /// }
    /// ```
    pub fn subcommand_matches(&self, name: &'static str) -> Option<&ArgMatches> {
        if let Some( ( sc_name, ref sc)) = self.subcommand {
            if sc_name != name { return None; }
            return Some(&sc.matches);
        }
        None
    }

    /// If a subcommand was found, returns the name associated with it
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp").subcommand(SubCommand::new("test")).get_matches();
    /// match app_matches.subcommand_name() {
    ///     Some("test")   => {}, // test was used
    ///     Some("config") => {}, // config was used
    ///     _              => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    pub fn subcommand_name(&self) -> Option<&'static str> {
        if let Some((name, _)) = self.subcommand {
            return Some(name);
        }
        None
    }
}