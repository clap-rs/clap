#[cfg(feature = "yaml")]
use std::collections::BTreeMap;
use std::rc::Rc;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;
use vec_map::VecMap;

use usage_parser::UsageParser;
use args::settings::{ArgSettings, ArgFlags};

/// The abstract representation of a command line argument. Used to set all the options and
/// relationships that define a valid argument for the program.
///
/// There are two methods for constructing `Arg`s, using the builder pattern and setting options
/// manually, or using a usage string which is far less verbose but has fewer options. You can also
/// use a combination of the two methods to achieve the best of both worlds.
///
/// # Examples
///
/// ```rust
/// # use clap::Arg;
/// // Using the traditional builder pattern and setting each option manually
/// let cfg = Arg::with_name("config")
///       .short("c")
///       .long("config")
///       .takes_value(true)
///       .value_name("FILE")
///       .help("Provides a config file to myprog");
/// // Using a usage string (setting a similar argument to the one above)
/// let input = Arg::from_usage("-i, --input=[FILE] 'Provides an input file to the program'");
/// ```
#[allow(missing_debug_implementations)]
pub struct Arg<'a, 'b> where 'a: 'b {
    #[doc(hidden)]
    pub name: &'a str,
    #[doc(hidden)]
    pub short: Option<char>,
    #[doc(hidden)]
    pub long: Option<&'b str>,
    #[doc(hidden)]
    pub help: Option<&'b str>,
    #[doc(hidden)]
    pub index: Option<u8>,
    #[doc(hidden)]
    pub blacklist: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub possible_vals: Option<Vec<&'b str>>,
    #[doc(hidden)]
    pub requires: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub group: Option<&'a str>,
    #[doc(hidden)]
    pub val_names: Option<VecMap<&'b str>>,
    #[doc(hidden)]
    pub num_vals: Option<u8>,
    #[doc(hidden)]
    pub max_vals: Option<u8>,
    #[doc(hidden)]
    pub min_vals: Option<u8>,
    #[doc(hidden)]
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    #[doc(hidden)]
    pub overrides: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub settings: ArgFlags,
    #[doc(hidden)]
    pub val_delim: Option<char>,
}

impl<'a, 'b> Default for Arg<'a, 'b> {
    fn default() -> Self {
        Arg {
            name: "".as_ref(),
            short: None,
            long: None,
            help: None,
            index: None,
            blacklist: None,
            possible_vals: None,
            requires: None,
            group: None,
            val_names: None,
            num_vals: None,
            max_vals: None,
            min_vals: None,
            validator: None,
            overrides: None,
            settings: ArgFlags::new(),
            val_delim: Some(','),
        }
    }
}


impl<'a, 'b> Arg<'a, 'b> {
    /// Creates a new instance of `Arg` using a unique string name. The name will be used to get
    /// information about whether or not the argument was used at runtime, get values, set
    /// relationships with other args, etc..
    ///
    /// **NOTE:** In the case of arguments that take values (i.e. `takes_value(true)`)
    /// and positional arguments (i.e. those without a preceding `-` or `--`) the name will also
    /// be displayed when the user prints the usage/help information of the program.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    /// # ;
    /// ```
    pub fn with_name(n: &'a str) -> Self {
        Arg {
            name: n,
            ..Default::default()
        }
    }

