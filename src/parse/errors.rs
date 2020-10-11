// Std
use std::{
    convert::From,
    error,
    fmt::{self, Debug, Display, Formatter},
    io,
    result::Result as StdResult,
};

// Internal
use crate::{
    build::Arg,
    output::fmt::Colorizer,
    parse::features::suggestions,
    util::{safe_exit, termcolor::ColorChoice},
};

/// Short hand for [`Result`] type
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type Result<T> = StdResult<T, Error>;

/// Command line argument parser kind of error
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    /// Occurs when an [`Arg`] has a set of possible values,
    /// and the user provides a value which isn't in that set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("speed")
    ///         .possible_value("fast")
    ///         .possible_value("slow"))
    ///     .try_get_matches_from(vec!["prog", "other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidValue);
    /// ```
    /// [`Arg`]: ./struct.Arg.html
    InvalidValue,

    /// Occurs when a user provides a flag, option, argument or subcommand which isn't defined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::from("--flag 'some flag'"))
    ///     .try_get_matches_from(vec!["prog", "--other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    UnknownArgument,

    /// Occurs when the user provides an unrecognized [``] which meets the threshold for
    /// being similar enough to an existing subcommand.
    /// If it doesn't meet the threshold, or the 'suggestions' feature is disabled,
    /// the more general [`UnknownArgument`] error is returned.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(feature = "suggestions"), doc = " ```no_run")]
    #[cfg_attr(feature = "suggestions", doc = " ```")]
    /// # use clap::{App, Arg, ErrorKind, };
    /// let result = App::new("prog")
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .about("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "confi"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidSubcommand);
    /// ```
    /// [``]: ./struct..html
    /// [`UnknownArgument`]: ./enum.ErrorKind.html#variant.UnknownArgument
    InvalidSubcommand,

    /// Occurs when the user provides an unrecognized [``] which either
    /// doesn't meet the threshold for being similar enough to an existing subcommand,
    /// or the 'suggestions' feature is disabled.
    /// Otherwise the more detailed [`InvalidSubcommand`] error is returned.
    ///
    /// This error typically happens when passing additional subcommand names to the `help`
    /// subcommand. Otherwise, the more general [`UnknownArgument`] error is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, };
    /// let result = App::new("prog")
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .about("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "help", "nothing"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
    /// ```
    /// [``]: ./struct..html
    /// [`InvalidSubcommand`]: ./enum.ErrorKind.html#variant.InvalidSubcommand
    /// [`UnknownArgument`]: ./enum.ErrorKind.html#variant.UnknownArgument
    UnrecognizedSubcommand,

    /// Occurs when the user provides an empty value for an option that does not allow empty
    /// values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("color")
    ///          .setting(ArgSettings::TakesValue)
    ///          .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--color="]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    EmptyValue,

    /// Occurs when the user provides a value for an argument with a custom validation and the
    /// value fails that validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// fn is_numeric(val: &str) -> Result<(), String> {
    ///     match val.parse::<i64>() {
    ///         Ok(..) => Ok(()),
    ///         Err(..) => Err(String::from("Value wasn't a number!")),
    ///     }
    /// }
    ///
    /// let result = App::new("prog")
    ///     .arg(Arg::new("num")
    ///          .validator(is_numeric))
    ///     .try_get_matches_from(vec!["prog", "NotANumber"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::ValueValidation);
    /// ```
    ValueValidation,

    /// Occurs when a user provides more values for an argument than were defined by setting
    /// [`Arg::max_values`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("arg")
    ///         .multiple(true)
    ///         .max_values(2))
    ///     .try_get_matches_from(vec!["prog", "too", "many", "values"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooManyValues);
    /// ```
    /// [`Arg::max_values`]: ./struct.Arg.html#method.max_values
    TooManyValues,

    /// Occurs when the user provides fewer values for an argument than were defined by setting
    /// [`Arg::min_values`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("some_opt")
    ///         .long("opt")
    ///         .min_values(3))
    ///     .try_get_matches_from(vec!["prog", "--opt", "too", "few"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooFewValues);
    /// ```
    /// [`Arg::min_values`]: ./struct.Arg.html#method.min_values
    TooFewValues,

    /// Occurs when the user provides a different number of values for an argument than what's
    /// been defined by setting [`Arg::number_of_values`] or than was implicitly set by
    /// [`Arg::value_names`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("some_opt")
    ///         .long("opt")
    ///         .takes_value(true)
    ///         .number_of_values(2))
    ///     .try_get_matches_from(vec!["prog", "--opt", "wrong"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
    /// ```
    ///
    /// [`Arg::number_of_values`]: ./struct.Arg.html#method.number_of_values
    /// [`Arg::value_names`]: ./struct.Arg.html#method.value_names
    WrongNumberOfValues,

    /// Occurs when the user provides two values which conflict with each other and can't be used
    /// together.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .conflicts_with("color"))
    ///     .arg(Arg::new("color")
    ///         .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--debug", "--color"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::ArgumentConflict);
    /// ```
    ArgumentConflict,

    /// Occurs when the user does not provide one or more required arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .required(true))
    ///     .try_get_matches_from(vec!["prog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    MissingRequiredArgument,

    /// Occurs when a subcommand is required (as defined by [`AppSettings::SubcommandRequired`]),
    /// but the user does not provide one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let err = App::new("prog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog",
    ///     ]);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err().kind, ErrorKind::MissingSubcommand);
    /// # ;
    /// ```
    /// [`AppSettings::SubcommandRequired`]: ./enum.AppSettings.html#variant.SubcommandRequired
    MissingSubcommand,

    /// Occurs when the user provides multiple values to an argument which doesn't allow that.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .multiple(false))
    ///     .try_get_matches_from(vec!["prog", "--debug", "--debug"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnexpectedMultipleUsage);
    /// ```
    UnexpectedMultipleUsage,

    /// Occurs when the user provides a value containing invalid UTF-8 for an argument and
    /// [`AppSettings::StrictUtf8`] is set.
    ///
    /// # Platform Specific
    ///
    /// Non-Windows platforms only (such as Linux, Unix, OSX, etc.)
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg, ErrorKind, AppSettings};
    /// # use std::os::unix::ffi::OsStringExt;
    /// # use std::ffi::OsString;
    /// let result = App::new("prog")
    ///     .setting(AppSettings::StrictUtf8)
    ///     .arg(Arg::new("utf8")
    ///         .short('u')
    ///         .takes_value(true))
    ///     .try_get_matches_from(vec![OsString::from("myprog"),
    ///                                 OsString::from("-u"),
    ///                                 OsString::from_vec(vec![0xE9])]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidUtf8);
    /// ```
    /// [`AppSettings::StrictUtf8`]: ./enum.AppSettings.html#variant.StrictUtf8
    InvalidUtf8,

    /// Not a true "error" as it means `--help` or similar was used.
    /// The help message will be sent to `stdout`.
    ///
    /// **Note**: If the help is displayed due to an error (such as missing subcommands) it will
    /// be sent to `stderr` instead of `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .try_get_matches_from(vec!["prog", "--help"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayHelp);
    /// ```
    DisplayHelp,

    /// Occurs when either an argument or a [`subcommand`] is required, as defined by
    /// [`AppSettings::ArgRequiredElseHelp`] and
    /// [`AppSettings::SubcommandRequiredElseHelp`], but the user did not provide
    /// one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, ErrorKind, };
    /// let result = App::new("prog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .about("The configuration file to use")))
    ///     .try_get_matches_from(vec!["prog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand);
    /// ```
    /// [`AppSettings::ArgRequiredElseHelp`]: ./enum.AppSettings.html#variant.ArgRequiredElseHelp
    /// [`AppSettings::SubcommandRequiredElseHelp`]: ./enum.AppSettings.html#variant.SubcommandRequiredElseHelp
    /// [`subcommand`]: ./struct.App.html#method.subcommand
    DisplayHelpOnMissingArgumentOrSubcommand,

    /// Not a true "error" as it means `--version` or similar was used.
    /// The message will be sent to `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .try_get_matches_from(vec!["prog", "--version"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayVersion);
    /// ```
    DisplayVersion,

    /// Occurs when using the [`ArgMathes::value_of_t`] and friends to convert an argument value
    /// into type `T`, but the argument you requested wasn't used. I.e. you asked for an argument
    /// with name `config` to be converted, but `config` wasn't used by the user.
    ArgumentNotFound,

    /// Represents an [I/O error].
    /// Can occur when writing to `stderr` or `stdout` or reading a configuration file.
    /// [I/O error]: https://doc.rust-lang.org/std/io/struct.Error.html
    Io,

    /// Represents a [Format error] (which is a part of [`Display`]).
    /// Typically caused by writing to `stderr` or `stdout`.
    ///
    /// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
    /// [Format error]: https://doc.rust-lang.org/std/fmt/struct.Error.html
    Format,
}

