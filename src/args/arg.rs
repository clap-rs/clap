#[cfg(feature = "yaml")]
use std::collections::BTreeMap;
use std::rc::Rc;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;
use vec_map::VecMap;

use usage_parser::UsageParser;
use args::settings::{ArgSettings, ArgFlags};

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
/// # use clap::Arg;
/// // Using the traditional builder pattern and setting each option manually
/// let cfg = Arg::with_name("config")
///       .short("c")
///       .long("config")
///       .takes_value(true)
///       .help("Provides a config file to myprog");
/// // Using a usage string (setting a similar argument to the one above)
/// let input = Arg::from_usage("-i --input=[input] 'Provides an input file to the program'");
#[allow(missing_debug_implementations)]
pub struct Arg<'a, 'b> where 'a: 'b {
    // The unique name of the argument
    #[doc(hidden)]
    pub name: &'a str,
    // The short version (i.e. single character) of the argument, no preceding `-`
    // **NOTE:** `short` is mutually exclusive with `index`
    #[doc(hidden)]
    pub short: Option<char>,
    // The long version of the flag (i.e. word) without the preceding `--`
    // **NOTE:** `long` is mutually exclusive with `index`
    #[doc(hidden)]
    pub long: Option<&'b str>,
    // The string of text that will displayed to the user when the application's
    // `help` text is displayed
    #[doc(hidden)]
    pub help: Option<&'b str>,
    // The index of the argument. `index` is mutually exclusive with `takes_value`
    // and `multiple`
    #[doc(hidden)]
    pub index: Option<u8>,
    // A list of names for other arguments that *may not* be used with this flag
    #[doc(hidden)]
    pub blacklist: Option<Vec<&'a str>>,
    // A list of possible values for an option or positional argument
    #[doc(hidden)]
    pub possible_vals: Option<Vec<&'b str>>,
    // A list of names of other arguments that are *required* to be used when
    // this flag is used
    #[doc(hidden)]
    pub requires: Option<Vec<&'a str>>,
    // A name of the group the argument belongs to
    #[doc(hidden)]
    pub group: Option<&'a str>,
    // A set of names (ordered) for the values to be displayed with the help message
    #[doc(hidden)]
    pub val_names: Option<VecMap<&'b str>>,
    // The exact number of values to satisfy this argument
    #[doc(hidden)]
    pub num_vals: Option<u8>,
    // The maximum number of values possible for this argument
    #[doc(hidden)]
    pub max_vals: Option<u8>,
    // The minimum number of values possible to satisfy this argument
    #[doc(hidden)]
    pub min_vals: Option<u8>,
    // A function used to check the validity of an argument value. Failing this validation results
    // in failed argument parsing.
    #[doc(hidden)]
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    // A list of names for other arguments that *mutually override* this flag
    #[doc(hidden)]
    pub overrides: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub settings: ArgFlags,
    // Delimiting character for value separation
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
    ///                  .args(&[
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
    pub fn from_usage(u: &'a str) -> Self {
        let parser = UsageParser::from_usage(u);
        parser.parse()
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
    pub fn long(mut self, l: &'b str) -> Self {
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
    pub fn help(mut self, h: &'b str) -> Self {
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
    pub fn required(self, r: bool) -> Self {
        if r { self.set(ArgSettings::Required) } else { self.unset(ArgSettings::Required) }
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
    pub fn conflicts_with(mut self, name: &'a str) -> Self {
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

    /// Sets a mutually overridable argument by name. I.e. this argument and
    /// the following argument will override each other in POSIX style
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
    /// .overrides_with("debug")
    /// # ).get_matches();
    pub fn overrides_with(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.overrides {
            vec.push(name.as_ref());
        } else {
            self.overrides = Some(vec![name.as_ref()]);
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
    /// .overrides_with_all(&config_overrides)
    /// # ).get_matches();
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let config_reqs = ["debug", "input"];
    /// # let myprog = App::new("myprog").arg(Arg::with_name("config")
    /// .requires_all(&config_reqs)
    /// # ).get_matches();
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
    pub fn takes_value(self, tv: bool) -> Self {
        if tv { self.set(ArgSettings::TakesValue) } else { self.unset(ArgSettings::TakesValue) }
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
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .global(true)
    /// # ).get_matches();
    pub fn global(self, g: bool) -> Self {
        if g { self.set(ArgSettings::Global) } else { self.unset(ArgSettings::Global) }
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
    pub fn empty_values(self, ev: bool) -> Self {
        if ev { self.set(ArgSettings::EmptyValues) } else { self.unset(ArgSettings::EmptyValues) }
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
    pub fn hidden(self, h: bool) -> Self {
        if h { self.set(ArgSettings::Hidden) } else { self.unset(ArgSettings::Hidden) }
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
    /// **NOTE:** For positional arguments this implicitly sets `multiple(true)` but does *not*
    /// for options. This is because `-o val -o val` is multiples occurrences but a single value
    /// and `-o val1 val2` is a single occurence with multple values. For positional arguments
    /// there is no way to determine the diffrence between multiple occureces and multiple values.
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
        self.max_vals = Some(qty);
        self
    }

    /// Specifies the *minimum* number of values are for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted at least 2 'files' you would set
    /// `.min_values(2)`, and this argument would be satisfied if the user provided, 2 or more
    /// values.
    ///
    /// **NOTE:** For positional arguments this implicitly sets `multiple(true)` but does *not*
    /// for options. This is because `-o val -o val` is multiples occurrences but a single value
    /// and `-o val1 val2` is a single occurence with multple values. For positional arguments
    /// there is no way to determine the diffrence between multiple occureces and multiple values.
    ///
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
        self.min_vals = Some(qty);
        self
    }

    /// Specifies the separator to use when values are clumped together, defaults to `,` (comma).
    ///
    /// **NOTE:** implicitly sets `Arg::takes_value(true)`
    ///
    /// # Examples
    ///
    /// ```no_run
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
    /// **NOTE:** This implicitly sets `.number_of_values()`, but be aware that the number of
    /// "names" you set for the values, will be the *exact* number of values required to satisfy
    /// this argument
    ///
    /// **NOTE:** implicitly sets `Arg::takes_value(true)`
    ///
    /// **NOTE:** Does *not* require or imply `.multiple(true)`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// Arg::with_name("speed")
    ///     .short("s")
    ///     .value_names(&["fast", "slow"])
    /// # ;
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

    /// Specifies the name for value of option or positional arguments inside of help documenation.
    /// This name is cosmetic only, the name is **not** used to access arguments.
    ///
    /// **NOTE:** implicitly sets `Arg::takes_value(true)`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// Arg::with_name("input")
    ///     .index(1)
    ///     .value_name("FILE")
    /// # ;
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
