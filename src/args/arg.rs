use std::iter::IntoIterator;
use std::collections::HashSet;

use usageparser::{UsageParser, UsageToken};

/// The abstract representation of a command line argument used by the consumer of the library.
/// Used to set all the options and relationships that define a valid argument for the program.
///
/// This struct is used by the library consumer and describes the command line arguments for
/// their program. Then evaluates the settings the consumer provided and determines the concret
/// argument type to use when parsing.
///
/// There are two methods for constructing `Arg`s, using the builder pattern and setting options
/// manually, or using a usage string which is far less verbose. You can also use a combination
/// of the two methods to achieve the best of both worlds.
///
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg};
/// # let matches = App::new("myprog")
/// #                 .arg(
/// // Using the traditional builder pattern and setting each option manually
/// Arg::with_name("conifg")
///       .short("c")
///       .long("config")
///       .takes_value(true)
///       .help("Provides a config file to myprog")
/// # ).arg(
/// // Using a usage string (setting a similar argument to the one above)
/// Arg::from_usage("-i --input=[input] 'Provides an input file to the program'")
/// # ).get_matches();
pub struct Arg<'n, 'l, 'h, 'g, 'p, 'r> {
    /// The unique name of the argument, required
    #[doc(hidden)]
    pub name: &'n str,
    /// The short version (i.e. single character) of the argument, no preceding `-`
    /// **NOTE:** `short` is mutually exclusive with `index`
    #[doc(hidden)]
    pub short: Option<char>,
    /// The long version of the flag (i.e. word) without the preceding `--`
    /// **NOTE:** `long` is mutually exclusive with `index`
    #[doc(hidden)]
    pub long: Option<&'l str>,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    #[doc(hidden)]
    pub help: Option<&'h str>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    #[doc(hidden)]
    pub required: bool,
    /// Determines if this argument is an option, vice a flag or positional and
    /// is mutually exclusive with `index` and `multiple`
    #[doc(hidden)]
    pub takes_value: bool,
    /// The index of the argument. `index` is mutually exclusive with `takes_value`
    /// and `multiple`
    #[doc(hidden)]
    pub index: Option<u8>,
    /// Determines if multiple instances of the same flag are allowed. `multiple`
    /// is mutually exclusive with `index` and `takes_value`.
    /// I.e. `-v -v -v` or `-vvv`
    #[doc(hidden)]
    pub multiple: bool,
    /// A list of names for other arguments that *may not* be used with this flag
    #[doc(hidden)]
    pub blacklist: Option<Vec<&'r str>>,
    /// A list of possible values for an option or positional argument
    #[doc(hidden)]
    pub possible_vals: Option<Vec<&'p str>>,
    /// A list of names of other arguments that are *required* to be used when
    /// this flag is used
    #[doc(hidden)]
    pub requires: Option<Vec<&'r str>>,
    /// A name of the group the argument belongs to
    #[doc(hidden)]
    pub group: Option<&'g str>,
    #[doc(hidden)]
    pub val_names: Option<Vec<&'n str>>,
    #[doc(hidden)]
    pub num_vals: Option<u8>,
    #[doc(hidden)]
    pub max_vals: Option<u8>,
    #[doc(hidden)]
    pub min_vals: Option<u8>,
    #[doc(hidden)]
    pub empty_vals: bool,
    #[doc(hidden)]
    pub global: bool
}

impl<'n, 'l, 'h, 'g, 'p, 'r> Arg<'n, 'l, 'h, 'g, 'p, 'r> {
    /// Creates a new instace of `Arg` using a unique string name.
    /// The name will be used by the library consumer to get information about
    /// whether or not the argument was used at runtime.
    ///
    /// **NOTE:** in the case of arguments that take values (i.e. `takes_value(true)`)
    /// and positional arguments (i.e. those without a `-` or `--`) the name will also
    /// be displayed when the user prints the usage/help information of the program.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// Arg::with_name("conifg")
    /// # .short("c")
    /// # ).get_matches();
    pub fn with_name(n: &'n str) -> Self {
        Arg {
            name: n,
            short: None,
            long: None,
            help: None,
            required: false,
            takes_value: false,
            multiple: false,
            index: None,
            possible_vals: None,
            blacklist: None,
            requires: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            val_names: None,
            group: None,
            global: false,
            empty_vals: true
        }
    }

