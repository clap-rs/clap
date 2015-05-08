use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::vec::IntoIter;
use std::process;

use args::{ ArgMatches, Arg, SubCommand, MatchedArg};
use args::{ FlagBuilder, OptBuilder, PosBuilder};
use args::ArgGroup;

#[cfg(feature = "suggestions")]
use strsim;
#[cfg(feature = "color")]
use ansi_term::Colour::Red;

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
#[cfg(feature = "suggestions")]
fn did_you_mean<'a, T, I>(v: &str, possible_values: I) -> Option<&'a str>
    where       T: AsRef<str> + 'a,
                I: IntoIterator<Item=&'a T> {

    let mut candidate: Option<(f64, &str)> = None;
    for pv in possible_values.into_iter() {
        let confidence = strsim::jaro_winkler(v, pv.as_ref());
        if confidence > 0.8 && (candidate.is_none() ||
                               (candidate.as_ref().unwrap().0 < confidence)) {
            candidate = Some((confidence, pv.as_ref()));
        }
    }
    match candidate {
        None => None,
        Some((_, candidate)) => Some(candidate),
    }
}

#[cfg(not(feature = "suggestions"))]
fn did_you_mean<'a, T, I>(_: &str, _: I) -> Option<&'a str>
    where       T: AsRef<str> + 'a,
                I: IntoIterator<Item=&'a T> {
    None
}

/// A helper to determine message formatting
enum DidYouMeanMessageStyle {
    /// Suggested value is a long flag
    LongFlag,
    /// Suggested value is one of various possible values
    EnumValue,
}

/// Used to create a representation of a command line program and all possible command line
/// arguments for parsing at runtime.
///
/// Application settings are set using the "builder pattern" with `.get_matches()` being the
/// terminal method that starts the runtime-parsing process and returns information about
/// the user supplied arguments (or lack there of).
///
/// The options set for the application are not mandatory, and may appear in any order (so
/// long as `.get_matches()` is last).
///
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg};
/// let myprog = App::new("myprog")
///                   .author("Me, me@mail.com")
///                   .version("1.0.2")
///                   .about("Explains in brief what the program does")
///                   .arg(
///                            Arg::with_name("in_file").index(1)
///                        // Add other possible command line argument options here...
///                    )
///                   .get_matches();
///
/// // Your pogram logic starts here...
/// ```
pub struct App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    // The name displayed to the user when showing version and help/usage information
    name: String,
    name_slice: &'ar str,
    // A string of author(s) if desired. Displayed when showing help/usage information
    author: Option<&'a str>,
    // The version displayed to the user
    version: Option<&'v str>,
    // A brief explanation of the program that gets displayed to the user when shown help/usage
    // information
    about: Option<&'ab str>,
    // Additional help information
    more_help: Option<&'h str>,
    // A list of possible flags
    flags: BTreeMap<&'ar str, FlagBuilder<'ar>>,
    // A list of possible options
    opts: BTreeMap<&'ar str, OptBuilder<'ar>>,
    // A list of positional arguments
    positionals_idx: BTreeMap<u8, PosBuilder<'ar>>,
    positionals_name: HashMap<&'ar str, u8>,
    // A list of subcommands
    subcommands: BTreeMap<String, App<'a, 'v, 'ab, 'u, 'h, 'ar>>,
    needs_long_help: bool,
    needs_long_version: bool,
    needs_short_help: bool,
    needs_short_version: bool,
    needs_subcmd_help: bool,
    required: HashSet<&'ar str>,
    short_list: HashSet<char>,
    long_list: HashSet<&'ar str>,
    blacklist: HashSet<&'ar str>,
    usage_str: Option<&'u str>,
    bin_name: Option<String>,
    groups: HashMap<&'ar str, ArgGroup<'ar, 'ar>>
}

