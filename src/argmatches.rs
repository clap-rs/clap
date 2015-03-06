use std::collections::HashMap;
// use std::collections::HashSet;

// use app::App;
use args::{ FlagArg, OptArg, PosArg };
use subcommand::SubCommand;

/// Used to get information about the arguments that
/// where supplied to the program at runtime.
///
///
/// Fields of `ArgMatches` aren't designed to be used directly, only 
/// the methods in order to query information.
///
/// ```no_run
/// # use clap::{App, Arg};
///  let matches = App::new("MyApp")
/// // adding of arguments and configuration goes here...
/// #                    .arg(Arg::new("config")
/// #                               .long("config")
/// #                               .required(true)
/// #                               .takes_value(true))
/// #                    .arg(Arg::new("debug")
/// #                                   .short("d")
/// #                                   .multiple(true))
///                     .get_matches();
///    // if you had an argument named "output" that takes a value 
///    if let Some(o) = matches.value_of("output") {
///        println!("Value for output: {}", o);
///    }
///
///    // Although not advised, if you have a required argument
///    // you can call .unwrap() because the program will exit long before
///    // here at noticing the user didn't supply a required argument...
///    // use at your own risk ;)
///    println!("Config file: {}", matches.value_of("config").unwrap());
///
///    // You can check the present of an argument
///    if matches.is_present("debug") {
///        // Checking if "debug" was present was necessary,
///        // as occurrences returns 0 if a flag isn't found
///        // but we can check how many times "debug" was found
///        // if we allow multiple (if multiple isn't allowed it always be 1 or 0)
///        if matches.occurrences_of("debug") > 1 {
///            println!("Debug mode is REALLY on");
///        } else {
///            println!("Debug mode kind of on");
///        }
/// }
pub struct ArgMatches {
    pub matches_of: &'static str,
    // pub author: Option<&'static str>,
    // pub about: Option<&'static str>,
    // pub version: Option<&'static str>,
    // pub required: Vec<&'static str>,
    // pub blacklist: HashSet<&'static str>,
    pub flags: HashMap<&'static str, FlagArg>,
    pub opts: HashMap<&'static str, OptArg>,
    pub positionals: HashMap<&'static str, PosArg>,
    pub subcommand: HashMap<&'static str, SubCommand>
}

impl ArgMatches {
    /// Creates a new instance of `ArgMatches`. This ins't called directly, but
    /// through the `.get_matches()` method of `App`
    ///
    /// Example:
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
            subcommand: HashMap::new()
    		// required: vec![],
    		// blacklist: HashSet::new(),
    		// about: app.about,
    		// author: app.author,
    		// version: app.version,
    	}
	}

    /// Gets the value of a specific option or positional argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`
    ///
    /// Example:
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
        	if let Some(ref v) = opt.value {
        		return Some(v);
        	} 
        }
        if let Some(ref pos) = self.positionals.get(name) {
        	if let Some(ref v) = pos.value {
        		return Some(v);
        	}  
        }
        None
	}

    /// Checks if a flag was argument was supplied at runtime. **DOES NOT** work for
    /// option or positional arguments (use `.value_of()` instead)
    ///
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::new("output").takes_value(true)).get_matches();
    /// if matches.is_present("output") {
    ///        println!("The output argument was used!");
    /// }
    /// ```
	pub fn is_present(&self, name: &'static str) -> bool {
        if self.subcommand.contains_key(name) || 
            self.flags.contains_key(name) ||
             self.opts.contains_key(name) ||
              self.positionals.contains_key(name) {
                return true;
              }
        false
	}

    /// Checks the number of occurrences of a flag at runtime.
    ///
    /// This **DOES NOT** work for option or positional arguments 
    /// (use `.value_of()` instead). If a flag isn't present it will
    /// return `0`, if a flag doesn't allow multiple occurrences, it will
    /// return `1` no matter how many times it occurred (unless it wasn't prsent)
    /// at all.
    ///
    ///
    /// Example:
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
        0
    }

    pub fn subcommand_matches(&self, name: &'static str) -> Option<&ArgMatches> {
        if let Some(ref sc) = self.subcommand.get(name) {
            return Some(&sc.matches);
        }
        None
    }

    pub fn subcommand_name(&self) -> Option<&'static str> {
        if self.subcommand.is_empty() { return None; }
        return Some(self.subcommand.keys().collect::<Vec<_>>()[0]);
    }
}