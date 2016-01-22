use std::ffi::{OsString, OsStr};
use std::collections::HashMap;
use std::iter::Map;
use std::slice;
use std::borrow::Cow;

use vec_map;

use args::SubCommand;
use args::MatchedArg;
use utf8::INVALID_UTF8;

/// Used to get information about the arguments that where supplied to the program at runtime by
/// the user. To get a new instance of this struct you use `.get_matches()` of the `App` struct.
///
///
/// # Examples
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
#[derive(Debug, Clone)]
pub struct ArgMatches<'a> {
    #[doc(hidden)]
    pub args: HashMap<&'a str, MatchedArg>,
    #[doc(hidden)]
    pub subcommand: Option<Box<SubCommand<'a>>>,
    #[doc(hidden)]
    pub usage: Option<String>,
}

impl<'a> Default for ArgMatches<'a> {
    fn default() -> Self {
        ArgMatches {
            args: HashMap::new(),
            subcommand: None,
            usage: None,
        }
    }
}

impl<'a> ArgMatches<'a> {
    /// Creates a new instance of `ArgMatches`. This ins't called directly, but
    /// through the `.get_matches()` method of `App`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog").get_matches();
    /// ```
    #[doc(hidden)]
    pub fn new() -> Self { ArgMatches { ..Default::default() } }

    /// Gets the value of a specific option or positional argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `values_of()` as `value_of()` will only return the _*first*_ value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp")
    /// #     .arg(Arg::with_name("output")
    /// #         .takes_value(true))
    /// #     .get_matches();
    /// if let Some(o) = matches.value_of("output") {
    ///        println!("Value for output: {}", o);
    /// }
    /// ```
    pub fn value_of<S: AsRef<str>>(&self, name: S) -> Option<&str> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            if let Some(v) = arg.vals.values().nth(0) {
                return Some(v.to_str().expect(INVALID_UTF8));
            }
        }
        None
    }

    pub fn lossy_value_of<S: AsRef<str>>(&'a self, name: S) -> Option<Cow<'a, str>> {
        if let Some(arg) = self.args.get(name.as_ref()) {
            if let Some(v) = arg.vals.values().nth(0) {
                return Some(v.to_string_lossy());
            }
        }
        None
    }

    pub fn os_value_of<S: AsRef<str>>(&self, name: S) -> Option<&OsStr> {
        self.args.get(name.as_ref()).map(|arg| arg.vals.values().nth(0).map(|v| v.as_os_str())).unwrap_or(None)
    }

    /// Gets the values of a specific option or positional argument in a vector (i.e. an argument
    /// that takes multiple values at runtime). If the option wasn't present at runtime it
    /// returns `None`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp")
    /// #     .arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// // If the program had option "-c" that took a value and was run
    /// // via "myapp -o some -o other -o file"
    /// // values_of() would return a [&str; 3] ("some", "other", "file")
    /// if let Some(os) = matches.values_of("output") {
    ///        for o in os {
    ///            println!("A value for output: {}", o);
    ///        }
    /// }
    /// ```
    pub fn values_of<S: AsRef<str>>(&'a self, name: S) -> Option<Values<'a>> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            fn to_str_slice<'a>(o: &'a OsString) -> &'a str { o.to_str().expect(INVALID_UTF8) }
            let to_str_slice: fn(&'a OsString) -> &'a str = to_str_slice; // coerce to fn pointer
            return Some(Values { iter: arg.vals.values().map(to_str_slice) });
        }
        None
    }

    pub fn lossy_values_of<S: AsRef<str>>(&'a self, name: S) -> Option<Vec<String>> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            return Some(arg.vals.values()
                           .map(|v| v.to_string_lossy().into_owned())
                           .collect());
        }
        None
    }

    pub fn os_values_of<S: AsRef<str>>(&'a self, name: S) -> Option<OsValues<'a>> {
        fn to_str_slice<'a>(o: &'a OsString) -> &'a OsStr { &*o }
        let to_str_slice: fn(&'a OsString) -> &'a OsStr = to_str_slice; // coerce to fn pointer
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            return Some(OsValues { iter: arg.vals.values().map(to_str_slice) });
        }
        None
    }

    /// Returns if an argument was present at runtime.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp")
    /// #     .arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// if matches.is_present("output") {
    ///        println!("The output argument was used!");
    /// }
    /// ```
    pub fn is_present<S: AsRef<str>>(&self, name: S) -> bool {
        if let Some(ref sc) = self.subcommand {
            if sc.name == name.as_ref() {
                return true;
            }
        }
        self.args.contains_key(name.as_ref())
    }

    /// Returns the number of occurrences of an option, flag, or positional argument at runtime.
    /// If an argument isn't present it will return `0`. Can be used on arguments which *don't*
    /// allow multiple occurrences, but will obviously only return `0` or `1`.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp")
    /// #     .arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// if matches.occurrences_of("debug") > 1 {
    ///     println!("Debug mode is REALLY on");
    /// } else {
    ///     println!("Debug mode kind of on");
    /// }
    /// ```
    pub fn occurrences_of<S: AsRef<str>>(&self, name: S) -> u8 {
        self.args.get(name.as_ref()).map(|a| a.occurs).unwrap_or(0)
    }

    /// Returns the `ArgMatches` for a particular subcommand or None if the subcommand wasn't
    /// present at runtime.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// if let Some(matches) = app_matches.subcommand_matches("test") {
    ///     // Use matches as normal
    /// }
    /// ```
    pub fn subcommand_matches<S: AsRef<str>>(&self, name: S) -> Option<&ArgMatches<'a>> {
        self.subcommand.as_ref().map(|s| if s.name == name.as_ref() { Some(&s.matches) } else { None } ).unwrap()
    }

    /// Returns the name of the subcommand used of the parent `App`, or `None` if one wasn't found
    ///
    /// *NOTE*: Only a single subcommand may be present per `App` at runtime, does *NOT* check for
    /// the name of sub-subcommand's names
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// match app_matches.subcommand_name() {
    ///     Some("test")   => {}, // test was used
    ///     Some("config") => {}, // config was used
    ///     _              => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    pub fn subcommand_name(&self) -> Option<&str> {
        self.subcommand.as_ref().map(|sc| &sc.name[..])
    }

    /// Returns the name and `ArgMatches` of the subcommand used at runtime or ("", None) if one
    /// wasn't found.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// match app_matches.subcommand() {
    ///     ("test", Some(matches))   => {}, // test was used
    ///     ("config", Some(matches)) => {}, // config was used
    ///     _                         => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    pub fn subcommand(&self) -> (&str, Option<&ArgMatches<'a>>) {
        self.subcommand.as_ref().map(|sc| (&sc.name[..], Some(&sc.matches))).unwrap_or(("", None))
    }

    /// Returns a string slice of the usage statement for the `App` (or `SubCommand`)
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// println!("{}",app_matches.usage());
    /// ```
    pub fn usage(&self) -> &str {
        self.usage.as_ref().map(|u| &u[..]).unwrap_or("")
    }
}