impl<'a, 'v, 'ab, 'u, 'h, 'ar> App<'a, 'v, 'ab, 'u, 'h, 'ar>{
    /// Creates a new instance of an application requiring a name (such as the binary). The name
    /// will be displayed to the user when they request to print version or help and usage
    /// information. The name should not contain spaces (hyphens '-' are ok).
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let prog = App::new("myprog")
    /// # .get_matches();
    /// ```
    pub fn new(n: &'ar str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        App {
            name: n.to_owned(),
            name_slice: n,
            author: None,
            about: None,
            more_help: None,
            version: None,
            flags: BTreeMap::new(),
            opts: BTreeMap::new(),
            positionals_idx: BTreeMap::new(),
            positionals_name: HashMap::new(),
            subcommands: BTreeMap::new(),
            needs_long_version: true,
            needs_long_help: true,
            needs_short_help: true,
            needs_subcmd_help: true,
            needs_short_version: true,
            required: HashSet::new(),
            short_list: HashSet::new(),
            long_list: HashSet::new(),
            usage_str: None,
            blacklist: HashSet::new(),
            bin_name: None,
            groups: HashMap::new(),
        }
    }

    /// Sets a string of author(s) and will be displayed to the user when they request the version
    /// or help information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .author("Kevin <kbknapp@gmail.com>")
    /// # .get_matches();
    /// ```
    pub fn author(mut self, a: &'a str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        self.author = Some(a);
        self
    }

    /// Sets a string briefly describing what the program does and will be displayed when
    /// displaying help information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .about("Does really amazing things to great people")
    /// # .get_matches();
    /// ```
    pub fn about(mut self, a: &'ab str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        self.about = Some(a);
        self
    }

    /// Adds additional help information to be displayed in addition to and directly after
    /// auto-generated help. This information is displayed **after** the auto-generated help
    /// information. This additional help is often used to describe how to use the arguments,
    /// or caveats to be noted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::App;
    /// # let app = App::new("myprog")
    /// .after_help("Does really amazing things to great people")
    /// # .get_matches();
    /// ```
    pub fn after_help(mut self, h: &'h str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        self.more_help = Some(h);
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .version("v0.1.24")
    /// # .get_matches();
    /// ```
    pub fn version(mut self, v: &'v str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar>  {
        self.version = Some(v);
        self
    }

    /// Sets a custom usage string to over-ride the auto-generated usage string. Will be
    /// displayed to the user when errors are found in argument parsing, or when you call
    /// `ArgMatches::usage()`
    ///
    /// *NOTE:* You do not need to specify the "USAGE: \n\t" portion, as that will
    /// still be applied by `clap`, you only need to specify the portion starting
    /// with the binary name.
    ///
    /// *NOTE:* This will not replace the entire help message, *only* the portion
    /// showing the usage.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .usage("myapp [-clDas] <some_file>")
    /// # .get_matches();
    /// ```
    pub fn usage(mut self, u: &'u str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        self.usage_str = Some(u);
        self
    }

    /// Adds an argument to the list of valid possibilties manually. This method allows you full
    /// control over the arguments settings and options (as well as dynamic generation). It also
    /// allows you specify several more advanced configuration options such as relational rules
    /// (exclusions and requirements).
    ///
    /// The only disadvantage to this method is that it's more verbose, and arguments must be added
    /// one at a time. Using `Arg::from_usage` helps with the verbosity, and still allows full
    /// control over the advanced configuration options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// // Adding a single "flag" argument with a short and help text, using Arg::with_name()
    /// .arg(Arg::with_name("debug")
    ///                .short("d")
    ///                .help("turns on debugging mode"))
    /// // Adding a single "option" argument with a short, a long, and help text using the less
    /// // verbose Arg::from_usage()
    /// .arg(Arg::from_usage("-c --config=[CONFIG] 'Optionally sets a configuration file to use'"))
    /// # .get_matches();
    /// ```
    pub fn arg(mut self, a: Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        if self.flags.contains_key(a.name) ||
           self.opts.contains_key(a.name) ||
           self.positionals_name.contains_key(a.name) {
            panic!("Argument name must be unique\n\n\t\"{}\" is already in use", a.name);
        }
        if let Some(grp) = a.group {
            let ag = self.groups.entry(grp).or_insert(ArgGroup::with_name(grp));
            ag.args.insert(a.name);
            // Leaving this commented out for now...I'm not sure if having a required argument in
            // a in required group is bad...It has it's uses
            // assert!(!a.required,
            //     format!("Arguments may not be required AND part of a required group\n\n\t{} is \
            //         required and also part of the {} group\n\n\tEither remove the requirement \
            //         from the group, or the argument.", a.name, grp));
        }
        if let Some(s) = a.short {
            if self.short_list.contains(&s) {
                panic!("Argument short must be unique\n\n\t-{} is already in use", s);
            } else {
                self.short_list.insert(s);
            }
            if s == 'h' {
                self.needs_short_help = false;
            } else if s == 'v' {
                self.needs_short_version = false;
            }
        }
        if let Some(l) = a.long {
            if self.long_list.contains(l) {
                panic!("Argument long must be unique\n\n\t--{} is already in use", l);
            } else {
                self.long_list.insert(l);
            }
            if l == "help" {
                self.needs_long_help = false;
            } else if l == "version" {
                self.needs_long_version = false;
            }
        }
        if a.required {
            self.required.insert(a.name);
        }
        if a.index.is_some() || (a.short.is_none() && a.long.is_none()) {
            let i = if a.index.is_none() {
                (self.positionals_idx.len() + 1) as u8
            } else {
                a.index.unwrap()
            };

            if a.short.is_some() || a.long.is_some() {
                panic!("Argument \"{}\" has conflicting requirements, both index() and short(), \
                    or long(), were supplied", a.name);
            }

            if self.positionals_idx.contains_key(&i) {
                panic!("Argument \"{}\" has the same index as another positional \
                    argument\n\n\tPerhaps try .multiple(true) to allow one positional argument \
                    to take multiple values", a.name);
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

            self.positionals_name.insert(a.name, i);
            // Create the Positional Arguemnt Builder with each HashSet = None to only allocate
            // those that require it
            let mut pb = PosBuilder {
                name: a.name,
                index: i,
                required: a.required,
                multiple: a.multiple,
                blacklist: None,
                requires: None,
                possible_vals: None,
                num_vals: a.num_vals,
                min_vals: a.min_vals,
                max_vals: a.max_vals,
                help: a.help,
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
                let mut bhs = HashSet::new();
                // without derefing n = &&str
                for n in bl { bhs.insert(*n); }
                pb.blacklist = Some(bhs);
            }
            // Check if there is anything in the requires list and add any values
            if let Some(ref r) = a.requires {
                let mut rhs = HashSet::new();
                // without derefing n = &&str
                for n in r {
                    rhs.insert(*n);
                    if pb.required {
                        self.required.insert(*n);
                    }
                }
                pb.requires = Some(rhs);
            }
            // Check if there is anything in the possible values and add those as well
            if let Some(ref p) = a.possible_vals {
                let mut phs = BTreeSet::new();
                // without derefing n = &&str
                for n in p { phs.insert(*n); }
                pb.possible_vals = Some(phs);
            }
            self.positionals_idx.insert(i, pb);
        } else if a.takes_value {
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
                possible_vals: None,
                num_vals: a.num_vals,
                min_vals: a.min_vals,
                max_vals: a.max_vals,
                val_names: a.val_names,
                requires: None,
                required: a.required,
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
                let mut bhs = HashSet::new();
                // without derefing n = &&str
                for n in bl { bhs.insert(*n); }
                ob.blacklist = Some(bhs);
            }
            // Check if there is anything in the requires list and add any values
            if let Some(ref r) = a.requires {
                let mut rhs = HashSet::new();
                // without derefing n = &&str
                for n in r {
                    rhs.insert(*n);
                    if ob.required {
                        self.required.insert(*n);
                    }
                }
                ob.requires = Some(rhs);
            }
            // Check if there is anything in the possible values and add those as well
            if let Some(ref p) = a.possible_vals {
                let mut phs = BTreeSet::new();
                // without derefing n = &&str
                for n in p { phs.insert(*n); }
                ob.possible_vals = Some(phs);
            }
            self.opts.insert(a.name, ob);
        } else {
            if a.short.is_none() && a.long.is_none() {
                // Could be a posistional constructed from usage string

            }
            if a.required {
                panic!("Argument \"{}\" cannot be required(true) because it has no index() or \
                    takes_value(true)", a.name);
            }
            if a.possible_vals.is_some() {
                panic!("Argument \"{}\" cannot have a specific value set because it doesn't have \
                    takes_value(true) set", a.name);
            }
            // No need to check for index() or takes_value() as that is handled above

            let mut fb = FlagBuilder {
                name: a.name,
                short: a.short,
                long: a.long,
                help: a.help,
                blacklist: None,
                multiple: a.multiple,
                requires: None,
            };
            // Check if there is anything in the blacklist (mutually excludes list) and add any
            // values
            if let Some(ref bl) = a.blacklist {
                let mut bhs = HashSet::new();
                // without derefing n = &&str
                for n in bl { bhs.insert(*n); }
                fb.blacklist = Some(bhs);
            }
            // Check if there is anything in the requires list and add any values
            if let Some(ref r) = a.requires {
                let mut rhs = HashSet::new();
                // without derefing n = &&str
                for n in r { rhs.insert(*n); }
                fb.requires = Some(rhs);
            }
            self.flags.insert(a.name, fb);
        }
        self
    }

    /// Adds multiple arguments to the list of valid possibilties by iterating over a Vec of Args
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .args( vec![Arg::from_usage("[debug] -d 'turns on debugging info"),
    ///             Arg::with_name("input").index(1).help("the input file to use")])
    /// # .get_matches();
    /// ```
    pub fn args(mut self, args: Vec<Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>>)
                -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        for arg in args.into_iter() {
            self = self.arg(arg);
        }
        self
    }

    /// A convienience method for adding a single basic argument (one without advanced
    /// relational rules) from a usage type string. The string used follows the same rules and
    /// syntax as `Arg::from_usage()`
    ///
    /// The downside to using this method is that you can not set any additional properties of the
    /// `Arg` other than what `Arg::from_usage()` supports.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .arg_from_usage("-c --conf=<config> 'Sets a configuration file to use'")
    /// # .get_matches();
    /// ```
    pub fn arg_from_usage(mut self, usage: &'ar str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        self = self.arg(Arg::from_usage(usage));
        self
    }

    /// Adds multiple arguments at once from a usage string, one per line. See `Arg::from_usage()`
    /// for details on the syntax and rules supported.
    ///
    /// Like `App::arg_from_usage()` the downside is you only set properties for the `Arg`s which
    /// `Arg::from_usage()` supports. But here the benefit is pretty strong, as the readability is
    /// greatly enhanced, especially if you don't need any of the more advanced configuration
    /// options.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .args_from_usage(
    ///    "-c --conf=[config] 'Sets a configuration file to use'
    ///    [debug]... -d 'Sets the debugging level'
    ///    <input> 'The input file to use'")
    /// # .get_matches();
    /// ```
    pub fn args_from_usage(mut self, usage: &'ar str) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        for l in usage.lines() {
            self = self.arg(Arg::from_usage(l.trim()));
        }
        self
    }

    /// Adds an ArgGroup to the application. ArgGroups are a family of related arguments. By
    /// placing them in a logical group, you make easier requirement and exclusion rules. For
    /// instance, you can make an ArgGroup required, this means that one (and *only* one) argument
    /// from that group must be present. Using more than one argument from an ArgGroup causes a
    /// failure (graceful exit).
    ///
    /// You can also do things such as name an ArgGroup as a confliction, meaning any of the
    /// arguments that belong to that group will cause a failure if present.
    ///
    /// Perhaps the most common use of ArgGroups is to require one and *only* one argument to be
    /// present out of a given set. For example, lets say that you were building an application
    /// where one could set a given version number by supplying a string using an option argument,
    /// such as `--set-ver v1.2.3`, you also wanted to support automatically using a previous
    /// version numer and simply incrementing one of the three numbers, so you create three flags
    /// `--major`, `--minor`, and `--patch`. All of these arguments shouldn't be used at one time
    /// but perhaps you want to specify that *at least one* of them is used. You can create a
    /// group
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let _ = App::new("app")
    /// .args_from_usage("--set-ver [ver] 'set the version manually'
    ///                   --major         'auto increase major'
    ///                   --minor         'auto increase minor'
    ///                   --patch         'auto increase patch")
    /// .arg_group(ArgGroup::with_name("vers")
    ///                     .add_all(vec!["ver", "major", "minor","patch"])
    ///                     .required(true))
    /// # .get_matches();
    pub fn arg_group(mut self, group: ArgGroup<'ar, 'ar>) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        if group.required {
            self.required.insert(group.name);
            if let Some(ref reqs) = group.requires {
                for r in reqs {
                    self.required.insert(r);
                }
            }
            if let Some(ref bl) = group.conflicts {
                for b in bl {
                    self.blacklist.insert(b);
                }
            }
        }
        let mut found = false;
        if let Some(ref mut grp) = self.groups.get_mut(group.name) {
            for a in group.args.iter() {
                grp.args.insert(a);
            }
            grp.requires = group.requires.clone();
            grp.conflicts = group.conflicts.clone();
            grp.required = group.required;
            found = true;
        }
        if !found {
            self.groups.insert(group.name, group);
        }
        self
    }

    /// Adds a ArgGroups to the application. ArgGroups are a family of related arguments. By
    /// placing them in a logical group, you make easier requirement and exclusion rules. For
    /// instance, you can make an ArgGroup required, this means that one (and *only* one) argument
    /// from that group must be present. Using more than one argument from an ArgGroup causes a
    /// failure (graceful exit).
    ///
    /// You can also do things such as name an ArgGroup as a confliction, meaning any of the
    /// arguments that belong to that group will cause a failure if present.
    ///
    /// Perhaps the most common use of ArgGroups is to require one and *only* one argument to be
    /// present out of a given set. For example, lets say that you were building an application
    /// where one could set a given version number by supplying a string using an option argument,
    /// such as `--set-ver v1.2.3`, you also wanted to support automatically using a previous
    /// version numer and simply incrementing one of the three numbers, so you create three flags
    /// `--major`, `--minor`, and `--patch`. All of these arguments shouldn't be used at one time
    /// but perhaps you want to specify that *at least one* of them is used. You can create a
    /// group
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # let _ = App::new("app")
    /// .args_from_usage("--set-ver [ver] 'set the version manually'
    ///                   --major         'auto increase major'
    ///                   --minor         'auto increase minor'
    ///                   --patch         'auto increase patch")
    /// .arg_group(ArgGroup::with_name("vers")
    ///                     .add_all(vec!["ver", "major", "minor","patch"])
    ///                     .required(true))
    /// # .get_matches();
    pub fn arg_groups(mut self, groups: Vec<ArgGroup<'ar, 'ar>>) -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        for g in groups {
            self = self.arg_group(g);
        }
        self
    }

    /// Adds a subcommand to the list of valid possibilties. Subcommands are effectively sub apps,
    /// because they can contain their own arguments, subcommands, version, usage, etc. They also
    /// function just like apps, in that they get their own auto generated help, version, and
    /// usage.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app = App::new("myprog")
    /// .subcommand(SubCommand::new("config")
    ///                .about("Controls configuration features")
    ///                .arg_from_usage("<config> 'Required configuration file to use'"))
    ///             // Additional subcommand configuration goes here, such as other arguments...
    /// # .get_matches();
    /// ```
    pub fn subcommand(mut self, subcmd: App<'a, 'v, 'ab, 'u, 'h, 'ar>)
                      -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        if subcmd.name == "help" { self.needs_subcmd_help = false; }
        self.subcommands.insert(subcmd.name.clone(), subcmd);
        self
    }

    /// Adds multiple subcommands to the list of valid possibilties by iterating over a Vec of
    /// `SubCommand`s
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app = App::new("myprog")
    /// .subcommands( vec![
    ///        SubCommand::new("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::with_name("config_file").index(1)),
    ///        SubCommand::new("debug").about("Controls debug functionality")])
    /// # .get_matches();
    /// ```
    pub fn subcommands(mut self, subcmds: Vec<App<'a, 'v, 'ab, 'u, 'h, 'ar>>)
                       -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
        for subcmd in subcmds.into_iter() {
            self = self.subcommand(subcmd);
        }
        self
    }

    fn get_group_members(&self, group: &str) -> Vec<String> {
        let mut g_vec = HashSet::new();
        let mut args = HashSet::new();

        for n in self.groups.get(group).unwrap().args.iter() {
            if let Some(ref f) = self.flags.get(n) {
                args.insert(format!("{}", f));
            } else if let Some(ref f) = self.opts.get(n) {
                args.insert(format!("{}", f));
            } else if self.groups.contains_key(n) {
                g_vec.insert(*n);
            } else {
                if let Some(idx) = self.positionals_name.get(n) {
                    if let Some(ref p) = self.positionals_idx.get(&idx) {
                        args.insert(format!("{}", p));
                    }
                }
            }
        }

        if g_vec.is_empty() {
            return args.iter().map(|s| s.to_owned()).collect()
        }
        return g_vec.iter().map(|g| self.get_group_members(g)).fold(vec![], |acc, v| acc + &v)
    }

    fn get_group_members_names(&self, group: &'ar str) -> Vec<&'ar str> {
        let mut g_vec = HashSet::new();
        let mut args = HashSet::new();

        for n in self.groups.get(group).unwrap().args.iter() {
            if self.flags.contains_key(n) {
                args.insert(*n);
            } else if self.opts.contains_key(n) {
                args.insert(*n);
            } else if self.groups.contains_key(n) {
                g_vec.insert(*n);
            } else {
                if self.positionals_name.contains_key(n) {
                    args.insert(*n);
                }
            }
        }

        if g_vec.is_empty() {
            return args.iter().map(|s| *s).collect()
        }
        return g_vec.iter()
                    .map(|g| self.get_group_members_names(g))
                    .fold(vec![], |acc, v| acc + &v)
    }

    fn get_required_from(&self, reqs: HashSet<&'ar str>) -> VecDeque<String> {
        let mut c_flags = HashSet::new();
        let mut c_pos = HashSet::new();
        let mut c_opt = HashSet::new();
        let mut grps = HashSet::new();
        for name in &reqs {
            if self.flags.contains_key(*name) {
                c_flags.insert(*name);
            } else if self.opts.contains_key(*name) {
                c_opt.insert(*name);
            } else if self.groups.contains_key(*name) {
                grps.insert(*name);
            } else {
                c_pos.insert(*name);
            }
        }
        let mut tmp_f = vec![];
        for f in &c_flags {
            if let Some(ref f) = self.flags.get(f) {
                if let Some(ref rl) = f.requires {
                    for r in rl {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                tmp_f.push(r);
                            } else if self.opts.contains_key(r) {
                                c_opt.insert(r);
                            } else if self.groups.contains_key(r) {
                                grps.insert(*r);
                            } else {
                                c_pos.insert(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_f {
            c_flags.insert(f);
        }
        let mut tmp_o = vec![];
        for f in &c_opt {
            if let Some(ref f) = self.opts.get(f) {
                if let Some(ref rl) = f.requires {
                    for r in rl {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                c_flags.insert(r);
                            } else if self.opts.contains_key(r) {
                                tmp_o.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.insert(*r);
                            } else {
                                c_pos.insert(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_o {
            c_opt.insert(f);
        }
        let mut tmp_p = vec![];
        for f in &c_pos {
            if let Some(ref f) = self.flags.get(f) {
                if let Some(ref rl) = f.requires {
                    for r in rl {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                c_flags.insert(r);
                            } else if self.opts.contains_key(r) {
                                c_opt.insert(r);
                            } else if self.groups.contains_key(r) {
                                grps.insert(*r);
                            } else {
                                tmp_p.push(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_p {
            c_flags.insert(f);
        }


        let mut ret_val = VecDeque::new();

        let mut pmap = BTreeMap::new();
        for p in &c_pos {
            if let Some(idx) = self.positionals_name.get(p) {
                if let Some(ref p) = self.positionals_idx.get(&idx) {
                    pmap.insert(p.index, format!("{}", p));
                }
            }
        }
        pmap.into_iter().map(|(_, s)| ret_val.push_back(s)).collect::<Vec<_>>();
        for f in &c_flags {
             ret_val.push_back(format!("{}", self.flags.get(*f).unwrap()));
        }
        for o in &c_opt {
             ret_val.push_back(format!("{}", self.opts.get(*o).unwrap()));
        }
        for g in grps {
            let g_string = self.get_group_members(g).iter()
                                                    .fold(String::new(), |acc, s| {
                                                        acc + &format!(" {} |",s)[..]
                                                    });
            ret_val.push_back(format!("[{}]", &g_string[..g_string.len()-1]));
        }

        ret_val
    }

    // Creates a usage string if one was not provided by the user manually. This happens just
    // after all arguments were parsed, but before any subcommands have been parsed (so as to
    // give subcommands their own usage recursively)
    fn create_usage(&self, matches: Option<Vec<&'ar str>>) -> String {
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n");
        usage.push_str("\t");
        if let Some(u) = self.usage_str {
            usage.push_str(u);
        } else if let Some(tmp_vec) = matches {
            let mut hs = self.required.iter().map(|n| *n).collect::<HashSet<_>>();
            tmp_vec.iter().map(|n| hs.insert(*n)).collect::<Vec<_>>();
            let reqs = self.get_required_from(hs);

            let r_string = reqs.iter().fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

            usage.push_str(&format!("{}{}",
                self.bin_name.clone().unwrap_or(self.name.clone()),
                r_string)[..]);
        } else {
            usage.push_str(&self.bin_name.clone().unwrap_or(self.name.clone())[..]);

            let mut reqs = self.required.iter().map(|n| *n).collect::<HashSet<_>>();
            // If it's required we also need to ensure all previous positionals are required too
            let mut found = false;
            for p in self.positionals_idx.values().rev() {
                if found {
                    reqs.insert(p.name);
                    continue;
                }
                if p.required {
                    found = true;
                    reqs.insert(p.name);
                }
            }
            let req_strings = self.get_required_from(reqs);
            let req_string = req_strings.iter()
                                        .fold(String::new(), |acc, s| {
                                            acc + &format!(" {}", s)[..]
                                        });
            usage.push_str(&req_string[..]);


            if !self.positionals_idx.is_empty() && self.positionals_idx.values()
                                                                       .any(|a| !a.required) {
                usage.push_str(" [POSITIONAL]");
            }
            if !self.flags.is_empty() {
                usage.push_str(" [FLAGS]");
            }
            if !self.opts.is_empty() && self.opts.values().any(|a| !a.required) {
                usage.push_str(" [OPTIONS]");
            }
            if !self.subcommands.is_empty() {
                usage.push_str(" [SUBCOMMANDS]");
            }
        }

        usage.shrink_to_fit();
        usage
    }

    // Prints the usage statement to the user
    fn print_usage(&self, more_info: bool, matches: Option<Vec<&str>>) {
        print!("{}",self.create_usage(matches));
        if more_info {
            println!("\nFor more information try --help");
        }
    }

    // Prints the full help message to the user
    fn print_help(&self) {
        self.print_version(false);
        let flags = !self.flags.is_empty();
        let pos = !self.positionals_idx.is_empty();
        let opts = !self.opts.is_empty();
        let subcmds = !self.subcommands.is_empty();

        let mut longest_flag = 0;
        for fl in self.flags
            .values()
            .filter(|ref f| f.long.is_some())
            // 2='--'
            .map(|ref a| a.to_string().len() ) {
            if fl > longest_flag { longest_flag = fl; }
        }
        let mut longest_opt= 0;
        for ol in self.opts
            .values()
            .filter(|ref o| o.long.is_some())
            .map(|ref a|
                a.to_string().len() + if a.short.is_some() { 4 } else { 0 }
            ) {
            if ol > longest_opt {
                longest_opt = ol;
            }
        }
        if longest_opt == 0 {
            for ol in self.opts
                .values()
                .filter(|ref o| o.short.is_some())
                // 3='...'
                // 4='- <>'
                .map(|ref a| a.to_string().len() + if a.long.is_some() { 4 } else { 0 }) {
                if ol > longest_opt {longest_opt = ol;}
            }
        }
        let mut longest_pos = 0;
        for pl in self.positionals_idx
            .values()
            .map(|ref f| f.to_string().len() ) {
            if pl > longest_pos {longest_pos = pl;}
        }
        let mut longest_sc = 0;
        for scl in self.subcommands
            .values()
            .map(|ref f| f.name.len() ) {
            if scl > longest_sc {longest_sc = scl;}
        }

        if let Some(author) = self.author {
            println!("{}", author);
        }
        if let Some(about) = self.about {
            println!("{}", about);
        }
        println!("");
        self.print_usage(false, None);
        if flags || opts || pos || subcmds {
            println!("");
        }

        let tab = "    ";
        if flags {
            println!("");
            println!("FLAGS:");
            for v in self.flags.values() {
                println!("{}{}{}{}",tab,
                        if let Some(s) = v.short{format!("-{}",s)}else{tab.to_owned()},
                        if let Some(l) = v.long {
                            format!("{}--{}{}",
                                if v.short.is_some() { ", " } else {""},
                                l,
                                self.get_spaces((longest_flag + 4) - (v.long.unwrap().len() + 2)))
                        } else {
                            // 6 is tab (4) + -- (2)
                            self.get_spaces(longest_flag + 6).to_owned()
                        },
                        v.help.unwrap_or(tab) );
            }
        }
        if opts {
            println!("");
            println!("OPTIONS:");
            for v in self.opts.values() {
                // if it supports multiple we add '...' i.e. 3 to the name length
                println!("{}{}{}{}{}{}",tab,
                        if let Some(s) = v.short{format!("-{}",s)}else{tab.to_owned()},
                        if let Some(l) = v.long {
                            format!("{}--{}",
                                if v.short.is_some() {", "} else {""},l)
                        } else {
                            "".to_owned()
                        },
                        format!("{}",
                            if let Some(ref vec) = v.val_names {
                                vec.iter().fold(String::new(), |acc, s| {
                                    acc + &format!(" <{}>", s)[..]
                                })
                            } else if let Some(num) = v.num_vals {
                                (0..num).fold(String::new(), |acc, _| {
                                    acc + &format!(" <{}>", v.name)[..]
                                })
                            } else {
                                format!(" <{}>{}", v.name, if v.multiple{"..."} else {""})
                            }),
                            if v.long.is_some() {
                                self.get_spaces(
                                    (longest_opt + 4) - (v.to_string().len())
                                )
                            } else {
                                // 8 = tab + '-a, '.len()
                                self.get_spaces((longest_opt + 9) - (v.to_string().len()))
                            },
                        get_help!(v) );
            }
        }
        if pos {
            println!("");
            println!("POSITIONAL ARGUMENTS:");
            for v in self.positionals_idx.values() {
                let mult = if v.multiple { 3 } else { 0 };
                println!("{}{}{}{}",tab,
                    if v.multiple {format!("{}...",v.name)} else {v.name.to_owned()},
                    self.get_spaces((longest_pos + 4) - (v.name.len() + mult)),
                    get_help!(v));
            }
        }
        if subcmds {
            println!("");
            println!("SUBCOMMANDS:");
            for sc in self.subcommands.values() {
                println!("{}{}{}{}",tab,
                 sc.name,
                 self.get_spaces((longest_sc + 4) - (sc.name.len())),
                 if let Some(a) = sc.about {a} else {tab} );
            }
        }

        if let Some(h) = self.more_help {
            println!("");
            println!("{}", h);
        }

        self.exit(0);
    }

    // Used when spacing arguments and their help message when displaying help information
    fn get_spaces(&self, num: usize) -> &'static str {
        match num {
            0 => "",
            1 => " ",
            2 => "  ",
            3 => "   ",
            4 => "    ",
            5 => "     ",
            6 => "      ",
            7 => "       ",
            8 => "        ",
            9 => "         ",
            10=> "          ",
            11=> "           ",
            12=> "            ",
            13=> "             ",
            14=> "              ",
            15=> "               ",
            16=> "                ",
            17=> "                 ",
            18=> "                  ",
            19=> "                   ",
            20=> "                    ",
            21=> "                     ",
            22=> "                      ",
            23=> "                       ",
            24=> "                        ",
            25=> "                         ",
            26=> "                          ",
            27=> "                           ",
            28=> "                            ",
            29=> "                             ",
            30|_=> "                             "
        }
    }

    // Prints the version to the user and exits if quit=true
    fn print_version(&self, quit: bool) {
        // Print the binary name if existing, but replace all spaces with hyphens in case we're
        // dealing with subcommands i.e. git mv is translated to git-mv
        println!("{} {}", &self.bin_name.clone().unwrap_or(
            self.name.clone())[..].replace(" ", "-"),
            self.version.unwrap_or("")
        );
        if quit { self.exit(0); }
    }

    // Exits with a status code passed to the OS
    // This is legacy from before std::process::exit() and may be removed evenutally
    fn exit(&self, status: i32) {
        process::exit(status);
    }

    // Reports and error to the users screen along with an optional usage statement and quits
    #[cfg(not(feature = "color"))]
    fn report_error(&self, msg: String, usage: bool, quit: bool, matches: Option<Vec<&str>>) {
        println!("{}\n", msg);
        if usage { self.print_usage(true, matches); }
        if quit { self.exit(1); }
    }

    #[cfg(feature = "color")]
    fn report_error(&self, msg: String, usage: bool, quit: bool, matches: Option<Vec<&str>>) {
        println!("{}\n", Red.paint(&msg[..]));
        if usage {
            print!("{}",&self.create_usage(matches)[..]);
            println!("{}","\n\nFor more information try --help");
        }
        if quit { self.exit(1); }
    }

    // Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    // the real parsing function for subcommands
    pub fn get_matches(mut self) -> ArgMatches<'ar, 'ar> {
        self.verify_positionals();
        for (_,sc) in self.subcommands.iter_mut() {
            sc.verify_positionals();
        }

        let mut matches = ArgMatches::new();

        let args: Vec<_> = env::args().collect();
        let mut it = args.into_iter();
        if let Some(name) = it.next() {
            let p = Path::new(&name[..]);
            if let Some(f) = p.file_name() {
                if let Ok(s) = f.to_os_string().into_string() {
                    self.bin_name = Some(s);
                }
            }
        }
        self.get_matches_from(&mut matches, &mut it );

        matches
    }

    fn verify_positionals(&mut self) {
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // Firt we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)
        //
        // Next we verify that only the highest index has a .multiple(true) (if any)
        if let Some((idx, ref p)) = self.positionals_idx.iter().rev().next() {
            if *idx as usize != self.positionals_idx.len() {
                panic!("Found positional argument \"{}\" who's index is {} but there are only {} \
                    positional arguments defined", p.name, idx, self.positionals_idx.len());
            }
        }
        if let Some(ref p) = self.positionals_idx.values()
                                                 .filter(|ref a| a.multiple)
                                                 .filter(|ref a| {
                                                    a.index as usize != self.positionals_idx.len()
                                                })
                                                 .next() {
            panic!("Found positional argument \"{}\" which accepts multiple values but it's not \
                the last positional argument (i.e. others have a higher index)", p.name);
        }

        // If it's required we also need to ensure all previous positionals are required too
        let mut found = false;
        for (_, p) in self.positionals_idx.iter_mut().rev() {
            if found {
                p.required = true;
                self.required.insert(p.name);
                continue;
            }
            if p.required {
                found = true;
            }
        }
    }

    /// Returns a suffix that can be empty, or is the standard 'did you mean phrase
    fn did_you_mean_suffix<'z, T, I>(arg: &str, values: I, style: DidYouMeanMessageStyle)
                                                     -> (String, Option<&'z str>)
                                                        where       T: AsRef<str> + 'z,
                                                                    I: IntoIterator<Item=&'z T> {
        match did_you_mean(arg, values) {
                Some(candidate) => {
                    let mut suffix = "\n\tDid you mean ".to_string();
                    match style {
                        DidYouMeanMessageStyle::LongFlag => suffix.push_str("--"),
                        DidYouMeanMessageStyle::EnumValue => suffix.push('\''),
                    }
                    suffix.push_str(candidate);
                    if let DidYouMeanMessageStyle::EnumValue = style {
                        suffix.push('\'');
                    }
                    suffix.push_str(" ?");
                    (suffix, Some(candidate))
                },
                None => (String::new(), None),
        }
    }

    fn possible_values_error(&self, arg: &str, opt: &str, p_vals: &BTreeSet<&str>,
                                                   matches: &ArgMatches<'ar, 'ar>) {
        let suffix = App::did_you_mean_suffix(arg, p_vals.iter(),
                                              DidYouMeanMessageStyle::EnumValue);

        self.report_error(format!("\"{}\" isn't a valid value for '{}'{}{}",
                                    arg,
                                    opt,
                                    format!("\n\t[valid values:{}]",
                                        p_vals.iter()
                                              .fold(String::new(), |acc, name| {
                                                  acc + &format!(" {}",name)[..]
                                              })),
                                    suffix.0),
                                        true,
                                        true,
                                        Some(matches.args.keys().map(|k| *k).collect()));
    }

    fn get_matches_from(&mut self, matches: &mut ArgMatches<'ar, 'ar>, it: &mut IntoIter<String>) {
        self.create_help_and_version();

        let mut pos_only = false;
        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: Option<&str> = None;
        let mut pos_counter = 1;
        while let Some(arg) = it.next() {
            let arg_slice = &arg[..];
            let mut skip = false;
            if !pos_only && !arg_slice.starts_with("-") && !self.subcommands.contains_key(arg_slice) {
                if let Some(nvo) = needs_val_of {
                    if let Some(ref opt) = self.opts.get(nvo) {
                        if let Some(ref p_vals) = opt.possible_vals {
                            if !p_vals.is_empty() {
                                if !p_vals.contains(arg_slice) {
                                    self.possible_values_error(arg_slice, &opt.to_string(),
                                                                          p_vals, matches);
                                }
                            }
                        }
                        if let Some(num) = opt.num_vals {
                            if let Some(ref ma) = matches.args.get(opt.name) {
                                if let Some(ref vals) = ma.values {
                                    if num == vals.len() as u8 && !opt.multiple {
                                        self.report_error(format!("The argument \"{}\" was found, \
                                            but '{}' only expects {} values",
                                                arg,
                                                opt,
                                                vals.len()),
                                            true,
                                            true,
                                            Some(
                                                matches.args.keys().map(|k| *k).collect()
                                            )
                                        );
                                    }
                                }
                            }
                        }
                        if let Some(ref mut o) = matches.args.get_mut(opt.name) {
                            // Options have values, so we can unwrap()
                            if let Some(ref mut vals) = o.values {
                                let len = vals.len() as u8 + 1;
                                vals.insert(len, arg.clone());
                            }

                            // if it's multiple the occurrences are increased when originall found
                            o.occurrences = if opt.multiple {
                                o.occurrences + 1
                            } else {
                                skip = true;
                                1
                            };
                            if let Some(ref mut vals) = o.values {
                                let len = vals.len() as u8;
                                if let Some(num) = opt.max_vals {
                                    if len != num { continue }
                                } else if let Some(num) = opt.num_vals {
                                    if len != num { continue }
                                } else if !skip {
                                    continue
                                }
                            }
                        }
                        skip = true;
                    }
                }
            }
            if skip {
                needs_val_of = None;
                continue;
            } else if let Some(ref name) = needs_val_of {
                if let Some(ref o) = self.opts.get(name) {
                    if !o.multiple {
                        self.report_error(
                            format!("The argument '{}' requires a value but none was supplied", o),
                            true,
                            true,
                            Some(matches.args.keys().map(|k| *k).collect() ) );
                    }
                }
            }

            if arg_slice.starts_with("--") && !pos_only {
                if arg_slice.len() == 2 {
                    pos_only = true;
                    continue;
                }
                // Single flag, or option long version
                needs_val_of = self.parse_long_arg(matches, &arg);
            } else if arg_slice.starts_with("-") && arg_slice.len() != 1 && ! pos_only {
                needs_val_of = self.parse_short_arg(matches, &arg);
            } else {
                // Positional or Subcommand
                // If the user pased `--` we don't check for subcommands, because the argument they
                // may be trying to pass might match a subcommand name
                if !pos_only {
                    if self.subcommands.contains_key(&arg) {
                        if arg_slice == "help" {
                            self.print_help();
                        }
                        subcmd_name = Some(arg.clone());
                        break;
                    }

                    if let Some(candidate_subcommand) = did_you_mean(&arg,
                                                                    self.subcommands.keys()) {
                        self.report_error(
                            format!("The subcommand '{}' isn't valid\n\tDid you mean '{}' ?\n\n\
                            If you received this message in error, try \
                            re-running with '{} -- {}'",
                                arg,
                                candidate_subcommand,
                                self.bin_name.clone().unwrap_or(self.name.clone()),
                                arg),
                            true,
                            true,
                            None);
                    }
                }

                if self.positionals_idx.is_empty() {
                    self.report_error(
                        format!("Found argument \"{}\", but {} wasn't expecting any",
                            arg,
                            self.bin_name.clone().unwrap_or(self.name.clone())),
                        true,
                        true,
                        Some(matches.args.keys().map(|k| *k).collect()));
                }
                // If we find that an argument requires a positiona, we need to update all the
                // previous positionals too. This will denote where to start
                // let mut req_pos_from_name = None;
                if let Some(p) = self.positionals_idx.get(&pos_counter) {


                    if self.blacklist.contains(p.name) {
                        matches.args.remove(p.name);
                        self.report_error(format!("The argument '{}' cannot be used with {}",
                            p,
                            match self.blacklisted_from(p.name, &matches) {
                                Some(name) => format!("'{}'", name),
                                None       => "one or more of the other specified \
                                               arguments".to_owned()
                            }),
                            true,
                            true,
                            Some(matches.args.keys().map(|k| *k).collect()));
                    }

                    if let Some(ref p_vals) = p.possible_vals {
                        if !p_vals.is_empty() {
                            if !p_vals.contains(arg_slice) {
                                self.possible_values_error(arg_slice, &p.to_string(),
                                                                       p_vals, matches);
                            }
                        }
                    }
                    // Have we made the update yet?
                    let mut done = false;
                    if p.multiple {
                        if let Some(num) = p.num_vals {
                            if let Some(ref ma) = matches.args.get(p.name) {
                                if let Some(ref vals) = ma.values {
                                    if vals.len() as u8 == num {
                                        self.report_error(format!("The argument \"{}\" was found, \
                                            but '{}' wasn't expecting any more values", arg, p),
                                            true,
                                            true,
                                            Some(matches.args.keys()
                                                             .map(|k| *k).collect()));
                                    }
                                }
                            }
                        }
                        // Check if it's already existing and update if so...
                        if let Some(ref mut pos) = matches.args.get_mut(p.name) {
                            done = true;
                            pos.occurrences += 1;
                            if let Some(ref mut vals) = pos.values {
                                let len = (vals.len() + 1) as u8;
                                vals.insert(len, arg.clone());
                            }
                        }
                    } else {
                        // Only increment the positional counter if it doesn't allow multiples
                        pos_counter += 1;
                    }
                    // Was an update made, or is this the first occurrence?
                    if !done {
                        let mut bm = BTreeMap::new();
                        bm.insert(1, arg.clone());
                        matches.args.insert(p.name, MatchedArg{
                            occurrences: 1,
                            values: Some(bm),
                        });
                    }

                    if let Some(ref bl) = p.blacklist {
                        for name in bl {
                            self.blacklist.insert(name);
                            self.required.remove(name);
                        }
                    }

                    self.required.remove(p.name);
                    if let Some(ref reqs) = p.requires {
                        // Add all required args which aren't already found in matches to the
                        // final required list
                        for n in reqs {
                            if matches.args.contains_key(n) {continue;}

                            self.required.insert(n);
                        }
                    }

                    parse_group_reqs!(self, p);

                } else {
                    self.report_error(format!("The argument \"{}\" was found, but '{}' wasn't \
                        expecting any", arg,
                            self.bin_name.clone().unwrap_or(self.name.clone())),
                        true,
                        true,
                        Some(matches.args.keys().map(|k| *k).collect()));
                }
            }
        }
        match needs_val_of {
            Some(ref a) => {
                if let Some(o) = self.opts.get(a) {
                    if o.multiple && self.required.is_empty() {
                        let should_err = match matches.values_of(o.name) {
                            Some(ref v) => if v.len() == 0 { true } else { false },
                            None        => true,
                        };
                        if should_err {
                            self.report_error(
                                format!("The argument '{}' requires a value but none was \
                                supplied", o),
                                true,
                                true,
                                Some(matches.args.keys().map(|k| *k).collect() ) );
                        }
                    }
                    else if !o.multiple {
                        self.report_error(
                            format!("The argument '{}' requires a value but none was supplied", o),
                            true,
                            true,
                            Some(matches.args.keys().map(|k| *k).collect() ) );
                    }
                    else {
                        self.report_error(format!("The following required arguments were not \
                            supplied:{}",
                            self.get_required_from(self.required.iter()
                                                                .map(|s| *s)
                                                                .collect::<HashSet<_>>())
                                .iter()
                                .fold(String::new(), |acc, s| acc + &format!("\n\t'{}'",s)[..])),
                            true,
                            true,
                            Some(matches.args.keys().map(|k| *k).collect()));
                    }
                } else {
                    self.report_error(
                        format!("The argument '{}' requires a value but none was supplied",
                            format!("{}", self.positionals_idx.get(
                                self.positionals_name.get(a).unwrap()).unwrap())),
                            true,
                            true,
                            Some(matches.args.keys().map(|k| *k).collect()));
                }
            }
            _ => {}
        }

        self.validate_blacklist(matches);
        self.validate_num_args(matches);

        if !self.required.is_empty() {
            if self.validate_required(&matches) {
                self.report_error(format!("The following required arguments were not \
                    supplied:{}",
                    self.get_required_from(self.required.iter()
                                                        .map(|s| *s)
                                                        .collect::<HashSet<_>>())
                        .iter()
                        .fold(String::new(), |acc, s| acc + &format!("\n\t'{}'",s)[..])),
                    true,
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }
        }

        matches.usage = Some(self.create_usage(None));

        if let Some(sc_name) = subcmd_name {
            if let Some(ref mut sc) = self.subcommands.get_mut(&sc_name) {
                let mut new_matches = ArgMatches::new();
                // bin_name should be parent's bin_name + the sc's name separated by a space
                sc.bin_name = Some(format!("{}{}{}",
                    self.bin_name.clone().unwrap_or("".to_owned()),
                    if self.bin_name.is_some() {
                        " "
                    } else {
                        ""
                    },
                    sc.name.clone()));
                sc.get_matches_from(&mut new_matches, it);
                matches.subcommand = Some(Box::new(SubCommand{
                    name: sc.name_slice,
                    matches: new_matches}));
            }
        }
    }

    fn blacklisted_from(&self, name: &str, matches: &ArgMatches) -> Option<String> {
        for k in matches.args.keys() {
            if let Some(f) = self.flags.get(k) {
                if let Some(ref bl) = f.blacklist {
                    if bl.contains(name) {
                        return Some(format!("{}", f))
                    }
                }
            }
            if let Some(o) = self.opts.get(k) {
                if let Some(ref bl) = o.blacklist {
                    if bl.contains(name) {
                        return Some(format!("{}", o))
                    }
                }
            }
            if let Some(idx) = self.positionals_name.get(k) {
                if let Some(pos) = self.positionals_idx.get(idx) {
                    if let Some(ref bl) = pos.blacklist {
                        if bl.contains(name) {
                            return Some(format!("{}", pos))
                        }
                    }
                }
            }
         }
        None
    }

    fn create_help_and_version(&mut self) {
        // name is "hclap_help" because flags are sorted by name
        if self.needs_long_help {
            let mut arg = FlagBuilder {
                name: "hclap_help",
                short: None,
                long: Some("help"),
                help: Some("Prints help information"),
                blacklist: None,
                multiple: false,
                requires: None,
            };
            if self.needs_short_help {
                arg.short = Some('h');
            }
            self.flags.insert("hclap_help", arg);
        }
        if self.needs_long_version {
            // name is "vclap_version" because flags are sorted by name
            let mut arg = FlagBuilder {
                name: "vclap_version",
                short: None,
                long: Some("version"),
                help: Some("Prints version information"),
                blacklist: None,
                multiple: false,
                requires: None,
            };
            if self.needs_short_version {
                arg.short = Some('v');
            }
            self.flags.insert("vclap_version", arg);
        }
        if self.needs_subcmd_help && !self.subcommands.is_empty() {
            self.subcommands.insert("help".to_owned(), App::new("help")
                                                            .about("Prints this message"));
        }
    }

    fn check_for_help_and_version(&self, arg: char) {
        if arg == 'h' && self.needs_short_help {
            self.print_help();
        } else if arg == 'v' && self.needs_short_version {
            self.print_version(true);
        }
    }

    fn parse_long_arg(&mut self, matches: &mut ArgMatches<'ar, 'ar> ,full_arg: &String)
                      -> Option<&'ar str> {
        let mut arg = full_arg.trim_left_matches(|c| c == '-');

        if arg == "help" && self.needs_long_help {
            self.print_help();
        } else if arg == "version" && self.needs_long_version {
            self.print_version(true);
        }

        let mut arg_val: Option<String> = None;

        if arg.contains("=") {
            let arg_vec: Vec<&str> = arg.split("=").collect();
            arg = arg_vec[0];
            // prevents "--config= value" typo
            if arg_vec[1].len() == 0 {
                self.report_error(format!("The argument --{} requires a value, but none was \
                    supplied", arg),
                    true,
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }
            arg_val = Some(arg_vec[1].to_owned());
        }

        if let Some(v) = self.opts.values()
                                  .filter(|&v| v.long.is_some())
                                  .filter(|&v| v.long.unwrap() == arg).nth(0) {
            // Ensure this option isn't on the master mutually excludes list
            if self.blacklist.contains(v.name) {
                matches.args.remove(v.name);
                self.report_error(format!("The argument --{} cannot be used with one or more of \
                    the other specified arguments", arg),
                    true, true, Some(matches.args.keys().map(|k| *k).collect()));
            }

            if matches.args.contains_key(v.name) {
                if !v.multiple {
                    self.report_error(format!("The argument --{} was supplied more than once, but \
                        does not support multiple values", arg),
                        true,
                        true,
                        Some(matches.args.keys().map(|k| *k).collect()));
                }
                if let Some(ref p_vals) = v.possible_vals {
                    if let Some(ref av) = arg_val {
                        if !p_vals.contains(&av[..]) {
                            self.possible_values_error(
                                    arg_val.as_ref().map(|v| &**v).unwrap_or(arg),
                                    &v.to_string(), p_vals, matches);
                        }
                    }
                }
                if arg_val.is_some() {
                    if let Some(ref mut o) = matches.args.get_mut(v.name) {
                        o.occurrences += 1;
                        if let Some(ref mut vals) = o.values {
                            let len = (vals.len() + 1) as u8;
                            vals.insert(len, arg_val.clone().unwrap());
                        }
                    }
                }
            } else {
                matches.args.insert(v.name, MatchedArg{
                    occurrences: if arg_val.is_some() { 1 } else { 0 },
                    values: if arg_val.is_some() {
                        let mut bm = BTreeMap::new();
                        bm.insert(1, arg_val.clone().unwrap());
                        Some(bm)
                    } else {
                        Some(BTreeMap::new())
                    }
                });
            }

            if let Some(ref bl) = v.blacklist {
                for name in bl {
                    self.blacklist.insert(name);
                    self.required.remove(name);
                }
            }

            self.required.remove(v.name);

            if let Some(ref reqs) = v.requires {
                // Add all required args which aren't already found in matches to the
                // final required list
                for n in reqs {
                    if matches.args.contains_key(n) { continue; }

                    self.required.insert(n);
                }
            }

            parse_group_reqs!(self, v);

            match arg_val {
                None => { return Some(v.name); },
                _    => { return None; }
            }
        }

        if let Some(v) = self.flags.values()
                                   .filter(|&v| v.long.is_some())
                                   .filter(|&v| v.long.unwrap() == arg).nth(0) {
            // Ensure this flag isn't on the mutually excludes list
            if self.blacklist.contains(v.name) {
                matches.args.remove(v.name);
                self.report_error(format!("The argument '{}' cannot be used with {}",
                    v,
                    match self.blacklisted_from(v.name, matches) {
                        Some(name) => format!("'{}'", name),
                        None       => "one or more of the specified arguments".to_owned()
                    }),
                    true,
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            // Make sure this isn't one being added multiple times if it doesn't suppor it
            if matches.args.contains_key(v.name) && !v.multiple {
                self.report_error(format!("The argument '{}' was supplied more than once, but does \
                    not support multiple values", v),
                    true,
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            let mut
            done = false;
            if let Some(ref mut f) = matches.args.get_mut(v.name) {
                done = true;
                f.occurrences = if v.multiple { f.occurrences + 1 } else { 1 };
            }
            if !done {
                matches.args.insert(v.name, MatchedArg{
                    // name: v.name.to_owned(),
                    occurrences: 1,
                    values: None
                });
            }

            // If this flag was requierd, remove it
            // .. even though Flags shouldn't be required
            self.required.remove(v.name);

            // Add all of this flags "mutually excludes" list to the master list
            if let Some(ref bl) = v.blacklist {
                for name in bl {
                    self.blacklist.insert(name);
                    self.required.remove(name);
                }
            }

            // Add all required args which aren't already found in matches to the master list
            if let Some(ref reqs) = v.requires {
                for n in reqs {
                    if matches.args.contains_key(n) { continue; }

                    self.required.insert(n);
                }
            }

            parse_group_reqs!(self, v);

            return None;
        }

        let suffix = App::did_you_mean_suffix(arg,
                                              self.long_list.iter(),
                                              DidYouMeanMessageStyle::LongFlag);
        if let Some(name) = suffix.1 {
            if let Some(ref opt) = self.opts.values()
                                          .filter_map(|ref o| {
                                              if o.long.is_some() && o.long.unwrap() == name {
                                                  Some(o.name)
                                              } else {
                                                  None
                                              }
                                          })
                                          .next() {
                matches.args.insert(opt, MatchedArg {
                    occurrences: 0,
                    values: None
                });
            } else if let Some(ref flg) = self.flags.values()
                                          .filter_map(|ref f| {
                                              if f.long.is_some() && f.long.unwrap() == name {
                                                  Some(f.name)
                                              } else {
                                                  None
                                              }
                                          })
                                          .next() {
                matches.args.insert(flg, MatchedArg {
                    occurrences: 0,
                    values: None
                });
            }
        }

        self.report_error(format!("The argument --{} isn't valid{}", arg, suffix.0),
            true,
            true,
            Some(matches.args.keys().map(|k| *k).collect()));

        unreachable!();
    }

    fn parse_short_arg(&mut self, matches: &mut ArgMatches<'ar, 'ar> ,full_arg: &String)
                       -> Option<&'ar str> {
        let arg = &full_arg[..].trim_left_matches(|c| c == '-');
        if arg.len() > 1 {
            // Multiple flags using short i.e. -bgHlS
            for c in arg.chars() {
                self.check_for_help_and_version(c);
                if !self.parse_single_short_flag(matches, c) {
                    self.report_error(format!("The argument -{} isn't valid",arg),
                        true,
                        true,
                        Some(matches.args.keys().map(|k| *k).collect()));
                }
            }
            return None;
        }
        // Short flag or opt
        let arg_c = arg.chars().nth(0).unwrap();

        // Ensure the arg in question isn't a help or version flag
        self.check_for_help_and_version(arg_c);

        // Check for a matching flag, and return none if found
        if self.parse_single_short_flag(matches, arg_c) { return None; }

        // Check for matching short in options, and return the name
        // (only ones with shorts, of course)
        if let Some(v) = self.opts.values()
                             .filter(|&v| v.short.is_some())
                             .filter(|&v| v.short.unwrap() == arg_c).nth(0) {
            // Ensure this option isn't on the master mutually excludes list
            if self.blacklist.contains(v.name) {
                matches.args.remove(v.name);
                self.report_error(format!("The argument -{} cannot be used with {}",
                        arg,
                        match self.blacklisted_from(v.name, matches) {
                            Some(name) => format!("'{}'", name),
                            None       => "one or more of the other specified arguments".to_owned()
                        }),
                    true,
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            if matches.args.contains_key(v.name) {
                if !v.multiple {
                    self.report_error(format!("The argument -{} was supplied more than once, but \
                        does not support multiple values", arg),
                        true,
                        true,
                        Some(matches.args.keys().map(|k| *k).collect()));
                }
            } else {
                matches.args.insert(v.name, MatchedArg{
                    // name: v.name.to_owned(),
                    // occurrences will be incremented on getting a value
                    occurrences: 0,
                    values: Some(BTreeMap::new())
                });
            }
            if let Some(ref bl) = v.blacklist {
                for name in bl {
                    self.blacklist.insert(name);
                    self.required.remove(name);
                }
            }

            self.required.remove(v.name);

            if let Some(ref reqs) = v.requires {
                // Add all required args which aren't already found in matches to the
                // final required list
                for n in reqs {
                    if matches.args.contains_key(n) { continue; }

                    self.required.insert(n);
                }
            }

            parse_group_reqs!(self, v);

            return Some(v.name)
        }

        // Didn't match a flag or option, must be invalid
        self.report_error(format!("The argument -{} isn't valid",arg_c),
            true,
            true,
            Some(matches.args.keys().map(|k| *k).collect()));

        unreachable!();
    }

    fn parse_single_short_flag(&mut self, matches: &mut ArgMatches<'ar, 'ar>, arg: char) -> bool {
        for v in self.flags.values()
                           .filter(|&v| v.short.is_some())
                           .filter(|&v| v.short.unwrap() == arg) {
            // Ensure this flag isn't on the mutually excludes list
            if self.blacklist.contains(v.name) {
                matches.args.remove(v.name);
                self.report_error(format!("The argument -{} cannot be used {}",
                        arg,
                        match self.blacklisted_from(v.name, matches) {
                            Some(name) => format!("'{}'", name),
                            None       => "with one or more of the other specified \
                                arguments".to_owned()
                        }),
                    true,
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            // Make sure this isn't one being added multiple times if it doesn't suppor it
            if matches.args.contains_key(v.name) && !v.multiple {
                self.report_error(format!("The argument -{} was supplied more than once, but does \
                        not support multiple values", arg),
                    true,
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            let mut done = false;
            if let Some(ref mut f) = matches.args.get_mut(v.name) {
                done = true;
                f.occurrences = if v.multiple { f.occurrences + 1 } else { 1 };
            }
            if !done {
                matches.args.insert(v.name, MatchedArg{
                    // name: v.name.to_owned(),
                    occurrences: 1,
                    values: None
                });
            }

            // If this flag was requierd, remove it
            // .. even though Flags shouldn't be required
            self.required.remove(v.name);

            // Add all of this flags "mutually excludes" list to the master list
            if let Some(ref bl) = v.blacklist {
                for name in bl {
                    self.blacklist.insert(name);
                    self.required.remove(name);
                }
            }

            // Add all required args which aren't already found in matches to the master list
            if let Some(ref reqs) = v.requires {
                for n in reqs {
                    if matches.args.contains_key(n) { continue; }

                    self.required.insert(n);
                }
            }

            parse_group_reqs!(self, v);

            return true;
        }
        false
    }

    fn validate_blacklist(&self, matches: &mut ArgMatches<'ar, 'ar>) {
        for name in self.blacklist.iter() {
            if matches.args.contains_key(name) {
                matches.args.remove(name);
                self.report_error(format!("The argument '{}' cannot be used with {}",
                    if let Some(ref flag) = self.flags.get(name) {
                        format!("{}", flag)
                    } else if let Some(ref opt) = self.opts.get(name) {
                        format!("{}", opt)
                    } else {
                        match self.positionals_idx.values().filter(|p| p.name == *name).next() {
                            Some(pos) => format!("{}", pos),
                            None      => format!("\"{}\"", name)
                        }
                    }, match self.blacklisted_from(name, matches) {
                        Some(name) => format!("'{}'", name),
                        None       => "one or more of the other specified arguments".to_owned()
                    }), true, true, Some(matches.args.keys().map(|k| *k).collect()));
            } else if self.groups.contains_key(name) {
                for n in self.get_group_members_names(name) {
                    if matches.args.contains_key(n) {
                        matches.args.remove(n);
                        self.report_error(format!("The argument '{}' cannot be used with one or \
                                more of the other specified arguments",
                                if let Some(ref flag) = self.flags.get(n) {
                                    format!("{}", flag)
                                } else if let Some(ref opt) = self.opts.get(n) {
                                    format!("{}", opt)
                                } else {
                                    match self.positionals_idx.values()
                                                              .filter(|p| p.name == *name)
                                                              .next() {
                                        Some(pos) => format!("{}", pos),
                                        None      => format!("\"{}\"", n)
                                    }
                                }),
                            true,
                            true,
                            Some(matches.args.keys().map(|k| *k).collect()));
                    }
                }
            }
        }
    }

    fn validate_num_args(&self, matches: &mut ArgMatches<'ar, 'ar>) {
        for (name, ma) in matches.args.iter() {
            if let Some(ref vals) = ma.values {
                if let Some(f) = self.opts.get(name) {
                    if let Some(num) = f.num_vals {
                        let should_err = if f.multiple {
                            ((vals.len() as u8) % num) != 0
                        } else {
                            num != (vals.len() as u8)
                        };
                        if should_err {
                            self.report_error(format!("The argument '{}' requires {} values, \
                                    but {} w{} provided",
                                    f,
                                    num,
                                    if f.multiple {
                                        vals.len() % num as usize
                                    } else {
                                        vals.len()
                                    },
                                    if vals.len() == 1 ||
                                        ( f.multiple &&
                                            ( vals.len() % num as usize) == 1) {"as"}else{"ere"}),
                                true,
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.max_vals {
                        if (vals.len() as u8) > num {
                            self.report_error(format!("The argument '{}' requires no more than {} \
                                    values, but {} w{} provided",
                                    f,
                                    num,
                                    vals.len(),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.min_vals {
                        if (vals.len() as u8) < num {
                            self.report_error(format!("The argument '{}' requires at least {} \
                                    values, but {} w{} provided",
                                    f,
                                    num,
                                    vals.len(),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                } else if let Some(f) = self.positionals_idx.get(
                    self.positionals_name.get(name).unwrap()) {
                    if let Some(num) = f.num_vals {
                        if num != vals.len() as u8 {
                            self.report_error(format!("The argument '{}' requires {} values, \
                                    but {} w{} provided",
                                    f,
                                    num,
                                    vals.len(),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.max_vals {
                        if num > vals.len() as u8 {
                            self.report_error(format!("The argument '{}' requires no more than {} \
                                    values, but {} w{} provided",
                                    f,
                                    num,
                                    vals.len(),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.min_vals {
                        if num < vals.len() as u8 {
                            self.report_error(format!("The argument '{}' requires at least {} \
                                    values, but {} w{} provided",
                                    f,
                                    num,
                                    vals.len(),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                }
            }
        }
    }

    fn validate_required(&self, matches: &ArgMatches<'ar, 'ar>) -> bool{
        for name in self.required.iter() {
            validate_reqs!(self, flags, matches, name);

            validate_reqs!(self, opts, matches, name);

            // because positions use different keys, we dont use the macro
            match self.positionals_idx.values().filter(|ref p| &p.name == name).next() {
                Some(p) =>{
                    if let Some(ref bl) = p.blacklist {
                        for n in bl.iter() {
                            if matches.args.contains_key(n) {
                                return false
                            } else if self.groups.contains_key(n) {
                                let grp = self.groups.get(n).unwrap();
                                for an in grp.args.iter() {
                                    if matches.args.contains_key(an) {
                                        return false
                                    }
                                }
                            }
                        }
                    }
                },
                None    =>(),
            }
        }
        true
    }
}
