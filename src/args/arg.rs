#[cfg(feature = "yaml")]
use std::collections::BTreeMap;
use std::rc::Rc;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;
use vec_map::VecMap;

use usage_parser::UsageParser;
use args::settings::{ArgFlags, ArgSettings};

/// The abstract representation of a command line argument. Used to set all the options and
/// relationships that define a valid argument for the program.
///
/// There are two methods for constructing [`Arg`]s, using the builder pattern and setting options
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
/// [`Arg`]: ./struct.Arg.html
#[allow(missing_debug_implementations)]
pub struct Arg<'a, 'b>
    where 'a: 'b
{
    #[doc(hidden)]
    pub name: &'a str,
    #[doc(hidden)]
    pub short: Option<char>,
    #[doc(hidden)]
    pub long: Option<&'b str>,
    #[doc(hidden)]
    pub help: Option<&'b str>,
    #[doc(hidden)]
    pub index: Option<u64>,
    #[doc(hidden)]
    pub blacklist: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub possible_vals: Option<Vec<&'b str>>,
    #[doc(hidden)]
    pub requires: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub group: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub val_names: Option<VecMap<&'b str>>,
    #[doc(hidden)]
    pub num_vals: Option<u64>,
    #[doc(hidden)]
    pub max_vals: Option<u64>,
    #[doc(hidden)]
    pub min_vals: Option<u64>,
    #[doc(hidden)]
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    #[doc(hidden)]
    pub overrides: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub settings: ArgFlags,
    #[doc(hidden)]
    pub val_delim: Option<char>,
    #[doc(hidden)]
    pub default_val: Option<&'a str>,
    #[doc(hidden)]
    pub disp_ord: usize,
    #[doc(hidden)]
    pub r_unless: Option<Vec<&'a str>>,
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
            default_val: None,
            disp_ord: 999,
            r_unless: None,
        }
    }
}


impl<'a, 'b> Arg<'a, 'b> {
    /// Creates a new instance of [`Arg`] using a unique string name. The name will be used to get
    /// information about whether or not the argument was used at runtime, get values, set
    /// relationships with other args, etc..
    ///
    /// **NOTE:** In the case of arguments that take values (i.e. [`Arg::takes_value(true)`])
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
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    /// [`Arg`]: ./struct.Arg.html
    pub fn with_name(n: &'a str) -> Self {
        Arg { name: n, ..Default::default() }
    }

