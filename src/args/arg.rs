use std::iter::IntoIterator;
#[cfg(feature = "yaml")]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::rc::Rc;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

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
/// **NOTE*: Fields of this struct are **not** meant to be used directly unless absolutely
/// required. 99.9% of the tasks can be performed without accessing these fields directly.
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg};
/// # let matches = App::new("myprog")
/// #                 .arg(
/// // Using the traditional builder pattern and setting each option manually
/// Arg::with_name("config")
///       .short("c")
///       .long("config")
///       .takes_value(true)
///       .help("Provides a config file to myprog")
/// # ).arg(
/// // Using a usage string (setting a similar argument to the one above)
/// Arg::from_usage("-i --input=[input] 'Provides an input file to the program'")
/// # ).get_matches();
#[allow(missing_debug_implementations)]
pub struct Arg<'n, 'l, 'h, 'g, 'p, 'r> {
    /// The unique name of the argument
    pub name: &'n str,
    /// The short version (i.e. single character) of the argument, no preceding `-`
    /// **NOTE:** `short` is mutually exclusive with `index`
    pub short: Option<char>,
    /// The long version of the flag (i.e. word) without the preceding `--`
    /// **NOTE:** `long` is mutually exclusive with `index`
    pub long: Option<&'l str>,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'h str>,
    /// If this is a required by default when using the command line program,
    /// e.g. a configuration file that's required for the program to function
    /// **NOTE:** required by default means it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// Determines if this argument is an option (as opposed to flag or positional) and
    /// is mutually exclusive with `index` and `multiple`
    pub takes_value: bool,
    /// The index of the argument. `index` is mutually exclusive with `takes_value`
    /// and `multiple`
    pub index: Option<u8>,
    /// Determines if multiple instances of the same flag are allowed. `multiple`
    /// is mutually exclusive with `index`.
    /// e.g. `-v -v -v` or `-vvv` or `--option foo --option bar`
    pub multiple: bool,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<Vec<&'r str>>,
    /// A list of possible values for an option or positional argument
    pub possible_vals: Option<Vec<&'p str>>,
    /// A list of names of other arguments that are *required* to be used when
    /// this flag is used
    pub requires: Option<Vec<&'r str>>,
    /// A name of the group the argument belongs to
    pub group: Option<&'g str>,
    /// A set of names (ordered) for the values to be displayed with the help message
    pub val_names: Option<BTreeSet<&'n str>>,
    /// The exact number of values to satisfy this argument
    pub num_vals: Option<u8>,
    /// The maximum number of values possible for this argument
    pub max_vals: Option<u8>,
    /// The minimum number of values possible to satisfy this argument
    pub min_vals: Option<u8>,
    /// Specifies whether or not this argument accepts explicit empty values such as `--option ""`
    pub empty_vals: bool,
    /// Specifies whether or not this argument is global and should be propagated through all
    /// child subcommands
    pub global: bool,
    /// A function used to check the validity of an argument value. Failing this validation results
    /// in failed argument parsing.
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    /// A list of names for other arguments that *mutually override* this flag
    pub overrides: Option<Vec<&'r str>>,
    /// Specifies whether the argument should show up in the help message
    pub hidden: bool,
}

