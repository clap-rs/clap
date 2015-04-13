use usageparser::{UsageParser, UsageToken};

/// The abstract representation of a command line argument used by the consumer of the library.
/// Used to set all the options and relationships that define a valid argument for the program.
/// 
///
/// This struct is used by the library consumer and describes the command line arguments for 
/// their program. Then evaluates the settings the consumer provided and determines the concret
/// argument type to use when parsing.
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg};
/// # let matches = App::new("myprog")
/// #                 .arg(
/// Arg::new("conifg")
///       .short("c")
///       .long("config")
///       .takes_value(true)
///       .help("Provides a config file to myprog")
/// # ).get_matches();
pub struct Arg<'n, 'l, 'h, 'b, 'p, 'r> {
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
    pub blacklist: Option<Vec<&'b str>>, 
    /// A list of possible values for an option or positional argument
    #[doc(hidden)]
    pub possible_vals: Option<Vec<&'p str>>,
    /// A list of names of other arguments that are *required* to be used when 
    /// this flag is used
    #[doc(hidden)]
    pub requires: Option<Vec<&'r str>>
}

impl<'n, 'l, 'h, 'b, 'p, 'r> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
    /// Creates a new instace of `Arg` using a unique string name. 
    /// The name will be used by the library consumer to get information about
    /// whether or not the argument was used at runtime. 
    ///
    /// **NOTE:** in the case of arguments that take values (i.e. `takes_value(true)`)
    /// and positional arguments (i.e. those without a `-` or `--`) the name will also 
    /// be displayed when the user prints the usage/help information of the program.
    ///
    /// **NOTE:** this function is deprecated in favor of Arg::with_name() to stay in line with
    /// Rust APIs
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// Arg::new("conifg")
    /// # .short("c")
    /// # ).get_matches();
    #[deprecated(since  = "0.5.15",
                 reason = "use Arg::with_name() instead")]
    pub fn new(n: &'n str) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
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
        }
    }

    /// Creates a new instace of `Arg` using a unique string name. 
    /// The name will be used by the library consumer to get information about
    /// whether or not the argument was used at runtime. 
    ///
    /// **NOTE:** in the case of arguments that take values (i.e. `takes_value(true)`)
    /// and positional arguments (i.e. those without a `-` or `--`) the name will also 
    /// be displayed when the user prints the usage/help information of the program.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// Arg::with_name("conifg")
    /// # .short("c")
    /// # ).get_matches();
    pub fn with_name(n: &'n str) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
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
        }
    }

    pub fn from_usage(u: &'n str) -> Arg<'n, 'n, 'n, 'b, 'p, 'r> {
        assert!(u.len() > 0, "Arg::from_usage() requires a non-zero-length usage string but none was provided");

         let mut name = None;
         let mut short = None;
         let mut long = None;
         let mut help = None;
         let mut required = false;
         let mut takes_value = false;
         let mut multiple = false;
        
        let parser = UsageParser::with_usage(u);
        for_match!{ parser,
            UsageToken::Name(n, req) => {
                if name.is_none() {
                    name = Some(n);
                    if let Some(m) = req {
                        required = m;
                    }
                } 
                if short.is_some() || long.is_some() {
                    takes_value = true;
                }
                if let Some(l) = long {
                    if n != name.unwrap() && name.unwrap() == l {
                        name = Some(n);
                        if let Some(m) = req {
                            required = m;
                        }
                    }
                }   
            },
            UsageToken::Short(s)     => { 
                short = Some(s); 
            },
            UsageToken::Long(l)      => { 
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
        }
    }

    /// Sets the short version of the argument without the preceding `-`.
    ///
    ///
    /// By default `clap` automatically assigns `v` and `h` to display version and help information 
    /// respectivly. You may use `v` or `h` for your own purposes, in which case `clap` simply
    /// will not asign those to the displaying of version or help.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version, i.e. for when the user
    /// mistakenly sets the short to `-o` or the like.
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("conifg")
    /// .short("c")
    /// # ).get_matches();
    pub fn short(mut self, s: &str) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        self.short = s.trim_left_matches(|c| c == '-').chars().nth(0);
        self
    }

    /// Sets the long version of the argument without the preceding `--`.
    ///
    /// By default `clap` automatically assigns `version` and `help` to display version and help information 
    /// respectivly. You may use `version` or `help` for your own purposes, in which case `clap` simply
    /// will not asign those to the displaying of version or help automatically, and you will have to do
    /// so manually.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped i.e. for 
    /// when the user mistakenly sets the short to `--out` or the like.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("conifg")
    /// .long("config")
    /// # ).get_matches();
    pub fn long(mut self, l: &'l str) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        self.long = Some(l.trim_left_matches(|c| c == '-'));
        self
    }

    /// Sets the help text of the argument that will be displayed to the user
    /// when they print the usage/help information. 
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("conifg")
    /// .help("The config file used by the myprog")
    /// # ).get_matches();
    pub fn help(mut self, h: &'h str) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
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
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("conifg")
    /// .required(true)
    /// # ).get_matches();
    pub fn required(mut self, r: bool) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
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
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::new("conifg")
    /// .mutually_excludes("debug")
    /// # ).get_matches();
    pub fn mutually_excludes(mut self, name: &'b str) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        if let Some(ref mut vec) = self.blacklist {
            vec.push(name);
        } else {
            self.blacklist = Some(vec![name]);
        }
        self
    }

    /// Sets a mutually exclusive arguments by names. I.e. when using this argument, 
    /// the following argument can't be present.
    ///
    /// **NOTE:** Mutually exclusive rules take precedence over being required
    /// by default. Mutually exclusive rules only need to be set for one of the two
    /// arguments, they do not need to be set for each.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::new("conifg")
    /// .mutually_excludes_all(
    ///        vec!["debug", "input"])
    /// # ).get_matches();
    pub fn mutually_excludes_all(mut self, names: Vec<&'b str>) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        if let Some(ref mut vec) = self.blacklist {
            for n in names {
                vec.push(n);
            }
        } else {
            self.blacklist = Some(names);
        }
        self
    }

    /// Sets an argument by name that is required when this one is presnet I.e. when
    /// using this argument, the following argument *must* be present.
    ///
    /// **NOTE:** Mutually exclusive rules take precedence over being required
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::new("conifg")
    /// .requires("debug")
    /// # ).get_matches();
    pub fn requires(mut self, name: &'r str) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
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
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::new("conifg")
    /// .requires_all(
    ///        vec!["debug", "input"])
    /// # ).get_matches();
    pub fn requires_all(mut self, names: Vec<&'r str>) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        if let Some(ref mut vec) = self.requires {
            for n in names {
                vec.push(n);
            }
        } else {
            self.requires = Some(names);
        }
        self
    }

    /// Specifies that the argument takes an additional value at run time.
    /// 
    /// **NOTE:** When setting this to `true` the `name` of the argument
    /// will be used when printing the help/usage information to the user. 
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("conifg")
    /// .takes_value(true)
    /// # ).get_matches();
    pub fn takes_value(mut self, tv: bool) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
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
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("conifg")
    /// .index(1)
    /// # ).get_matches();
    pub fn index(mut self, idx: u8) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        self.index = Some(idx);
        self
    }

    /// Specifies if the flag may appear more than once such as for multiple debugging
    /// levels (as an example). `-ddd` for three levels of debugging, or `-d -d -d`. 
    /// When this is set to `true` you recieve the number of occurances the user supplied
    /// of a particular flag at runtime.
    /// 
    /// **NOTE:** When setting this,  any `takes_value` or `index` values you set
    /// are ignored as flags cannot have a values or an `index`.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("debug")
    /// .multiple(true)
    /// # ).get_matches();
    pub fn multiple(mut self, multi: bool) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        self.multiple = multi;
        self
    }

    /// Specifies a list of possible values for this argument. At runtime, clap verifies that only
    /// one of the specified values was used, or fails with a usage string.
    /// 
    /// **NOTE:** This setting only applies to options and positional arguments 
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::new("debug").index(1)
    /// .possible_values(vec!["fast", "slow"])
    /// # ).get_matches();
    pub fn possible_values(mut self, names: Vec<&'p str>) -> Arg<'n, 'l, 'h, 'b, 'p, 'r> {
        if let Some(ref mut vec) = self.possible_vals {
            for n in names {
                vec.push(n);
            }
        } else {
            self.possible_vals = Some(names);
        }
        self
    }
}
