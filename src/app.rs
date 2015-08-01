use std::collections::{BTreeMap, BTreeSet, HashSet, HashMap, VecDeque};
use std::env;
use std::io::{self, BufRead, Write};
use std::path::Path;

use args::{ArgMatches, Arg, SubCommand, MatchedArg};
use args::{FlagBuilder, OptBuilder, PosBuilder};
use args::ArgGroup;
use fmt::Format;

#[cfg(feature = "suggestions")]
use strsim;

const INTERNAL_ERROR_MSG: &'static str = "Internal Error: Failed to write string. Please \
                                          consider filing a bug report!";

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
#[cfg(feature = "suggestions")]
fn did_you_mean<'a, T, I>(v: &str, possible_values: I) -> Option<&'a str>
                    where T: AsRef<str> + 'a,
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
                    where T: AsRef<str> + 'a,
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
/// arguments.
///
/// Application settings are set using the "builder pattern" with `.get_matches()` being the
/// terminal method that starts the runtime-parsing process and returns information about
/// the user supplied arguments (or lack there of).
///
/// There aren't any mandatory "options" that one must set. The "options" may also appear in any
/// order (so long as `.get_matches()` is the last method called).
///
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg};
/// let matches = App::new("myprog")
///                   .author("Me, me@mail.com")
///                   .version("1.0.2")
///                   .about("Explains in brief what the program does")
///                   .arg(
///                            Arg::with_name("in_file").index(1)
///                    )
///                   .after_help("Longer explaination to appear after the options when \
///                                displaying the help information from --help or -h")
///                   .get_matches();
///
/// // Your program logic starts here...
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
    help_short: Option<char>,
    version_short: Option<char>,
    required: HashSet<&'ar str>,
    short_list: HashSet<char>,
    long_list: HashSet<&'ar str>,
    blacklist: HashSet<&'ar str>,
    usage_str: Option<&'u str>,
    bin_name: Option<String>,
    usage: Option<String>,
    groups: HashMap<&'ar str, ArgGroup<'ar, 'ar>>,
    global_args: Vec<Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>>,
    help_str: Option<&'u str>,
    no_sc_error: bool,
    wait_on_error: bool,
    help_on_no_args: bool,
    needs_long_help: bool,
    needs_long_version: bool,
    needs_subcmd_help: bool,
    subcmds_neg_reqs: bool,
    help_on_no_sc: bool,
    global_ver: bool,
    // None = not set, Some(true) set for all children, Some(false) = disable version
    versionless_scs: Option<bool>,
    unified_help: bool
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
    pub fn new(n: &'ar str) -> Self {
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
            needs_subcmd_help: true,
            help_short: None,
            version_short: None,
            required: HashSet::new(),
            short_list: HashSet::new(),
            long_list: HashSet::new(),
            usage_str: None,
            usage: None,
            blacklist: HashSet::new(),
            bin_name: None,
            groups: HashMap::new(),
            subcmds_neg_reqs: false,
            global_args: vec![],
            no_sc_error: false,
            help_str: None,
            wait_on_error: false,
            help_on_no_args: false,
            help_on_no_sc: false,
            global_ver: false,
            versionless_scs: None,
            unified_help: false
        }
    }

    /// Sets a string of author(s) and will be displayed to the user when they request the help
    /// information with `--help` or `-h`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///      .author("Me, me@mymain.com")
    /// # ;
    /// ```
    pub fn author(mut self, a: &'a str) -> Self {
        self.author = Some(a);
        self
    }

    /// Overrides the system-determined binary name. This should only be used when absolutely
    /// neccessary, such as the binary name for your application is misleading, or perhaps *not*
    /// how the user should invoke your program.
    ///
    /// **NOTE:** This command **should not** be used for SubCommands.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///      .bin_name("my_binary")
    /// # ;
    /// ```
    pub fn bin_name(mut self, a: &str) -> Self {
        self.bin_name = Some(a.to_owned());
        self
    }

    /// Sets a string briefly describing what the program does and will be displayed when
    /// displaying help information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .about("Does really amazing things to great people")
    /// # ;
    /// ```
    pub fn about(mut self, a: &'ab str) -> Self {
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
    /// App::new("myprog")
    ///     .after_help("Does really amazing things to great people")
    /// # ;
    /// ```
    pub fn after_help(mut self, h: &'h str) -> Self {
        self.more_help = Some(h);
        self
    }

    /// Allows subcommands to override all requirements of the parent (this command). For example
    /// if you had a subcommand or even top level application which had a required arguments that
    /// are only required as long as there is no subcommand present.
    ///
    /// **NOTE:** This defaults to false (using subcommand does *not* negate requirements)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .subcommands_negate_reqs(true)
    /// # ;
    /// ```
    pub fn subcommands_negate_reqs(mut self, n: bool) -> Self {
        self.subcmds_neg_reqs = n;
        self
    }

    /// Allows specifying that if no subcommand is present at runtime, error and exit gracefully
    ///
    /// **NOTE:** This defaults to false (subcommands do *not* need to be present)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .subcommand_required(true)
    /// # ;
    /// ```
    pub fn subcommand_required(mut self, n: bool) -> Self {
        self.no_sc_error = n;
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .version("v0.1.24")
    /// # ;
    /// ```
    pub fn version(mut self, v: &'v str) -> Self {
        self.version = Some(v);
        self
    }

    /// Sets a custom usage string to override the auto-generated usage string.
    ///
    /// This will be displayed to the user when errors are found in argument parsing, or when you
    /// call `ArgMatches::usage()`
    ///
    /// **NOTE:** You do not need to specify the "USAGE: \n\t" portion, as that will
    /// still be applied by `clap`, you only need to specify the portion starting
    /// with the binary name.
    ///
    /// **NOTE:** This will not replace the entire help message, *only* the portion
    /// showing the usage.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .usage("myapp [-clDas] <some_file>")
    /// # ;
    /// ```
    pub fn usage(mut self, u: &'u str) -> Self {
        self.usage_str = Some(u);
        self
    }

    /// Sets a custom help message and overrides the auto-generated one. This should only be used
    /// when the auto-generated message does not suffice.
    ///
    /// This will be displayed to the user when they use the default `--help` or `-h`
    ///
    /// **NOTE:** This replaces the **entire** help message, so nothing will be auto-generated.
    ///
    /// **NOTE:** This **only** replaces the help message for the current command, meaning if you
    /// are using subcommands, those help messages will still be auto-generated unless you
    /// specify a `.help()` for them as well.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myapp")
    ///     .help("myapp v1.0\n\
    ///            Does awesome things\n\
    ///            (C) me@mail.com\n\n\
    ///
    ///            USAGE: myapp <opts> <comamnd>\n\n\
    ///
    ///            Options:\n\
    ///            -h, --helpe      Dispay this message\n\
    ///            -V, --version    Display version info\n\
    ///            -s <stuff>       Do something with stuff\n\
    ///            -v               Be verbose\n\n\
    ///
    ///            Commmands:\n\
    ///            help             Prints this message\n\
    ///            work             Do some work")
    /// # ;
    /// ```
    pub fn help(mut self, h: &'u str) -> Self {
        self.help_str = Some(h);
        self
    }

    /// Sets the short version of the `help` argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `h`, but this can be overridden
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     // Using an uppercase `H` instead of the default lowercase `h`
    ///     .help_short("H")
    /// # ;
    pub fn help_short(mut self, s: &str) -> Self {
        self.help_short = s.trim_left_matches(|c| c == '-')
                           .chars()
                           .nth(0);
        self
    }

    /// Sets the short version of the `version` argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `V`, but this can be overridden
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     // Using a lowercase `v` instead of the default capital `V`
    ///     .version_short("v")
    /// # ;
    pub fn version_short(mut self, s: &str) -> Self {
        self.version_short = s.trim_left_matches(|c| c == '-')
                           .chars()
                           .nth(0);
        self
    }

    /// Specifies that the help text sould be displayed (and then exit gracefully), if no
    /// arguments are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **NOTE:** Subcommands count as arguments
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .arg_required_else_help(true)
    /// # ;
    /// ```
    pub fn arg_required_else_help(mut self, tf: bool) -> Self {
        self.help_on_no_args = tf;
        self
    }

    /// Uses version of the current command for all subcommands. (Defaults to false; subcommands
    /// have independant version strings)
    ///
    /// **NOTE:** The version for the current command and this setting must be set **prior** to
    /// adding any subcommands
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .global_version(true)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches();
    /// // running `myprog test --version` will display
    /// // "myprog-test v1.1"
    /// ```
    pub fn global_version(mut self, gv: bool) -> Self {
        self.global_ver = gv;
        self
    }

    /// Disables `-V` and `--version` for all subcommands (Defaults to false; subcommands have
    /// version flags)
    ///
    /// **NOTE:** This setting must be set **prior** adding any subcommands
    ///
    /// **NOTE:** Do not set this value to false, it will have undesired results!
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .versionless_subcommands(true)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches();
    /// // running `myprog test --version` will display unknown argument error
    /// ```
    pub fn versionless_subcommands(mut self, vers: bool) -> Self {
        self.versionless_scs = Some(vers);
        self
    }

    /// By default the auto-generated help message groups flags, options, and positional arguments
    /// separately. This setting disable that and groups flags and options together presenting a
    /// more unified help message (a la getopts or docopt style).
    ///
    /// **NOTE:** This setting is cosmetic only and does not affect any functionality.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .unified_help_message(true)
    ///     .get_matches();
    /// // running `myprog --help` will display a unified "docopt" or "getopts" style help message
    /// ```
    pub fn unified_help_message(mut self, uni_help: bool) -> Self {
        self.unified_help = uni_help;
        self
    }

    /// Will display a message "Press [ENTER]/[RETURN] to continue..." and wait user before
    /// exiting
    ///
    /// This is most useful when writing an application which is run from a GUI shortcut, or on
    /// Windows where a user tries to open the binary by double-clicking instead of using the
    /// command line (i.e. set `.arg_required_else_help(true)` and `.wait_on_error(true)` to
    /// display the help in such a case).
    ///
    /// **NOTE:** This setting is **not** recursive with subcommands, meaning if you wish this
    /// behavior for all subcommands, you must set this on each command (needing this is extremely
    /// rare)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .arg_required_else_help(true)
    /// # ;
    /// ```
    pub fn wait_on_error(mut self, w: bool) -> Self {
        self.wait_on_error = w;
        self
    }

    /// Specifies that the help text sould be displayed (and then exit gracefully), if no
    /// subcommands are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **NOTE:** This should *not* be used with `.subcommand_required()` as they do the same
    /// thing, except one prints the help text, and one prints an error.
    ///
    /// **NOTE:** If the user specifies arguments at runtime, but no subcommand the help text will
    /// still be displayed and exit. If this is *not* the desired result, consider using
    /// `.arg_required_else_help()`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .subcommand_required_else_help(true)
    /// # ;
    /// ```
    pub fn subcommand_required_else_help(mut self, tf: bool) -> Self {
        self.help_on_no_sc = tf;
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
    /// App::new("myprog")
    ///     // Adding a single "flag" argument with a short and help text, using Arg::with_name()
    ///     .arg(
    ///         Arg::with_name("debug")
    ///            .short("d")
    ///            .help("turns on debugging mode")
    ///     )
    ///     // Adding a single "option" argument with a short, a long, and help text using the less
    ///     // verbose Arg::from_usage()
    ///     .arg(
    ///         Arg::from_usage("-c --config=[CONFIG] 'Optionally sets a config file to use'")
    ///     )
    /// # ;
    /// ```
    pub fn arg(mut self, a: Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>) -> Self {
        self.add_arg(a);
        self
    }

    fn add_arg(&mut self, a: Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>) {
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
            // if s == 'V' {
            //     self.version_short = None;
            // } else if s == 'h' {
            //     self.help_short = None;
            // }
            if self.short_list.contains(&s) {
                panic!("Argument short must be unique\n\n\t-{} is already in use", s);
            } else {
                self.short_list.insert(s);
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
                global: a.global,
                empty_vals: a.empty_vals
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
                global: a.global,
                possible_vals: None,
                num_vals: a.num_vals,
                min_vals: a.min_vals,
                max_vals: a.max_vals,
                val_names: a.val_names.clone(),
                requires: None,
                required: a.required,
                empty_vals: a.empty_vals
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
            if !a.empty_vals {
                // Empty vals defaults to true, so if it's false it was manually set
                panic!("The argument '{}' cannot have empty_values() set because it is a flag. \
                    Perhaps you mean't to set takes_value(true) as well?", a.name);
            }
            if a.required {
                panic!("The argument '{}' cannot be required(true) because it has no index() or \
                    takes_value(true)", a.name);
            }
            if a.possible_vals.is_some() {
                panic!("The argument '{}' cannot have a specific value set because it doesn't \
                have takes_value(true) set", a.name);
            }
            // No need to check for index() or takes_value() as that is handled above

            let mut fb = FlagBuilder {
                name: a.name,
                short: a.short,
                long: a.long,
                help: a.help,
                blacklist: None,
                global: a.global,
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
        if a.global {
            if a.required {
                panic!("Global arguments cannot be required.\n\n\t'{}' is marked as global and required", a.name);
            }
            self.global_args.push(a);
        }
    }

    /// Adds multiple arguments to the list of valid possibilties by iterating over a Vec of Args
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args(
    ///         vec![Arg::from_usage("[debug] -d 'turns on debugging info"),
    ///              Arg::with_name("input").index(1).help("the input file to use")]
    ///     )
    /// # ;
    /// ```
    pub fn args(mut self, args: Vec<Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>>)
                -> Self {
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
    /// App::new("myprog")
    ///     .arg_from_usage("-c --conf=<config> 'Sets a configuration file to use'")
    /// # ;
    /// ```
    pub fn arg_from_usage(mut self, usage: &'ar str) -> Self {
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
    /// App::new("myprog")
    ///     .args_from_usage(
    ///         "-c --conf=[config] 'Sets a configuration file to use'
    ///          [debug]... -d 'Sets the debugging level'
    ///          <input> 'The input file to use'"
    ///     )
    /// # ;
    /// ```
    pub fn args_from_usage(mut self, usage: &'ar str) -> Self {
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
    /// # App::new("app")
    /// .args_from_usage("--set-ver [ver] 'set the version manually'
    ///                   --major         'auto increase major'
    ///                   --minor         'auto increase minor'
    ///                   --patch         'auto increase patch")
    /// .arg_group(ArgGroup::with_name("vers")
    ///                     .add_all(vec!["ver", "major", "minor","patch"])
    ///                     .required(true))
    /// # ;
    pub fn arg_group(mut self, group: ArgGroup<'ar, 'ar>) -> Self {
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
    /// # App::new("app")
    /// .args_from_usage("--set-ver [ver] 'set the version manually'
    ///                   --major         'auto increase major'
    ///                   --minor         'auto increase minor'
    ///                   --patch         'auto increase patch")
    /// .arg_group(ArgGroup::with_name("vers")
    ///                     .add_all(vec!["ver", "major", "minor","patch"])
    ///                     .required(true))
    /// # ;
    pub fn arg_groups(mut self, groups: Vec<ArgGroup<'ar, 'ar>>) -> Self {
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
    /// # App::new("myprog")
    /// .subcommand(SubCommand::with_name("config")
    ///                .about("Controls configuration features")
    ///                .arg_from_usage("<config> 'Required configuration file to use'"))
    ///             // Additional subcommand configuration goes here, such as other arguments...
    /// # ;
    /// ```
    pub fn subcommand(mut self, mut subcmd: App<'a, 'v, 'ab, 'u, 'h, 'ar>)
                      -> Self {
        if subcmd.name == "help" { self.needs_subcmd_help = false; }
        if self.versionless_scs.is_some() && self.versionless_scs.unwrap() {
            subcmd.versionless_scs = Some(false);
        }
        if self.global_ver && subcmd.version.is_none() && self.version.is_some() {
            subcmd.version = Some(self.version.unwrap());
        }
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
    /// # App::new("myprog")
    /// .subcommands( vec![
    ///        SubCommand::with_name("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::with_name("config_file").index(1)),
    ///        SubCommand::with_name("debug").about("Controls debug functionality")])
    /// # ;
    /// ```
    pub fn subcommands(mut self, subcmds: Vec<App<'a, 'v, 'ab, 'u, 'h, 'ar>>)
                       -> Self {
        for subcmd in subcmds.into_iter() {
            self = self.subcommand(subcmd);
        }
        self
    }

    fn get_group_members(&self, group: &str) -> Vec<String> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in self.groups.get(group).unwrap().args.iter() {
            if let Some(f) = self.flags.get(n) {
                args.push(f.to_string());
            } else if let Some(f) = self.opts.get(n) {
                args.push(f.to_string());
            } else if self.groups.contains_key(n) {
                g_vec.push(*n);
            } else {
                if let Some(idx) = self.positionals_name.get(n) {
                    if let Some(p) = self.positionals_idx.get(&idx) {
                        args.push(p.to_string());
                    }
                }
            }
        }

        g_vec.dedup();
        if !g_vec.is_empty() {
            for av in g_vec.iter().map(|g| self.get_group_members(g)) {
                for a in av {
                    args.push(a);
                }
            }
        }
        args.dedup();
        args.iter().map(ToOwned::to_owned).collect()
    }

    fn get_group_members_names(&self, group: &'ar str) -> Vec<&'ar str> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in self.groups.get(group).unwrap().args.iter() {
            if self.flags.contains_key(n) {
                args.push(*n);
            } else if self.opts.contains_key(n) {
                args.push(*n);
            } else if self.groups.contains_key(n) {
                g_vec.push(*n);
            } else {
                if self.positionals_name.contains_key(n) {
                    args.push(*n);
                }
            }
        }

        g_vec.dedup();
        if !g_vec.is_empty() {
            for av in g_vec.iter().map(|g| self.get_group_members_names(g)) {
                for a in av {
                    args.push(a);
                }
            }
        }
        args.dedup();
        args.iter().map(|s| *s).collect()
    }

    fn get_required_from(&self, mut reqs: Vec<&'ar str>) -> VecDeque<String> {
        reqs.dedup();
        let mut c_flags = vec![];
        let mut c_pos = vec![];
        let mut c_opt = vec![];
        let mut grps = vec![];
        for name in reqs.iter() {
            if self.flags.contains_key(name) {
                c_flags.push(name);
            } else if self.opts.contains_key(name) {
                c_opt.push(name);
            } else if self.groups.contains_key(name) {
                grps.push(*name);
            } else {
                c_pos.push(name);
            }
        }
        let mut tmp_f = vec![];
        for f in c_flags.iter() {
            if let Some(f) = self.flags.get(*f) {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                tmp_f.push(r);
                            } else if self.opts.contains_key(r) {
                                c_opt.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(*r);
                            } else {
                                c_pos.push(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_f.into_iter() {
            c_flags.push(f);
        }
        let mut tmp_o = vec![];
        for f in &c_opt {
            if let Some(f) = self.opts.get(*f) {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                c_flags.push(r);
                            } else if self.opts.contains_key(r) {
                                tmp_o.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(*r);
                            } else {
                                c_pos.push(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_o.into_iter() {
            c_opt.push(f);
        }
        let mut tmp_p = vec![];
        for f in c_pos.iter() {
            if let Some(f) = self.flags.get(*f) {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                c_flags.push(r);
                            } else if self.opts.contains_key(r) {
                                c_opt.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(*r);
                            } else {
                                tmp_p.push(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_p.into_iter() {
            c_flags.push(f);
        }


        let mut ret_val = VecDeque::new();

        let mut pmap = BTreeMap::new();
        for p in c_pos.into_iter() {
            if let Some(idx) = self.positionals_name.get(p) {
                if let Some(ref p) = self.positionals_idx.get(&idx) {
                    pmap.insert(p.index, format!("{}", p));
                }
            }
        }
        pmap.into_iter().map(|(_, s)| ret_val.push_back(s)).collect::<Vec<_>>();
        for f in c_flags.into_iter() {
             ret_val.push_back(format!("{}", self.flags.get(*f).unwrap()));
        }
        for o in c_opt.into_iter() {
             ret_val.push_back(format!("{}", self.opts.get(*o).unwrap()));
        }
        for g in grps.into_iter() {
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
        use ::std::fmt::Write;
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n\t");
        if let Some(u) = self.usage_str {
            usage.push_str(u);
        } else if let Some(tmp_vec) = matches {
            let mut hs = self.required.iter().map(|n| *n).collect::<Vec<_>>();
            tmp_vec.iter().map(|n| hs.push(*n)).collect::<Vec<_>>();
            let reqs = self.get_required_from(hs);

            let r_string = reqs.iter().fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

            write!(&mut usage, "{}{}",
                self.usage.clone().unwrap_or(self.bin_name.clone().unwrap_or(self.name.clone())),
                r_string
            ).ok().expect(INTERNAL_ERROR_MSG);
            if self.no_sc_error {
                write!(&mut usage, " <SUBCOMMAND>").ok().expect(INTERNAL_ERROR_MSG);
            }
        } else {
            usage.push_str(&*self.usage.clone()
                                       .unwrap_or(self.bin_name.clone()
                                                               .unwrap_or(self.name.clone())));

            let mut reqs = self.required.iter().map(|n| *n).collect::<Vec<_>>();
            // If it's required we also need to ensure all previous positionals are required too
            let mut found = false;
            for p in self.positionals_idx.values().rev() {
                if found {
                    reqs.push(p.name);
                    continue;
                }
                if p.required {
                    found = true;
                    reqs.push(p.name);
                }
            }
            let req_strings = self.get_required_from(reqs);
            let req_string = req_strings.iter()
                                        .fold(String::new(), |acc, s| {
                                            acc + &format!(" {}", s)[..]
                                        });


            if !self.flags.is_empty() && !self.unified_help {
                usage.push_str(" [FLAGS]");
            } else {
                usage.push_str(" [OPTIONS]");
            }
            if !self.unified_help 
                && !self.opts.is_empty() && self.opts.values().any(|a| !a.required) {
                usage.push_str(" [OPTIONS]");
            }
            // places a '--' in the usage string if there are args and options 
            // supporting multiple values
            if !self.positionals_idx.is_empty()
                && self.opts.values().any(|a| a.multiple )
                && !self.opts.values().any(|a| a.required)
                && self.subcommands.is_empty() {
                usage.push_str(" [--]")
            }
            if !self.positionals_idx.is_empty() && self.positionals_idx.values()
                                                                       .any(|a| !a.required) {
                usage.push_str(" [ARGS]");
            }

            usage.push_str(&req_string[..]);

            if !self.subcommands.is_empty() && !self.no_sc_error {
                usage.push_str(" [SUBCOMMAND]");
            } else if self.no_sc_error && !self.subcommands.is_empty() {
                usage.push_str(" <SUBCOMMAND>");
            }
        }

        usage.shrink_to_fit();
        usage
    }

    // // Prints the usage statement to the user
    // fn print_usage(&self, more_info: bool, matches: Option<Vec<&str>>) {
    //     print!("{}",self.create_usage(matches));
    //     if more_info {
    //         println!("\n\nFor more information try {}", Format::Good("--help"));
    //     }
    // }

    // Prints the full help message to the user
    fn print_help(&self) {
        if let Some(h) = self.help_str {
            println!("{}", h);
            return
        }

        // Print the version
        print!("{} {}\n", &self.bin_name.clone().unwrap_or(
            self.name.clone())[..].replace(" ", "-"),
            self.version.unwrap_or("")
        );
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
            // .filter(|ref o| o.long.is_some())
            .map(|ref a|
                a.to_string().len() // + if a.short.is_some() { 4 } else { 0 }
            ) {
            if ol > longest_opt {
                longest_opt = ol;
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
            print!("{}\n", author);
        }
        if let Some(about) = self.about {
            print!("{}\n", about);
        }

        print!("\n{}", self.create_usage(None));

        if flags || opts || pos || subcmds {
            print!("\n");
        }

        let tab = "    ";
        if flags {
            if !self.unified_help {
                print!("\nFLAGS:\n"); 
            } else {
                print!("\nOPTIONS:\n")
            }
            for v in self.flags.values() {
                print!("{}", tab);
                if let Some(s) = v.short {
                    print!("-{}",s);
                } else { 
                    print!("{}", tab);
                }
                if let Some(l) = v.long {
                    print!("{}--{}",
                        if v.short.is_some() { ", " } else {""},
                        l
                    );
                    self.print_spaces(
                            if !self.unified_help {
                                (longest_flag + 4)
                            } else {
                                (longest_opt + 4)
                            } - (l.len() + 2)
                    );
                } else {
                    // 6 is tab (4) + -- (2)
                    self.print_spaces(
                        if !self.unified_help {
                            (longest_flag + 6)
                        } else {
                            (longest_opt + 6)
                        }
                    );
                }
                if let Some(h) = v.help {
                    if h.contains("{n}") {
                        let mut hel = h.split("{n}");
                        while let Some(part) = hel.next() {
                            print!("{}\n", part);
                            self.print_spaces(
                                if !self.unified_help {
                                    longest_flag
                                } else {
                                    longest_opt
                                } + 12);
                            print!("{}", hel.next().unwrap_or(""));
                        }
                    } else {
                        print!("{}", h);
                    }
                }
                print!("\n");
            }
        }
        if opts {
            if !self.unified_help {
                print!("\nOPTIONS:\n"); 
            } else {
                // maybe erase
            }
            for v in self.opts.values() {
                // if it supports multiple we add '...' i.e. 3 to the name length
                print!("{}", tab);
                if let Some(s) = v.short {
                    print!("-{}",s);
                } else {
                    print!("{}", tab);
                }
                if let Some(l) = v.long {
                    print!("{}--{}", if v.short.is_some() {", "} else {""}, l);
                } 
                if let Some(ref vec) = v.val_names {
                    for val in vec.iter() {
                        print!(" <{}>", val);
                    }
                } else if let Some(num) = v.num_vals {
                    for _ in (0..num) {
                        print!(" <{}>", v.name);
                    }
                } else {
                    print!(" <{}>{}", v.name, if v.multiple{"..."} else {""});
                }
                if v.long.is_some() {
                    self.print_spaces(
                        (longest_opt + 4) - (v.to_string().len())
                    );
                } else {
                    // 8 = tab + '-a, '.len()
                    self.print_spaces((longest_opt + 8) - (v.to_string().len()));
                };
                print_opt_help!(self, v, longest_opt + 12);
                print!("\n");
            }
        }
        if pos {
            print!("\nARGS:\n");
            for v in self.positionals_idx.values() {
                // let mult = if v.multiple { 3 } else { 0 };
                print!("{}", tab);
                print!("{}", v.name);
                if v.multiple {
                    print!("...");
                }
                self.print_spaces((longest_pos + 4) - (v.to_string().len()));
                if let Some(h) = v.help {
                    if h.contains("{n}") {
                        let mut hel = h.split("{n}");
                        while let Some(part) = hel.next() {
                            print!("{}\n", part);
                            self.print_spaces(longest_pos + 6);
                            print!("{}", hel.next().unwrap_or(""));
                        }
                    } else {
                        print!("{}", h);
                    }
                }
                print!("\n");
            }
        }
        if subcmds {
            print!("\nSUBCOMMANDS:\n");
            for sc in self.subcommands.values() {
                print!("{}{}", tab, sc.name);
                self.print_spaces((longest_sc + 4) - (sc.name.len()));
                if let Some(a) = sc.about {
                    if a.contains("{n}") {
                        let mut ab = a.split("{n}");
                        while let Some(part) = ab.next() {
                            print!("{}\n", part);
                            self.print_spaces(longest_sc + 8);
                            print!("{}", ab.next().unwrap_or(""));
                        }
                    } else {
                        print!("{}", a);
                    }
                } 
                print!("\n");
            }
        }

        if let Some(h) = self.more_help {
            print!("\n{}", h);
        }

        // flush the buffer
        println!("");

        self.exit(0);
    }

    // Used when spacing arguments and their help message when displaying help information
    fn print_spaces(&self, num: usize) {
        for _ in (0..num) {
            print!(" ");
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
        if self.wait_on_error {
            wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
            let mut s = String::new();
            let i = io::stdin();
            i.lock().read_line(&mut s).unwrap();
        }
        ::std::process::exit(status);
    }

    // Reports and error to stderr along with an optional usage statement and optionally quits
    fn report_error(&self, msg: String, quit: bool, matches: Option<Vec<&str>>) {
        wlnerr!("{} {}\n\n{}\n\nFor more information try {}",
            Format::Error(&format!("error:")[..]),
            msg,
            self.create_usage(matches),
            Format::Good("--help")
        );
       if quit { self.exit(1); }
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches();
    /// ```
    pub fn get_matches(self) -> ArgMatches<'ar, 'ar> {
        let args: Vec<_> = env::args().collect();

        self.get_matches_from(args)
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands
    ///
    /// **NOTE:** The first argument will be parsed as the binary name.
    ///
    /// **NOTE:** This method should only be used when absolutely necessary, such as needing to
    /// parse arguments from something other than `std::env::args()`. If you are unsure, use
    /// `App::get_matches()`
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches_from(arg_vec);
    /// ```
    pub fn get_matches_from<I, T>(mut self, itr: I)
                                  -> ArgMatches<'ar, 'ar>
                                  where I: IntoIterator<Item=T>,
                                        T: AsRef<str> {
        self.verify_positionals();
        self.propogate_globals();

        let mut matches = ArgMatches::new();

        let mut it = itr.into_iter();
        if let Some(name) = it.next() {
            let p = Path::new(name.as_ref());
            if let Some(f) = p.file_name() {
                if let Ok(s) = f.to_os_string().into_string() {
                    if let None = self.bin_name {
                        self.bin_name = Some(s);
                    }
                }
            }
        }
        self.get_matches_with(&mut matches, &mut it);

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

    fn propogate_globals(&mut self) {
        for (_,sc) in self.subcommands.iter_mut() {
            {
                for a in self.global_args.iter() {
                    sc.add_arg(a.into());
                }
            }
            sc.propogate_globals();
        }
    }


    fn possible_values_error(&self,
                             arg: &str,
                             opt: &str,
                             p_vals: &BTreeSet<&str>,
                             matches: &ArgMatches<'ar, 'ar>) {
        let suffix = App::did_you_mean_suffix(arg, p_vals.iter(),
                                              DidYouMeanMessageStyle::EnumValue);

        self.report_error(format!("'{}' isn't a valid value for '{}'{}{}",
                                    Format::Warning(arg),
                                    Format::Warning(opt),
                                    format!("\n\t[valid values:{}]\n",
                                        p_vals.iter()
                                              .fold(String::new(), |acc, name| {
                                                  acc + &format!(" {}",name)[..]
                                              })),
                                    suffix.0),
                                        true,
                                        Some(matches.args.keys().map(|k| *k).collect()));
    }

    // The actual parsing function
    fn get_matches_with<I, T>(&mut self, matches: &mut ArgMatches<'ar, 'ar>, it: &mut I)
                        where I: Iterator<Item=T>,
                              T: AsRef<str> {
        self.create_help_and_version();

        let mut pos_only = false;
        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: Option<&str> = None;
        let mut pos_counter = 1;
        let mut val_counter = 0;
        while let Some(arg) = it.next() {
            let arg_slice = arg.as_ref();
            let mut skip = false;
            let new_arg = if arg_slice.starts_with("-") {
                if arg_slice.len() == 1 { false } else { true }
            } else {
                false
            };
            if !pos_only && !new_arg && !self.subcommands.contains_key(arg_slice) {
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
                                        self.report_error(format!("The argument '{}' was found, \
                                            but '{}' only expects {} values",
                                                Format::Warning(arg.as_ref()),
                                                Format::Warning(opt.to_string()),
                                                Format::Good(vals.len().to_string())),
                                            true,
                                            Some(
                                                matches.args.keys().map(|k| *k).collect()
                                            )
                                        );
                                    }
                                }
                            }
                        }

                        if !opt.empty_vals &&
                            matches.args.contains_key(opt.name) &&
                            arg_slice.is_empty() {
                            self.report_error(format!("The argument '{}' does not allow empty \
                                    values, but one was found.", Format::Warning(opt.to_string())),
                                true,
                                Some(matches.args.keys()
                                                 .map(|k| *k).collect()));
                        }
                        if let Some(ref mut o) = matches.args.get_mut(opt.name) {
                            // Options have values, so we can unwrap()
                            if let Some(ref mut vals) = o.values {
                                let len = vals.len() as u8 + 1;
                                vals.insert(len, arg_slice.to_owned());
                            }

                            // if it's multiple the occurrences are increased when originall found
                            o.occurrences = if opt.multiple {
                                o.occurrences + 1
                            } else {
                                skip = true;
                                1
                            };
                            if let Some(ref vals) = o.values {
                                let len = vals.len() as u8;
                                if let Some(num) = opt.max_vals {
                                    if len != num { continue }
                                } else if let Some(num) = opt.num_vals {
                                    if opt.multiple {
                                        val_counter += 1;
                                        if val_counter != num { 
                                            continue 
                                        } else {
                                            val_counter = 0;
                                        }
                                    } else {
                                        if len != num { continue }
                                    }
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
                            format!("The argument '{}' requires a value but none was supplied",
                                Format::Warning(o.to_string())),
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
                needs_val_of = self.parse_long_arg(matches, arg_slice);
            } else if arg_slice.starts_with("-") && arg_slice.len() != 1 && ! pos_only {
                needs_val_of = self.parse_short_arg(matches, arg_slice);
            } else {
                // Positional or Subcommand
                // If the user pased `--` we don't check for subcommands, because the argument they
                // may be trying to pass might match a subcommand name
                if !pos_only {
                    if self.subcommands.contains_key(arg_slice) {
                        if arg_slice == "help" {
                            self.print_help();
                        }
                        subcmd_name = Some(arg_slice.to_owned());
                        break;
                    }

                    if let Some(candidate_subcommand) = did_you_mean(arg_slice,
                                                                     self.subcommands.keys()) {
                        self.report_error(
                            format!("The subcommand '{}' isn't valid\n\tDid you mean '{}' ?\n\n\
                            If you received this message in error, try \
                            re-running with '{} {} {}'",
                                Format::Warning(arg.as_ref()),
                                Format::Good(candidate_subcommand),
                                self.bin_name.clone().unwrap_or(self.name.clone()),
                                Format::Good("--"),
                                arg_slice),
                            true,
                            None);
                    }
                }

                if self.positionals_idx.is_empty() {
                    self.report_error(
                        format!("Found argument '{}', but {} wasn't expecting any",
                            Format::Warning(arg.as_ref()),
                            self.bin_name.clone().unwrap_or(self.name.clone())),
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
                            Format::Warning(p.to_string()),
                            match self.blacklisted_from(p.name, &matches) {
                                Some(name) => format!("'{}'", Format::Warning(name)),
                                None       => "one or more of the other specified \
                                               arguments".to_owned()
                            }),
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
                                        self.report_error(format!("The argument '{}' was found, \
                                            but '{}' wasn't expecting any more values",
                                                Format::Warning(arg.as_ref()),
                                                Format::Warning(p.to_string())),
                                            true,
                                            Some(matches.args.keys()
                                                             .map(|k| *k).collect()));
                                    }
                                }
                            }
                        }
                        if !p.empty_vals && matches.args.contains_key(p.name)
                            && arg_slice.is_empty()  {
                            self.report_error(format!("The argument '{}' does not allow empty \
                                    values, but one was found.", Format::Warning(p.to_string())),
                                true,
                                Some(matches.args.keys()
                                                 .map(|k| *k).collect()));
                        }
                        // Check if it's already existing and update if so...
                        if let Some(ref mut pos) = matches.args.get_mut(p.name) {
                            done = true;
                            pos.occurrences += 1;
                            if let Some(ref mut vals) = pos.values {
                                let len = (vals.len() + 1) as u8;
                                vals.insert(len, arg_slice.to_owned());
                            }
                        }
                    } else {
                        // Only increment the positional counter if it doesn't allow multiples
                        pos_counter += 1;
                    }
                    // Was an update made, or is this the first occurrence?
                    if !done {
                        let mut bm = BTreeMap::new();
                        if !p.empty_vals && arg_slice.is_empty() {
                            self.report_error(format!("The argument '{}' does not allow empty \
                                values, but one was found.", Format::Warning(p.to_string())),
                                true,
                                Some(matches.args.keys()
                                                 .map(|k| *k).collect()));
                        }
                        bm.insert(1, arg_slice.to_owned());
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
                    self.report_error(format!("The argument '{}' was found, but '{}' wasn't \
                        expecting any", Format::Warning(arg.as_ref()),
                            self.bin_name.clone().unwrap_or(self.name.clone())),
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
                                format!("The argument '{}' requires a value but there wasn't any \
                                supplied", Format::Warning(o.to_string())),
                                true,
                                Some(matches.args.keys().map(|k| *k).collect() ) );
                        }
                    }
                    else if !o.multiple {
                        self.report_error(
                            format!("The argument '{}' requires a value but none was supplied",
                                Format::Warning(o.to_string())),
                            true,
                            Some(matches.args.keys().map(|k| *k).collect() ) );
                    }
                    else {
                        self.report_error(format!("The following required arguments were not \
                            supplied:{}",
                            self.get_required_from(self.required.iter()
                                                                .map(|s| *s)
                                                                .collect::<Vec<_>>())
                                .iter()
                                .fold(String::new(), |acc, s| acc + &format!("\n\t'{}'",
                                    Format::Error(s.to_string()))[..])),
                            true,
                            Some(matches.args.keys().map(|k| *k).collect()));
                    }
                } else {
                    self.report_error(
                        format!("The argument '{}' requires a value but none was supplied",
                            Format::Warning(format!("{}", self.positionals_idx.get(
                                self.positionals_name.get(a).unwrap()).unwrap()))),
                            true,
                            Some(matches.args.keys().map(|k| *k).collect()));
                }
            }
            _ => {}
        }

        self.validate_blacklist(matches);
        self.validate_num_args(matches);


        matches.usage = Some(self.create_usage(None));

        if let Some(sc_name) = subcmd_name {
            use ::std::fmt::Write;
            let mut mid_string = String::new();
            if !self.subcmds_neg_reqs {
                let mut hs = self.required.iter().map(|n| *n).collect::<Vec<_>>();
                matches.args.keys().map(|k| hs.push(*k)).collect::<Vec<_>>();
                let reqs = self.get_required_from(hs);

                for s in reqs.iter() {
                    write!(&mut mid_string, " {}", s).ok().expect(INTERNAL_ERROR_MSG);
                }
            }
            mid_string.push_str(" ");
            if let Some(ref mut sc) = self.subcommands.get_mut(&sc_name) {
                let mut new_matches = ArgMatches::new();
                // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by a
                // space
                sc.usage = Some(format!("{}{}{}",
                    self.bin_name.clone().unwrap_or("".to_owned()),
                    if self.bin_name.is_some() {
                        mid_string
                    } else {
                        "".to_owned()
                    },
                    sc.name.clone()));
                sc.bin_name = Some(format!("{}{}{}",
                    self.bin_name.clone().unwrap_or("".to_owned()),
                    if self.bin_name.is_some() {
                        " "
                    } else {
                        ""
                    },
                    sc.name.clone()));
                sc.get_matches_with(&mut new_matches, it);
                matches.subcommand = Some(Box::new(SubCommand {
                    name: sc.name_slice,
                    matches: new_matches
                }));
            }
        } else if self.no_sc_error {
            let bn = self.bin_name.clone().unwrap_or(self.name.clone());
            self.report_error(format!("'{}' requires a subcommand but none was provided",
                    Format::Warning(&bn[..])),
                true,
                Some(matches.args.keys().map(|k| *k).collect()));
        } else if self.help_on_no_sc {
            self.print_help();
            self.exit(1);
        }
        if !self.required.is_empty() && !self.subcmds_neg_reqs {
            if self.validate_required(&matches) {
                self.report_error(format!("The following required arguments were not \
                    supplied:{}",
                    self.get_required_from(self.required.iter()
                                                        .map(|s| *s)
                                                        .collect::<Vec<_>>())
                        .iter()
                        .fold(String::new(), |acc, s| acc + &format!("\n\t'{}'",
                            Format::Error(s))[..])),
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }
        }
        if matches.args.is_empty() && matches.subcommand_name().is_none() && self.help_on_no_args {
            self.print_help();
            self.exit(1);
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
            if self.help_short.is_none() && !self.short_list.contains(&'h') {
                self.help_short = Some('h');
            }
            let arg = FlagBuilder {
                name: "hclap_help",
                short: self.help_short,
                long: Some("help"),
                help: Some("Prints help information"),
                blacklist: None,
                multiple: false,
                global: false,
                requires: None,
            };
            self.long_list.insert("help");
            self.flags.insert("hclap_help", arg);
        }
        if self.needs_long_version 
            && self.versionless_scs.is_none() 
            || (self.versionless_scs.unwrap()) {
            if self.version_short.is_none() && !self.short_list.contains(&'V') {
                self.version_short = Some('V');
            }
            // name is "vclap_version" because flags are sorted by name
            let arg = FlagBuilder {
                name: "vclap_version",
                short: self.version_short,
                long: Some("version"),
                help: Some("Prints version information"),
                blacklist: None,
                multiple: false,
                global: false,
                requires: None,
            };
            self.long_list.insert("version");
            self.flags.insert("vclap_version", arg);
        }
        if self.needs_subcmd_help && !self.subcommands.is_empty() {
            self.subcommands.insert("help".to_owned(), App::new("help")
                                                            .about("Prints this message"));
        }
    }

    fn check_for_help_and_version(&self, arg: char) {
        if let Some(h) = self.help_short {
            if h == arg { self.print_help(); }
        }
        if let Some(v) = self.version_short {
            if v == arg { self.print_version(true); }
        }
    }

    fn parse_long_arg(&mut self, matches: &mut ArgMatches<'ar, 'ar> ,full_arg: &str)
                      -> Option<&'ar str> {
        let mut arg = full_arg.trim_left_matches(|c| c == '-');

        if arg == "help" && self.needs_long_help {
            self.print_help();
        } else if arg == "version" && self.needs_long_version {
            self.print_version(true);
        }

        let mut arg_val: Option<String> = None;

        if arg.contains("=") {
            let arg_vec: Vec<_> = arg.split("=").collect();
            arg = arg_vec[0];
            if let Some(ref v) = self.opts.values()
                                      .filter(|&v| v.long.is_some())
                                      .filter(|&v| v.long.unwrap() == arg).nth(0) {
                // prevents "--config= value" typo
                if arg_vec[1].len() == 0 && !v.empty_vals {
                    matches.args.insert(v.name, MatchedArg {
                        occurrences: 1,
                        values: None
                    });
                    self.report_error(format!("The argument '{}' requires a value, but none was \
                            supplied", Format::Warning(format!("--{}", arg))),
                        true,
                        Some(matches.args.keys().map(|k| *k).collect()));
                }
                arg_val = Some(arg_vec[1].to_owned());
            }
        }

        if let Some(ref v) = self.opts.values()
                                  .filter(|&v| v.long.is_some())
                                  .filter(|&v| v.long.unwrap() == arg).nth(0) {
            // Ensure this option isn't on the master mutually excludes list
            if self.blacklist.contains(v.name) {
                matches.args.remove(v.name);
                self.report_error(format!("The argument '{}' cannot be used with one or more of \
                    the other specified arguments", Format::Warning(format!("--{}", arg))),
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            if matches.args.contains_key(v.name) {
                if !v.multiple {
                    self.report_error(format!("The argument '{}' was supplied more than once, but \
                            does not support multiple values",
                            Format::Warning(format!("--{}", arg))),
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
                    if !v.empty_vals && arg.is_empty() && matches.args.contains_key(v.name) {
                        self.report_error(format!("The argument '{}' does not allow empty \
                                values, but one was found.", Format::Warning(v.to_string())),
                            true,
                            Some(matches.args.keys()
                                             .map(|k| *k).collect()));
                    }
                    if let Some(ref mut o) = matches.args.get_mut(v.name) {
                        o.occurrences += 1;
                        if let Some(ref mut vals) = o.values {
                            let len = (vals.len() + 1) as u8;
                            vals.insert(len, arg_val.clone().unwrap());
                        }
                    }
                }
            } else {
                if !v.empty_vals && arg_val.is_some() && arg_val.clone().unwrap().is_empty() {
                    self.report_error(format!("The argument '{}' does not allow empty \
                            values, but one was found.", Format::Warning(v.to_string())),
                        true,
                        Some(matches.args.keys()
                                         .map(|k| *k).collect()));
                }
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
                        Format::Warning(v.to_string()),
                        match self.blacklisted_from(v.name, matches) {
                            Some(name) => format!("'{}'", Format::Warning(name)),
                            None       => "one or more of the specified arguments".to_owned()
                        }),
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            // Make sure this isn't one being added multiple times if it doesn't suppor it
            if matches.args.contains_key(v.name) && !v.multiple {
                self.report_error(format!("The argument '{}' was supplied more than once, but does \
                        not support multiple values", Format::Warning(v.to_string())),
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

        self.report_error(format!("The argument '{}' isn't valid{}",
                Format::Warning(format!("--{}", arg)),
                suffix.0),
            true,
            Some(matches.args.keys().map(|k| *k).collect()));

        unreachable!();
    }

    fn parse_short_arg(&mut self, matches: &mut ArgMatches<'ar, 'ar> ,full_arg: &str)
                       -> Option<&'ar str> {
        let arg = &full_arg[..].trim_left_matches(|c| c == '-');
        if arg.len() > 1 {
            // Multiple flags using short i.e. -bgHlS
            for c in arg.chars() {
                self.check_for_help_and_version(c);
                if !self.parse_single_short_flag(matches, c) {
                    self.report_error(format!("The argument '{}' isn't valid",
                            Format::Warning(format!("-{}", c))),
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
                self.report_error(format!("The argument '{}' cannot be used with {}",
                            Format::Warning(format!("-{}", arg)),
                        match self.blacklisted_from(v.name, matches) {
                            Some(name) => format!("'{}'", Format::Warning(name)),
                            None       => "one or more of the other specified arguments".to_owned()
                        }),
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            if matches.args.contains_key(v.name) {
                if !v.multiple {
                    self.report_error(format!("The argument '{}' was supplied more than once, but \
                        does not support multiple values",
                            Format::Warning(format!("-{}", arg))),
                        true,
                        Some(matches.args.keys().map(|k| *k).collect()));
                }
            } else {
                matches.args.insert(v.name, MatchedArg{
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
        self.report_error(format!("The argument '{}' isn't valid",
                            Format::Warning(format!("-{}", arg_c))),
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
                self.report_error(format!("The argument '{}' cannot be used {}",
                            Format::Warning(format!("-{}", arg)),
                        match self.blacklisted_from(v.name, matches) {
                            Some(name) => format!("'{}'", Format::Warning(name)),
                            None       => "with one or more of the other specified \
                                arguments".to_owned()
                        }),
                    true,
                    Some(matches.args.keys().map(|k| *k).collect()));
            }

            // Make sure this isn't one being added multiple times if it doesn't suppor it
            if matches.args.contains_key(v.name) && !v.multiple {
                self.report_error(format!("The argument '{}' was supplied more than once, but does \
                        not support multiple values",
                            Format::Warning(format!("-{}", arg))),
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
                        format!("{}", Format::Warning(flag.to_string()))
                    } else if let Some(ref opt) = self.opts.get(name) {
                        format!("{}", Format::Warning(opt.to_string()))
                    } else {
                        match self.positionals_idx.values().filter(|p| p.name == *name).next() {
                            Some(pos) => format!("{}", Format::Warning(pos.to_string())),
                            None      => format!("\"{}\"", Format::Warning(name))
                        }
                    }, match self.blacklisted_from(name, matches) {
                        Some(name) => format!("'{}'", Format::Warning(name)),
                        None       => "one or more of the other specified arguments".to_owned()
                    }),
                true,
                Some(matches.args.keys().map(|k| *k).collect()));
            } else if self.groups.contains_key(name) {
                for n in self.get_group_members_names(name) {
                    if matches.args.contains_key(n) {
                        matches.args.remove(n);
                        self.report_error(format!("The argument '{}' cannot be used with one or \
                                more of the other specified arguments",
                                if let Some(ref flag) = self.flags.get(n) {
                                    format!("{}", Format::Warning(flag.to_string()))
                                } else if let Some(ref opt) = self.opts.get(n) {
                                    format!("{}", Format::Warning(opt.to_string()))
                                } else {
                                    match self.positionals_idx.values()
                                                              .filter(|p| p.name == *name)
                                                              .next() {
                                        Some(pos) => format!("{}", Format::Warning(pos.to_string())),
                                        None      => format!("\"{}\"", Format::Warning(n))
                                    }
                                }),
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
                                    Format::Warning(f.to_string()),
                                    Format::Good(num.to_string()),
                                    Format::Error(if f.multiple {
                                        (vals.len() % num as usize).to_string()
                                    } else {
                                        vals.len().to_string()
                                    }),
                                    if vals.len() == 1 ||
                                        ( f.multiple &&
                                            ( vals.len() % num as usize) == 1) {"as"}else{"ere"}),
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.max_vals {
                        if (vals.len() as u8) > num {
                            self.report_error(format!("The argument '{}' requires no more than {} \
                                    values, but {} w{} provided",
                                    Format::Warning(f.to_string()),
                                    Format::Good(num.to_string()),
                                    Format::Error(vals.len().to_string()),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.min_vals {
                        if (vals.len() as u8) < num {
                            self.report_error(format!("The argument '{}' requires at least {} \
                                    values, but {} w{} provided",
                                    Format::Warning(f.to_string()),
                                    Format::Good(num.to_string()),
                                    Format::Error(vals.len().to_string()),
                                    if vals.len() == 1 {"as"}else{"ere"}),
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
                                    Format::Warning(f.to_string()),
                                    Format::Good(num.to_string()),
                                    Format::Error(vals.len().to_string()),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.max_vals {
                        if num > vals.len() as u8 {
                            self.report_error(format!("The argument '{}' requires no more than {} \
                                    values, but {} w{} provided",
                                    Format::Warning(f.to_string()),
                                    Format::Good(num.to_string()),
                                    Format::Error(vals.len().to_string()),
                                    if vals.len() == 1 {"as"}else{"ere"}),
                                true,
                                Some(matches.args.keys().map(|k| *k).collect()));
                        }
                    }
                    if let Some(num) = f.min_vals {
                        if num < vals.len() as u8 {
                            self.report_error(format!("The argument '{}' requires at least {} \
                                    values, but {} w{} provided",
                                    Format::Warning(f.to_string()),
                                    Format::Good(num.to_string()),
                                    Format::Error(vals.len().to_string()),
                                    if vals.len() == 1 {"as"}else{"ere"}),
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

    /// Returns a suffix that can be empty, or is the standard 'did you mean phrase
    fn did_you_mean_suffix<'z, T, I>(arg: &str, values: I, style: DidYouMeanMessageStyle)
                                                     -> (String, Option<&'z str>)
                                                        where       T: AsRef<str> + 'z,
                                                                    I: IntoIterator<Item=&'z T> {
        match did_you_mean(arg, values) {
                Some(candidate) => {
                    let mut suffix = "\n\tDid you mean ".to_string();
                    match style {
                        DidYouMeanMessageStyle::LongFlag => suffix.push_str(&Format::Good("--").to_string()[..]),
                        DidYouMeanMessageStyle::EnumValue => suffix.push('\''),
                    }
                    suffix.push_str(&Format::Good(candidate).to_string()[..]);
                    if let DidYouMeanMessageStyle::EnumValue = style {
                        suffix.push('\'');
                    }
                    suffix.push_str(" ?");
                    (suffix, Some(candidate))
                },
                None => (String::new(), None),
        }
    }
}