impl<'n, 'l, 'h, 'g, 'p, 'r> Arg<'n, 'l, 'h, 'g, 'p, 'r> {
    /// Creates a new instance of `Arg` using a unique string name.
    /// The name will be used by the library consumer to get information about
    /// whether or not the argument was used at runtime.
    ///
    /// **NOTE:** in the case of arguments that take values (i.e. `takes_value(true)`)
    /// and positional arguments (i.e. those without a `-` or `--`) the name will also
    /// be displayed when the user prints the usage/help information of the program.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// Arg::with_name("config")
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
            empty_vals: true,
            validator: None,
            overrides: None,
            hidden: false,
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
    pub fn from_yaml<'y>(y: &'y BTreeMap<Yaml, Yaml>) -> Arg<'y, 'y, 'y, 'y, 'y, 'y> {
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
                "mutually_overrides_with" => {
                    for ys in v.as_vec().unwrap() {
                        if let Some(s) = ys.as_str() {
                            a = a.mutually_overrides_with(s);
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

    /// Creates a new instance of `Arg` from a usage string. Allows creation of basic settings
    /// for Arg (i.e. everything except relational rules). The syntax is flexible, but there are
    /// some rules to follow.
    ///
    /// **NOTE**: only properties which you wish to set must be present
    ///
    /// 1. Name (arguments with a `long` or that take a value can omit this if desired),
    ///    use `[]` for non-required arguments, or `<>` for required arguments.
    /// 2. Short preceded by a `-`
    /// 3. Long preceded by a `--` (this may be used as the name, if the name is omitted. If the
    ///    name is *not* omitted, the name takes precedence over the `long`)
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
    /// # Examples
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
        assert!(u.len() > 0,
                "Arg::from_usage() requires a non-zero-length usage string but none \
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
        let mut val_names = BTreeSet::new();

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
            if (val_names.len() > 1) && (name.unwrap() != l && !name_first) {
                name = Some(l);
            }
        }

        Arg {
            name: name.unwrap_or_else(|| {
                panic!("Missing flag name in \"{}\", check from_usage call", u)
            }),
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
            num_vals: if num_names > 1 {
                Some(num_names)
            } else {
                None
            },
            val_names: if val_names.len() > 1 {
                Some(val_names)
            } else {
                None
            },
            max_vals: None,
            min_vals: None,
            group: None,
            global: false,
            empty_vals: true,
            validator: None,
            overrides: None,
            hidden: false,
        }
    }

    /// Sets the short version of the argument without the preceding `-`.
    ///
    ///
    /// By default `clap` automatically assigns `V` and `h` to display version and help information
    /// respectively. You may use `V` or `h` for your own purposes, in which case `clap` simply
    /// will not assign those to the displaying of version or help.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` character will be used as the `short` version
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("config")
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
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("config")
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
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("config")
    /// .help("The config file used by the myprog")
    /// # ).get_matches();
    pub fn help(mut self, h: &'h str) -> Self {
        self.help = Some(h);
        self
    }

    /// Sets whether or not the argument is required by default. Required by
    /// default means it is required, when no other mutually exclusive rules have
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
    /// # Arg::with_name("config")
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
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
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
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let config_conflicts = ["debug", "input"];
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
    /// .conflicts_with_all(&config_conflicts)
    /// # ).get_matches();
    pub fn conflicts_with_all<T, I>(mut self, names: I) -> Self
        where T: AsRef<str> + 'r,
              I: IntoIterator<Item = &'r T>
    {
        if let Some(ref mut vec) = self.blacklist {
            for s in names {
                vec.push(s.as_ref());
            }
        } else {
            self.blacklist = Some(names.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        }
        self
    }

    /// Sets a mutually overridable argument by name. I.e. this argument and
    /// the following argument will override each other in POSIX style
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
    /// .mutually_overrides_with("debug")
    /// # ).get_matches();
    pub fn mutually_overrides_with(mut self, name: &'r str) -> Self {
        if let Some(ref mut vec) = self.overrides {
            vec.push(name);
        } else {
            self.overrides = Some(vec![name]);
        }
        self
    }

    /// Sets a mutually overridable arguments by name. I.e. this argument and
    /// the following argument will override each other in POSIX style
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let config_overrides = ["debug", "input"];
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
    /// .mutually_overrides_with_all(&config_overrides)
    /// # ).get_matches();
    pub fn mutually_overrides_with_all<T, I>(mut self, names: I) -> Self
        where T: AsRef<str> + 'r,
              I: IntoIterator<Item = &'r T>
    {
        if let Some(ref mut vec) = self.overrides {
            for s in names {
                vec.push(s.as_ref());
            }
        } else {
            self.overrides = Some(names.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        }
        self
    }

    /// Sets an argument by name that is required when this one is present I.e. when
    /// using this argument, the following argument *must* be present.
    ///
    /// **NOTE:** Mutually exclusive and override rules take precedence over being required
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
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

    /// Sets arguments by names that are required when this one is present I.e. when
    /// using this argument, the following arguments *must* be present.
    ///
    /// **NOTE:** Mutually exclusive and override rules take precedence over being required
    /// by default.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let config_reqs = ["debug", "input"];
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
    /// .requires_all(&config_reqs)
    /// # ).get_matches();
    pub fn requires_all<T, I>(mut self, names: I) -> Self
        where T: AsRef<str> + 'r,
              I: IntoIterator<Item = &'r T>
    {
        if let Some(ref mut vec) = self.requires {
            for s in names {
                vec.push(s.as_ref());
            }
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
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("config")
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
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("config")
    /// .index(1)
    /// # ).get_matches();
    pub fn index(mut self, idx: u8) -> Self {
        self.index = Some(idx);
        self
    }

    /// Specifies that the flag or option may appear more than once. For flags, this results
    /// in the number of occurrences of the flag being recorded. For example `-ddd` would count as
    /// three occurrences. The form `-d -d -d` would also be recognized as three occurrences. For
    /// options, more than one value may be provided. The forms `--optional foo --optional bar`,
    /// `--optional foo bar` and `-ofoo -obar` are all recognized, assuming the relevant `short`
    /// and `long` option names have been set.
    ///
    /// **NOTE:** When setting this, `index` is ignored as it only makes sense for positional
    /// arguments.
    ///
    ///
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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

    /// Hides an argument from help message output.
    ///
    /// **NOTE:** This does **not** hide the argument from usage strings on error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .hidden(true)
    /// # ).get_matches();
    pub fn hidden(mut self, h: bool) -> Self {
        self.hidden = h;
        self
    }

    /// Specifies a list of possible values for this argument. At runtime, clap verifies that only
    /// one of the specified values was used, or fails with a usage string.
    ///
    /// **NOTE:** This setting only applies to options and positional arguments
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let mode_vals = ["fast", "slow"];
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .possible_values(&mode_vals)
    /// # ).get_matches();
    pub fn possible_values<T, I>(mut self, names: I) -> Self
        where T: AsRef<str> + 'p,
              I: IntoIterator<Item = &'p T>
    {
        if let Some(ref mut vec) = self.possible_vals {
            for s in names {
                vec.push(s.as_ref());
            }
        } else {
            self.possible_vals = Some(names.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>());
        }
        self
    }

    /// Specifies a possible value for this argument. At runtime, clap verifies that only
    /// one of the specified values was used, or fails with a usage string.
    ///
    /// **NOTE:** This setting only applies to options and positional arguments
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .possible_value("fast")
    /// .possible_value("slow")
    /// # ).get_matches();
    pub fn possible_value(mut self, name: &'p str) -> Self {
        if let Some(ref mut vec) = self.possible_vals {
            vec.push(name);
        } else {
            self.possible_vals = Some(vec![name]);
        }
        self
    }

    /// Specifies the name of the group the argument belongs to.
    ///
    ///
    /// # Examples
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
    /// # Examples
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

    /// Allows one to perform a validation on the argument value. You provide a closure which
    /// accepts a `String` value, a `Result` where the `Err(String)` is a message displayed to the
    /// user.
    ///
    /// **NOTE:** The error message does *not* need to contain the `error:` portion, only the
    /// message.
    ///
    /// **NOTE:** There is a small performance hit for using validators, as they are implemented
    /// with `Rc` pointers. And the value to be checked will be allocated an extra time in order to
    /// to be passed to the closure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .validator(|val| {
    ///     if val.contains("@") {
    ///         Ok(())
    ///     } else {
    ///         Err(String::from("the value must contain at least one '@' character"))
    ///     }
    /// })
    /// # ).get_matches();
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
    /// **NOTE:** `qty` must be > 1
    ///
    /// **NOTE:** This implicitly sets `.multiple(true)`
    ///
    /// # Examples
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
    /// **NOTE:** This implicitly sets `.multiple(true)`
    ///
    /// **NOTE:** `qty` must be > 0
    ///
    /// **NOTE:** `qty` *must* be > 0. If you wish to have an argument with 0 or more values prefer
    /// two separate arguments (a flag, and an option with multiple values).
    ///
    /// # Examples
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
    /// # Examples
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
    pub fn value_names<T, I>(mut self, names: I) -> Self
        where T: AsRef<str> + 'n,
              I: IntoIterator<Item = &'n T>
    {
        if let Some(ref mut vec) = self.val_names {
            for s in names {
                vec.insert(s.as_ref());
            }
        } else {
            self.val_names = Some(names.into_iter().map(|s| s.as_ref()).collect::<BTreeSet<_>>());
        }
        self
    }

    /// Specifies the name for value of option or positional arguments. This name is cosmetic only,
    /// used for help and usage strings. The name is **not** used to access arguments.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// Arg::with_name("debug")
    ///     .index(1)
    ///     .value_name("file")
    /// # ).get_matches();
    pub fn value_name(mut self, name: &'n str) -> Self {
        if let Some(ref mut vec) = self.val_names {
            vec.insert(name);
        } else {
            let mut bts = BTreeSet::new();
            bts.insert(name);
            self.val_names = Some(bts);
        }
        self
    }
}

impl<'n, 'l, 'h, 'g, 'p, 'r, 'z> From<&'z Arg<'n, 'l, 'h, 'g, 'p, 'r>>
    for Arg<'n, 'l, 'h, 'g, 'p, 'r> {
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
            empty_vals: a.empty_vals,
            validator: a.validator.clone(),
            overrides: a.overrides.clone(),
            hidden: a.hidden,
        }
    }
}
