use std::ffi::{OsString, OsStr};
use std::collections::HashMap;
use std::iter::Map;
use std::slice;
use std::borrow::Cow;
use std::str::FromStr;

use vec_map;

use args::SubCommand;
use args::MatchedArg;
use args::SubCommandKey;
use INVALID_UTF8;

/// Used to get information about the arguments that where supplied to the program at runtime by
/// the user. New instances of this struct are obtained by using the `App::get_matches` family of
/// methods.
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg};
/// let matches = App::new("MyApp")
///     .arg(Arg::with_name("out")
///         .long("output")
///         .required(true)
///         .takes_value(true))
///     .arg(Arg::with_name("debug")
///         .short("d")
///         .multiple(true))
///     .arg(Arg::with_name("cfg")
///         .short("c")
///         .takes_value(true))
///     .get_matches(); // builds the instance of ArgMatches
///
/// // to get information about the "cfg" argument we created, such as the value supplied we use
/// // various ArgMatches methods, such as ArgMatches::value_of
/// if let Some(c) = matches.value_of("cfg") {
///     println!("Value for -c: {}", c);
/// }
///
/// // The ArgMatches::value_of method returns an Option because the user may not have supplied
/// // that argument at runtime. But if we specified that the argument was "required" as we did
/// // with the "out" argument, we can safely unwrap because `clap` verifies that was actually
/// // used at runtime.
/// println!("Value for --output: {}", matches.value_of("out").unwrap());
///
/// // You can check the presence of an argument
/// if matches.is_present("out") {
///     // Another way to check if an argument was present, or if it occurred multiple times is to
///     // use occurrences_of() which returns 0 if an argument isn't found at runtime, or the
///     // number of times that it occurred, if it was. To allow an argument to appear more than
///     // once, you must use the .multiple(true) method, otherwise it will only return 1 or 0.
///     if matches.occurrences_of("debug") > 2 {
///         println!("Debug mode is REALLY on, don't be crazy");
///     } else {
///         println!("Debug mode kind of on");
///     }
/// }
/// ```
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
    #[doc(hidden)]
    pub fn new() -> Self { ArgMatches { ..Default::default() } }

    /// Gets the value of a specific option or positional argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `values_of()` as `value_of()` will only return the *first* value.
    ///
    /// # Panics
    ///
    /// This method will `panic!` if the value contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::with_name("output")
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["myapp", "something"]);
    ///
    /// assert_eq!(m.value_of("output"), Some("something"));
    /// ```
    pub fn value_of<S: AsRef<str>>(&self, name: S) -> Option<&str> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            if let Some(v) = arg.vals.values().nth(0) {
                return Some(v.to_str().expect(INVALID_UTF8));
            }
        }
        None
    }

    /// Gets the lossy value of a specific argument. If the argument wasn't present at runtime
    /// it returns `None`. A lossy value is one which contains invalid UTF-8 code points, those
    /// invalid points will be replaced with `\u{FFFD}`
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `values_of_lossy()` as `value_of_lossy()` will only return the *first* value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.value_of_lossy("arg").unwrap(), "Hi \u{FFFD}!");
    /// ```
    pub fn value_of_lossy<S: AsRef<str>>(&'a self, name: S) -> Option<Cow<'a, str>> {
        if let Some(arg) = self.args.get(name.as_ref()) {
            if let Some(v) = arg.vals.values().nth(0) {
                return Some(v.to_string_lossy());
            }
        }
        None
    }

    /// Gets the OS version of a string value of a specific argument. If the option wasn't present
    /// at runtime it returns `None`. An OS value on Unix-like systems is any series of bytes,
    /// regardless of whether or not they contain valid UTF-8 code points. Since `String`s in Rust
    /// are guaranteed to be valid UTF-8, a valid filename on a Unix system as an argument value may
    /// contain invalid UTF-8 code points.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `values_of_os()` as `value_of_os()` will only return the *first* value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.value_of_os("arg").unwrap().as_bytes(), [b'H', b'i', b' ', 0xe9, b'!']);
    /// ```
    pub fn value_of_os<S: AsRef<str>>(&self, name: S) -> Option<&OsStr> {
        self.args.get(name.as_ref()).map_or(None, |arg| arg.vals.values().nth(0).map(|v| v.as_os_str()))
    }

    /// Gets an Iterator of values of a specific argument (i.e. an argument that takes multiple
    /// values at runtime). If the option wasn't present at runtime it returns `None`
    ///
    /// # Panics
    ///
    /// This method will panic if any of the values contain invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .arg(Arg::with_name("output")
    ///         .multiple(true)
    ///         .short("o")
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "myprog", "-o", "val1", "val2", "val3"
    ///     ]);
    /// let vals: Vec<&str> = m.values_of("output").unwrap().collect();
    /// assert_eq!(vals, ["val1", "val2", "val3"]);
    /// ```
    pub fn values_of<S: AsRef<str>>(&'a self, name: S) -> Option<Values<'a>> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            fn to_str_slice(o: &OsString) -> &str { o.to_str().expect(INVALID_UTF8) }
            let to_str_slice: fn(&OsString) -> &str = to_str_slice; // coerce to fn pointer
            return Some(Values { iter: arg.vals.values().map(to_str_slice) });
        }
        None
    }

    /// Gets the lossy values of a specific argument If the option wasn't present at runtime
    /// it returns `None`. A lossy value is one which contains invalid UTF-8 code points, those
    /// invalid points will be replaced with `\u{FFFD}`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// let itr = m.values_of_lossy("arg").unwrap();
    /// assert_eq!(&*itr.next().unwrap(), "Hi");
    /// assert_eq!(&*itr.next().unwrap(), "\u{FFFD}!");
    /// assert_eq!(itr.next(), None);
    /// ```
    pub fn values_of_lossy<S: AsRef<str>>(&'a self, name: S) -> Option<Vec<String>> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            return Some(arg.vals.values()
                           .map(|v| v.to_string_lossy().into_owned())
                           .collect());
        }
        None
    }

    /// Gets the OS version of a string value of a specific argument If the option wasn't present
    /// at runtime it returns `None`. An OS value on Unix-like systems is any series of bytes,
    /// regardless of whether or not they contain valid UTF-8 code points. Since `String`s in Rust
    /// are guaranteed to be valid UTF-8, a valid filename as an argument value on Linux (for
    /// example) may contain invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                                 // "Hi"
    ///                                 OsString::from_vec(vec![b'H', b'i']),
    ///                                 // "{0xe9}!"
    ///                                 OsString::from_vec(vec![0xe9, b'!'])]);
    ///
    /// let itr = m.values_of_os("arg").unwrap();
    /// assert_eq!(itr.next(), Some(&*OsString::from("Hi")));
    /// assert_eq!(itr.next(), Some(&*OsString::from_vec(vec![0xe9, b'!'])));
    /// assert_eq!(itr.next(), None);
    /// ```
    pub fn values_of_os<S: AsRef<str>>(&'a self, name: S) -> Option<OsValues<'a>> {
        fn to_str_slice(o: &OsString) -> &OsStr { &*o }
        let to_str_slice: fn(&'a OsString) -> &'a OsStr = to_str_slice; // coerce to fn pointer
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            return Some(OsValues { iter: arg.vals.values().map(to_str_slice) });
        }
        None
    }

    /// Returns `true` if an argument was present at runtime, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .short("d"))
    ///     .get_matches_from(vec![
    ///         "myprog", "-d"
    ///     ]);
    ///
    /// assert!(m.is_present("debug"));
    /// ```
    pub fn is_present<S: AsRef<str>>(&self, name: S) -> bool {
        if let Some(ref sc) = self.subcommand {
            if &sc.name[..] == name.as_ref() {
                return true;
            }
        }
        self.args.contains_key(name.as_ref())
    }

    /// Returns the number of times an argument was used at runtime. If an argument isn't present
    /// it will return `0`.
    ///
    /// **NOTE:** This returns the number of times the argument was used, *not* the number of
    /// values. For example, `-o val1 val2 val3 -o val4` would return `2`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .short("d")
    ///         .multiple(true))
    ///     .get_matches_from(vec![
    ///         "myprog", "-d", "-d", "-d"
    ///     ]);
    ///
    /// assert_eq!(m.occurrences_of("debug"), 3);
    /// ```
    ///
    /// This next example shows that counts actual uses of the argument, not just `-`'s
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .short("d")
    ///         .multiple(true))
    ///     .arg(Arg::with_name("flag")
    ///         .short("f"))
    ///     .get_matches_from(vec![
    ///         "myprog", "-ddfd"
    ///     ]);
    ///
    /// assert_eq!(m.occurrences_of("debug"), 3);
    /// assert_eq!(m.occurrences_of("flag"), 1);
    /// ```
    pub fn occurrences_of<S: AsRef<str>>(&self, name: S) -> u64 {
        self.args.get(name.as_ref()).map_or(0, |a| a.occurs)
    }

    /// Because subcommands are essentially "sub-apps" they have their own `ArgMatches` as well.
    /// This method returns the `ArgMatches` for a particular subcommand or None if the subcommand
    /// wasn't present at runtime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand};
    /// let app_m = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .short("d"))
    ///     .subcommand(SubCommand::with_name("test")
    ///         .arg(Arg::with_name("opt")
    ///             .long("option")
    ///             .takes_value(true)))
    ///     .get_matches_from(vec![
    ///         "myprog", "-d", "test", "--option", "val"
    ///     ]);
    ///
    /// // Both parent commands, and child subcommands can have arguments present at the same times
    /// assert!(app_m.is_present("debug"));
    ///
    /// // Get the subcommand's ArgMatches instance
    /// if let Some(sub_m) = app_m.subcommand_matches("test") {
    ///     // Use the struct like normal
    ///     assert_eq!(sub_m.value_of("opt"), Some("val"));
    /// }
    /// ```
    pub fn subcommand_matches<S: AsRef<str>>(&self, name: S) -> Option<&ArgMatches<'a>> {
        if let Some(ref s) = self.subcommand {
            if &s.name[..] == name.as_ref() { return Some(&s.matches) }
        }
        None
    }

    /// Because subcommands are essentially "sub-apps" they have their own `ArgMatches` as well.
    /// But simply getting the sub-`ArgMatches` doesn't help much if we don't also know which
    /// subcommand was actually used. This method returns the name of the subcommand that was used
    /// at runtime, or `None` if one wasn't.
    ///
    /// *NOTE*: Subcommands form a hierarchy, where multiple subcommands can be used at runtime,
    /// but only a single subcommand from any group of sibling commands may used at once.
    ///
    /// An ASCII art depiction may help explain this better...Using a fictional version of `git` as
    /// the demo subject. Imagine the following are all subcommands of `git` (note, the author is
    /// aware these aren't actually all subcommands in the real `git` interface, but it makes
    /// explaination easier)
    ///
    /// ```notrust
    ///              Top Level App (git)                         TOP
    ///                              |
    ///       -----------------------------------------
    ///      /             |                \          \
    ///   clone          push              add       commit      LEVEL 1
    ///     |           /    \            /    \       |
    ///    url      origin   remote    ref    name   message     LEVEL 2
    ///             /                  /\
    ///          path            remote  local                   LEVEL 3
    /// ```
    ///
    /// Given the above fictional subcommand hierarchy, valid runtime uses would be (not an all
    /// inclusive list, and not including argument options per command for brevity and clarity):
    ///
    /// ```ignore
    /// $ git clone url
    /// $ git push origin path
    /// $ git add ref local
    /// $ git commit message
    /// ```
    ///
    /// Notice only one command per "level" may be used. You could *not*, for example, do `$ git
    /// clone url push origin path`
    ///
    /// # Examples
    ///
    /// Subcommands can use either strings or enum variants as names and accessors. Using strings
    /// is convienient, but can lead to simple typing errors and other such issues. This is
    /// sometimes referred to as being, "stringly typed" and generally avoided if possible.
    ///
    /// This is why subcommands also have the option to use enum variants as their accessors, which
    /// allows `rustc` to do some compile time checking for, to avoid all the common "stringly 
    /// typed" issues.
    ///
    /// This first example shows a "stringly" typed version.
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    ///  let app_m = App::new("git")
    ///      .subcommand(SubCommand::with_name("clone"))
    ///      .subcommand(SubCommand::with_name("push"))
    ///      .subcommand(SubCommand::with_name("commit"))
    ///      .get_matches();
    ///
    /// match app_m.subcommand_name() {
    ///     Some("clone")  => {}, // clone was used
    ///     Some("push")   => {}, // push was used
    ///     Some("commit") => {}, // commit was used
    ///     _              => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    /// This next example shows a functionally equivolent strong typed version.
    ///
    /// ```no_run
    /// # #[macro_use]
    /// # extern crate clap;
    /// # use clap::{App, Arg, SubCommand};
    /// subcommands!{
    ///     enum Git {
    ///         clone,
    ///         push,
    ///         commit
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let app_m = App::new("git")
    ///          .subcommand(SubCommand::with_name(Git::clone))
    ///          .subcommand(SubCommand::with_name(Git::push))
    ///          .subcommand(SubCommand::with_name(Git::commit))
    ///          .get_matches();
    ///
    ///     match app_m.subcommand_name() {
    ///         Some(Git::clone)  => {}, // clone was used
    ///         Some(Git::push)   => {}, // push was used
    ///         Some(Git::commit) => {}, // commit was used
    ///         _                 => {}, // No subcommand was used
    ///     }
    /// }
    /// ```
    pub fn subcommand_name<'s, S>(&'s self) -> Option<S> where S: SubCommandKey<'s> {
        self.subcommand.as_ref().map(|sc| S::from_str(&sc.name[..]))
    }

    /// This brings together `ArgMatches::subcommand_matches` and `ArgMatches::subcommand_name` by
    /// returning a tuple with both pieces of information.
    ///
    /// Like the other methods, can either be stringly typed, or use enum variants.
    ///
    /// # Examples
    ///
    /// An example using strings.
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    ///  let app_m = App::new("git")
    ///      .subcommand(SubCommand::with_name("clone"))
    ///      .subcommand(SubCommand::with_name("push"))
    ///      .subcommand(SubCommand::with_name("commit"))
    ///      .get_matches();
    ///
    /// match app_m.subcommand() {
    ///     ("clone",  Some(sub_m)) => {}, // clone was used
    ///     ("push",   Some(sub_m)) => {}, // push was used
    ///     ("commit", Some(sub_m)) => {}, // commit was used
    ///     _                       => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    ///
    /// This next example shows a functionally equivolent strong typed version.
    ///
    /// ```no_run
    /// # #[macro_use]
    /// # extern crate clap;
    /// # use clap::{App, Arg, SubCommand};
    /// subcommands!{
    ///     enum Git {
    ///         clone,
    ///         push,
    ///         commit
    ///     }
    ///  }
    ///
    /// fn main() {
    ///     let app_m = App::new("git")
    ///          .subcommand(SubCommand::with_name(Git::clone))
    ///          .subcommand(SubCommand::with_name(Git::push))
    ///          .subcommand(SubCommand::with_name(Git::commit))
    ///          .get_matches();
    ///
    ///     match app_m.subcommand() {
    ///         (Git::clone, Some(sub_m))  => {}, // clone was used
    ///         (Git::push, Some(sub_m))   => {}, // push was used
    ///         (Git::commit, Some(sub_m)) => {}, // commit was used
    ///         (Git::None, _)             => {}, // No subcommand was used
    ///         (_, None)                  => {}, // Unreachable
    ///     }
    /// }
    /// ```
    ///
    /// Another useful scenario is when you want to support third party, or external, subcommands.
    /// In these cases you can't know the subcommand name ahead of time, so use a variable instead
    /// with pattern matching!
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let app_m = App::new("myprog")
    ///     .setting(AppSettings::AllowExternalSubcommands)
    ///     .get_matches_from(vec![
    ///         "myprog", "subcmd", "--option", "value", "-fff", "--flag"
    ///     ]);
    ///
    /// // All trailing arguments will be stored under the subcommand's sub-matches using a value
    /// // of the runtime subcommand name (in this case "subcmd")
    /// match app_m.subcommand() {
    ///     ("do-stuff", Some(sub_m)) => { /* do-stuff was used, internal subcommand */ },
    ///     (external, Some(sub_m)) => {
    ///          let ext_args: Vec<&str> = sub_m.values_of(external).unwrap().collect();
    ///          assert_eq!(ext_args, ["--option", "value", "-fff", "--flag"]);
    ///     },
    ///     _ => {},
    /// }
    /// ```
    pub fn subcommand<'s, S>(&'s self) -> (S, Option<&ArgMatches<'a>>) where S: SubCommandKey<'s> {
        self.subcommand.as_ref().map_or((S::none(), None), |sc| (S::from_str(&sc.name[..]), Some(&sc.matches)))
    }

    /// Returns a string slice of the usage statement for the `App` (or `SubCommand`)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// let app_m = App::new("myprog")
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches();
    ///
    /// println!("{}", app_m.usage());
    /// ```
    pub fn usage(&self) -> &str {
        self.usage.as_ref().map_or("", |u| &u[..])
    }
}


// The following were taken and adapated from vec_map source
// repo: https://github.com/contain-rs/vec-map
// commit: be5e1fa3c26e351761b33010ddbdaf5f05dbcc33
// license: MIT - Copyright (c) 2015 The Rust Project Developers

#[derive(Clone)]
#[allow(missing_debug_implementations)]
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
            if let Some(elem) = self.iter.next() {
                if let Some(x) = elem.as_ref() {
                    self.front += 1;
                    return Some(x);
                }
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
            if let Some(elem) = self.iter.next_back() {
                if let Some(x) = elem.as_ref() {
                    self.back -= 1;
                    return Some(x);
                }
            }
            self.back -= 1;
        }
        None
    }
}

#[derive(Clone)]
#[allow(missing_debug_implementations)]
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

