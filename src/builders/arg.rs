use std::borrow::Cow;
use std::fmt::{self, Formatter, Display, Error};
use std::result::Result as StdResult;
use std::rc::Rc;
use std::ffi::{OsString, OsStr};
#[cfg(target_os = "windows")]
use osstringext::OsStrExt3;
#[cfg(not(target_os = "windows"))]
use std::os::unix::ffi::OsStrExt;
// @TODO-v3-beta: remove
#[cfg(feature = "yaml")]
use std::collections::BTreeMap;

use vec_map::VecMap;
// @TODO-v3-beta: remove
#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

use INTERNAL_ERROR_MSG;
use ArgSettings;
use builders::arg_settings::ArgFlags;
use builders::UsageParser;
use parsing::DispOrder;

// Gives the default display order value
#[doc(hidden)]
#[cfg(feature = "serde")]
pub fn default_dispaly_order() -> usize { 999 }


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
/// let cfg = Arg::new("config")
///       .short("c")
///       .long("config")
///       .takes_value(true)
///       .value_name("FILE")
///       .help("Provides a config file to myprog");
/// // Using a usage string (setting a similar argument to the one above)
/// let input = Arg::from("-i, --input=[FILE] 'Provides an input file to the program'");
/// ```
/// [`Arg`]: ./struct.Arg.html
#[derive(Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Arg<'a, 'b>
where
    'a: 'b,
{
    #[doc(hidden)]
    pub name: &'a str,
    #[doc(hidden)]
    pub help: Option<&'b str>,
    #[doc(hidden)]
    pub long_help: Option<&'b str>,
    #[doc(hidden)]
    pub conflicts_with: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub settings: Vec<ArgSettings>,
    #[doc(hidden)]
    pub required_unless: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub overrides_with: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub groups: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub requires: Option<Vec<&'a str>>,
    #[doc(hidden)]
    pub requires_ifs: Option<Vec<(&'b str, &'a str)>>,
    #[doc(hidden)]
    pub required_ifs: Option<Vec<(&'a str, &'b str)>>,
    #[doc(hidden)]
    pub short: Option<char>,
    #[doc(hidden)]
    pub index: Option<usize>,
    #[doc(hidden)]
    pub long: Option<&'b str>,
    #[doc(hidden)]
    pub aliases: Option<Vec<&'b str>>,
    #[doc(hidden)]
    pub visible_aliases: Option<Vec<&'b str>>,
    #[doc(hidden)]
    pub possible_values: Option<Vec<&'b str>>,
    #[doc(hidden)]
    pub value_names: Option<VecMap<&'b str>>,
    #[doc(hidden)]
    pub number_of_values: Option<usize>,
    #[doc(hidden)]
    pub max_values: Option<usize>,
    #[doc(hidden)]
    pub min_values: Option<usize>,
    #[doc(hidden)]
    pub value_delimiter: Option<char>,
    #[doc(hidden)]
    pub default_value: Option<&'b OsStr>,
    #[doc(hidden)]
    pub default_value_ifs: Option<VecMap<(&'a str, Option<&'b OsStr>, &'b OsStr)>>,
    #[doc(hidden)]
    pub value_terminator: Option<&'b str>,
    #[cfg_attr(feature = "serde", serde(default = "default_display_order"))]
    #[doc(hidden)]
    pub display_order: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    #[doc(hidden)]
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    #[doc(hidden)]
    pub validator_os: Option<Rc<Fn(&OsStr) -> StdResult<(), OsString>>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    #[doc(hidden)]
    pub _settings: ArgFlags,
    #[cfg_attr(feature = "serde", serde(skip))]
    #[doc(hidden)]
    pub _unified_order: usize
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
    /// Arg::new("config")
    /// # ;
    /// ```
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    /// [`Arg`]: ./struct.Arg.html
    pub fn new(n: &'a str) -> Self {
        Arg {
            name: n,
            display_order: 999,
            ..Default::default()
        }
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
    /// Arg::new("config")
    ///     .short("c")
    /// # ;
    /// ```
    ///
    /// Setting [`short`] allows using the argument via a single hyphen (`-`) such as `-c`
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("config")
    ///         .short("c"))
    ///     .get_matches_from(vec![
    ///         "prog", "-c"
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
    /// Arg::new("cfg")
    ///     .long("config")
    /// # ;
    /// ```
    ///
    /// Setting `long` allows using the argument via a double hyphen (`--`) such as `--config`
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .long("config"))
    ///     .get_matches_from(vec![
    ///         "prog", "--config"
    ///     ]);
    ///
    /// assert!(m.is_present("cfg"));
    /// ```
    pub fn long(mut self, l: &'b str) -> Self {
        self.long = Some(l.trim_left_matches(|c| c == '-'));
        self
    }

    /// Allows adding a [`Arg`] alias, which function as "hidden" arguments that
    /// automatically dispatch as if this argument was used. This is more efficient, and easier
    /// than creating multiple hidden arguments as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///             .arg(Arg::new("test")
    ///             .long("test")
    ///             .alias("alias")
    ///             .takes_value(true))
    ///        .get_matches_from(vec![
    ///             "prog", "--alias", "cool"
    ///         ]);
    /// assert!(m.is_present("test"));
    /// assert_eq!(m.value_of("test"), Some("cool"));
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    pub fn alias<S: Into<&'b str>>(mut self, name: S) -> Self {
        add_to_option_vec!(self, aliases, name);
        self
    }

    /// Allows adding [`Arg`] aliases, which function as "hidden" arguments that
    /// automatically dispatch as if this argument was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///             .arg(Arg::new("test")
    ///                     .long("test")
    ///                     .aliases(&["do-stuff", "do-tests", "tests"])
    ///                     .help("the file to add")
    ///                     .required(false))
    ///             .get_matches_from(vec![
    ///                 "prog", "--do-tests"
    ///             ]);
    /// assert!(m.is_present("test"));
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    pub fn aliases(mut self, names: &[&'b str]) -> Self {
        add_slice_to_option_vec!(self, aliases, names);
        self
    }

    /// Allows adding a [`Arg`] alias that functions exactly like those defined with
    /// [`Arg::alias`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///             .arg(Arg::new("test")
    ///                 .visible_alias("something-awesome")
    ///                 .long("test")
    ///                 .takes_value(true))
    ///        .get_matches_from(vec![
    ///             "prog", "--something-awesome", "coffee"
    ///         ]);
    /// assert!(m.is_present("test"));
    /// assert_eq!(m.value_of("test"), Some("coffee"));
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    /// [`App::alias`]: ./struct.Arg.html#method.alias
    pub fn visible_alias<S: Into<&'b str>>(mut self, name: S) -> Self {
        add_to_option_vec!(self, visible_aliases, name);
        self
    }

    /// Allows adding multiple [`Arg`] aliases that functions exactly like those defined
    /// with [`Arg::aliases`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///             .arg(Arg::new("test")
    ///                 .long("test")
    ///                 .visible_aliases(&["something", "awesome", "cool"]))
    ///        .get_matches_from(vec![
    ///             "prog", "--awesome"
    ///         ]);
    /// assert!(m.is_present("test"));
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    /// [`App::aliases`]: ./struct.Arg.html#method.aliases
    pub fn visible_aliases(mut self, names: &[&'b str]) -> Self {
        add_slice_to_option_vec!(self, visible_aliases, names);
        self
    }

    /// Sets the short help text of the argument that will be displayed to the user when they print
    /// the help information with `-h`. Typically, this is a short (one line) description of the
    /// arg.
    ///
    /// **NOTE:** If only `Arg::help` is provided, and not [`Arg::long_help`] but the user requests
    /// `--help` clap will still display the contents of `help` appropriately
    ///
    /// **NOTE:** Only `Arg::help` is used in completion script generation in order to be concise
    ///
    /// # Examples
    ///
    /// Any valid UTF-8 is allowed in the help text. The one exception is when one wishes to
    /// include a newline in the help text and have the following text be properly aligned with all
    /// the other help text.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::new("config")
    ///     .help("The config file used by the myprog")
    /// # ;
    /// ```
    ///
    /// Setting `help` displays a short message to the side of the argument when the user passes
    /// `-h` or `--help` (by default).
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .long("config")
    ///         .help("Some help text describing the --config arg"))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
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
    /// [`Arg::long_help`]: ./struct.Arg.html#method.long_help
    pub fn help(mut self, h: &'b str) -> Self {
        self.help = Some(h);
        self
    }

    /// Sets the long help text of the argument that will be displayed to the user when they print
    /// the help information with `--help`. Typically this a more detailed (multi-line) message
    /// that describes the arg.
    ///
    /// **NOTE:** If only `long_help` is provided, and not [`Arg::help`] but the user requests `-h`
    /// clap will still display the contents of `long_help` appropriately
    ///
    /// **NOTE:** Only [`Arg::help`] is used in completion script generation in order to be concise
    ///
    /// # Examples
    ///
    /// Any valid UTF-8 is allowed in the help text. The one exception is when one wishes to
    /// include a newline in the help text and have the following text be properly aligned with all
    /// the other help text.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::new("config")
    ///     .long_help(
    /// "The config file used by the myprog must be in JSON format
    /// with only valid keys and may not contain other nonsense
    /// that cannot be read by this program. Obviously I'm going on
    /// and on, so I'll stop now.")
    /// # ;
    /// ```
    ///
    /// Setting `help` displays a short message to the side of the argument when the user passes
    /// `-h` or `--help` (by default).
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .long("config")
    ///         .long_help(
    /// "The config file used by the myprog must be in JSON format
    /// with only valid keys and may not contain other nonsense
    /// that cannot be read by this program. Obviously I'm going on
    /// and on, so I'll stop now."))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
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
    ///    --config
    ///         The config file used by the myprog must be in JSON format
    ///         with only valid keys and may not contain other nonsense
    ///         that cannot be read by this program. Obviously I'm going on
    ///         and on, so I'll stop now.
    ///
    /// -h, --help
    ///         Prints help information
    ///
    /// -V, --version
    ///         Prints version information
    /// ```
    /// [`Arg::help`]: ./struct.Arg.html#method.help
    pub fn long_help(mut self, h: &'b str) -> Self {
        self.long_help = Some(h);
        self
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
    /// Arg::new("config")
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_unless("dbg")
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("dbg")
    ///         .long("debug"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--debug"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`Arg::required_unless(name)`] and *not* supplying `name` or this arg is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_unless("dbg")
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("dbg")
    ///         .long("debug"))
    ///     .get_matches_from_safe(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::required_unless`]: ./struct.Arg.html#method.required_unless
    /// [`Arg::required`]: ./struct.Arg.html#method.required
    /// [`Arg::required_unless(name)`]: ./struct.Arg.html#method.required_unless
    pub fn required_unless(mut self, name: &'a str) -> Self {
        add_to_option_vec!(self, required_unless, name);
        self.setting(ArgSettings::Required)
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
    /// Arg::new("config")
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_unless_all(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::new("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--debug", "-i", "file"
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_unless_all(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::new("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::required_unless_one`]: ./struct.Arg.html#method.required_unless_one
    /// [`Arg::required_unless_all(names)`]: ./struct.Arg.html#method.required_unless_all
    pub fn required_unless_all(mut self, names: &[&'a str]) -> Self {
        add_slice_to_option_vec!(self, required_unless, names);
        self.setting(ArgSettings::RequiredUnlessAll).setting(ArgSettings::Required)
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
    /// Arg::new("config")
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_unless_one(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::new("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--debug"
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_unless_one(&["dbg", "infile"])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("dbg")
    ///         .long("debug"))
    ///     .arg(Arg::new("infile")
    ///         .short("i")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [required]: ./struct.Arg.html#method.required
    /// [`Arg::required_unless_one(names)`]: ./struct.Arg.html#method.required_unless_one
    /// [`Arg::required_unless_all`]: ./struct.Arg.html#method.required_unless_all
    pub fn required_unless_one(mut self, names: &[&'a str]) -> Self {
        add_slice_to_option_vec!(self, required_unless, names);
        self.setting(ArgSettings::Required)
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
    /// Arg::new("config")
    ///     .conflicts_with("debug")
    /// # ;
    /// ```
    ///
    /// Setting conflicting argument, and having both arguments present at runtime is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .conflicts_with("debug")
    ///         .long("config"))
    ///     .arg(Arg::new("debug")
    ///         .long("debug"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--debug", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::ArgumentConflict);
    /// ```
    pub fn conflicts_with(mut self, name: &'a str) -> Self {
        add_to_option_vec!(self, conflicts_with, name);
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
    /// Arg::new("config")
    ///     .conflicts_with_all(&["debug", "input"])
    /// # ;
    /// ```
    ///
    /// Setting conflicting argument, and having any of the arguments present at runtime with a
    /// conflicting argument is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .conflicts_with_all(&["debug", "input"])
    ///         .long("config"))
    ///     .arg(Arg::new("debug")
    ///         .long("debug"))
    ///     .arg(Arg::new("input")
    ///         .index(1))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "file.conf", "file.txt"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::ArgumentConflict);
    /// ```
    /// [`Arg::conflicts_with`]: ./struct.Arg.html#method.conflicts_with
    pub fn conflicts_with_all(mut self, names: &[&'a str]) -> Self {
        add_slice_to_option_vec!(self, conflicts_with, names);
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
    /// let m = App::new("prog")
    ///     .arg(Arg::from("-f, --flag 'some flag'")
    ///         .conflicts_with("debug"))
    ///     .arg(Arg::from("-d, --debug 'other flag'"))
    ///     .arg(Arg::from("-c, --color 'third flag'")
    ///         .overrides_with("flag"))
    ///     .get_matches_from(vec![
    ///         "prog", "-f", "-d", "-c"]);
    ///             //    ^~~~~~~~~~~~^~~~~ flag is overridden by color
    ///
    /// assert!(m.is_present("color"));
    /// assert!(m.is_present("debug")); // even though flag conflicts with debug, it's as if flag
    ///                                 // was never used because it was overridden with color
    /// assert!(!m.is_present("flag"));
    /// ```
    pub fn overrides_with(mut self, name: &'a str) -> Self {
        add_to_option_vec!(self, overrides_with, name);
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
    /// let m = App::new("prog")
    ///     .arg(Arg::from("-f, --flag 'some flag'")
    ///         .conflicts_with("color"))
    ///     .arg(Arg::from("-d, --debug 'other flag'"))
    ///     .arg(Arg::from("-c, --color 'third flag'")
    ///         .overrides_with_all(&["flag", "debug"]))
    ///     .get_matches_from(vec![
    ///         "prog", "-f", "-d", "-c"]);
    ///             //    ^~~~~~^~~~~~~~~ flag and debug are overridden by color
    ///
    /// assert!(m.is_present("color")); // even though flag conflicts with color, it's as if flag
    ///                                 // and debug were never used because they were overridden
    ///                                 // with color
    /// assert!(!m.is_present("debug"));
    /// assert!(!m.is_present("flag"));
    /// ```
    pub fn overrides_with_all(mut self, names: &[&'a str]) -> Self {
        add_slice_to_option_vec!(self, overrides_with, names);
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
    /// Arg::new("config")
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .requires("input")
    ///         .long("config"))
    ///     .arg(Arg::new("input")
    ///         .index(1))
    ///     .get_matches_from_safe(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert!(res.is_ok()); // We didn't use cfg, so input wasn't required
    /// ```
    ///
    /// Setting [`Arg::requires(name)`] and *not* supplying that argument is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .requires("input")
    ///         .long("config"))
    ///     .arg(Arg::new("input")
    ///         .index(1))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::requires(name)`]: ./struct.Arg.html#method.requires
    /// [Conflicting]: ./struct.Arg.html#method.conflicts_with
    /// [override]: ./struct.Arg.html#method.overrides_with
    pub fn requires(mut self, name: &'a str) -> Self {
        add_to_option_vec!(self, requires, name);
        self
    }

    /// Allows a conditional requirement. The requirement will only become valid if this arg's value
    /// equals `val`.
    ///
    /// **NOTE:** If using YAML the values should be laid out as follows
    ///
    /// ```yaml
    /// requires_if:
    ///     - [val, arg]
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::new("config")
    ///     .requires_if("val", "arg")
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::requires_if(val, arg)`] requires that the `arg` be used at runtime if the
    /// defining argument's value is equal to `val`. If the defining argument is anything other than
    /// `val`, the other argument isn't required.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .requires_if("my.cfg", "other")
    ///         .long("config"))
    ///     .arg(Arg::new("other"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "some.cfg"
    ///     ]);
    ///
    /// assert!(res.is_ok()); // We didn't use --config=my.cfg, so other wasn't required
    /// ```
    ///
    /// Setting [`Arg::requires_if(val, arg)`] and setting the value to `val` but *not* supplying
    /// `arg` is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .requires_if("my.cfg", "input")
    ///         .long("config"))
    ///     .arg(Arg::new("input"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "my.cfg"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::requires(name)`]: ./struct.Arg.html#method.requires
    /// [Conflicting]: ./struct.Arg.html#method.conflicts_with
    /// [override]: ./struct.Arg.html#method.overrides_with
    pub fn requires_if(mut self, val: &'b str, arg: &'a str) -> Self {
        if let Some(ref mut als) = self.requires_ifs {
            als.push((val, arg));
        } else {
            self.requires_ifs = Some(vec![(val, arg)]);
        }
        self
    }

    /// Allows multiple conditional requirements. The requirement will only become valid if this arg's value
    /// equals `val`.
    ///
    /// **NOTE:** If using YAML the values should be laid out as follows
    ///
    /// ```yaml
    /// requires_if:
    ///     - [val, arg]
    ///     - [val2, arg2]
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::new("config")
    ///     .requires_ifs(&[
    ///         ("val", "arg"),
    ///         ("other_val", "arg2"),
    ///     ])
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::requires_ifs(&["val", "arg"])`] requires that the `arg` be used at runtime if the
    /// defining argument's value is equal to `val`. If the defining argument's value is anything other
    /// than `val`, `arg` isn't required.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .requires_ifs(&[
    ///             ("special.conf", "opt"),
    ///             ("other.conf", "other"),
    ///         ])
    ///         .long("config"))
    ///     .arg(Arg::new("opt")
    ///         .long("option")
    ///         .takes_value(true))
    ///     .arg(Arg::new("other"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "special.conf"
    ///     ]);
    ///
    /// assert!(res.is_err()); // We  used --config=special.conf so --option <val> is required
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::requires(name)`]: ./struct.Arg.html#method.requires
    /// [Conflicting]: ./struct.Arg.html#method.conflicts_with
    /// [override]: ./struct.Arg.html#method.overrides_with
    pub fn requires_ifs(mut self, ifs: &[(&'b str, &'a str)]) -> Self {
        if let Some(ref mut als) = self.requires_ifs {
            als.extend_from_slice(ifs);
        } else {
            self.requires_ifs = Some(ifs.into());
        }
        self
    }

    /// Allows specifying that an argument is [required] conditionally. The requirement will only
    /// become valid if the specified `arg`'s value equals `val`.
    ///
    /// **NOTE:** If using YAML the values should be laid out as follows
    ///
    /// ```yaml
    /// required_if:
    ///     - [arg, val]
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::new("config")
    ///     .required_if("other_arg", "value")
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::required_if(arg, val)`] makes this arg required if the `arg` is used at
    /// runtime and it's value is equal to `val`. If the `arg`'s value is anything other than `val`,
    /// this argument isn't required.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .required_if("other", "special")
    ///         .long("config"))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--other", "not-special"
    ///     ]);
    ///
    /// assert!(res.is_ok()); // We didn't use --other=special, so "cfg" wasn't required
    /// ```
    ///
    /// Setting [`Arg::required_if(arg, val)`] and having `arg` used with a vaue of `val` but *not*
    /// using this arg is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .required_if("other", "special")
    ///         .long("config"))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--other", "special"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::requires(name)`]: ./struct.Arg.html#method.requires
    /// [Conflicting]: ./struct.Arg.html#method.conflicts_with
    /// [required]: ./struct.Arg.html#method.required
    pub fn required_if(mut self, arg: &'a str, val: &'b str) -> Self {
        if let Some(ref mut als) = self.required_ifs {
            als.push((arg, val));
        } else {
            self.required_ifs = Some(vec![(arg, val)]);
        }
        self
    }

    /// Allows specifying that an argument is [required] based on multiple conditions. The
    /// conditions are set up in a `(arg, val)` style tuple. The requirement will only become valid
    /// if one of the specified `arg`'s value equals it's corresponding `val`.
    ///
    /// **NOTE:** If using YAML the values should be laid out as follows
    ///
    /// ```yaml
    /// required_if:
    ///     - [arg, val]
    ///     - [arg2, val2]
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Arg;
    /// Arg::new("config")
    ///     .required_ifs(&[
    ///         ("extra", "val"),
    ///         ("option", "spec")
    ///     ])
    /// # ;
    /// ```
    ///
    /// Setting [`Arg::required_ifs(&[(arg, val)])`] makes this arg required if any of the `arg`s
    /// are used at runtime and it's corresponding value is equal to `val`. If the `arg`'s value is
    /// anything other than `val`, this argument isn't required.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_ifs(&[
    ///             ("extra", "val"),
    ///             ("option", "spec")
    ///         ])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("extra")
    ///         .takes_value(true)
    ///         .long("extra"))
    ///     .arg(Arg::new("option")
    ///         .takes_value(true)
    ///         .long("option"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--option", "other"
    ///     ]);
    ///
    /// assert!(res.is_ok()); // We didn't use --option=spec, or --extra=val so "cfg" isn't required
    /// ```
    ///
    /// Setting [`Arg::required_ifs(&[(arg, val)])`] and having any of the `arg`s used with it's
    /// vaue of `val` but *not* using this arg is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .required_ifs(&[
    ///             ("extra", "val"),
    ///             ("option", "spec")
    ///         ])
    ///         .takes_value(true)
    ///         .long("config"))
    ///     .arg(Arg::new("extra")
    ///         .takes_value(true)
    ///         .long("extra"))
    ///     .arg(Arg::new("option")
    ///         .takes_value(true)
    ///         .long("option"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--option", "spec"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Arg::requires(name)`]: ./struct.Arg.html#method.requires
    /// [Conflicting]: ./struct.Arg.html#method.conflicts_with
    /// [required]: ./struct.Arg.html#method.required
    pub fn required_ifs(mut self, ifs: &[(&'a str, &'b str)]) -> Self {
        if let Some(ref mut als) = self.required_ifs {
            als.extend_from_slice(ifs);
        } else {
            self.required_ifs = Some(ifs.into());
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
    /// Arg::new("config")
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .requires("input")
    ///         .long("config"))
    ///     .arg(Arg::new("input")
    ///         .index(1))
    ///     .arg(Arg::new("output")
    ///         .index(2))
    ///     .get_matches_from_safe(vec![
    ///         "prog"
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("cfg")
    ///         .takes_value(true)
    ///         .requires_all(&["input", "output"])
    ///         .long("config"))
    ///     .arg(Arg::new("input")
    ///         .index(1))
    ///     .arg(Arg::new("output")
    ///         .index(2))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--config", "file.conf", "in.txt"
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
        add_slice_to_option_vec!(self, requires, names);
        self
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
    /// Arg::new("config")
    ///     .index(1)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("mode")
    ///         .index(1))
    ///     .arg(Arg::new("debug")
    ///         .long("debug"))
    ///     .get_matches_from(vec![
    ///         "prog", "--debug", "fast"
    ///     ]);
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
    pub fn index(mut self, idx: usize) -> Self {
        self.index = Some(idx);
        self
    }

    /// Specifies a value that *stops* parsing multiple values of a give argument. By default when
    /// one sets [`multiple(true)`] on an argument, clap will continue parsing values for that
    /// argument until it reaches another valid argument, or one of the other more specific settings
    /// for multiple values is used (such as [`min_values`], [`max_values`] or
    /// [`number_of_values`]).
    ///
    /// **NOTE:** This setting only applies to [options] and [positional arguments]
    ///
    /// **NOTE:** When the terminator is passed in on the command line, it is **not** stored as one
    /// of the vaues
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::new("vals")
    ///     .takes_value(true)
    ///     .multiple(true)
    ///     .value_terminator(";")
    /// # ;
    /// ```
    /// The following example uses two arguments, a sequence of commands, and the location in which
    /// to perform them
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("cmds")
    ///         .multiple(true)
    ///         .allow_hyphen_values(true)
    ///         .value_terminator(";"))
    ///     .arg(Arg::new("location"))
    ///     .get_matches_from(vec![
    ///         "prog", "find", "-type", "f", "-name", "special", ";", "/home/clap"
    ///     ]);
    /// let cmds: Vec<_> = m.values_of("cmds").unwrap().collect();
    /// assert_eq!(&cmds, &["find", "-type", "f", "-name", "special"]);
    /// assert_eq!(m.value_of("location"), Some("/home/clap"));
    /// ```
    /// [options]: ./struct.Arg.html#method.takes_value
    /// [positional arguments]: ./struct.Arg.html#method.index
    /// [`multiple(true)`]: ./struct.Arg.html#method.multiple
    /// [`min_values`]: ./struct.Arg.html#method.min_values
    /// [`number_of_values`]: ./struct.Arg.html#method.number_of_values
    /// [`max_values`]: ./struct.Arg.html#method.max_values
    pub fn value_terminator(mut self, term: &'b str) -> Self {
        self.value_terminator = Some(term);
        self.setting(ArgSettings::TakesValue)
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
    /// Arg::new("mode")
    ///     .takes_value(true)
    ///     .possible_values(&["fast", "slow", "medium"])
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_values(&["fast", "slow", "medium"]))
    ///     .get_matches_from(vec![
    ///         "prog", "--mode", "fast"
    ///     ]);
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast"));
    /// ```
    ///
    /// The next example shows a failed parse from using a value which wasn't defined as one of the
    /// possible values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_values(&["fast", "slow", "medium"]))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--mode", "wrong"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::InvalidValue);
    /// ```
    /// [options]: ./struct.Arg.html#method.takes_value
    /// [positional arguments]: ./struct.Arg.html#method.index
    pub fn possible_values(mut self, names: &[&'b str]) -> Self {
        add_slice_to_option_vec!(self, possible_values, names);
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
    /// Arg::new("mode")
    ///     .takes_value(true)
    ///     .possible_value("fast")
    ///     .possible_value("slow")
    ///     .possible_value("medium")
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_value("fast")
    ///         .possible_value("slow")
    ///         .possible_value("medium"))
    ///     .get_matches_from(vec![
    ///         "prog", "--mode", "fast"
    ///     ]);
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast"));
    /// ```
    ///
    /// The next example shows a failed parse from using a value which wasn't defined as one of the
    /// possible values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("mode")
    ///         .long("mode")
    ///         .takes_value(true)
    ///         .possible_value("fast")
    ///         .possible_value("slow")
    ///         .possible_value("medium"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "--mode", "wrong"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::InvalidValue);
    /// ```
    /// [options]: ./struct.Arg.html#method.takes_value
    /// [positional arguments]: ./struct.Arg.html#method.index
    pub fn possible_value(mut self, name: &'b str) -> Self {
        add_to_option_vec!(self, possible_values, name);
        self
    }

    /// Specifies the name of the [`ArgGroup`] the argument belongs to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::new("debug")
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
    /// let m = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .group("mode"))
    ///     .arg(Arg::new("verbose")
    ///         .long("verbose")
    ///         .group("mode"))
    ///     .get_matches_from(vec![
    ///         "prog", "--debug"
    ///     ]);
    /// assert!(m.is_present("mode"));
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    pub fn group(mut self, name: &'a str) -> Self {
        add_to_option_vec!(self, groups, name);
        self
    }

    /// Specifies the names of multiple [`ArgGroup`]'s the argument belongs to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::new("debug")
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
    /// let m = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .groups(&["mode", "verbosity"]))
    ///     .arg(Arg::new("verbose")
    ///         .long("verbose")
    ///         .groups(&["mode", "verbosity"]))
    ///     .get_matches_from(vec![
    ///         "prog", "--debug"
    ///     ]);
    /// assert!(m.is_present("mode"));
    /// assert!(m.is_present("verbosity"));
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    pub fn groups(mut self, names: &[&'a str]) -> Self {
        add_slice_to_option_vec!(self, groups, names);
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
    /// Arg::new("file")
    ///     .short("f")
    ///     .number_of_values(3)
    /// # ;
    /// ```
    ///
    /// Not supplying the correct number of values is an error
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("file")
    ///         .takes_value(true)
    ///         .number_of_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "-F", "file1"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
    /// ```
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn number_of_values(mut self, qty: usize) -> Self {
        self.number_of_values = Some(qty);
        if qty > 1 {
            self = self.setting(ArgSettings::Multiple);
        }
        self.setting(ArgSettings::TakesValue)
    }

    /// Allows one to perform a custom validation on the argument value. You provide a closure
    /// which accepts a [`String`] value, and return a [`Result`] where the [`Err(String)`] is a
    /// message displayed to the user.
    ///
    /// **NOTE:** The error message does *not* need to contain the `error:` portion, only the
    /// message as all errors will appear as
    /// `error: Invalid value for '<arg>': <YOUR MESSAGE>` where `<arg>` is replaced by the actual
    /// arg, and `<YOUR MESSAGE>` is the `String` you return as the error.
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("file")
    ///         .index(1)
    ///         .validator(has_at))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "some@file"
    ///     ]);
    /// assert!(res.is_ok());
    /// assert_eq!(res.unwrap().value_of("file"), Some("some@file"));
    /// ```
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Err(String)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
    pub fn validator<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> StdResult<(), String> + 'static,
    {
        self.validator = Some(Rc::new(f));
        self
    }

    /// Works identically to Validator but is intended to be used with values that could
    /// contain non UTF-8 formatted strings.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```rust")]
    /// # use clap::{App, Arg};
    /// # use std::ffi::{OsStr, OsString};
    /// # use std::os::unix::ffi::OsStrExt;
    /// fn has_ampersand(v: &OsStr) -> Result<(), OsString> {
    ///     if v.as_bytes().iter().any(|b| *b == b'&') { return Ok(()); }
    ///     Err(OsString::from("The value did not contain the required & sigil"))
    /// }
    /// let res = App::new("prog")
    ///     .arg(Arg::new("file")
    ///         .index(1)
    ///         .validator_os(has_ampersand))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "Fish & chips"
    ///     ]);
    /// assert!(res.is_ok());
    /// assert_eq!(res.unwrap().value_of("file"), Some("Fish & chips"));
    /// ```
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    /// [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    /// [`OsString`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Err(String)`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
    pub fn validator_os<F>(mut self, f: F) -> Self
    where
        F: Fn(&OsStr) -> StdResult<(), OsString> + 'static,
    {
        self.validator_os = Some(Rc::new(f));
        self
    }

    /// Specifies the *maximum* number of values are for this argument. For example, if you had a
    /// `-f <file>` argument where you wanted up to 3 'files' you would set `.max_values(3)`, and
    /// this argument would be satisfied if the user provided, 1, 2, or 3 values.
    ///
    /// **NOTE:** This does *not* implicitly set [`Arg::multiple(true)`]. This is because
    /// `-o val -o val` is multiple occurrences but a single value and `-o val1 val2` is a single
    /// occurence with multiple values. For positional arguments this **does** set
    /// [`Arg::multiple(true)`] because there is no way to determine the difference between multiple
    /// occurences and multiple values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// Arg::new("file")
    ///     .short("f")
    ///     .max_values(3)
    /// # ;
    /// ```
    ///
    /// Supplying less than the maximum number of values is allowed
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("file")
    ///         .takes_value(true)
    ///         .max_values(3)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "-F", "file1", "file2"
    ///     ]);
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("file")
    ///         .takes_value(true)
    ///         .max_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "-F", "file1", "file2", "file3"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::TooManyValues);
    /// ```
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn max_values(mut self, qty: usize) -> Self {
        self.max_values = Some(qty);
        if qty > 1 {
            self = self.setting(ArgSettings::Multiple);
        }
        self.setting(ArgSettings::TakesValue)
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
    /// Arg::new("file")
    ///     .short("f")
    ///     .min_values(3)
    /// # ;
    /// ```
    ///
    /// Supplying more than the minimum number of values is allowed
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("file")
    ///         .takes_value(true)
    ///         .min_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "-F", "file1", "file2", "file3"
    ///     ]);
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
    /// let res = App::new("prog")
    ///     .arg(Arg::new("file")
    ///         .takes_value(true)
    ///         .min_values(2)
    ///         .short("F"))
    ///     .get_matches_from_safe(vec![
    ///         "prog", "-F", "file1"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::TooFewValues);
    /// ```
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    pub fn min_values(mut self, qty: usize) -> Self {
        self.min_values = Some(qty);
        if qty > 1 {
            self = self.setting(ArgSettings::Multiple);
        }
        self.setting(ArgSettings::TakesValue)
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
    /// let m = App::new("prog")
    ///     .arg(Arg::new("config")
    ///         .short("c")
    ///         .long("config")
    ///         .value_delimiter(";"))
    ///     .get_matches_from(vec![
    ///         "prog", "--config=val1;val2;val3"
    ///     ]);
    ///
    /// assert_eq!(m.values_of("config").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"])
    /// ```
    /// [`Arg::use_delimiter(true)`]: ./struct.Arg.html#method.use_delimiter
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    pub fn value_delimiter(mut self, d: &str) -> Self {
        self.value_delimiter = Some(
            d.chars()
                .nth(0)
                .expect("Failed to get value_delimiter from arg"),
        );
        self.unset_setting(ArgSettings::ValueDelimiterNotSet)
            .setting(ArgSettings::TakesValue)
            .setting(ArgSettings::UseValueDelimiter)
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
    /// Arg::new("speed")
    ///     .short("s")
    ///     .value_names(&["fast", "slow"])
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("io")
    ///         .long("io-files")
    ///         .value_names(&["INFILE", "OUTFILE"]))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
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
        // @TODO-v3-release: look at functional approach
        if let Some(ref mut vals) = self.value_names {
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
            self.value_names = Some(vm);
        }

        self = self.setting(ArgSettings::TakesValue);
        if self.is_set(ArgSettings::ValueDelimiterNotSet) {
            self = self.unset_setting(ArgSettings::ValueDelimiterNotSet)
                .setting(ArgSettings::UseValueDelimiter);
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
    /// Arg::new("cfg")
    ///     .long("config")
    ///     .value_name("FILE")
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("config")
    ///         .long("config")
    ///         .value_name("FILE"))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
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
        if let Some(ref mut vals) = self.value_names {
            let l = vals.len();
            vals.insert(l, name);
        } else {
            let mut vm = VecMap::new();
            vm.insert(0, name);
            self.value_names = Some(vm);
        }
        self.setting(ArgSettings::TakesValue)
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
    /// **NOTE:** This setting is perfectly compatible with [`Arg::default_value_if`] but slightly
    /// different. `Arg::default_value` *only* takes affect when the user has not provided this arg
    /// at runtime. `Arg::default_value_if` however only takes affect when the user has not provided
    /// a value at runtime **and** these other conditions are met as well. If you have set
    /// `Arg::default_value` and `Arg::default_value_if`, and the user **did not** provide a this
    /// arg at runtime, nor did were the conditions met for `Arg::default_value_if`, the
    /// `Arg::default_value` will be applied.
    ///
    /// **NOTE:** This implicitly sets [`Arg::takes_value(true)`].
    ///
    /// **NOTE:** This setting effectively disables `AppSettings::ArgRequiredElseHelp` if used in
    /// conjuction as it ensures that some argument will always be present.
    ///
    /// # Examples
    ///
    /// First we use the default value without providing any value at runtime.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("opt")
    ///         .long("myopt")
    ///         .default_value("myval"))
    ///     .get_matches_from(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("opt"), Some("myval"));
    /// assert!(m.is_present("opt"));
    /// assert_eq!(m.occurrences_of("opt"), 0);
    /// ```
    ///
    /// Next we provide a value at runtime to override the default.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("opt")
    ///         .long("myopt")
    ///         .default_value("myval"))
    ///     .get_matches_from(vec![
    ///         "prog", "--myopt=non_default"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("opt"), Some("non_default"));
    /// assert!(m.is_present("opt"));
    /// assert_eq!(m.occurrences_of("opt"), 1);
    /// ```
    /// [`ArgMatches::occurrences_of`]: ./struct.ArgMatches.html#method.occurrences_of
    /// [`ArgMatches::value_of`]: ./struct.ArgMatches.html#method.value_of
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    /// [`ArgMatches::is_present`]: ./struct.ArgMatches.html#method.is_present
    /// [`Arg::default_value_if`]: ./struct.Arg.html#method.default_value_if
    pub fn default_value(self, val: &'a str) -> Self {
        self.default_value_os(OsStr::from_bytes(val.as_bytes()))
    }

    /// Provides a default value in the exact same manner as [`Arg::default_value`]
    /// only using [`OsStr`]s instead.
    /// [`Arg::default_value`]: ./struct.Arg.html#method.default_value
    /// [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    pub fn default_value_os(mut self, val: &'a OsStr) -> Self {
        self.default_value = Some(val);
        self.setting(ArgSettings::TakesValue)
    }

    /// Specifies the value of the argument if `arg` has been used at runtime. If `val` is set to
    /// `None`, `arg` only needs to be present. If `val` is set to `"some-val"` then `arg` must be
    /// present at runtime **and** have the value `val`.
    ///
    /// **NOTE:** This setting is perfectly compatible with [`Arg::default_value`] but slightly
    /// different. `Arg::default_value` *only* takes affect when the user has not provided this arg
    /// at runtime. This setting however only takes affect when the user has not provided a value at
    /// runtime **and** these other conditions are met as well. If you have set `Arg::default_value`
    /// and `Arg::default_value_if`, and the user **did not** provide a this arg at runtime, nor did
    /// were the conditions met for `Arg::default_value_if`, the `Arg::default_value` will be
    /// applied.
    ///
    /// **NOTE:** This implicitly sets [`Arg::takes_value(true)`].
    ///
    /// **NOTE:** If using YAML the values should be laid out as follows (`None` can be represented
    /// as `null` in YAML)
    ///
    /// ```yaml
    /// default_value_if:
    ///     - [arg, val, default]
    /// ```
    ///
    /// # Examples
    ///
    /// First we use the default value only if another arg is present at runtime.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("flag")
    ///         .long("flag"))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .default_value_if("flag", None, "default"))
    ///     .get_matches_from(vec![
    ///         "prog", "--flag"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("other"), Some("default"));
    /// ```
    ///
    /// Next we run the same test, but without providing `--flag`.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("flag")
    ///         .long("flag"))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .default_value_if("flag", None, "default"))
    ///     .get_matches_from(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("other"), None);
    /// ```
    ///
    /// Now lets only use the default value if `--opt` contains the value `special`.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("opt")
    ///         .takes_value(true)
    ///         .long("opt"))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .default_value_if("opt", Some("special"), "default"))
    ///     .get_matches_from(vec![
    ///         "prog", "--opt", "special"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("other"), Some("default"));
    /// ```
    ///
    /// We can run the same test and provide any value *other than* `special` and we won't get a
    /// default value.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("opt")
    ///         .takes_value(true)
    ///         .long("opt"))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .default_value_if("opt", Some("special"), "default"))
    ///     .get_matches_from(vec![
    ///         "prog", "--opt", "hahaha"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("other"), None);
    /// ```
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    /// [`Arg::default_value`]: ./struct.Arg.html#method.default_value
    pub fn default_value_if(self, arg: &'a str, val: Option<&'b str>, default: &'b str) -> Self {
        self.default_value_if_os(
            arg,
            val.map(str::as_bytes).map(OsStr::from_bytes),
            OsStr::from_bytes(default.as_bytes()),
        )
    }

    /// Provides a conditional default value in the exact same manner as [`Arg::default_value_if`]
    /// only using [`OsStr`]s instead.
    /// [`Arg::default_value_if`]: ./struct.Arg.html#method.default_value_if
    /// [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    pub fn default_value_if_os(
        mut self,
        arg: &'a str,
        val: Option<&'b OsStr>,
        default: &'b OsStr,
    ) -> Self {
        if let Some(ref mut vm) = self.default_value_ifs {
            let l = vm.len();
            vm.insert(l, (arg, val, default));
        } else {
            let mut vm = VecMap::new();
            vm.insert(0, (arg, val, default));
            self.default_value_ifs = Some(vm);
        }
        self.setting(ArgSettings::TakesValue)
    }

    /// Specifies multiple values and conditions in the same manner as [`Arg::default_value_if`].
    /// The method takes a slice of tuples in the `(arg, Option<val>, default)` format.
    ///
    /// **NOTE**: The conditions are stored in order and evaluated in the same order. I.e. the first
    /// if multiple conditions are true, the first one found will be applied and the ultimate value.
    ///
    /// **NOTE:** If using YAML the values should be laid out as follows
    ///
    /// ```yaml
    /// default_value_if:
    ///     - [arg, val, default]
    ///     - [arg2, null, default2]
    /// ```
    ///
    /// # Examples
    ///
    /// First we use the default value only if another arg is present at runtime.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("flag")
    ///         .long("flag"))
    ///     .arg(Arg::new("opt")
    ///         .long("opt")
    ///         .takes_value(true))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .default_value_ifs(&[
    ///             ("flag", None, "default"),
    ///             ("opt", Some("channal"), "chan"),
    ///         ]))
    ///     .get_matches_from(vec![
    ///         "prog", "--opt", "channal"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("other"), Some("chan"));
    /// ```
    ///
    /// Next we run the same test, but without providing `--flag`.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("flag")
    ///         .long("flag"))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .default_value_ifs(&[
    ///             ("flag", None, "default"),
    ///             ("opt", Some("channal"), "chan"),
    ///         ]))
    ///     .get_matches_from(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("other"), None);
    /// ```
    ///
    /// We can also see that these values are applied in order, and if more than one condition is
    /// true, only the first evaluatd "wins"
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("prog")
    ///     .arg(Arg::new("flag")
    ///         .long("flag"))
    ///     .arg(Arg::new("opt")
    ///         .long("opt")
    ///         .takes_value(true))
    ///     .arg(Arg::new("other")
    ///         .long("other")
    ///         .default_value_ifs(&[
    ///             ("flag", None, "default"),
    ///             ("opt", Some("channal"), "chan"),
    ///         ]))
    ///     .get_matches_from(vec![
    ///         "prog", "--opt", "channal", "--flag"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("other"), Some("default"));
    /// ```
    /// [`Arg::takes_value(true)`]: ./struct.Arg.html#method.takes_value
    /// [`Arg::default_value`]: ./struct.Arg.html#method.default_value
    pub fn default_value_ifs(mut self, ifs: &[(&'a str, Option<&'b str>, &'b str)]) -> Self {
        for &(arg, val, default) in ifs {
            self = self.default_value_if_os(
                arg,
                val.map(str::as_bytes).map(OsStr::from_bytes),
                OsStr::from_bytes(default.as_bytes()),
            );
        }
        self
    }

    /// Provides multiple conditional default values in the exact same manner as
    /// [`Arg::default_value_ifs`] only using [`OsStr`]s instead.
    /// [`Arg::default_value_ifs`]: ./struct.Arg.html#method.default_value_ifs
    /// [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    #[cfg_attr(feature = "lints", allow(explicit_counter_loop))]
    pub fn default_value_ifs_os(mut self, ifs: &[(&'a str, Option<&'b OsStr>, &'b OsStr)]) -> Self {
        for &(arg, val, default) in ifs {
            self = self.default_value_if_os(arg, val, default);
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
    /// let m = App::new("prog")
    ///     .arg(Arg::new("a") // Typically args are grouped alphabetically by name.
    ///                              // Args without a display_order have a value of 999 and are
    ///                              // displayed alphabetically with all other 999 valued args.
    ///         .long("long-option")
    ///         .short("o")
    ///         .takes_value(true)
    ///         .help("Some help and text"))
    ///     .arg(Arg::new("b")
    ///         .long("other-option")
    ///         .short("O")
    ///         .takes_value(true)
    ///         .display_order(1)   // In order to force this arg to appear *first*
    ///                             // all we have to do is give it a value lower than 999.
    ///                             // Any other args with a value of 1 will be displayed
    ///                             // alphabetically with this one...then 2 values, then 3, etc.
    ///         .help("I should be first!"))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
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
        self.display_order = ord;
        self
    }

    /// Checks if one of the [`ArgSettings`] settings is set for the argument
    /// [`ArgSettings`]: ./enum.ArgSettings.html
    pub fn is_set(&self, s: ArgSettings) -> bool { self.settings.contains(&s) }

    /// Sets one of the [`ArgSettings`] settings for the argument
    /// [`ArgSettings`]: ./enum.ArgSettings.html
    pub fn setting(mut self, s: ArgSettings) -> Self {
        self.setb(s);
        self
    }

    /// Unsets one of the [`ArgSettings`] settings for the argument
    /// [`ArgSettings`]: ./enum.ArgSettings.html
    pub fn unset_setting(mut self, s: ArgSettings) -> Self {
        self.unsetb(s);
        self
    }

    #[doc(hidden)]
    pub fn setb(&mut self, s: ArgSettings) { self.settings.push(s); }

    #[doc(hidden)]
    pub fn unsetb(&mut self, s: ArgSettings) {
        'start: for i in (0..self.settings.len()).rev() {
            let should_remove = self.settings[i] == s;
            if should_remove {
                self.settings.swap_remove(i);
                break 'start;
            }
        }

    }

    #[doc(hidden)]
    pub fn multiple_str(&self) -> &str {
        let mult_vals = self.value_names.as_ref().map_or(
            true,
            |names| names.len() < 2,
        );
        if self.is_set(ArgSettings::Multiple) && mult_vals {
            "..."
        } else {
            ""
        }
    }

    #[doc(hidden)]
    pub fn name_no_brackets(&self) -> Cow<str> {
        debugln!("Arg::name_no_brackets;");
        // Should only be positionals
        assert!(self.index.is_some());
        if let Some(ref names) = self.value_names {
            debugln!("Arg:name_no_brackets: val_names={:#?}", names);
            if names.len() > 1 {
                Cow::Owned(
                    names
                        .values()
                        .map(|n| format!("<{}>", n))
                        .collect::<Vec<_>>()
                        .join(" "),
                )
            } else {
                Cow::Borrowed(names.values().next().expect(INTERNAL_ERROR_MSG))
            }
        } else {
            debugln!("Arg::name_no_brackets: just name");
            Cow::Borrowed(self.name)
        }
    }

    #[doc(hidden)]
    pub fn _has_switch(&self) -> bool {
        self.long.is_some() || self.short.is_some()
    }

    // Gets an arg ready for processing
    #[doc(hidden)]
    pub fn _build(&mut self) {
        for s in &self.settings {
            self._settings.set(*s);
        }
    }

    #[doc(hidden)]
    pub fn _longest_filter(&self) -> bool {
        self.long.is_some() || self.is_set(ArgSettings::TakesValue) || self.index.is_some()
    }

    // --------- DEPRECATIONS ----------

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::UseValueDelimiter) instead")]
    pub fn use_delimiter(mut self, d: bool) -> Self {
        if d {
            if self.value_delimiter.is_none() {
                self.value_delimiter = Some(',');
            }
            self.setb(ArgSettings::TakesValue);
            self.setb(ArgSettings::UseValueDelimiter);
            self.unset_setting(ArgSettings::ValueDelimiterNotSet)
        } else {
            self.value_delimiter = None;
            self.unsetb(ArgSettings::UseValueDelimiter);
            self.unset_setting(ArgSettings::ValueDelimiterNotSet)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::NextLineHelp) instead")]
    pub fn next_line_help(mut self, nlh: bool) -> Self {
        if nlh {
            self.setb(ArgSettings::NextLineHelp);
        } else {
            self.unsetb(ArgSettings::NextLineHelp);
        }
        self
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::RequireDelimiter) instead")]
    pub fn require_delimiter(mut self, d: bool) -> Self {
        if d {
            self = self.setting(ArgSettings::RequireDelimiter);
            self.unsetb(ArgSettings::ValueDelimiterNotSet);
            self.setb(ArgSettings::UseValueDelimiter);
            self.setting(ArgSettings::RequireDelimiter)
        } else {
            self = self.unset_setting(ArgSettings::RequireDelimiter);
            self.unsetb(ArgSettings::UseValueDelimiter);
            self.unset_setting(ArgSettings::RequireDelimiter)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::Global) instead")]
    pub fn global(self, g: bool) -> Self {
        if g {
            self.setting(ArgSettings::Global)
        } else {
            self.unset_setting(ArgSettings::Global)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::Hidden) instead")]
    pub fn hidden(self, h: bool) -> Self {
        if h {
            self.setting(ArgSettings::Hidden)
        } else {
            self.unset_setting(ArgSettings::Hidden)
        }
    }


    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::EmptyValues) instead")]
    pub fn empty_values(mut self, ev: bool) -> Self {
        if ev {
            self.setting(ArgSettings::EmptyValues)
        } else {
            self = self.setting(ArgSettings::TakesValue);
            self.unset_setting(ArgSettings::EmptyValues)
        }
    }


    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::Multiple) instead")]
    pub fn multiple(self, multi: bool) -> Self {
        if multi {
            self.setting(ArgSettings::Multiple)
        } else {
            self.unset_setting(ArgSettings::Multiple)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::HidePossibleValues) instead")]
    pub fn hide_possible_values(self, hide: bool) -> Self {
        if hide {
            self.setting(ArgSettings::HidePossibleValues)
        } else {
            self.unset_setting(ArgSettings::HidePossibleValues)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::TakesValue) instead")]
    pub fn takes_value(self, tv: bool) -> Self {
        if tv {
            self.setting(ArgSettings::TakesValue)
        } else {
            self.unset_setting(ArgSettings::TakesValue)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::AllowHyphenValues) instead")]
    pub fn allow_hyphen_values(self, a: bool) -> Self {
        if a {
            self.setting(ArgSettings::AllowHyphenValues)
        } else {
            self.unset_setting(ArgSettings::AllowHyphenValues)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::new instead")]
    pub fn with_name(n: &'a str) -> Self { Arg::new(n) }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::from or serde instead")]
    #[cfg(feature = "yaml")]
    pub fn from_yaml(y: &BTreeMap<Yaml, Yaml>) -> Arg<'n, 'e> { Arg::from(y) }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::from instead")]
    pub fn from_usage(u: &'a str) -> Self { Arg::from(u) }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::Last) instead")]
    pub fn last(self, l: bool) -> Self {
        if l {
            self.setting(ArgSettings::Last)
        } else {
            self.unset_setting(ArgSettings::Last)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::Required) instead")]
    pub fn required(self, r: bool) -> Self {
        if r {
            self.setting(ArgSettings::Required)
        } else {
            self.unset_setting(ArgSettings::Required)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::RequireEquals) instead")]
    pub fn require_equals(mut self, r: bool) -> Self {
        if r {
            self.unsetb(ArgSettings::EmptyValues);
            self.setting(ArgSettings::RequireEquals)
        } else {
            self.unset_setting(ArgSettings::RequireEquals)
        }
    }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "use Arg::set(ArgSettings::HideDefaultValue) instead")]
    pub fn hide_default_value(self, hide: bool) -> Self {
        if hide {
            self.setting(ArgSettings::HideDefaultValue)
        } else {
            self.unset_setting(ArgSettings::HideDefaultValue)
        }
    }
}

#[cfg(feature = "yaml")]
impl<'n, 'e, 'z> From<&'z BTreeMap<Yaml, Yaml>> for Arg<'n, 'e> {
    /// Creates a new instance of [`Arg`] from a .yml (YAML) file.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[macro_use]
    /// # extern crate clap;
    /// # use clap::Arg;
    /// # fn main() {
    /// let yml = load_yaml!("arg.yml");
    /// let arg = Arg::from_yaml(yml);
    /// # }
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    pub fn from(y: &'z BTreeMap<Yaml, Yaml>) -> Self {
        // We WANT this to panic on error...so expect() is good.
        let name_yml = y.keys().nth(0).unwrap();
        let name_str = name_yml.as_str().unwrap();
        let mut a = Arg::new(name_str);
        let arg_settings = y.get(name_yml).unwrap().as_hash().unwrap();

        for (k, v) in arg_settings.iter() {
            a = match k.as_str().unwrap() {
                "short" => yaml_to_str!(a, v, short),
                "long" => yaml_to_str!(a, v, long),
                "aliases" => yaml_vec_or_str!(v, a, alias),
                "help" => yaml_to_str!(a, v, help),
                "long_help" => yaml_to_str!(a, v, long_help),
                "required" => yaml_to_bool!(a, v, required),
                "required_if" => yaml_tuple2!(a, v, required_if),
                "required_ifs" => yaml_tuple2!(a, v, required_if),
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
                "allow_hyphen_values" => yaml_to_bool!(a, v, allow_hyphen_values),
                "require_delimiter" => yaml_to_bool!(a, v, require_delimiter),
                "value_delimiter" => yaml_to_str!(a, v, value_delimiter),
                "required_unless" => yaml_to_str!(a, v, required_unless),
                "display_order" => yaml_to_usize!(a, v, display_order),
                "default_value" => yaml_to_str!(a, v, default_value),
                "default_value_if" => yaml_tuple3!(a, v, default_value_if),
                "default_value_ifs" => yaml_tuple3!(a, v, default_value_if),
                "value_names" => yaml_vec_or_str!(v, a, value_name),
                "groups" => yaml_vec_or_str!(v, a, group),
                "requires" => yaml_vec_or_str!(v, a, requires),
                "requires_if" => yaml_tuple2!(a, v, requires_if),
                "requires_ifs" => yaml_tuple2!(a, v, requires_if),
                "conflicts_with" => yaml_vec_or_str!(v, a, conflicts_with),
                "overrides_with" => yaml_vec_or_str!(v, a, overrides_with),
                "possible_values" => yaml_vec_or_str!(v, a, possible_value),
                "required_unless_one" => yaml_vec_or_str!(v, a, required_unless),
                "required_unless_all" => {
                    a = yaml_vec_or_str!(v, a, required_unless);
                    a.setb(ArgSettings::RequiredUnlessAll);
                    a
                }
                s => {
                    panic!(
                        "Unknown Arg setting '{}' in YAML file for arg '{}'",
                        s,
                        name_str
                    )
                }
            }
        }

        a
    }
}

impl<'n, 'e> From<&'n str> for Arg<'n, 'e> {
    /// Creates a new instance of [`Arg`] from a usage string. Allows creation of basic settings
    /// for the [`Arg`]. The syntax is flexible, but there are some rules to follow.
    ///
    /// **NOTE**: Not all settings may be set using the usage string method. Some properties are
    /// only available via the builder pattern.
    ///
    /// **NOTE**: Only ASCII values are officially supported in [`Arg::from`] strings. Some
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
    /// App::new("prog")
    ///     .args(&[
    ///         Arg::from("--config <FILE> 'a required file for the configuration and no short'"),
    ///         Arg::from("-d, --debug... 'turns on debugging information and allows multiples'"),
    ///         Arg::from("[input] 'an optional input file to use'")
    /// ])
    /// # ;
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    /// [`Arg::from`]: ./struct.Arg.html#method.from_usage
    fn from(u: &'n str) -> Arg<'n, 'e> {
        let parser = UsageParser::from_usage(u);
        parser.parse()
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Arg<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        a.clone()
    }
}

impl<'n, 'e> Display for Arg<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self._settings.is_set(ArgSettings::TakesValue) {
            if self.index.is_some() {
                // Display for Positionals
                if let Some(ref names) = self.value_names {
                    try!(write!(
                        f,
                        "{}",
                        names
                            .values()
                            .map(|n| format!("<{}>", n))
                            .collect::<Vec<_>>()
                            .join(" ")
                    ));
                } else {
                    try!(write!(f, "<{}>", self.name));
                }
                if self.is_set(ArgSettings::Multiple) &&
                    (self.value_names.is_none() || self.value_names.as_ref().unwrap().len() == 1)
                {
                    try!(write!(f, "..."));
                }
            } else {
                // Display for Opts
                debugln!("Opt::fmt:{}", self.name);
                let sep = if self.is_set(ArgSettings::RequireEquals) {
                    "="
                } else {
                    " "
                };
                // Write the name such --long or -l
                if let Some(l) = self.long {
                    try!(write!(f, "--{}{}", l, sep));
                } else {
                    try!(write!(f, "-{}{}", self.short.unwrap(), sep));
                }

                // Write the values such as <name1> <name2>
                if let Some(ref vec) = self.value_names {
                    let mut it = vec.iter().peekable();
                    while let Some((_, val)) = it.next() {
                        try!(write!(f, "<{}>", val));
                        if it.peek().is_some() {
                            try!(write!(f, " "));
                        }
                    }
                    let num = vec.len();
                    if self.is_set(ArgSettings::Multiple) && num == 1 {
                        try!(write!(f, "..."));
                    }
                } else if let Some(num) = self.number_of_values {
                    let mut it = (0..num).peekable();
                    while let Some(_) = it.next() {
                        try!(write!(f, "<{}>", self.name));
                        if it.peek().is_some() {
                            try!(write!(f, " "));
                        }
                    }
                    if self.is_set(ArgSettings::Multiple) && num == 1 {
                        try!(write!(f, "..."));
                    }
                } else {
                    try!(write!(
                        f,
                        "<{}>{}",
                        self.name,
                        if self.is_set(ArgSettings::Multiple) {
                            "..."
                        } else {
                            ""
                        }
                    ));
                }
            }
        } else {
            // Display for Flags
            if let Some(l) = self.long {
                try!(write!(f, "--{}", l));
            } else {
                try!(write!(f, "-{}", self.short.unwrap()));
            }
        }

        Ok(())
    }
}

impl<'n, 'e> DispOrder for Arg<'n, 'e> {
    fn disp_ord(&self) -> usize { self.display_order }
}

impl<'n, 'e> PartialEq for Arg<'n, 'e> {
    fn eq(&self, other: &Arg<'n, 'e>) -> bool { self.name == other.name }
}

impl<'n, 'e> fmt::Debug for Arg<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Arg {{ name: {:?}, help: {:?}, long_help: {:?}, conflicts_with: {:?}, \
            settings: {:?}, required_unless: {:?}, overrides_with: {:?}, groups: {:?}, \
            requires: {:?}, requires_ifs: {:?}, short: {:?}, index: {:?}, long: {:?}, \
            aliases: {:?}, visible_aliases: {:?}, possible_values: {:?}, value_names: {:?}, \
            number_of_values: {:?}, max_values: {:?}, min_values: {:?}, value_delimiter: {:?}, \
            default_value_ifs: {:?}, value_terminator: {:?}, display_order: {:?}, validator: {}, \
            validator_os: {} \
        }}", 
            self.name,
            self.help,
            self.long_help,
            self.conflicts_with,
            self.settings,
            self.required_unless,
            self.overrides_with,
            self.groups,
            self.requires,
            self.requires_ifs,
            self.short,
            self.index,
            self.long,
            self.aliases,
            self.visible_aliases,
            self.possible_values,
            self.value_names,
            self.number_of_values,
            self.max_values,
            self.min_values,
            self.value_delimiter,
            self.default_value_ifs,
            self.value_terminator,
            self.display_order,
            self.validator.as_ref().map_or("None", |_| "Some(Fn)"),
            self.validator_os.as_ref().map_or("None", |_| "Some(Fn)")
        )
    }
}

#[cfg(test)]
mod test {
    use args::settings::ArgSettings;
    use super::Arg;
    use vec_map::VecMap;

    // Flags
    #[test]
    fn flag_long_display() {
        let mut f = Arg::new("flg");
        f._settings.set(ArgSettings::Multiple);
        f.long = Some("flag");

        assert_eq!(&*format!("{}", f), "--flag");
    }

    #[test]
    fn flag_short_display() {
        let mut f2 = Arg::new("flg");
        f2.short = Some('f');

        assert_eq!(&*format!("{}", f2), "-f");
    }

    #[test]
    fn flag_display_single_alias() {
        let mut f = Arg::new("flg");
        f.long = Some("flag");
        f.visible_aliases = Some(vec!["als"]);

        assert_eq!(&*format!("{}", f), "--flag");
    }

    #[test]
    fn flag_display_multiple_aliases() {
        let mut f = Flag::new("flg");
        f.long = Some("fl");
        f.visible_aliases = Some(vec!["f2", "f3", "f4"]);
        assert_eq!(&*format!("{}", f), "--fl");
    }

    // Opts
    #[test]
    fn opt_display1() {
        let mut o = Arg::new("opt");
        o.long = Some("option");
        o._settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", o), "--option <opt>...");
    }

    #[test]
    fn opt_display2() {
        let v_names = vec!["file", "name"];

        let mut o2 = Arg::new("opt");
        o2.short = Some('o');
        o2.value_names = Some(v_names);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }

    #[test]
    fn opt_display3() {
        let v_names = vec!["file", "name"];

        let mut o2 = Arg::new("opt");
        o2.short = Some('o');
        o2.value_names = Some(v_names);
        o2._settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }

    #[test]
    fn opt_display_single_alias() {
        let mut o = Arg::new("opt");
        o.long = Some("option");
        o.visible_aliases = Some(vec!["als"]);

        assert_eq!(&*format!("{}", o), "--option <opt>");
    }

    #[test]
    fn opt_display_multiple_aliases() {
        let mut o = Arg::new("opt");
        o.long = Some("option");
        o.aliases = Some(vec!["als2", "als3", "als4"]);
        assert_eq!(&*format!("{}", o), "--option <opt>");
    }

    // Positionals
    #[test]
    fn display_mult() {
        let mut p = Arg::new("pos").index(1);
        p._settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", p), "<pos>...");
    }

    #[test]
    fn display_required() {
        let mut p2 = Arg::new("pos").index(1);
        p2._settings.set(ArgSettings::Required);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }

    #[test]
    fn display_val_names() {
        let mut p2 = Arg::new("pos").index(1);
        let mut vm = vec!["file1", "file2"];
        p2.value_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }

    #[test]
    fn display_val_names_req() {
        let mut p2 = Arg::new("pos", 1);
        p2._settings.set(ArgSettings::Required);
        let mut vm = vec!["file1", "file2"];
        p2.value_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }
}