/// Command Line Argument Parser Error
#[derive(Debug)]
pub struct Error {
    /// Formatted error message, enhancing the cause message with extra information
    pub(crate) message: Colorizer,
    /// The type of error
    pub kind: ErrorKind,
    /// Additional information depending on the error kind, like values and argument names.
    /// Useful when you want to render an error of your own.
    pub info: Vec<String>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.message, f)
    }
}

fn start_error(c: &mut Colorizer, msg: impl Into<String>) {
    c.error("error:");
    c.none(" ");
    c.none(msg);
}

fn put_usage(c: &mut Colorizer, usage: impl Into<String>) {
    c.none("\n\n");
    c.none(usage);
}

fn try_help(c: &mut Colorizer) {
    c.none("\n\nFor more information try ");
    c.good("--help");
    c.none("\n");
}

impl Error {
    /// Returns the singular or plural form on the verb to be based on the argument's value.
    fn singular_or_plural(n: usize) -> String {
        if n > 1 {
            String::from("were")
        } else {
            String::from("was")
        }
    }

    /// Should the message be written to `stdout` or not
    #[inline]
    pub fn use_stderr(&self) -> bool {
        !matches!(
            self.kind,
            ErrorKind::DisplayHelp
                | ErrorKind::DisplayVersion
                | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        )
    }