    /// Creates a new instace of `Arg` from a usage string. Allows creation of basic settings
    /// for Arg (i.e. everything except relational rules). The syntax is flexible, but there are
    /// some rules to follow.
    ///
    /// **NOTE**: only properties which you wish to set must be present
    ///
    /// 1. Name (arguments with a `long` or that take a value can ommit this if desired),
    ///    use `[]` for non-required arguments, or `<>` for required arguments.
    /// 2. Short preceded by a `-`
    /// 3. Long preceded by a `--` (this may be used as the name, if the name is omitted. If the
    ///    name is *not* omittied, the name takes precedence over the `long`)
    /// 4. Value (this can be used as the name if the name is not manually specified. If the name
    ///    is manually specified, it takes precedence. If this value is used as the name, it uses
    ///    the same `[]` and `<>` requirement specification rules. If it is *not* used as the name,
    ///    it still needs to be surrounded by either `[]` or `<>` but there is no requirement
    ///    effect, as the requirement rule is determined by the real name. This value may follow
    ///    the `short` or `long`, it doesn't matter. If it follows the `long`, it may follow either
    ///    a `=` or ` ` there is no difference, just personal preference. If this follows a `short`
    ///    it can only be after a ` `) i.e. `-c [name]`, `--config [name]`, `--config=[name]`, etc.
    /// 5. Multiple specifier `...` (the `...` may follow the name, `short`, `long`, or value
    ///    *without* a ` ` space) i.e. `<name>... -c`, `--config <name>...`, `[name] -c...`, etc.
    /// 6. The help info surrounded by `'`s (single quotes)
    /// 7. The index of a positional argument will be the next available index (you don't need to
    ///    specify one) i.e. all arguments without a `short` or `long` will be treated as
    ///    positional
    /// 8. If the value names are all the same, and their multiple ones (i.e `-o <val> <val>`)
    ///    they are counted and used as the number of values. If they are different, they are used
    ///    as the value names (i.e. `--opt <file> <mode>`). In this case, if no name was specified
    ///    prior to the value names, the long is used as the name by which to access the argument.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    ///                  .args(vec![
    ///
    /// // A option argument with a long, named "conf" (note: because the name was specified
    /// // the portion after the long can be called anything, only the first name will be displayed
    /// // to the user. Also, requirement is set with the *name*, so the portion after the long
    /// // could be either <> or [] and it wouldn't matter, so long as it's one of them. Had the
    /// // name been omitted, the name would have been derived from the portion after the long and
    /// // those rules would have mattered)
    /// Arg::from_usage("[conf] --config=[c] 'a required file for the configuration'"),
    ///
    /// // A flag with a short, a long, named "debug", and accepts multiple values
    /// Arg::from_usage("-d --debug... 'turns on debugging information"),
    ///
    /// // A required positional argument named "input"
    /// Arg::from_usage("<input> 'the input file to use'")
    /// ])
    /// # .get_matches();
    pub fn from_usage(u: &'n str) -> Arg<'n, 'n, 'n, 'g, 'p, 'r> {
        assert!(u.len() > 0, "Arg::from_usage() requires a non-zero-length usage string but none \
            was provided");

         let mut name = None;
         let mut short = None;
         let mut long = None;
         let mut help = None;
         let mut required = false;
         let mut takes_value = false;
         let mut multiple = false;
         let mut num_names = 1;
         let mut name_first = false;
         let mut consec_names = false;
         let mut val_names = HashSet::new();

        let parser = UsageParser::with_usage(u);
        for_match!{ parser,
            UsageToken::Name(n, req) => {
                if consec_names {
                    num_names += 1;
                }
                let mut use_req = false;
                let mut use_name = false;
                if name.is_none() && long.is_none() && short.is_none() {
                    name_first = true;
                    use_name = true;
                    use_req = true;
                } else if let Some(l) = long {
                    if l == name.unwrap_or("") {
                        if !name_first {
                            use_name = true;
                            use_req = true;
                        }
                    }
                } else {
                    // starting with short
                    if !name_first {
                        use_name = true;
                        use_req = true;
                    }
                }
                if use_name && !consec_names {
                    name = Some(n);
                }
                if use_req && !consec_names {
                    if let Some(r) = req {
                        required = r;
                    }
                }
                if short.is_some() || long.is_some() {
                    val_names.insert(n);
                    takes_value = true;
                }
                consec_names = true;
            },
            UsageToken::Short(s)     => {
                consec_names = false;
                short = Some(s);
            },
            UsageToken::Long(l)      => {
                consec_names = false;
                long = Some(l);
                if name.is_none() {
                    name = Some(l);
                }
            },
            UsageToken::Help(h)      => {
                help = Some(h);
            },
            UsageToken::Multiple     => {
                multiple = true;
            }
        }

        if let Some(l) = long {
            val_names.remove(l);
            if val_names.len() > 1 {
                if name.unwrap() != l && !name_first {
                    name = Some(l);
                }
            }
        }

        Arg {
            name: name.unwrap(),
            short: short,
            long: long,
            help: help,
            required: required,
            takes_value: takes_value,
            multiple: multiple,
            index: None,
            possible_vals: None,
            blacklist: None,
            requires: None,
            num_vals: if num_names > 1 { Some(num_names) } else { None },
            val_names: if val_names.len() > 1 {Some(val_names.iter().map(|s| *s).collect::<Vec<_>>())}else{None},
            max_vals: None,
            min_vals: None,
            group: None,
            global: false,
            empty_vals: true
        }
    }

