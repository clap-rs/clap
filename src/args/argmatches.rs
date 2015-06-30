use std::collections::HashMap;

use args::SubCommand;
use args::MatchedArg;

/// Used to get information about the arguments that where supplied to the program at runtime by
/// the user. To get a new instance of this struct you use `.get_matches()` of the `App` struct.
///
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg};
/// let matches = App::new("MyApp")
/// // adding of arguments and configuration goes here...
/// #                    .arg(Arg::with_name("config")
/// #                               .long("config")
/// #                               .required(true)
/// #                               .takes_value(true))
/// #                    .arg(Arg::with_name("debug")
/// #                                   .short("d")
/// #                                   .multiple(true))
///                     .get_matches();
/// // if you had an argument named "output" that takes a value
/// if let Some(o) = matches.value_of("output") {
///     println!("Value for output: {}", o);
/// }
///
/// // If you have a required argument you can call .unwrap() because the program will exit long
/// // before this point if the user didn't specify it at runtime.
/// println!("Config file: {}", matches.value_of("config").unwrap());
///
/// // You can check the presence of an argument
/// if matches.is_present("debug") {
///     // Another way to check if an argument was present, or if it occurred multiple times is to
///     // use occurrences_of() which returns 0 if an argument isn't found at runtime, or the
///     // number of times that it occurred, if it was. To allow an argument to appear more than
///     // once, you must use the .multiple(true) method, otherwise it will only return 1 or 0.
///     if matches.occurrences_of("debug") > 2 {
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
pub struct ArgMatches<'n, 'a> {
    #[doc(hidden)]
    pub args: HashMap<&'a str, MatchedArg>,
    #[doc(hidden)]
    pub subcommand: Option<Box<SubCommand<'n, 'a>>>,
    #[doc(hidden)]
    pub usage: Option<String>
}

impl<'n, 'a> ArgMatches<'n, 'a> {
    /// Creates a new instance of `ArgMatches`. This ins't called directly, but
    /// through the `.get_matches()` method of `App`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog").get_matches();
    /// ```
    #[doc(hidden)]
    pub fn new() -> ArgMatches<'n, 'a> {
        ArgMatches {
            args: HashMap::new(),
            subcommand: None,
            usage: None
        }
    }

    /// Gets the value of a specific option or positional argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `values_of()` as `value_of()` will only return the _*first*_ value.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// if let Some(o) = matches.value_of("output") {
    ///        println!("Value for output: {}", o);
    /// }
    /// ```
    pub fn value_of<'na>(&self, name: &'na str) -> Option<&str> {
        if let Some(ref arg) = self.args.get(name) {
            if let Some(ref vals) = arg.values {
                if let Some(ref val) = vals.values().nth(0) {
                    return Some(&val[..]);
                }
            }
        }
        None
    }

    /// Gets the values of a specific option or positional argument in a vector (i.e. an argument
    /// that takes multiple values at runtime). If the option wasn't present at runtime it
    /// returns `None`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// // If the program had option "-c" that took a value and was run
    /// // via "myapp -o some -o other -o file"
    /// // values_of() would return a [&str; 3] ("some", "other", "file")
    /// if let Some(os) = matches.values_of("output") {
    ///        for o in os {
    ///            println!("A value for output: {}", o);
    ///        }
    /// }
    /// ```
    pub fn values_of<'na>(&'a self, name: &'na str) -> Option<Vec<&'a str>> {
        if let Some(ref arg) = self.args.get(name) {
            if let Some(ref vals) = arg.values {
                return Some(vals.values().map(|s| &s[..]).collect::<Vec<_>>());
            }
        }
        None
    }

    /// Returns if an argument was present at runtime.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// if matches.is_present("output") {
    ///        println!("The output argument was used!");
    /// }
    /// ```
    pub fn is_present<'na>(&self, name: &'na str) -> bool {
        if let Some(ref sc) = self.subcommand {
            if sc.name == name { return true; }
        }
        if self.args.contains_key(name) {return true;}
        false
    }

    /// Returns the number of occurrences of an option, flag, or positional argument at runtime.
    /// If an argument isn't present it will return `0`. Can be used on arguments which *don't*
    /// allow multiple occurrences, but will obviously only return `0` or `1`.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp").arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// if matches.occurrences_of("debug") > 1 {
    ///     println!("Debug mode is REALLY on");
    /// } else {
    ///     println!("Debug mode kind of on");
    /// }
    /// ```
    pub fn occurrences_of<'na>(&self, name: &'na str) -> u8 {
        if let Some(ref arg) = self.args.get(name) {
            return arg.occurrences;
        }
        0
    }

    /// Returns the `ArgMatches` for a particular subcommand or None if the subcommand wasn't
    /// present at runtime.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp").subcommand(SubCommand::with_name("test")).get_matches();
    /// if let Some(matches) = app_matches.subcommand_matches("test") {
    ///     // Use matches as normal
    /// }
    /// ```
    pub fn subcommand_matches<'na>(&self, name: &'na str) -> Option<&ArgMatches> {
        if let Some( ref sc) = self.subcommand {
            if sc.name != name { return None; }
            return Some(&sc.matches);
        }
        None
    }

    /// Returns the name of the subcommand used of the parent `App`, or `None` if one wasn't found
    ///
    /// *NOTE*: Only a single subcommand may be present per `App` at runtime, does *NOT* check for
    /// the name of sub-subcommand's names
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp").subcommand(SubCommand::with_name("test")).get_matches();
    /// match app_matches.subcommand_name() {
    ///     Some("test")   => {}, // test was used
    ///     Some("config") => {}, // config was used
    ///     _              => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    pub fn subcommand_name(&self) -> Option<&str> {
        if let Some( ref sc ) = self.subcommand {
            return Some(&sc.name[..]);
        }
        None
    }

    /// Returns the name and `ArgMatches` of the subcommand used at runtime or ("", None) if one
    /// wasn't found.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp").subcommand(SubCommand::with_name("test")).get_matches();
    /// match app_matches.subcommand() {
    ///     ("test", Some(matches))   => {}, // test was used
    ///     ("config", Some(matches)) => {}, // config was used
    ///     _                         => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    pub fn subcommand(&self) -> (&str, Option<&ArgMatches>) {
        if let Some( ref sc ) = self.subcommand {
            return (&sc.name[..], Some(&sc.matches));
        }
        ("", None)
    }

    /// Returns a string slice of the usage statement for the `App` (or `SubCommand`)
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp").subcommand(SubCommand::with_name("test")).get_matches();
    /// println!("{}",app_matches.usage());
    /// ```
    pub fn usage(&self) -> &str {
        if let Some( ref u ) = self.usage {
            return &u[..];
        }

        // Should be un-reachable
        ""
    }
}