    /// Creates a new instance of [`Arg`] from a .yml (YAML) file.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::Arg;
    /// let yml = load_yaml!("arg.yml");
    /// let arg = Arg::from_yaml(yml);
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    #[cfg(feature = "yaml")]
    pub fn from_yaml<'y>(y: &'y BTreeMap<Yaml, Yaml>) -> Arg<'y, 'y> {
        // We WANT this to panic on error...so expect() is good.
        let name_yml = y.keys().nth(0).unwrap();
        let name_str = name_yml.as_str().unwrap();
        let mut a = Arg::with_name(name_str);
        let arg_settings = y.get(name_yml).unwrap().as_hash().unwrap();

        for (k, v) in arg_settings.iter() {
            a = match k.as_str().unwrap() {
                "short" => yaml_to_str!(a, v, short),
                "long" => yaml_to_str!(a, v, long),
                "help" => yaml_to_str!(a, v, help),
                "required" => yaml_to_bool!(a, v, required),
                "takes_value" => yaml_to_bool!(a, v, takes_value),
                "index" => yaml_to_u64!(a, v, index),
                "global" => yaml_to_bool!(a, v, global),
                "multiple" => yaml_to_bool!(a, v, multiple),
                "hidden" => yaml_to_bool!(a, v, hidden),
                "next_line_help" => yaml_to_bool!(a, v, next_line_help),
                "empty_values" => yaml_to_bool!(a, v, empty_values),
                "group" => yaml_to_str!(a, v, group),
                "number_of_values" => yaml_to_u64!(a, v, number_of_values),
                "max_values" => yaml_to_u64!(a, v, max_values),
                "min_values" => yaml_to_u64!(a, v, min_values),
                "value_name" => yaml_to_str!(a, v, value_name),
                "use_delimiter" => yaml_to_bool!(a, v, use_delimiter),
                "value_delimiter" => yaml_to_str!(a, v, value_delimiter),
                "required_unless" => yaml_to_str!(a, v, required_unless),
                "display_order" => yaml_to_usize!(a, v, display_order),
                "default_value" => yaml_to_str!(a, v, default_value),
                "value_names" => {
                    yaml_vec_or_str!(v, a, value_name)
                }
                "groups" => {
                    yaml_vec_or_str!(v, a, group)
                }
                "requires" => {
                    yaml_vec_or_str!(v, a, requires)
                }
                "conflicts_with" => {
                    yaml_vec_or_str!(v, a, conflicts_with)
                }
                "overrides_with" => {
                    yaml_vec_or_str!(v, a, overrides_with)
                }
                "possible_values" => {
                    yaml_vec_or_str!(v, a, possible_value)
                }
                "required_unless_one" => {
                    yaml_vec_or_str!(v, a, required_unless)
                }
                "required_unless_all" => {
                    a = yaml_vec_or_str!(v, a, required_unless);
                    a.setb(ArgSettings::RequiredUnlessAll);
                    a
                }
                s => {
                    panic!("Unknown Arg setting '{}' in YAML file for arg '{}'",
                           s,
                           name_str)
                }
            }
        }

        a
    }

    /// Creates a new instance of [`Arg`] from a usage string. Allows creation of basic settings
    /// for the [`Arg`]. The syntax is flexible, but there are some rules to follow.
    ///
    /// **NOTE**: Not all settings may be set using the usage string method. Some properties are
    /// only available via the builder pattern.
    ///
    /// **NOTE**: Only ASCII values are officially supported in [`Arg::from_usage`] strings. Some
    /// UTF-8 codepoints may work just fine, but this is not guaranteed.
    ///
    /// # Syntax
    ///
    /// Usage strings typically following the form:
    ///
    /// ```notrust
    /// [explicit name] [short] [long] [value names] [help string]
    /// ```
    ///
    /// This is not a hard rule as the attributes can appear in other orders. There are also
    /// several additional sigils which denote additional settings. Below are the details of each
    /// portion of the string.
    ///
    /// ### Explicit Name
    ///
    /// This is an optional field, if it's omitted the argument will use one of the additional
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
    /// ```notrust
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
    /// ```notrust
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
    /// ```notrust
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
    /// ```notrust
    /// -s, --some [FILE] 'some option'
    /// --rapid=<SPEED>... 'some required multiple option'
    /// ```
    ///
    /// ### Help String
    ///
    /// The help string is denoted between a pair of single quotes `''` and may contain any
    /// characters.
    ///
    /// Example help strings are as follows:
    ///
    /// ```notrust
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
    /// [`Arg`]: ./struct.Arg.html
    /// [`Arg::from_usage`]: ./struct.Arg.html#method.from_usage
    pub fn from_usage(u: &'a str) -> Self {
        let parser = UsageParser::from_usage(u);
        parser.parse()
    }

    /// Sets the short version of the argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `V` and `h` to the auto-generated `version` and
    /// `help` arguments respectively. You may use the uppercase `V` or lowercase `h` for your own
    /// arguments, in which case `clap` simply will not assign those to the auto-generated
    /// `version` or `help` arguments.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` character will be used as the [`short`] version
    ///
    /// # Examples
    ///
    /// To set [`short`] use a single valid UTF-8 code point. If you supply a leading `-` such as
    /// `-c`, the `-` will be stripped.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .short("c")
    /// # ;
    /// ```
    ///
    /// Setting [`short`] allows using the argument via a single hyphen (`-`) such as `-c`
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("shorttest")
    ///     .arg(Arg::with_name("config")
    ///         .short("c"))
    ///     .get_matches_from(vec![
    ///         "shorttest", "-c"
    ///     ]);
    ///
    /// assert!(m.is_present("config"));
    /// ```
    /// [`short`]: ./struct.Arg.html#method.short
    pub fn short<S: AsRef<str>>(mut self, s: S) -> Self {
        self.short = s.as_ref().trim_left_matches(|c| c == '-').chars().nth(0);
        self
    }

    /// Sets the long version of the argument without the preceding `--`.
    ///
    /// By default `clap` automatically assigns `version` and `help` to the auto-generated
    /// `version` and `help` arguments respectively. You may use the word `version` or `help` for
    /// the long form of your own arguments, in which case `clap` simply will not assign those to
    /// the auto-generated `version` or `help` arguments.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped
    ///
    /// # Examples
    ///
    /// To set `long` use a word containing valid UTF-8 codepoints. If you supply a double leading
    /// `--` such as `--config` they will be stripped. Hyphens in the middle of the word, however,
    /// will *not* be stripped (i.e. `config-file` is allowed)
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("cfg")
    ///     .long("config")
    /// # ;
    /// ```
    ///
    /// Setting `long` allows using the argument via a double hyphen (`--`) such as `--config`
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("longtest")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config"))
    ///     .get_matches_from(vec![
    ///         "longtest", "--config"
    ///     ]);
    ///
    /// assert!(m.is_present("cfg"));
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
    /// Any valid UTF-8 is allowed in the help text. The one exception is when one wishes to
    /// include a newline in the help text and have the following text be properly aligned with all
    /// the other help text. To include a newline **and** be properly aligned with all other
    /// arguments help text, it must be specified via a tag inside curly braces, like so `{n}`.
    /// Using the standard `\n` will produce a newline, but it will not be properly aligned.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .help("The config file used by the myprog")
    /// # ;
    /// ```
    ///
    /// Setting `help` displays a short message to the side of the argument when the user passes
    /// `-h` or `--help` (by default).
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("helptest")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .help("Some help text describing the --config arg"))
    ///     .get_matches_from(vec![
    ///         "shorttest", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays
    ///
    /// ```notrust
    /// helptest
    ///
    /// USAGE:
    ///    helptest [FLAGS]
    ///
    /// FLAGS:
    ///     --config     Some help text describing the --config arg
    /// -h, --help       Prints help information
    /// -V, --version    Prints version information
    /// ```
    pub fn help(mut self, h: &'b str) -> Self {
        self.help = Some(h);
        self
    }

    /// Sets whether or not the argument is required by default. Required by default means it is
    /// required, when no other conflicting rules have been evaluated. Conflicting rules take
    /// precedence over being required. **Default:** `false`
    ///
    /// **NOTE:** Flags (i.e. not positional, or arguments that take values) cannot be required by
    /// default. This is simply because if a flag should be required, it should simply be implied
    /// as no additional information is required from user. Flags by their very nature are simply
    /// yes/no, or true/false.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .required(true)
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::required(true)`] requires that the argument be used at runtime.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("longtest")
    ///     .arg(Arg::with_name("cfg")
    ///         .required(true)
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .get_matches_from_safe(vec![
    ///         "shorttest", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`Arg::required(true)`] and *not* supplying that argument is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("longtest")
    ///     .arg(Arg::with_name("cfg")
    ///         .required(true)
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .get_matches_from_safe(vec![
    ///         "shorttest"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::required(true)`]: ./struct.Arg.html#method.required
    pub fn required(self, r: bool) -> Self {
        if r {
            self.set(ArgSettings::Required)
        } else {
            self.unset(ArgSettings::Required)
        }
    }

    /// Sets an arg that override this arg's required setting. (i.e. this arg will be required
    /// unless this other argument is present).
    ///
    /// **Pro Tip:** Using [`Arg::required_unless`] implies [`Arg::required`] and is therefore not
    /// mandatory to also set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .required_unless("debug")
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::required_unless(name)`] requires that the argument be used at runtime
    /// *unless* `name` is present. In the following example, the required argument is *not*
    /// provided, but it's not an error because the `unless` arg has been supplied.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("unlesstest")
    ///     .arg(Arg::with_name("cfg")
    ///         .required_unless("dbg")
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::with_name("dbg")
    ///         .long("debug"))
    ///     .get_matches_from_safe(vec![
    ///         "unlesstest", "--debug"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`Arg::required_unless(name)`] and *not* supplying `name` or this arg is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("unlesstest")
    ///     .arg(Arg::with_name("cfg")
    ///         .required_unless("dbg")
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::with_name("dbg")
    ///         .long("debug"))
    ///     .get_matches_from_safe(vec![
    ///         "unlesstest"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::required_unless`]: ./struct.Arg.html#method.required_unless
    /// [`Arg::required`]: ./struct.Arg.html#method.required
    /// [`Arg::required_unless(name)`]: ./struct.Arg.html#method.required_unless
    pub fn required_unless(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.r_unless {
            vec.push(name);
        } else {
            self.r_unless = Some(vec![name]);
        }
        self.required(true)
    }

    /// Sets args that override this arg's required setting. (i.e. this arg will be required unless
    /// all these other arguments are present).
    ///
    /// **NOTE:** If you wish for this argument to only be required if *one of* these args are
    /// present see [`Arg::required_unless_one`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .required_unless_all(&["cfg", "dbg"])
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::required_unless_all(names)`] requires that the argument be used at runtime
    /// *unless* *all* the args in `names` are present. In the following example, the required
    /// argument is *not* provided, but it's not an error because all the `unless` args have been
    /// supplied.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("unlessall")
    ///     .arg(Arg::with_name("cfg")
    ///         .required_unless_all(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::with_name("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::with_name("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "unlessall", "--debug", "-i", "file"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`Arg::required_unless_all(names)`] and *not* supplying *all* of `names` or this
    /// arg is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("unlessall")
    ///     .arg(Arg::with_name("cfg")
    ///         .required_unless_all(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::with_name("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::with_name("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "unlessall"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::required_unless_one`]: ./struct.Arg.html#method.required_unless_one
    /// [`Arg::required_unless_all(names)`]: ./struct.Arg.html#method.required_unless_all
    pub fn required_unless_all(mut self, names: &[&'a str]) -> Self {
        if let Some(ref mut vec) = self.r_unless {
            for s in names {
                vec.push(s);
            }
        } else {
            self.r_unless = Some(names.iter().map(|s| *s).collect::<Vec<_>>());
        }
        self.setb(ArgSettings::RequiredUnlessAll);
        self.required(true)
    }

    /// Sets args that override this arg's [required] setting. (i.e. this arg will be required
    /// unless *at least one of* these other arguments are present).
    ///
    /// **NOTE:** If you wish for this argument to only be required if *all of* these args are
    /// present see [`Arg::required_unless_all`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .required_unless_all(&["cfg", "dbg"])
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::required_unless_one(names)`] requires that the argument be used at runtime
    /// *unless* *at least one of* the args in `names` are present. In the following example, the
    /// required argument is *not* provided, but it's not an error because one the `unless` args
    /// have been supplied.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("unlessone")
    ///     .arg(Arg::with_name("cfg")
    ///         .required_unless_one(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::with_name("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::with_name("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "unlessone", "--debug"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`Arg::required_unless_one(names)`] and *not* supplying *at least one of* `names`
    /// or this arg is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("unlessone")
    ///     .arg(Arg::with_name("cfg")
    ///         .required_unless_one(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::with_name("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::with_name("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "unlessone"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [required]: ./struct.Arg.html#method.required
    /// [`Arg::required_unless_one(names)`]: ./struct.Arg.html#method.required_unless_one
    /// [`Arg::required_unless_all`]: ./struct.Arg.html#method.required_unless_all
    pub fn required_unless_one(mut self, names: &[&'a str]) -> Self {
        if let Some(ref mut vec) = self.r_unless {
            for s in names {
                vec.push(s);
            }
        } else {
            self.r_unless = Some(names.iter().map(|s| *s).collect::<Vec<_>>());
        }
        self.required(true)
    }

    /// Sets a conflicting argument by name. I.e. when using this argument,
    /// the following argument can't be present and vice versa.
    ///
    /// **NOTE:** Conflicting rules take precedence over being required by default. Conflict rules
    /// only need to be set for one of the two arguments, they do not need to be set for each.
    ///
    /// **NOTE:** Defining a conflict is two-way, but does *not* need to defined for both arguments
    /// (i.e. if A conflicts with B, defining A.conflicts_with(B) is sufficient. You do not need
    /// need to also do B.conflicts_with(A))
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .conflicts_with("debug")
    /// # ;
    /// ```
    ///
    /// Setting conflicting argument, and having both arguments present at runtime is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("conflictions")
    ///     .arg(Arg::with_name("cfg")
    ///         .takes_value(true)
    ///         .conflicts_with("debug")
    ///         .long("config"))
    ///     .arg(Arg::with_name("debug")
    ///         .long("debug"))
    ///     .get_matches_from_safe(vec![
    ///         "conflictions", "--debug", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::ArgumentConflict);
    /// ```
    pub fn conflicts_with(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.blacklist {
            vec.push(name);
        } else {
            self.blacklist = Some(vec![name]);
        }
        self
    }

    /// The same as [`Arg::conflicts_with`] but allows specifying multiple two-way conlicts per
    /// argument.
    ///
    /// **NOTE:** Conflicting rules take precedence over being required by default. Conflict rules
    /// only need to be set for one of the two arguments, they do not need to be set for each.
    ///
    /// **NOTE:** Defining a conflict is two-way, but does *not* need to defined for both arguments
    /// (i.e. if A conflicts with B, defining A.conflicts_with(B) is sufficient. You do not need
    /// need to also do B.conflicts_with(A))
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .conflicts_with_all(&["debug", "input"])
    /// # ;
    /// ```
    ///
    /// Setting conflicting argument, and having any of the arguments present at runtime with a
    /// conflicting argument is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("conflictions")
    ///     .arg(Arg::with_name("cfg")
    ///         .takes_value(true)
    ///         .conflicts_with_all(&["debug", "input"])
    ///         .long("config"))
    ///     .arg(Arg::with_name("debug")
    ///         .long("debug"))
    ///     .arg(Arg::with_name("input")
    ///         .index(1))
    ///     .get_matches_from_safe(vec![
    ///         "conflictions", "--config", "file.conf", "file.txt"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::ArgumentConflict);
    /// ```
    /// [`Arg::conflicts_with`]: ./struct.Arg.html#method.conflicts_with
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
    /// **NOTE:** When an argument is overridden it is essentially as if it never was used, any
    /// conflicts, requirements, etc. are evaluated **after** all "overrides" have been removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("posix")
    ///     .arg(Arg::from_usage("-f, --flag 'some flag'")
    ///         .conflicts_with("debug"))
    ///     .arg(Arg::from_usage("-d, --debug 'other flag'"))
    ///     .arg(Arg::from_usage("-c, --color 'third flag'")
    ///         .overrides_with("flag"))
    ///     .get_matches_from(vec!["posix", "-f", "-d", "-c"]);
    ///                                 //    ^~~~~~~~~~~~^~~~~ flag is overridden by color
    ///
    /// assert!(m.is_present("color"));
    /// assert!(m.is_present("debug")); // even though flag conflicts with debug, it's as if flag
    ///                                 // was never used because it was overridden with color
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

    /// Sets multiple mutually overridable arguments by name. I.e. this argument and the following
    /// argument will override each other in POSIX style (whichever argument was specified at
    /// runtime **last** "wins")
    ///
    /// **NOTE:** When an argument is overridden it is essentially as if it never was used, any
    /// conflicts, requirements, etc. are evaluated **after** all "overrides" have been removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("posix")
    ///     .arg(Arg::from_usage("-f, --flag 'some flag'")
    ///         .conflicts_with("color"))
    ///     .arg(Arg::from_usage("-d, --debug 'other flag'"))
    ///     .arg(Arg::from_usage("-c, --color 'third flag'")
    ///         .overrides_with_all(&["flag", "debug"]))
    ///     .get_matches_from(vec!["posix", "-f", "-d", "-c"]);
    ///                                 //    ^~~~~~^~~~~~~~~ flag and debug are overridden by color
    ///
    /// assert!(m.is_present("color")); // even though flag conflicts with color, it's as if flag
    ///                                 // and debug were never used because they were overridden
    ///                                 // with color
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
    /// **NOTE:** [Conflicting] rules and [override] rules take precedence over being required
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .requires("input")
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::requires(name)`] requires that the argument be used at runtime if the
    /// defining argument is used. If the defining argument isn't used, the other argument isn't
    /// required
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("reqtest")
    ///     .arg(Arg::with_name("cfg")
    ///         .takes_value(true)
    ///         .requires("input")
    ///         .long("config"))
    ///     .arg(Arg::with_name("input")
    ///         .index(1))
    ///     .get_matches_from_safe(vec![
    ///         "reqtest"
    ///     ]);
    ///
    /// assert!(res.is_ok()); // We didn't use cfg, so input wasn't required
    /// ```
    ///
    /// Setting [`Arg::requires(name)`] and *not* supplying that argument is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("reqtest")
    ///     .arg(Arg::with_name("cfg")
    ///         .takes_value(true)
    ///         .requires("input")
    ///         .long("config"))
    ///     .arg(Arg::with_name("input")
    ///         .index(1))
    ///     .get_matches_from_safe(vec![
    ///         "reqtest", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::requires(name)`]: ./struct.Arg.html#method.requires
    /// [Conflicting]: ./struct.Arg.html#method.conflicts_with
    /// [override]: ./struct.Arg.html#method.overrides_with
    pub fn requires(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.requires {
            vec.push(name);
        } else {
            self.requires = Some(vec![name]);
        }
        self
    }

    /// Sets multiple arguments by names that are required when this one is present I.e. when
    /// using this argument, the following arguments *must* be present.
    ///
    /// **NOTE:** [Conflicting] rules and [override] rules take precedence over being required
    /// by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::with_name("config")
    ///     .requires_all(&["input", "output"])
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::requires_all(&[arg, arg2])`] requires that all the arguments be used at
    /// runtime if the defining argument is used. If the defining argument isn't used, the other
    /// argument isn't required
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("reqtest")
    ///     .arg(Arg::with_name("cfg")
    ///         .takes_value(true)
    ///         .requires("input")
    ///         .long("config"))
    ///     .arg(Arg::with_name("input")
    ///         .index(1))
    ///     .arg(Arg::with_name("output")
    ///         .index(2))
    ///     .get_matches_from_safe(vec![
    ///         "reqtest"
    ///     ]);
    ///
    /// assert!(res.is_ok()); // We didn't use cfg, so input and output weren't required
    /// ```
    ///
    /// Setting [`Arg::requires_all(&[arg, arg2])`] and *not* supplying all the arguments is an
    /// error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("reqtest")
    ///     .arg(Arg::with_name("cfg")
    ///         .takes_value(true)
    ///         .requires_all(&["input", "output"])
    ///         .long("config"))
    ///     .arg(Arg::with_name("input")
    ///         .index(1))
    ///     .arg(Arg::with_name("output")
    ///         .index(2))
    ///     .get_matches_from_safe(vec![
    ///         "reqtest", "--config", "file.conf", "in.txt"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// // We didn't use output
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [Conflicting]: ./struct.Arg.html#method.conflicts_with
    /// [override]: ./struct.Arg.html#method.overrides_with
    /// [`Arg::requires_all(&[arg, arg2])`]: ./struct.Arg.html#method.requires_all
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

    /// Specifies that the argument takes a value at run time.
    ///
    /// **NOTE:** values for arguments may be specified in any of the following methods
    ///
    /// * Using a space such as `-o value` or `--option value`
    /// * Using an equals and no space such as `-o=value` or `--option=value`
    /// * Use a short and no space such as `-ovalue`
    ///
    /// **NOTE:** By default, args which allow [multiple values] are delimited by commas, meaning
    /// `--option=val1,val2,val3` is three values for the `--option` argument. If you wish to
    /// change the delimiter to another character you can use [`Arg::value_delimiter(char)`],
    /// alternatively you can turn delimiting values **OFF** by using [`Arg::use_delimiter(false)`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .takes_value(true)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("posvals")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["posvals", "--mode", "fast"]);
    ///
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast"));
    /// ```
    /// [`Arg::value_delimiter(char)`]: ./struct.Arg.html#method.value_delimiter
    /// [`Arg::use_delimiter(false)`]: ./struct.Arg.html#method.use_delimiter
    /// [multiple values]: ./struct.Arg.html#method.multiple
    pub fn takes_value(self, tv: bool) -> Self {
        if tv {
            self.set(ArgSettings::TakesValue)
        } else {
            self.unset(ArgSettings::TakesValue)
        }
    }

    /// Specifies the index of a positional argument **starting at** 1.
    ///
    /// **NOTE:** The index refers to position according to **other positional argument**. It does
    /// not define position in the argument list as a whole.
    ///
    /// **NOTE:** If no [`Arg::short`], or [`Arg::long`] have been defined, you can optionally
    /// leave off the `index` method, and the index will be assigned in order of evaluation.
    /// Utilizing the `index` method allows for setting indexes out of order
    ///
    /// **NOTE:** When utilized with [`Arg::multiple(true)`], only the **last** positional argument
    /// may be defined as multiple (i.e. with the highest index)
    ///
    /// # Panics
    ///
    /// Although not in this method directly, [`App`] will [`panic!`] if indexes are skipped (such
    /// as defining `index(1)` and `index(3)` but not `index(2)`, or a positional argument is
    /// defined as multiple and is not the highest index
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("config")
    ///     .index(1)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("posvals")
    ///     .arg(Arg::with_name("mode")
    ///         .index(1))
    ///     .arg(Arg::with_name("debug")
    ///         .long("debug"))
    ///     .get_matches_from(vec!["posvals", "--debug", "fast"]);
    ///
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast")); // notice index(1) means "first positional"
    ///                                               // *not* first argument
    /// ```
    /// [`Arg::short`]: ./struct.Arg.html#method.short
    /// [`Arg::long`]: ./struct.Arg.html#method.long
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    /// [`App`]: ./struct.App.html
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn index(mut self, idx: u64) -> Self {
        self.index = Some(idx);
        self
    }

    /// Specifies that the argument may appear more than once. For flags, this results
    /// in the number of occurrences of the flag being recorded. For example `-ddd` or `-d -d -d`
    /// would count as three occurrences. For options there is a distinct difference in multiple
    /// occurrences vs multiple values.
    ///
    /// For example, `--opt val1 val2` is one occurrence, but two values. Whereas
    /// `--opt val1 --opt val2` is two occurrences.
    ///
    /// **WARNING:**
    ///
    /// Setting `multiple(true)` for an [option] with no other details, allows multiple values
    /// **and** multiple occurrences because it isn't possible to have more occurrences than values for
    /// options. Because multiple values are allowed, `--option val1 val2 val3` is perfectly valid,
    /// be careful when designing a CLI where positional arguments are expected after a option which
    /// accepts multiple values, as `clap` will continue parsing *values* until it reaches the max
    /// or specific number of values defined, or another flag or option.
    ///
    /// **Pro Tip**:
    ///
    /// It's possible to define an option which allows multiple occurrences, but only one value per
    /// occurrence. To do this use [`Arg::number_of_values(1)`] in coordination with
    /// [`Arg::multiple(true)`].
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
    /// An example with flags
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("mults")
    ///     .arg(Arg::with_name("verbose")
    ///         .multiple(true)
    ///         .short("v"))
    ///     .get_matches_from(vec!["mults", "-v", "-v", "-v"]); // note, -vvv would have same result
    ///
    /// assert!(m.is_present("verbose"));
    /// assert_eq!(m.occurrences_of("verbose"), 3);
    /// ```
    ///
    /// An example with options
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("mults")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .short("F"))
    ///     .get_matches_from(vec!["mults", "-F", "file1", "file2", "file3"]);
    ///
    /// assert!(m.is_present("file"));
    /// assert_eq!(m.occurrences_of("file"), 1); // notice only one occurrence
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// ```
    /// This is functionally equivilant to the example above
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("mults")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .short("F"))
    ///     .get_matches_from(vec!["mults", "-F", "file1", "-F", "file2", "-F", "file3"]);
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    ///
    /// assert!(m.is_present("file"));
    /// assert_eq!(m.occurrences_of("file"), 3); // Notice 3 occurrences
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// ```
    ///
    /// A common mistake is to define an option which allows multiples, and a positional argument
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("mults")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from(vec!["mults", "-F", "file1", "file2", "file3", "word"]);
    ///
    /// assert!(m.is_present("file"));
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3", "word"]); // wait...what?!
    /// assert!(!m.is_present("word")); // but we clearly used word!
    /// ```
    /// The problem is clap doesn't know when to stop parsing values for "files". This is further
    /// compounded by if we'd said `word -F file1 file2` it would have worked fine, so it would
    /// appear to only fail sometimes...not good!
    ///
    /// A solution for the example above is to specify that `-F` only accepts one value, but is
    /// allowed to appear multiple times
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("mults")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .number_of_values(1)
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from(vec!["mults", "-F", "file1", "-F", "file2", "-F", "file3", "word"]);
    ///
    /// assert!(m.is_present("file"));
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// assert!(m.is_present("word"));
    /// assert_eq!(m.value_of("word"), Some("word"));
    /// ```
    /// As a final example, notice if we define [`Arg::number_of_values(1)`] and try to run the
    /// problem example above, it would have been a runtime error with a pretty message to the
    /// user :)
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("mults")
    ///     .arg(Arg::with_name("file")
    ///         .multiple(true)
    ///         .takes_value(true)
    ///         .number_of_values(1)
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from_safe(vec!["mults", "-F", "file1", "file2", "file3", "word"]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [option]: ./struct.Arg.html#method.takes_value
    /// [`Arg::number_of_values(1)`]: ./struct.Arg.html#method.number_of_values
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn multiple(self, multi: bool) -> Self {
        if multi {
            self.set(ArgSettings::Multiple)
        } else {
            self.unset(ArgSettings::Multiple)
        }
    }

    /// Specifies that an argument can be matched to all child [`SubCommand`]s.
    ///
    /// **NOTE:** Global arguments *only* propagate down, **not** up (to parent commands)
    ///
    /// **NOTE:** Global arguments *cannot* be [required].
    ///
    /// **NOTE:** Global arguments, when matched, *only* exist in the command's matches that they
    /// were matched to. For example, if you defined a `--flag` global argument in the top most
    /// parent command, but the user supplied the arguments `top cmd1 cmd2 --flag` *only* `cmd2`'s
    /// [`ArgMatches`] would return `true` if tested for [`ArgMatches::is_present("flag")`].
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
    ///
    /// For example, assume an appliction with two subcommands, and you'd like to define a
    /// `--verbose` flag that can be called on any of the subcommands and parent, but you don't
    /// want to clutter the source with three duplicate [`Arg`] definitions.
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand};
    /// let m = App::new("mults")
    ///     .arg(Arg::with_name("verb")
    ///         .long("verbose")
    ///         .short("v")
    ///         .global(true))
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .subcommand(SubCommand::with_name("do-stuff"))
    ///     .get_matches_from(vec!["mults", "do-stuff", "--verbose"]);
    ///
    /// assert_eq!(m.subcommand_name(), Some("do-stuff"));
    /// let sub_m = m.subcommand_matches("do-stuff").unwrap();
    /// assert!(sub_m.is_present("verb"));
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [required]: ./struct.Arg.html#method.required
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    /// [`ArgMatches::is_present("flag")`]: ./struct.ArgMatches.html#method.is_present
    /// [`Arg`]: ./struct.Arg.html
    pub fn global(self, g: bool) -> Self {
        if g {
            self.set(ArgSettings::Global)
        } else {
            self.unset(ArgSettings::Global)
        }
    }

    /// Allows an argument to accept explicitly empty values. An empty value must be specified at
    /// the command line with an explicit `""`, or `''`
    ///
    /// **NOTE:** Defaults to `true` (Explicitly empty values are allowed)
    ///
    /// **NOTE:** Implicitly sets [`Arg::takes_value(true)`] when set to `false`
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
    /// The default is to allow empty values, such as `--option ""` would be an empty value. But
    /// we can change to make empty values become an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("evals")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .short("v")
    ///         .empty_values(false))
    ///     .get_matches_from_safe(vec!["evals", "--config="]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
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
    /// Setting `hidden(true)` will hide the argument when displaying help text
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("helptest")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .hidden(true)
    ///         .help("Some help text describing the --config arg"))
    ///     .get_matches_from(vec![
    ///         "shorttest", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays
    ///
    /// ```notrust
    /// helptest
    ///
    /// USAGE:
    ///    helptest [FLAGS]
    ///
    /// FLAGS:
    /// -h, --help       Prints help information
    /// -V, --version    Prints version information
    /// ```
    pub fn hidden(self, h: bool) -> Self {
        if h {
            self.set(ArgSettings::Hidden)
        } else {
            self.unset(ArgSettings::Hidden)
        }
    }

    /// Specifies a list of possible values for this argument. At runtime, `clap` verifies that
    /// only one of the specified values was used, or fails with an error message.
    ///
    /// **NOTE:** This setting only applies to [options] and [positional arguments]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("mode")
    ///     .takes_value(true)
    ///     .possible_values(&["fast", "slow", "medium"])
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("posvals")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_values(&["fast", "slow", "medium"]))
    ///     .get_matches_from(vec!["posvals", "--mode", "fast"]);
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast"));
    /// ```
    ///
    /// The next example shows a failed parse from using a value which wasn't defined as one of the
    /// possible values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("posvals")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_values(&["fast", "slow", "medium"]))
    ///     .get_matches_from_safe(vec!["myprog", "--mode", "wrong"]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::InvalidValue);
    /// ```
    /// [options]: ./struct.Arg.html#method.takes_value
    /// [positional arguments]: ./struct.Arg.html#method.index
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

    /// Specifies a possible value for this argument, one at a time. At runtime, `clap` verifies
    /// that only one of the specified values was used, or fails with error message.
    ///
    /// **NOTE:** This setting only applies to [options] and [positional arguments]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("mode")
    ///     .takes_value(true)
    ///     .possible_value("fast")
    ///     .possible_value("slow")
    ///     .possible_value("medium")
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("posvals")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_value("fast")
    ///         .possible_value("slow")
    ///         .possible_value("medium"))
    ///     .get_matches_from(vec!["posvals", "--mode", "fast"]);
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast"));
    /// ```
    ///
    /// The next example shows a failed parse from using a value which wasn't defined as one of the
    /// possible values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("posvals")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_value("fast")
    ///         .possible_value("slow")
    ///         .possible_value("medium"))
    ///     .get_matches_from_safe(vec!["myprog", "--mode", "wrong"]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::InvalidValue);
    /// ```
    /// [options]: ./struct.Arg.html#method.takes_value
    /// [positional arguments]: ./struct.Arg.html#method.index
    pub fn possible_value(mut self, name: &'b str) -> Self {
        if let Some(ref mut vec) = self.possible_vals {
            vec.push(name);
        } else {
            self.possible_vals = Some(vec![name]);
        }
        self
    }

    /// Specifies the name of the [`ArgGroup`] the argument belongs to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .long("debug")
    ///     .group("mode")
    /// # ;
    /// ```
    ///
    /// Multiple arguments can be a member of a single group and then the group checked as if it
    /// was one of said arguments.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("groups")
    ///     .arg(Arg::with_name("debug")
    ///         .long("debug")
    ///         .group("mode"))
    ///     .arg(Arg::with_name("verbose")
    ///         .long("verbose")
    ///         .group("mode"))
    ///     .get_matches_from(vec!["posvals", "--debug"]);
    /// assert!(m.is_present("mode"));
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    pub fn group(mut self, name: &'a str) -> Self {
        if let Some(ref mut vec) = self.requires {
            vec.push(name);
        } else {
            self.group = Some(vec![name]);
        }
        self
    }

    /// Specifies the names of multiple [`ArgGroup`]'s the argument belongs to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("debug")
    ///     .long("debug")
    ///     .groups(&["mode", "verbosity"])
    /// # ;
    /// ```
    ///
    /// Arguments can be members of multiple groups and then the group checked as if it
    /// was one of said arguments.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("groups")
    ///     .arg(Arg::with_name("debug")
    ///         .long("debug")
    ///         .groups(&["mode", "verbosity"]))
    ///     .arg(Arg::with_name("verbose")
    ///         .long("verbose")
    ///         .groups(&["mode", "verbosity"]))
    ///     .get_matches_from(vec!["posvals", "--debug"]);
    /// assert!(m.is_present("mode"));
    /// assert!(m.is_present("verbosity"));
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    pub fn groups(mut self, names: &[&'a str]) -> Self {
        if let Some(ref mut vec) = self.group {
            for s in names {
                vec.push(s);
            }
        } else {
            self.group = Some(names.into_iter().map(|s| *s).collect::<Vec<_>>());
        }
        self
    }

    /// Specifies how many values are required to satisfy this argument. For example, if you had a
    /// `-f <file>` argument where you wanted exactly 3 'files' you would set
    /// `.number_of_values(3)`, and this argument wouldn't be satisfied unless the user provided
    /// 3 and only 3 values.
    ///
    /// **NOTE:** Does *not* require [`Arg::multiple(true)`] to be set. Setting
    /// [`Arg::multiple(true)`] would allow `-f <file> <file> <file> -f <file> <file> <file>` where
    /// as *not* setting [`Arg::multiple(true)`] would only allow one occurrence of this argument.
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
    ///
    /// Not supplying the correct number of values is an error
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("numvals")
    ///     .arg(Arg::with_name("file")
    ///         .takes_value(true)
    ///         .number_of_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec!["mults", "-F", "file1"]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
    /// ```
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn number_of_values(mut self, qty: u64) -> Self {
        self.num_vals = Some(qty);
        self
    }

    /// Allows one to perform a custom validation on the argument value. You provide a closure
    /// which accepts a [`String`] value, and return a [`Result`] where the [`Err(String)`] is a
    /// message displayed to the user.
    ///
    /// **NOTE:** The error message does *not* need to contain the `error:` portion, only the
    /// message.
    ///
    /// **NOTE:** There is a small performance hit for using validators, as they are implemented
    /// with [`Rc`] pointers. And the value to be checked will be allocated an extra time in order
    /// to to be passed to the closure. This performance hit is extremely minimal in the grand
    /// scheme of things.
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
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Err(String)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
    pub fn validator<F>(mut self, f: F) -> Self
        where F: Fn(String) -> Result<(), String> + 'static
    {
        self.validator = Some(Rc::new(f));
        self
    }

    /// Specifies the *maximum* number of values are for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted up to 3 'files' you would set `.max_values(3)`, and
    /// this argument would be satisfied if the user provided, 1, 2, or 3 values.
    ///
    /// **NOTE:** This does *not* implicitly set [`Arg::multiple(true)`]. This is because
    /// `-o val -o val` is multiple occurrences but a single value and `-o val1 val2` is a single
    /// occurence with multple values. For positional arguments this **does** set
    /// [`Arg::multiple(true)`] because there is no way to determine the difference between multiple
    /// occurences and multiple values.
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
    ///
    /// Supplying less than the maximum number of values is allowed
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("numvals")
    ///     .arg(Arg::with_name("file")
    ///         .takes_value(true)
    ///         .max_values(3)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec!["mults", "-F", "file1", "file2"]);
    ///
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2"]);
    /// ```
    ///
    /// Supplying more than the maximum number of values is an error
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("numvals")
    ///     .arg(Arg::with_name("file")
    ///         .takes_value(true)
    ///         .max_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec!["mults", "-F", "file1", "file2", "file3"]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::TooManyValues);
    /// ```
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn max_values(mut self, qty: u64) -> Self {
        self.max_vals = Some(qty);
        self
    }

    /// Specifies the *minimum* number of values for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted at least 2 'files' you would set
    /// `.min_values(2)`, and this argument would be satisfied if the user provided, 2 or more
    /// values.
    ///
    /// **NOTE:** This does not implicitly set [`Arg::multiple(true)`]. This is because
    /// `-o val -o val` is multiple occurrences but a single value and `-o val1 val2` is a single
    /// occurence with multiple values. For positional arguments this **does** set
    /// [`Arg::multiple(true)`] because there is no way to determine the difference between multiple
    /// occurences and multiple values.
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
    ///
    /// Supplying more than the minimum number of values is allowed
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("numvals")
    ///     .arg(Arg::with_name("file")
    ///         .takes_value(true)
    ///         .min_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec!["mults", "-F", "file1", "file2", "file3"]);
    ///
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// ```
    ///
    /// Supplying less than the minimum number of values is an error
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("numvals")
    ///     .arg(Arg::with_name("file")
    ///         .takes_value(true)
    ///         .min_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec!["mults", "-F", "file1"]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::TooFewValues);
    /// ```
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn min_values(mut self, qty: u64) -> Self {
        self.min_vals = Some(qty);
        self.set(ArgSettings::TakesValue)
    }

    /// Specifies whether or not an argument should allow grouping of multiple values via a
    /// delimiter. I.e. should `--option=val1,val2,val3` be parsed as three values (`val1`, `val2`,
    /// and `val3`) or as a single value (`val1,val2,val3`). Defaults to using `,` (comma) as the
    /// value delimiter for all arguments that accept values (options and positional arguments)
    ///
    /// **NOTE:** The default is `true`. Setting the value to `true` will reset any previous use of
    /// [`Arg::value_delimiter`] back to the default of `,` (comma).
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
    /// [`Arg::value_delimiter`]: ./struct.Arg.html#method.value_delimiter
    pub fn use_delimiter(mut self, d: bool) -> Self {
        if d {
            self.val_delim = Some(',');
            self.set(ArgSettings::UseValueDelimiter)
        } else {
            self.val_delim = None;
            self.unset(ArgSettings::UseValueDelimiter)
        }
    }

    /// Specifies that *multiple values* may only be set using the delimiter. This means if an
    /// if an option is encountered, and no delimiter is found, it automatically assumed that no
    /// additional values for that option follow. This is unlike the default, where it is generally
    /// assumed that more values will follow regardless of whether or not a delimiter is used.
    ///
    /// **NOTE:** The default is `false`.
    ///
    /// **NOTE:** It's a good idea to inform the user that use of a delimiter is required, either
    /// through help text or other means.
    ///
    /// # Examples
    ///
    /// These examples demonstrate what happens when `require_delimiter(true)` is used. Notice
    /// everything works in this first example, as we use a delimiter, as expected.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let delims = App::new("reqdelims")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .multiple(true)
    ///         .require_delimiter(true))
    ///     // Simulate "$ reqdelims -o val1,val2,val3"
    ///     .get_matches_from(vec![
    ///         "reqdelims", "-o", "val1,val2,val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("opt"));
    /// assert_eq!(delims.values_of("opt").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// In this next example, we will *not* use a delimiter. Notice it's now an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("reqdelims")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .multiple(true)
    ///         .require_delimiter(true))
    ///     // Simulate "$ reqdelims -o val1 val2 val3"
    ///     .get_matches_from_safe(vec![
    ///         "reqdelims", "-o", "val1", "val2", "val3",
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// assert_eq!(err.kind, ErrorKind::UnknownArgument);
    /// ```
    /// What's happening is `-o` is getting `val1`, and because delimiters are required yet none
    /// were present, it stops parsing `-o`. At this point it reaches `val2` and because no
    /// positional arguments have been defined, it's an error of an unexpected argument.
    ///
    /// In this final example, we contrast the above with `clap`'s default behavior where the above
    /// is *not* an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let delims = App::new("reqdelims")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .multiple(true))
    ///     // Simulate "$ reqdelims -o val1 val2 val3"
    ///     .get_matches_from(vec![
    ///         "reqdelims", "-o", "val1", "val2", "val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("opt"));
    /// assert_eq!(delims.values_of("opt").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    pub fn require_delimiter(mut self, d: bool) -> Self {
        if d {
            self.setb(ArgSettings::UseValueDelimiter);
            self.set(ArgSettings::RequireDelimiter)
        } else {
            self.unsetb(ArgSettings::UseValueDelimiter);
            self.unset(ArgSettings::RequireDelimiter)
        }
    }

    /// Specifies the separator to use when values are clumped together, defaults to `,` (comma).
    ///
    /// **NOTE:** implicitly sets [`Arg::use_delimiter(true)`]
    ///
    /// **NOTE:** implicitly sets [`Arg::takes_value(true)`]
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
    /// [`Arg::use_delimiter(true)`]: ./struct.Arg.html#method.use_delimiter
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    pub fn value_delimiter(mut self, d: &str) -> Self {
        self = self.set(ArgSettings::TakesValue);
        self = self.set(ArgSettings::UseValueDelimiter);
        self.val_delim = Some(d.chars()
                               .nth(0)
                               .expect("Failed to get value_delimiter from arg"));
        self
    }

    /// Specify multiple names for values of option arguments. These names are cosmetic only, used
    /// for help and usage strings only. The names are **not** used to access arguments. The values
    /// of the arguments are accessed in numeric order (i.e. if you specify two names `one` and
    /// `two` `one` will be the first matched value, `two` will be the second).
    ///
    /// This setting can be very helpful when describing the type of input the user should be
    /// using, such as `FILE`, `INTERFACE`, etc. Although not required, it's somewhat convention to
    /// use all capital letters for the value name.
    ///
    /// **Pro Tip:** It may help to use [`Arg::next_line_help(true)`] if there are long, or
    /// multiple value names in order to not throw off the help text alignment of all options.
    ///
    /// **NOTE:** This implicitly sets [`Arg::number_of_values`] if the number of value names is
    /// greater than one. I.e. be aware that the number of "names" you set for the values, will be
    /// the *exact* number of values required to satisfy this argument
    ///
    /// **NOTE:** implicitly sets [`Arg::takes_value(true)`]
    ///
    /// **NOTE:** Does *not* require or imply [`Arg::multiple(true)`].
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
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let app = App::new("valnames")
    ///     .arg(Arg::with_name("io")
    ///         .long("io-files")
    ///         .value_names(&["INFILE", "OUTFILE"]))
    ///     .get_matches_from(vec![
    ///         "valnames", "--help"
    ///     ]);
    /// ```
    /// Running the above program produces the following output
    ///
    /// ```notrust
    /// valnames
    ///
    /// USAGE:
    ///    valnames [FLAGS] [OPTIONS]
    ///
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// OPTIONS:
    ///     --io-files <INFILE> <OUTFILE>    Some help text
    /// ```
    /// [`Arg::next_line_help(true)`]: ./struct.Arg.html#method.next_line_help
    /// [`Arg::number_of_values`]: ./struct.Arg.html#method.number_of_values
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn value_names(mut self, names: &[&'b str]) -> Self {
        self.setb(ArgSettings::TakesValue);
        if let Some(ref mut vals) = self.val_names {
            let mut l = vals.len();
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

    /// Specifies the name for value of [option] or [positional] arguments inside of help
    /// documentation. This name is cosmetic only, the name is **not** used to access arguments.
    /// This setting can be very helpful when describing the type of input the user should be
    /// using, such as `FILE`, `INTERFACE`, etc. Although not required, it's somewhat convention to
    /// use all capital letters for the value name.
    ///
    /// **NOTE:** implicitly sets [`Arg::takes_value(true)`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::with_name("cfg")
    ///     .long("config")
    ///     .value_name("FILE")
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let app = App::new("valnames")
    ///     .arg(Arg::with_name("config")
    ///         .long("config")
    ///         .value_name("FILE"))
    ///     .get_matches_from(vec![
    ///         "valnames", "--help"
    ///     ]);
    /// ```
    /// Running the above program produces the following output
    ///
    /// ```notrust
    /// valnames
    ///
    /// USAGE:
    ///    valnames [FLAGS] [OPTIONS]
    ///
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// OPTIONS:
    ///     --config <FILE>     Some help text
    /// ```
    /// [option]: ./struct.Arg.html#method.takes_value
    /// [positional]: ./struct.Arg.html#method.index
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
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

    /// Specifies the value of the argument when *not* specified at runtime.
    ///
    /// **NOTE:** If the user *does not* use this argument at runtime, [`ArgMatches::occurrences_of`]
    /// will return `0` even though the [`ArgMatches::value_of`] will return the default specified.
    ///
    /// **NOTE:** If the user *does not* use this argument at runtime [`ArgMatches::is_present`] will
    /// still return `true`. If you wish to determine whether the argument was used at runtime or
    /// not, consider [`ArgMatches::occurrences_of`] which will return `0` if the argument was *not*
    /// used at runtmie.
    ///
    /// **NOTE:** This implicitly sets [`Arg::takes_value(true)`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("defvals")
    ///     .arg(Arg::with_name("opt")
    ///         .long("myopt")
    ///         .default_value("myval"))
    ///     .get_matches_from(vec![
    ///         "defvals"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("opt"), Some("myval"));
    /// assert!(m.is_present("opt"));
    /// assert_eq!(m.occurrences_of("opt"), 0);
    /// ```
    /// [`ArgMatches::occurrences_of`]: /struct.ArgMatches.html#method.occurrences_of
    /// [`ArgMatches::value_of`]: ./struct.ArgMatches.html#method.value_of
    /// [`Arg::takes_value(true)`]: /struct.Arg.html#method.takes_value
    /// [`ArgMatches::is_present`]: /struct.ArgMatches.html#method.is_present
    pub fn default_value(mut self, val: &'a str) -> Self {
        self.setb(ArgSettings::TakesValue);
        self.default_val = Some(val);
        self
    }

    /// When set to `true` the help string will be displayed on the line after the argument and
    /// indented once. This can be helpful for arguments with very long or complex help messages.
    /// This can also be helpful for arguments with very long flag names, or many/long value names.
    ///
    /// **NOTE:** To apply this setting to all arguments consider using
    /// [`AppSettings::NextLineHelp`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("nlh")
    ///     .arg(Arg::with_name("opt")
    ///         .long("long-option-flag")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .value_names(&["value1", "value2"])
    ///         .help("Some really long help and complex{n}\
    ///                help that makes more sense to be{n}\
    ///                on a line after the option")
    ///         .next_line_help(true))
    ///     .get_matches_from(vec![
    ///         "nlh", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays the following help message
    ///
    /// ```notrust
    /// nlh
    ///
    /// USAGE:
    ///     nlh [FLAGS] [OPTIONS]
    ///
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// OPTIONS:
    ///     -o, --long-option-flag <value1> <value2>
    ///         Some really long help and complex
    ///         help that makes more sense to be
    ///         on a line after the option
    /// ```
    /// [`AppSettings::NextLineHelp`]: ./enum.AppSettings.html#variant.NextLineHelp
    pub fn next_line_help(mut self, nlh: bool) -> Self {
        if nlh {
            self.setb(ArgSettings::NextLineHelp);
        } else {
            self.unsetb(ArgSettings::NextLineHelp);
        }
        self
    }

    /// Allows custom ordering of args within the help message. Args with a lower value will be
    /// displayed first in the help message. This is helpful when one would like to emphasise
    /// frequently used args, or prioritize those towards the top of the list. Duplicate values
    /// **are** allowed. Args with duplicate display orders will be displayed in alphabetical
    /// order.
    ///
    /// **NOTE:** The default is 999 for all arguments.
    ///
    /// **NOTE:** This setting is ignored for [positional arguments] which are always displayed in
    /// [index] order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("cust-ord")
    ///     .arg(Arg::with_name("a") // Typically args are grouped alphabetically by name.
    ///                              // Args without a display_order have a value of 999 and are
    ///                              // displayed alphabetically with all other 999 valued args.
    ///         .long("long-option")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .help("Some help and text"))
    ///     .arg(Arg::with_name("b")
    ///         .long("other-option")
    ///         .short("O")
    ///         .takes_value(true)
    ///         .display_order(1)   // In order to force this arg to appear *first*
    ///                             // all we have to do is give it a value lower than 999.
    ///                             // Any other args with a value of 1 will be displayed
    ///                             // alphabetically with this one...then 2 values, then 3, etc.
    ///         .help("I should be first!"))
    ///     .get_matches_from(vec![
    ///         "cust-ord", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays the following help message
    ///
    /// ```notrust
    /// cust-ord
    ///
    /// USAGE:
    ///     cust-ord [FLAGS] [OPTIONS]
    ///
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// OPTIONS:
    ///     -O, --other-option <b>    I should be first!
    ///     -o, --long-option <a>     Some help and text
    /// ```
    /// [positional arguments]: ./struct.Arg.html#method.index
    /// [index]: ./struct.Arg.html#method.index
    pub fn display_order(mut self, ord: usize) -> Self {
        self.disp_ord = ord;
        self
    }

    /// Checks if one of the [`ArgSettings`] settings is set for the argument
    /// [`ArgSettings`]: ./enum.ArgSettings.html
    pub fn is_set(&self, s: ArgSettings) -> bool {
        self.settings.is_set(s)
    }

    /// Sets one of the [`ArgSettings`] settings for the argument
    /// [`ArgSettings`]: ./enum.ArgSettings.html
    pub fn set(mut self, s: ArgSettings) -> Self {
        self.setb(s);
        self
    }

    /// Unsets one of the [`ArgSettings`] settings for the argument
    /// [`ArgSettings`]: ./enum.ArgSettings.html
    pub fn unset(mut self, s: ArgSettings) -> Self {
        self.unsetb(s);
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
}

impl<'a, 'b, 'z> From<&'z Arg<'a, 'b>> for Arg<'a, 'b> {
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
            group: a.group.clone(),
            validator: a.validator.clone(),
            overrides: a.overrides.clone(),
            settings: a.settings,
            val_delim: a.val_delim,
            default_val: a.default_val,
            disp_ord: a.disp_ord,
            r_unless: a.r_unless.clone(),
        }
    }
}

impl<'a, 'b> Clone for Arg<'a, 'b> {
    fn clone(&self) -> Self {
        Arg {
            name: self.name,
            short: self.short,
            long: self.long,
            help: self.help,
            index: self.index,
            possible_vals: self.possible_vals.clone(),
            blacklist: self.blacklist.clone(),
            requires: self.requires.clone(),
            num_vals: self.num_vals,
            min_vals: self.min_vals,
            max_vals: self.max_vals,
            val_names: self.val_names.clone(),
            group: self.group.clone(),
            validator: self.validator.clone(),
            overrides: self.overrides.clone(),
            settings: self.settings,
            val_delim: self.val_delim,
            default_val: self.default_val,
            disp_ord: self.disp_ord,
            r_unless: self.r_unless.clone(),
        }
    }
}