    /// Sets the short version of the argument without the preceding `-`.
    ///
    ///
    /// By default `clap` automatically assigns `v` and `h` to display version and help information
    /// respectively. You may use `v` or `h` for your own purposes, in which case `clap` simply
    /// will not assign those to the displaying of version or help.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("conifg")
    /// .short("c")
    /// # ).get_matches();
    pub fn short(mut self, s: &str) -> Self {
        self.short = s.trim_left_matches(|c| c == '-').chars().nth(0);
        self
    }

    /// Sets the long version of the argument without the preceding `--`.
    ///
    /// By default `clap` automatically assigns `version` and `help` to display version and help
    /// information respectively. You may use `version` or `help` for your own purposes, in which
    /// case `clap` simply will not assign those to the displaying of version or help automatically,
    /// and you will have to do so manually.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("conifg")
    /// .long("config")
    /// # ).get_matches();
    pub fn long(mut self, l: &'l str) -> Self {
        self.long = Some(l.trim_left_matches(|c| c == '-'));
        self
    }

    /// Sets the help text of the argument that will be displayed to the user
    /// when they print the usage/help information.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("conifg")
    /// .help("The config file used by the myprog")
    /// # ).get_matches();
    pub fn help(mut self, h: &'h str) -> Self {
        self.help = Some(h);
        self
    }