    /// Creates a new instance of `Arg` from a .yml (YAML) file.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::Arg;
    /// let yml = load_yaml!("arg.yml");
    /// let arg = Arg::from_yaml(yml);
    /// ```
    #[cfg(feature = "yaml")]
    pub fn from_yaml<'y>(y: &'y BTreeMap<Yaml, Yaml>) -> Arg<'y, 'y> {
        // We WANT this to panic on error...so expect() is good.
        let name_yml = y.keys().nth(0).unwrap();
        let name_str = name_yml.as_str().unwrap();
        let mut a = Arg::with_name(name_str);
        let arg_settings = y.get(name_yml).unwrap().as_hash().unwrap();

        for (k, v) in arg_settings.iter() {
            a = match k.as_str().unwrap() {
                "short" => a.short(v.as_str().unwrap()),
                "long" => a.long(v.as_str().unwrap()),
                "help" => a.help(v.as_str().unwrap()),
                "required" => a.required(v.as_bool().unwrap()),
                "takes_value" => a.takes_value(v.as_bool().unwrap()),
                "index" => a.index(v.as_i64().unwrap() as u8),
                "global" => a.global(v.as_bool().unwrap()),
                "multiple" => a.multiple(v.as_bool().unwrap()),
                "empty_values" => a.empty_values(v.as_bool().unwrap()),
                "group" => a.group(v.as_str().unwrap()),
                "number_of_values" => a.number_of_values(v.as_i64().unwrap() as u8),
                "max_values" => a.max_values(v.as_i64().unwrap() as u8),
                "min_values" => a.min_values(v.as_i64().unwrap() as u8),
                "value_name" => a.value_name(v.as_str().unwrap()),
                "use_delimiter" => a.use_delimiter(v.as_bool().unwrap()),
                "value_delimiter" => a.value_delimiter(v.as_str().unwrap()),
                "value_names" => {
                    for ys in v.as_vec().unwrap() {
                        if let Some(s) = ys.as_str() {
                            a = a.value_name(s);
                        }
                    }
                    a
                }
                "requires" => {
                    for ys in v.as_vec().unwrap() {
                        if let Some(s) = ys.as_str() {
                            a = a.requires(s);
                        }
                    }
                    a
                }
                "conflicts_with" => {
                    for ys in v.as_vec().unwrap() {
                        if let Some(s) = ys.as_str() {
                            a = a.conflicts_with(s);
                        }
                    }
                    a
                }
                "overrides_with" => {
                    for ys in v.as_vec().unwrap() {
                        if let Some(s) = ys.as_str() {
                            a = a.overrides_with(s);
                        }
                    }
                    a
                }
                "possible_values" => {
                    for ys in v.as_vec().unwrap() {
                        if let Some(s) = ys.as_str() {
                            a = a.possible_value(s);
                        }
                    }
                    a
                }
                s => panic!("Unknown Arg setting '{}' in YAML file for arg '{}'",
                            s,
                            name_str),
            }
        }

        a
    }

    /// Creates a new instance of `Arg` from a usage string. Allows creation of basic settings for
    /// the `Arg`. The syntax is flexible, but there are some rules to follow.
    ///
    /// **NOTE**: Not all settings may be set using the usage string method. Some properties are
    /// only available via the builder pattern.
    ///
    /// # Syntax
    ///
    /// Usage strings typically following the form:
    ///
    /// ```ignore
    /// [explicit name] [short] [long] [value names] [help string]
    /// ```
    ///
    /// This is not a hard rule as the attributes can appear in other orders. There are also
    /// several additional sigils which denote additional settings. Below are the details of each
    /// portion of the string.
    ///
    /// ### Explicit Name
    ///
    /// This is an optional field, if it's omitted the argumenet will use one of the additioinal
    /// fields as the name using the following priority order:
    ///
    ///  * Explicit Name (This always takes precedence when present)
    ///  * Long
    ///  * Short
    ///  * Value Name
    ///
    /// `clap` determines explicit names as the first string of characters between either `[]` or
    /// `<>` where `[]` has the dual notation of meaning the argument is optional, and `<>` meaning
    /// the argument is required.
    ///
    /// Explicit names may be followed by:
    ///  * The multiple denotation `...`
    ///
    /// Example explicit names as follows (`ename` for an optional argument, and `rname` for a
    /// required argument):
    ///
    /// ```ignore
    /// [ename] -s, --long 'some flag'
    /// <rname> -r, --longer 'some other flag'
    /// ```
    ///
    /// ### Short
    ///
    /// This is set by placing a single character after a leading `-`.
    ///
    /// Shorts may be followed by
    ///  * The multiple denotation `...`
    ///  * An optional comma `,` which is cosmetic only
    ///  * Value notation
    ///
    /// Example shorts are as follows (`-s`, and `-r`):
    ///
    /// ```ignore
    /// -s, --long 'some flag'
    /// <rname> -r [val], --longer 'some option'
    /// ```
    ///
    /// ### Long
    ///
    /// This is set by placing a word (no spaces) after a leading `--`.
    ///
    /// Shorts may be followed by
    ///  * The multiple denotation `...`
    ///  * Value notation
    ///
    /// Example longs are as follows (`--some`, and `--rapid`):
    ///
    /// ```ignore
    /// -s, --some 'some flag'
    /// --rapid=[FILE] 'some option'
    /// ```
    ///
    /// ### Values (Value Notation)
    ///
    /// This is set by placing a word(s) between `[]` or `<>` optionally after `=` (although this
    /// is cosmetic only and does not affect functionality). If an explicit name has **not** been
    /// set, using `<>` will denote a required argument, and `[]` will denote an optional argument
    ///
    /// Values may be followed by
    ///  * The multiple denotation `...`
    ///  * More Value notation
    ///
    /// More than one value will also implicitly set the arguments number of values, i.e. having
    /// two values, `--option [val1] [val2]` specifies that in order for option to be satisified it
    /// must receive exactly two values
    ///
    /// Example values are as follows (`FILE`, and `SPEED`):
    ///
    /// ```ignore
    /// -s, --some [FILE] 'some option'
    /// --rapid=<SPEED>... 'some required multiple option'
    /// ```
    ///
    /// ### Help String
    ///
    /// The help string is denoted between a pair of single quotes `''` and may contain any characters.
    ///
    /// Example help strings are as follows:
    ///
    /// ```ignore
    /// -s, --some [FILE] 'some option'
    /// --rapid=<SPEED>... 'some required multiple option'
    /// ```
    ///
    /// ### Additional Sigils
    ///
    /// Multiple notation `...` (three consecutive dots/periods) specifies that this argument may
    /// be used multiple times. Do not confuse multiple occurrences (`...`) with multiple values.
    /// `--option val1 val2` is a single occurrence with multiple values. `--flag --flag` is
    /// multiple occurrences (and then you can obviously have instances of both as well)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args(&[
    ///         Arg::from_usage("--config <FILE> 'a required file for the configuration and no short'"),
    ///         Arg::from_usage("-d, --debug... 'turns on debugging information and allows multiples'"),
    ///         Arg::from_usage("[input] 'an optional input file to use'")
    /// ])
    /// # ;
    /// ```
    pub fn from_usage(u: &'a str) -> Self {
        let parser = UsageParser::from_usage(u);
        parser.parse()
    }

    /// Sets the short version of the argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `V` and `h` to display version and help information
    /// respectively. You may use `V` or `h` for your own purposes, in which case `clap` simply
    /// will not assign those to the displaying of version or help.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` character will be used as the `short` version
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .short("c")
    /// # ;
    /// ```
    pub fn short<S: AsRef<str>>(mut self, s: S) -> Self {
        self.short = s.as_ref().trim_left_matches(|c| c == '-').chars().nth(0);
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
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("cfg")
    ///     .long("config")
    /// # ;
    /// ```
    pub fn long(mut self, l: &'b str) -> Self {
        self.long = Some(l.trim_left_matches(|c| c == '-'));
        self
    }

    /// Sets the help text of the argument that will be displayed to the user when they print the
    /// usage/help information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .help("The config file used by the myprog")
    /// # ;
    /// ```
    pub fn help(mut self, h: &'b str) -> Self {
        self.help = Some(h);
        self
    }

    /// Sets whether or not the argument is required by default. Required by default means it is
    /// required, when no other conflicting rules have been evaluated. Conflicting rules take
    /// precedence over being required.
    ///
    /// **NOTE:** Flags (i.e. not positional, or arguments that take values) cannot be required.
    ///
    /// #Example
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .required(true)
    /// # ;
    /// ```
    pub fn required(self, r: bool) -> Self {
        if r { self.set(ArgSettings::Required) } else { self.unset(ArgSettings::Required) }
    }

    /// Sets a conflicting argument by name. I.e. when using this argument,
    /// the following argument can't be present and vice versa.
    ///
    /// **NOTE:** Conflicting rules take precedence over being required by default. Conflict rules
    /// only need to be set for one of the two arguments, they do not need to be set for each.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug");
    /// // ...
    /// Arg::with_name("config")
    ///     .conflicts_with("debug")
    /// # ;
    /// ```
    pub fn conflicts_with(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.blacklist {
            vec.push(name);
        } else {
            self.blacklist = Some(vec![name]);
        }
        self
    }

    /// Sets multiple conflicting arguments by names. I.e. when using this argument,
    /// the following arguments can't be present.
    ///
    /// **NOTE:** Conflicting rules take precedence over being required by default. Conflict rules
    /// only need to be set for one of the two arguments, they do not need to be set for each.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug");
    /// Arg::with_name("input");
    /// // ...
    /// Arg::with_name("config")
    ///     .conflicts_with_all(&["debug", "input"])
    /// # ;
    /// ```
    pub fn conflicts_with_all(mut self, names: &[&'a str]) -> Self {
        if let Some(ref mut vec) = self.blacklist {
            for s in names {
                vec.push(s);
            }
        } else {
            self.blacklist = Some(names.iter().map(|s| *s).collect::<Vec<_>>());
        }
        self
    }

    /// Sets a overridable argument by name. I.e. this argument and the following argument
    /// will override each other in POSIX style (whichever argument was specified at runtime
    /// **last** "wins")
    ///
    /// **NOTE:** When an argument is overriden it is essentially as if it never was used, any
    /// conflicts, requirements, etc. are evaluated **after** all "overrides" have been removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("posix")
    ///     .arg(Arg::from_usage("-f, --flag 'some flag'")
    ///         .conflicts_with("debug"))
    ///     .arg(Arg::from_usage("-d, --debug 'other flag'"))
    ///     .arg(Arg::from_usage("-c, --color 'third flag'")
    ///         .overrides_with("flag"))
    ///     .get_matches_from_safe(vec!["", "-f", "-d", "-c"]);
    ///                                 //    ^~~~~~~~~~~~^~~~~ flag is overriden by --color
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert!(m.is_present("color"));
    /// assert!(m.is_present("debug"));
    /// assert!(!m.is_present("flag"));
    /// ```
    pub fn overrides_with(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.overrides {
            vec.push(name.as_ref());
        } else {
            self.overrides = Some(vec![name.as_ref()]);
        }
        self
    }

    /// Sets a mutually overridable argument by name. I.e. this argument and the following argument
    /// will override each other in POSIX style (whichever argument was specified at runtime
    /// **last** "wins")
    ///
    /// **NOTE:** When an argument is overriden it is essentially as if it never was used, any
    /// conflicts, requirements, etc. are evaluated **after** all "overrides" have been removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("posix")
    ///     .arg(Arg::from_usage("-f, --flag 'some flag'")
    ///         .conflicts_with("debug"))
    ///     .arg(Arg::from_usage("-d, --debug 'other flag'"))
    ///     .arg(Arg::from_usage("-c, --color 'third flag'")
    ///         .overrides_with_all(&["flag", "debug"]))
    ///     .get_matches_from_safe(vec!["posix", "-f", "-d", "-c"]);
    ///                                 //        ^~~~~~^~~~~~^~~~~ flag and debug are overriden by --color
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert!(m.is_present("color"));
    /// assert!(!m.is_present("debug"));
    /// assert!(!m.is_present("flag"));
    /// ```
    pub fn overrides_with_all(mut self, names: &[&'a str]) -> Self {
        if let Some(ref mut vec) = self.overrides {
            for s in names {
                vec.push(s);
            }
        } else {
            self.overrides = Some(names.iter().map(|s| *s).collect::<Vec<_>>());
        }
        self
    }

    /// Sets an argument by name that is required when this one is present I.e. when
    /// using this argument, the following argument *must* be present.
    ///
    /// **NOTE:** Conflicting rules and override rules take precedence over being required
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgGroup};
    /// let m = App::new("group_required")
    ///     .arg(Arg::from_usage("-f, --flag 'some flag'"))
    ///     .group(ArgGroup::with_name("gr")
    ///         .required(true)
    ///         .arg("some")
    ///         .arg("other"))
    ///     .arg(Arg::from_usage("--some 'some arg'"))
    ///     .arg(Arg::from_usage("--other 'other arg'"))
    ///     .get_matches_from(vec!["", "-f", "--some"]);
    /// assert!(m.is_present("some"));
    /// assert!(!m.is_present("other"));
    /// assert!(m.is_present("flag"));
    /// ```
    pub fn requires(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.requires {
            vec.push(name);
        } else {
            self.requires = Some(vec![name]);
        }
        self
    }

    /// Sets arguments by names that are required when this one is present I.e. when
    /// using this argument, the following arguments *must* be present.
    ///
    /// **NOTE:** Mutually exclusive and override rules take precedence over being required
    /// by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("flag_required")
    ///     .arg(Arg::from_usage("-d 'debugging mode'"))
    ///     .arg(Arg::from_usage("-f, --flag 'some flag'")
    ///         .requires_all(&["color", "d"]))
    ///     .arg(Arg::from_usage("-c, --color 'third flag'"))
    ///     .get_matches_from_safe(vec!["flag_required", "-f"]);
    /// assert!(result.is_err());
    /// let err = result.err().unwrap();
    /// assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
    /// #
    /// ```
    pub fn requires_all(mut self, names: &[&'a str]) -> Self {
        if let Some(ref mut vec) = self.requires {
            for s in names {
                vec.push(s);
            }
        } else {
            self.requires = Some(names.into_iter().map(|s| *s).collect::<Vec<_>>());
        }
        self
    }

    /// Specifies that the argument takes an additional value at run time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .takes_value(true)
    /// # ;
    /// ```
    pub fn takes_value(self, tv: bool) -> Self {
        if tv { self.set(ArgSettings::TakesValue) } else { self.unset(ArgSettings::TakesValue) }
    }

    /// Specifies the index of a positional argument **starting at** 1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("file")
    /// .index(1)
    /// # ;
    /// ```
    pub fn index(mut self, idx: u8) -> Self {
        self.index = Some(idx);
        self
    }

    /// Specifies that the flag or option may appear more than once. For flags, this results
    /// in the number of occurrences of the flag being recorded. For example `-ddd` would count as
    /// three occurrences. The form `-d -d -d` would also be recognized as three occurrences. For
    /// options there is a distinct difference in multiple occurrences vs multiple values.
    ///
    /// For example, `--opt val1 val2` is one occurrence, but multiple values. `--opt val1 --opt
    /// val2` is multiple occurrences. This setting applies to occurrences and **not** values.
    ///
    /// To specify that an option may receive multiple values, use `Arg::min_values`,
    /// `Arg::max_values`, or `Arg::number_of_values` depending on your use case. Note also, that
    /// `Arg::value_names` implicitly sets multiple values, but not multiple occurrences.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .short("d")
    ///     .multiple(true)
    /// # ;
    /// ```
    pub fn multiple(self, multi: bool) -> Self {
        if multi { self.set(ArgSettings::Multiple) } else { self.unset(ArgSettings::Multiple) }
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
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .short("d")
    ///     .global(true)
    /// # ;
    /// ```
    pub fn global(self, g: bool) -> Self {
        if g { self.set(ArgSettings::Global) } else { self.unset(ArgSettings::Global) }
    }

    /// Allows an argument to accept explicit empty values. An empty value must be specified at the
    /// command line with an explicit `""`, or `''`
    ///
    /// **NOTE:** Defaults to `true` (Explicit empty values are allowed)
    ///
    /// **NOTE:** Implicitly sets `takes_value(true)` when set to `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("file")
    ///     .long("file")
    ///     .empty_values(false)
    /// # ;
    /// ```
    pub fn empty_values(mut self, ev: bool) -> Self {
        if ev {
            self.set(ArgSettings::EmptyValues)
        } else {
            self = self.set(ArgSettings::TakesValue);
            self.unset(ArgSettings::EmptyValues)
        }
    }

    /// Hides an argument from help message output.
    ///
    /// **NOTE:** This does **not** hide the argument from usage strings on error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .hidden(true)
    /// # ;
    /// ```
    pub fn hidden(self, h: bool) -> Self {
        if h { self.set(ArgSettings::Hidden) } else { self.unset(ArgSettings::Hidden) }
    }

    /// Specifies a list of possible values for this argument. At runtime, `clap` verifies that only
    /// one of the specified values was used, or fails with an error message.
    ///
    /// **NOTE:** This setting only applies to options and positional arguments
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("possible_values")
    ///     .arg(Arg::with_name("option")
    ///         .short("-o")
    ///         .long("--option")
    ///         .takes_value(true)
    ///         .possible_values(&["fast", "slow"]))
    ///     .get_matches_from_safe(vec!["myprog", "--option", "fast"]);
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert!(m.is_present("option"));
    /// assert_eq!(m.value_of("option"), Some("fast"));
    /// ```
    ///
    /// The next example shows a failed parse
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("possible_values")
    ///     .arg(Arg::with_name("option")
    ///         .short("-o")
    ///         .long("--option")
    ///         .takes_value(true)
    ///         .possible_values(&["fast", "slow"]))
    ///     .get_matches_from_safe(vec!["myprog", "--option", "wrong"]);
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// assert_eq!(err.kind, ErrorKind::InvalidValue);
    /// ```
    pub fn possible_values(mut self, names: &[&'b str]) -> Self {
        if let Some(ref mut vec) = self.possible_vals {
            for s in names {
                vec.push(s);
            }
        } else {
            self.possible_vals = Some(names.iter().map(|s| *s).collect::<Vec<_>>());
        }
        self
    }

    /// Specifies a possible value for this argument. At runtime, `clap` verifies that only
    /// one of the specified values was used, or fails with error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("possible_values")
    ///     .arg(Arg::with_name("option")
    ///         .short("-o")
    ///         .long("--option")
    ///         .takes_value(true)
    ///         .possible_value("slow")
    ///         .possible_value("fast"))
    ///     .get_matches_from_safe(vec!["myprog", "--option", "fast"]);
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert!(m.is_present("option"));
    /// assert_eq!(m.value_of("option"), Some("fast"));
    /// ```
    ///
    /// The next example shows a failed parse
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("possible_values")
    ///     .arg(Arg::with_name("option")
    ///         .short("-o")
    ///         .long("--option")
    ///         .takes_value(true)
    ///         .possible_value("slow")
    ///         .possible_value("fast"))
    ///     .get_matches_from_safe(vec!["myprog", "--option", "wrong"]);
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// assert_eq!(err.kind, ErrorKind::InvalidValue);
    /// ```
    pub fn possible_value(mut self, name: &'b str) -> Self {
        if let Some(ref mut vec) = self.possible_vals {
            vec.push(name);
        } else {
            self.possible_vals = Some(vec![name]);
        }
        self
    }

    /// Specifies the name of the group the argument belongs to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .index(1)
    ///     .group("mode")
    /// # ;
    /// ```
    pub fn group(mut self, name: &'a str) -> Self {
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
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("file")
    ///     .short("f")
    ///     .number_of_values(3)
    /// # ;
    /// ```
    pub fn number_of_values(mut self, qty: u8) -> Self {
        self.num_vals = Some(qty);
        self
    }

    /// Allows one to perform a custom validation on the argument value. You provide a closure which
    /// accepts a `String` value, a `Result` where the `Err(String)` is a message displayed to the
    /// user.
    ///
    /// **NOTE:** The error message does *not* need to contain the `error:` portion, only the
    /// message.
    ///
    /// **NOTE:** There is a small performance hit for using validators, as they are implemented
    /// with `Rc` pointers. And the value to be checked will be allocated an extra time in order to
    /// to be passed to the closure. This performance hit is extremely minimal in the grand scheme
    /// of things.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// fn has_at(v: String) -> Result<(), String> {
    ///     if v.contains("@") { return Ok(()); }
    ///     Err(String::from("The value did not contain the required @ sigil"))
    /// }
    /// let res = App::new("validators")
    ///     .arg(Arg::with_name("file")
    ///         .index(1)
    ///         .validator(has_at))
    ///     .get_matches_from_safe(vec![
    ///         "validators", "some@file"
    ///     ]);
    /// assert!(res.is_ok());
    /// assert_eq!(res.unwrap().value_of("file"), Some("some@file"));
    /// ```
    pub fn validator<F>(mut self, f: F) -> Self
        where F: Fn(String) -> Result<(), String> + 'static
    {
        self.validator = Some(Rc::new(f));
        self
    }

    /// Specifies the *maximum* number of values are for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted up to 3 'files' you would set
    /// `.max_values(3)`, and this argument would be satisfied if the user provided, 1, 2, or 3
    /// values.
    ///
    /// **NOTE:** This does not implicitly set `mulitple(true)`. This is because `-o val -o val` is
    /// multiples occurrences but a single value and `-o val1 val2` is a single occurence with
    /// multple values. For positional arguments this **does** set `multiple(true)` because there
    /// is no way to determine the diffrence between multiple occureces and multiple values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("file")
    ///     .short("f")
    ///     .max_values(3)
    /// # ;
    /// ```
    pub fn max_values(mut self, qty: u8) -> Self {
        self.max_vals = Some(qty);
        self
    }

    /// Specifies the *minimum* number of values are for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted at least 2 'files' you would set
    /// `.min_values(2)`, and this argument would be satisfied if the user provided, 2 or more
    /// values.
    ///
    /// **NOTE:** This does not implicitly set `mulitple(true)`. This is because `-o val -o val` is
    /// multiples occurrences but a single value and `-o val1 val2` is a single occurence with
    /// multple values. For positional arguments this **does** set `multiple(true)` because there
    /// is no way to determine the diffrence between multiple occureces and multiple values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("file")
    ///     .short("f")
    ///     .min_values(3)
    /// # ;
    /// ```
    pub fn min_values(mut self, qty: u8) -> Self {
        self.min_vals = Some(qty);
        self.set(ArgSettings::TakesValue)
    }

    /// Specifies whether or not an arugment should allow grouping of multiple values via a
    /// delimter. I.e. shoulde `--option=val1,val2,val3` be parsed as three values (`val1`, `val2`,
    /// and `val3`) or as a single value (`val1,val2,val3`). Defaults to using `,` (comma) as the
    /// value delimiter for all arguments that accept values (options and positional arguments)
    ///
    /// **NOTE:** The defalt is `true`. Setting the value to `true` will reset any previous use of
    /// `Arg::value_delimiter` back to the default of `,` (comma).
    ///
    /// # Examples
    ///
    /// The following example shows the default behavior.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let delims = App::new("delims")
    ///     .arg(Arg::with_name("option")
    ///         .long("option")
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "delims",
    ///         "--option=val1,val2,val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("option"));
    /// assert_eq!(delims.occurrences_of("option"), 1);
    /// assert_eq!(delims.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// The next example shows the difference when turning delimiters off.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let nodelims = App::new("nodelims")
    ///     .arg(Arg::with_name("option")
    ///         .long("option")
    ///         .use_delimiter(false)
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "nodelims",
    ///         "--option=val1,val2,val3",
    ///     ]);
    ///
    /// assert!(nodelims.is_present("option"));
    /// assert_eq!(nodelims.occurrences_of("option"), 1);
    /// assert_eq!(nodelims.value_of("option").unwrap(), "val1,val2,val3");
    /// ```
    pub fn use_delimiter(mut self, d: bool) -> Self {
        if d {
            self.val_delim = Some(',');
            self.set(ArgSettings::UseValueDelimiter)
        } else {
            self.val_delim = None;
            self.unset(ArgSettings::UseValueDelimiter)
        }
    }

    /// Specifies the separator to use when values are clumped together, defaults to `,` (comma).
    ///
    /// **NOTE:** implicitly sets `Arg::use_delimiter(true)`
    ///
    /// **NOTE:** implicitly sets `Arg::takes_value(true)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let app = App::new("fake")
    ///     .arg(Arg::with_name("config")
    ///         .short("c")
    ///         .long("config")
    ///         .value_delimiter(";"));
    ///
    /// let m = app.get_matches_from(vec![
    ///     "fake", "--config=val1;val2;val3"
    /// ]);
    ///
    /// assert_eq!(m.values_of("config").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"])
    /// ```
    pub fn value_delimiter(mut self, d: &str) -> Self {
        self = self.set(ArgSettings::TakesValue);
        self = self.set(ArgSettings::UseValueDelimiter);
        self.val_delim = Some(d.chars()
                               .nth(0)
                               .expect("Failed to get value_delimiter from arg"));
        self
    }

    /// Specifies names for values of option arguments. These names are cosmetic only, used for
    /// help and usage strings only. The names are **not** used to access arguments. The values of
    /// the arguments are accessed in numeric order (i.e. if you specify two names `one` and `two`
    /// `one` will be the first matched value, `two` will be the second).
    ///
    /// **NOTE:** This implicitly sets `.number_of_values()` if the number of value names is
    /// greater than one. I.e. be aware that the number of "names" you set for the values, will be
    /// the *exact* number of values required to satisfy this argument
    ///
    /// **NOTE:** implicitly sets `Arg::takes_value(true)`
    ///
    /// **NOTE:** Does *not* require or imply `.multiple(true)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("speed")
    ///     .short("s")
    ///     .value_names(&["fast", "slow"])
    /// # ;
    /// ```
    pub fn value_names(mut self, names: &[&'b str]) -> Self {
        self.setb(ArgSettings::TakesValue);
        if let Some(ref mut vals) = self.val_names {
            let mut l =  vals.len();
            for s in names {
                vals.insert(l, s);
                l += 1;
            }
        } else {
            let mut vm = VecMap::new();
            for (i, n) in names.iter().enumerate() {
                vm.insert(i, *n);
            }
            self.val_names = Some(vm);
        }
        self
    }

    /// Specifies the name for value of option or positional arguments inside of help documenation.
    /// This name is cosmetic only, the name is **not** used to access arguments.
    ///
    /// **NOTE:** implicitly sets `Arg::takes_value(true)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("input")
    ///     .index(1)
    ///     .value_name("FILE")
    /// # ;
    /// ```
    pub fn value_name(mut self, name: &'b str) -> Self {
        self.setb(ArgSettings::TakesValue);
        if let Some(ref mut vals) = self.val_names {
            let l = vals.len();
            vals.insert(l, name);
        } else {
            let mut vm = VecMap::new();
            vm.insert(0, name);
            self.val_names = Some(vm);
        }
        self
    }

    #[doc(hidden)]
    pub fn setb(&mut self, s: ArgSettings) {
        self.settings.set(s);
    }

    #[doc(hidden)]
    pub fn unsetb(&mut self, s: ArgSettings) {
        self.settings.unset(s);
    }

    /// Checks if one of the `ArgSettings` settings is set for the argument
    pub fn is_set(&self, s: ArgSettings) -> bool {
        self.settings.is_set(s)
    }

    /// Sets one of the `ArgSettings` settings for the argument
    pub fn set(mut self, s: ArgSettings) -> Self {
        self.setb(s);
        self
    }

    /// Unsets one of the `ArgSettings` settings for the argument
    pub fn unset(mut self, s: ArgSettings) -> Self {
        self.unsetb(s);
        self
    }
}

impl<'a, 'b, 'z> From<&'z Arg<'a, 'b>>
    for Arg<'a, 'b> {
    fn from(a: &'z Arg<'a, 'b>) -> Self {
        Arg {
            name: a.name,
            short: a.short,
            long: a.long,
            help: a.help,
            index: a.index,
            possible_vals: a.possible_vals.clone(),
            blacklist: a.blacklist.clone(),
            requires: a.requires.clone(),
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names.clone(),
            group: a.group,
            validator: a.validator.clone(),
            overrides: a.overrides.clone(),
            settings: a.settings.clone(),
            val_delim: a.val_delim,
        }
    }
}