    /// Prints the error to `stderr` and exits with a status of `1`
    pub fn exit(&self) -> ! {
        if self.use_stderr() {
            self.message.print().expect("Error writing Error to stderr");
            safe_exit(1);
        }

        self.message.print().expect("Error writing Error to stdout");
        safe_exit(0)
    }

    pub(crate) fn argument_conflict(
        arg: &Arg,
        other: Option<String>,
        usage: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' cannot be used with ");

        match other {
            Some(ref name) => {
                c.none("'");
                c.warning(name);
                c.none("'");
            }
            None => {
                c.none("one or more of the other specified arguments");
            }
        };

        put_usage(&mut c, usage);
        try_help(&mut c);

        let mut info = vec![arg];
        if let Some(other) = other {
            info.push(other);
        }

        Error {
            message: c,
            kind: ErrorKind::ArgumentConflict,
            info,
        }
    }

    pub(crate) fn empty_value(arg: &Arg, usage: String, color: ColorChoice) -> Self {
        let mut c = Colorizer::new(true, color);
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' requires a value but none was supplied");
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::EmptyValue,
            info: vec![arg],
        }
    }

    pub(crate) fn invalid_value<G>(
        bad_val: String,
        good_vals: &[G],
        arg: &Arg,
        usage: String,
        color: ColorChoice,
    ) -> Self
    where
        G: AsRef<str> + Display,
    {
        let mut c = Colorizer::new(true, color);
        let suffix = suggestions::did_you_mean(&bad_val, good_vals.iter()).pop();

        let mut sorted: Vec<String> = good_vals
            .iter()
            .map(|v| v.to_string())
            .map(|v| {
                if v.contains(char::is_whitespace) {
                    format!("{:?}", v)
                } else {
                    v
                }
            })
            .collect();
        sorted.sort();

        start_error(&mut c, "");
        c.warning(format!("{:?}", bad_val));
        c.none(" isn't a valid value for '");
        c.warning(arg.to_string());
        c.none("'\n\t[possible values: ");

        if let Some((last, elements)) = sorted.split_last() {
            for v in elements {
                c.good(v);
                c.none(", ");
            }

            c.good(last);
        }

        c.none("]");

        if let Some(val) = suffix {
            c.none("\n\n\tDid you mean ");
            c.good(format!("{:?}", val));
            c.none("?");
        }

        put_usage(&mut c, usage);
        try_help(&mut c);

        let mut info = vec![arg.to_string(), bad_val];
        info.extend(sorted);

        Error {
            message: c,
            kind: ErrorKind::InvalidValue,
            info: vec![],
        }
    }

    pub(crate) fn invalid_subcommand(
        subcmd: String,
        did_you_mean: String,
        name: String,
        usage: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "The subcommand '");
        c.warning(subcmd.clone());
        c.none("' wasn't recognized\n\n\tDid you mean ");
        c.good(did_you_mean);
        c.none("");
        c.none(format!(
            "?\n\nIf you believe you received this message in error, try re-running with '{} ",
            name
        ));
        c.good("--");
        c.none(format!(" {}'", subcmd));
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::InvalidSubcommand,
            info: vec![subcmd],
        }
    }

    pub(crate) fn unrecognized_subcommand(
        subcmd: String,
        name: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, " The subcommand '");
        c.warning(subcmd.clone());
        c.none("' wasn't recognized\n\n");
        c.warning("USAGE:");
        c.none(format!("\n\t{} help <subcommands>...", name));
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::UnrecognizedSubcommand,
            info: vec![subcmd],
        }
    }

    pub(crate) fn missing_required_argument(
        required: Vec<String>,
        usage: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(
            &mut c,
            "The following required arguments were not provided:",
        );

        let mut info = vec![];
        for v in required {
            c.none("\n    ");
            c.good(v.to_string());
            info.push(v.to_string());
        }

        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::MissingRequiredArgument,
            info,
        }
    }

    pub(crate) fn missing_subcommand(name: String, usage: String, color: ColorChoice) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "'");
        c.warning(name);
        c.none("' requires a subcommand, but one was not provided");
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::MissingSubcommand,
            info: vec![],
        }
    }

    pub(crate) fn invalid_utf8(usage: String, color: ColorChoice) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(
            &mut c,
            "Invalid UTF-8 was detected in one or more arguments",
        );
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::InvalidUtf8,
            info: vec![],
        }
    }

    pub(crate) fn too_many_values(
        val: String,
        arg: &Arg,
        usage: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "The value '");
        c.warning(val.clone());
        c.none("' was provided to '");
        c.warning(arg.to_string());
        c.none("' but it wasn't expecting any more values");
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::TooManyValues,
            info: vec![arg.to_string(), val],
        }
    }

    pub(crate) fn too_few_values(
        arg: &Arg,
        min_vals: u64,
        curr_vals: usize,
        usage: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);
        let verb = Error::singular_or_plural(curr_vals);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' requires at least ");
        c.warning(min_vals.to_string());
        c.none(" values, but only ");
        c.warning(curr_vals.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::TooFewValues,
            info: vec![arg.to_string(), curr_vals.to_string(), min_vals.to_string()],
        }
    }

    pub(crate) fn value_validation(
        arg: String,
        val: String,
        err: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "Invalid value");

        c.none(" for '");
        c.warning(arg.clone());
        c.none("'");

        c.none(format!(": {}", err));
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::ValueValidation,
            info: vec![arg, val, err],
        }
    }

    pub(crate) fn wrong_number_of_values(
        arg: &Arg,
        num_vals: u64,
        curr_vals: usize,
        usage: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);
        let verb = Error::singular_or_plural(curr_vals);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' requires ");
        c.warning(num_vals.to_string());
        c.none(" values, but ");
        c.warning(curr_vals.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::WrongNumberOfValues,
            info: vec![arg.to_string(), curr_vals.to_string(), num_vals.to_string()],
        }
    }

    pub(crate) fn unexpected_multiple_usage(arg: &Arg, usage: String, color: ColorChoice) -> Self {
        let mut c = Colorizer::new(true, color);
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' was provided more than once, but cannot be used multiple times");
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::UnexpectedMultipleUsage,
            info: vec![arg],
        }
    }

    pub(crate) fn unknown_argument(
        arg: String,
        did_you_mean: Option<(String, Option<String>)>,
        usage: String,
        color: ColorChoice,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "Found argument '");
        c.warning(arg.clone());
        c.none("' which wasn't expected, or isn't valid in this context");

        if let Some((flag, subcmd)) = did_you_mean {
            let flag = format!("--{}", flag);
            c.none("\n\n\tDid you mean ");

            if let Some(subcmd) = subcmd {
                c.none("to put '");
                c.good(flag);
                c.none("' after the subcommand '");
                c.good(subcmd);
                c.none("'?");
            } else {
                c.none("'");
                c.good(flag);
                c.none("'?");
            }
        }

        // If the user wants to supply things like `--a-flag` or `-b` as a value,
        // suggest `--` for disambiguation.
        if arg.starts_with('-') {
            c.none(format!(
                "\n\n\tIf you tried to supply `{}` as a value rather than a flag, use `-- {}`",
                arg, arg
            ));
        }

        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::UnknownArgument,
            info: vec![arg],
        }
    }

    pub(crate) fn unnecessary_double_dash(arg: String, usage: String, color: ColorChoice) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "Found argument '");
        c.warning(arg.clone());
        c.none("' which wasn't expected, or isn't valid in this context");

        c.none(format!(
            "\n\n\tIf you tried to supply `{}` as a subcommand, remove the '--' before it.",
            arg
        ));
        put_usage(&mut c, usage);
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::UnknownArgument,
            info: vec![arg],
        }
    }

    pub(crate) fn argument_not_found_auto(arg: String) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Auto);

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' wasn't found");
        try_help(&mut c);

        Error {
            message: c,
            kind: ErrorKind::ArgumentNotFound,
            info: vec![arg],
        }
    }

    /// Create an error with a custom description.
    ///
    /// This can be used in combination with `Error::exit` to exit your program
    /// with a custom error message.
    pub fn with_description(description: String, kind: ErrorKind) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Auto);

        start_error(&mut c, description);

        Error {
            message: c,
            kind,
            info: vec![],
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::with_description(e.to_string(), ErrorKind::Io)
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::with_description(e.to_string(), ErrorKind::Format)
    }
}

impl error::Error for Error {}