// The following were taken and adapated from vec_map source
// repo: https://github.com/contain-rs/vec-map
// commit: be5e1fa3c26e351761b33010ddbdaf5f05dbcc33
// license: MIT - Copyright (c) 2015 The Rust Project Developers

#[derive(Clone)]
pub struct Values<'a> {
    iter: Map<vec_map::Values<'a, OsString>, fn(&'a OsString) -> &'a str>
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a> DoubleEndedIterator for Values<'a> {
    fn next_back(&mut self) -> Option<&'a str> { self.iter.next_back() }
}

/// An iterator over the key-value pairs of a map.
#[derive(Clone)]
pub struct Iter<'a, V:'a> {
    front: usize,
    back: usize,
    iter: slice::Iter<'a, Option<V>>
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        while self.front < self.back {
            match self.iter.next() {
                Some(elem) => {
                    match elem.as_ref() {
                        Some(x) => {
                            // let index = self.front;
                            self.front += 1;
                            return Some(x);
                        },
                        None => {},
                    }
                }
                _ => ()
            }
            self.front += 1;
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.back - self.front))
    }
}

impl<'a, V> DoubleEndedIterator for Iter<'a, V> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a V> {
        while self.front < self.back {
            match self.iter.next_back() {
                Some(elem) => {
                    match elem.as_ref() {
                        Some(x) => {
                            self.back -= 1;
                            return Some(x);
                        },
                        None => {},
                    }
                }
                _ => ()
            }
            self.back -= 1;
        }
        None
    }
}

#[derive(Clone)]
pub struct OsValues<'a> {
    iter: Map<vec_map::Values<'a, OsString>, fn(&'a OsString) -> &'a OsStr>
}

impl<'a> Iterator for OsValues<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<&'a OsStr> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a> DoubleEndedIterator for OsValues<'a> {
    fn next_back(&mut self) -> Option<&'a OsStr> { self.iter.next_back() }
}