    /// Sets whether or not the argument is required by default. Required by
    /// default means it is required, when no other mutually exlusive rules have
    /// been evaluated. Mutually exclusive rules take precedence over being required
    /// by default.
    ///
    /// **NOTE:** Flags (i.e. not positional, or arguments that take values)
    /// cannot be required by default.
    /// when they print the usage/help information.
    ///
    ///
    /// #Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("conifg")
    /// .required(true)
    /// # ).get_matches();
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }

    /// Sets a mutually exclusive argument by name. I.e. when using this argument,
    /// the following argument can't be present.
    ///
    /// **NOTE:** Mutually exclusive rules take precedence over being required
    /// by default. Mutually exclusive rules only need to be set for one of the two
    /// arguments, they do not need to be set for each.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::with_name("conifg")
    /// .conflicts_with("debug")
    /// # ).get_matches();
    pub fn conflicts_with(mut self, name: &'r str) -> Self {
        if let Some(ref mut vec) = self.blacklist {
            vec.push(name);
        } else {
            self.blacklist = Some(vec![name]);
        }
        self
    }

    /// Sets mutually exclusive arguments by names. I.e. when using this argument,
    /// the following argument can't be present.
    ///
    /// **NOTE:** Mutually exclusive rules take precedence over being required
    /// by default. Mutually exclusive rules only need to be set for one of the two
    /// arguments, they do not need to be set for each.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let config_conflicts = ["debug", "input"];
    /// # let myprog = App::new("myprog").arg(Arg::with_name("conifg")
    /// .conflicts_with_all(&config_conflicts)
    /// # ).get_matches();
    pub fn conflicts_with_all<T, I>(mut self, names: I)
                                    -> Self
                                    where T: AsRef<str> + 'r,
                                          I: IntoIterator<Item=&'r T> {
        if let Some(ref mut vec) = self.blacklist {
            names.into_iter().map(|s| vec.push(s.as_ref())).collect::<Vec<_>>();
        } else {
            self.blacklist = Some(names.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        }
        self
    }

    /// Sets an argument by name that is required when this one is presnet I.e. when
    /// using this argument, the following argument *must* be present.
    ///
    /// **NOTE:** Mutually exclusive rules take precedence over being required
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::with_name("conifg")
    /// .requires("debug")
    /// # ).get_matches();
    pub fn requires(mut self, name: &'r str) -> Self {
        if let Some(ref mut vec) = self.requires {
            vec.push(name);
        } else {
            self.requires = Some(vec![name]);
        }
        self
    }

    /// Sets arguments by names that are required when this one is presnet I.e. when
    /// using this argument, the following arguments *must* be present.
    ///
    /// **NOTE:** Mutually exclusive rules take precedence over being required
    /// by default.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let config_reqs = ["debug", "input"];
    /// # let myprog = App::new("myprog").arg(Arg::with_name("conifg")
    /// .requires_all(&config_reqs)
    /// # ).get_matches();
    pub fn requires_all<T, I>(mut self, names: I)
                              -> Self
                              where T: AsRef<str> + 'r,
                                    I: IntoIterator<Item=&'r T> {
        if let Some(ref mut vec) = self.requires {
            names.into_iter().map(|s| vec.push(s.as_ref())).collect::<Vec<_>>();
        } else {
            self.requires = Some(names.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        }
        self
    }

    /// Specifies that the argument takes an additional value at run time.
    ///
    /// **NOTE:** When setting this to `true` the `name` of the argument
    /// will be used when printing the help/usage information to the user.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("conifg")
    /// .takes_value(true)
    /// # ).get_matches();
    pub fn takes_value(mut self, tv: bool) -> Self {
        self.takes_value = tv;
        self
    }

    /// Specifies the index of a positional argument starting at 1.
    ///
    /// **NOTE:** When setting this,  any `short` or `long` values you set
    /// are ignored as positional arguments cannot have a `short` or `long`.
    /// Also, the name will be used when printing the help/usage information
    /// to the user.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("conifg")
    /// .index(1)
    /// # ).get_matches();
    pub fn index(mut self, idx: u8) -> Self {
        self.index = Some(idx);
        self
    }

    /// Specifies if the flag may appear more than once such as for multiple debugging
    /// levels (as an example). `-ddd` for three levels of debugging, or `-d -d -d`.
    /// When this is set to `true` you receive the number of occurrences the user supplied
    /// of a particular flag at runtime.
    ///
    /// **NOTE:** When setting this,  any `takes_value` or `index` values you set
    /// are ignored as flags cannot have a values or an `index`.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .multiple(true)
    /// # ).get_matches();
    pub fn multiple(mut self, multi: bool) -> Self {
        self.multiple = multi;
        self
    }

    /// Specifies that an argument can be matched to all child subcommands.
    ///
    /// **NOTE:** Global arguments *only* propagate down, **not** up (to parent commands)
    ///
    /// **NOTE:** Global arguments *cannot* be required.
    ///
    /// **NOTE:** Global arguments, when matched, *only* exist in the command's matches that they
    /// were matched to. For example, if you defined a `--flag` global argument in the top most
    /// parent command, but the user supplied the arguments `top cmd1 cmd2 --flag` *only* `cmd2`'s
    /// `ArgMatches` would return `true` if tested for `.is_present("flag")`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .global(true)
    /// # ).get_matches();
    pub fn global(mut self, g: bool) -> Self {
        self.global = g;
        self
    }

    /// Allows an argument to accept explicit empty values. An empty value must be specified at the
    /// command line with an explicit `""`, or `''`
    ///
    /// **NOTE:** Defaults to `true` (Explicit empty values are allowed)
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .empty_values(true)
    /// # ).get_matches();
    pub fn empty_values(mut self, ev: bool) -> Self {
        self.empty_vals = ev;
        self
    }

    /// Specifies a list of possible values for this argument. At runtime, clap verifies that only
    /// one of the specified values was used, or fails with a usage string.
    ///
    /// **NOTE:** This setting only applies to options and positional arguments
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let mode_vals = ["fast", "slow"];
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .possible_values(&mode_vals)
    /// # ).get_matches();
    pub fn possible_values<T, I>(mut self, names: I)
                                 -> Self
                                 where T: AsRef<str> + 'p,
                                       I: IntoIterator<Item=&'p T> {
        if let Some(ref mut vec) = self.possible_vals {
            names.into_iter().map(|s| vec.push(s.as_ref())).collect::<Vec<_>>();
        } else {
            self.possible_vals = Some(names.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        }
        self
    }

    /// Specifies the name of the group the argument belongs to.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .group("mode")
    /// # ).get_matches();
    pub fn group(mut self, name: &'g str) -> Self {
        self.group = Some(name);
        self
    }

    /// Specifies how many values are required to satisfy this argument. For example, if you had a
    /// `-f <file>` argument where you wanted exactly 3 'files' you would set
    /// `.number_of_values(3)`, and this argument wouldn't be satisfied unless the user provided
    /// 3 and only 3 values.
    ///
    /// **NOTE:** Does *not* require `.multiple(true)` to be set. Setting `.multiple(true)` would
    /// allow `-f <file> <file> <file> -f <file> <file> <file>` where as *not* setting
    /// `.multiple(true)` would only allow one occurrence of this argument.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .number_of_values(3)
    /// # ).get_matches();
    pub fn number_of_values(mut self, qty: u8) -> Self {
        self.num_vals = Some(qty);
        self
    }

    /// Specifies the *maximum* number of values are for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted up to 3 'files' you would set
    /// `.max_values(3)`, and this argument would be satisfied if the user provided, 1, 2, or 3
    /// values.
    ///
    /// **NOTE:** `qty` must be > 1
    ///
    /// **NOTE:** This implicity sets `.multiple(true)`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .max_values(3)
    /// # ).get_matches();
    pub fn max_values(mut self, qty: u8) -> Self {
        if qty < 2 {
            panic!("Arguments with max_values(qty) qty must be > 1. Prefer \
                takes_value(true) for arguments with only one value, or flags for arguments \
                with 0 values.");
        }
        self.max_vals = Some(qty);
        self.multiple = true;
        self
    }

    /// Specifies the *minimum* number of values are for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted at least 2 'files' you would set
    /// `.min_values(2)`, and this argument would be satisfied if the user provided, 2 or more
    /// values.
    ///
    /// **NOTE:** This implicity sets `.multiple(true)`
    ///
    /// **NOTE:** `qty` must be > 0
    ///
    /// **NOTE:** `qty` *must* be > 0. If you wish to have an argument with 0 or more values prefer
    /// two separate arguments (a flag, and an option with multiple values).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .min_values(2)
    /// # ).get_matches();
    pub fn min_values(mut self, qty: u8) -> Self {
        if qty < 1 {
            panic!("Arguments with min_values(qty) qty must be > 0. Prefer flags for arguments \
                with 0 values.");
        }
        self.min_vals = Some(qty);
        self.multiple = true;
        self
    }

    /// Specifies names for values of option arguments. These names are cosmetic only, used for
    /// help and usage strings only. The names are **not** used to access arguments. The values of
    /// the arguments are accessed in numeric order (i.e. if you specify two names `one` and `two`
    /// `one` will be the first matched value, `two` will be the second).
    ///
    /// **NOTE:** This implicitly sets `.number_of_values()` so there is no need to set that, but
    /// be aware that the number of "names" you set for the values, will be the *exact* number of
    /// values required to satisfy this argument
    ///
    /// **NOTE:** Does *not* require `.multiple(true)` to be set. Setting `.multiple(true)` would
    /// allow `-f <file> <file> <file> -f <file> <file> <file>` where as *not* setting
    /// `.multiple(true)` would only allow one occurrence of this argument.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let val_names = ["one", "two"];
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// // ...
    /// .value_names(&val_names)
    /// # ).get_matches();
    pub fn value_names<T, I>(mut self, names: I)
                                 -> Self
                                 where T: AsRef<str> + 'n,
                                       I: IntoIterator<Item=&'n T> {
        if let Some(ref mut vec) = self.val_names {
            names.into_iter().map(|s| vec.push(s.as_ref())).collect::<Vec<_>>();
        } else {
            self.val_names = Some(names.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        }
        self
    }
}

impl<'n, 'l, 'h, 'g, 'p, 'r, 'z> From<&'z Arg<'n, 'l, 'h, 'g, 'p, 'r>> for Arg<'n, 'l, 'h, 'g, 'p, 'r> {
    fn from(a: &'z Arg<'n, 'l, 'h, 'g, 'p, 'r>) -> Self {
        Arg {
            name: a.name,
            short: a.short,
            long: a.long,
            help: a.help,
            required: a.required,
            takes_value: a.takes_value,
            multiple: a.multiple,
            index: a.index,
            possible_vals: a.possible_vals.clone(),
            blacklist: a.blacklist.clone(),
            requires: a.requires.clone(),
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names.clone(),
            group: a.group,
            global: a.global,
            empty_vals: a.empty_vals 
        }
    }
}