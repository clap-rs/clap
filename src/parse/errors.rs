// Std
use std::{
    borrow::Cow,
    convert::From,
    error,
    fmt::{self, Debug, Display, Formatter},
    io::{self, BufRead},
    result::Result as StdResult,
};

// Internal
use crate::{
    build::Arg,
    output::fmt::Colorizer,
    parse::features::suggestions,
    util::{color::ColorChoice, safe_exit, SUCCESS_CODE, USAGE_CODE},
    App, AppSettings,
};

/// Short hand for [`Result`] type
///
/// [`Result`]: std::result::Result
pub type Result<T> = StdResult<T, Error>;

/// Command line argument parser kind of error
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
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
    InvalidValue,

    /// Occurs when a user provides a flag, option, argument or subcommand which isn't defined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(arg!(--flag "some flag"))
    ///     .try_get_matches_from(vec!["prog", "--other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    UnknownArgument,

    /// Occurs when the user provides an unrecognized [`Subcommand`] which meets the threshold for
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
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "confi"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`UnknownArgument`]: ErrorKind::UnknownArgument
    InvalidSubcommand,

    /// Occurs when the user provides an unrecognized [`Subcommand`] which either
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
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "help", "nothing"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnrecognizedSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`InvalidSubcommand`]: ErrorKind::InvalidSubcommand
    /// [`UnknownArgument`]: ErrorKind::UnknownArgument
    UnrecognizedSubcommand,

    /// Occurs when the user provides an empty value for an option that does not allow empty
    /// values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("color")
    ///          .takes_value(true)
    ///          .forbid_empty_values(true)
    ///          .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--color="]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    EmptyValue,

    /// Occurs when the user doesn't use equals for an option that requires equal
    /// sign to provide values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("color")
    ///          .takes_value(true)
    ///          .require_equals(true)
    ///          .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--color", "red"]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::NoEquals);
    /// ```
    NoEquals,

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
    ///         .max_values(2))
    ///     .try_get_matches_from(vec!["prog", "too", "many", "values"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooManyValues);
    /// ```
    /// [`Arg::max_values`]: Arg::max_values()
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
    /// [`Arg::min_values`]: Arg::min_values()
    TooFewValues,

    /// Occurs when a user provides more occurrences for an argument than were defined by setting
    /// [`Arg::max_occurrences`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("verbosity")
    ///         .short('v')
    ///         .max_occurrences(2))
    ///     .try_get_matches_from(vec!["prog", "-vvv"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooManyOccurrences);
    /// ```
    /// [`Arg::max_occurrences`]: Arg::max_occurrences()
    TooManyOccurrences,

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
    /// [`Arg::number_of_values`]: Arg::number_of_values()
    /// [`Arg::value_names`]: Arg::value_names()
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
    ///
    /// [`AppSettings::SubcommandRequired`]: crate::AppSettings::SubcommandRequired
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
    ///         .multiple_occurrences(false))
    ///     .try_get_matches_from(vec!["prog", "--debug", "--debug"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnexpectedMultipleUsage);
    /// ```
    UnexpectedMultipleUsage,

    /// Occurs when the user provides a value containing invalid UTF-8.
    ///
    /// To allow arbitrary data
    /// - Set [`Arg::allow_invalid_utf8`] for argument values
    /// - Set [`AppSettings::AllowInvalidUtf8ForExternalSubcommands`] for external-subcommand
    ///   values
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
    ///     .arg(Arg::new("utf8")
    ///         .short('u')
    ///         .takes_value(true))
    ///     .try_get_matches_from(vec![OsString::from("myprog"),
    ///                                 OsString::from("-u"),
    ///                                 OsString::from_vec(vec![0xE9])]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidUtf8);
    /// ```
    ///
    /// [`Arg::allow_invalid_utf8`]: crate::Arg::allow_invalid_utf8
    /// [`AppSettings::AllowInvalidUtf8ForExternalSubcommands`]: crate::AppSettings::AllowInvalidUtf8ForExternalSubcommands
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

    /// Occurs when either an argument or a [`Subcommand`] is required, as defined by
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
    ///             .help("The configuration file to use")))
    ///     .try_get_matches_from(vec!["prog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`AppSettings::ArgRequiredElseHelp`]: crate::AppSettings::ArgRequiredElseHelp
    /// [`AppSettings::SubcommandRequiredElseHelp`]: crate::AppSettings::SubcommandRequiredElseHelp
    DisplayHelpOnMissingArgumentOrSubcommand,

    /// Not a true "error" as it means `--version` or similar was used.
    /// The message will be sent to `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .version("3.0")
    ///     .try_get_matches_from(vec!["prog", "--version"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::DisplayVersion);
    /// ```
    DisplayVersion,

    /// Occurs when using the [`ArgMatches::value_of_t`] and friends to convert an argument value
    /// into type `T`, but the argument you requested wasn't used. I.e. you asked for an argument
    /// with name `config` to be converted, but `config` wasn't used by the user.
    ///
    /// [`ArgMatches::value_of_t`]: crate::ArgMatches::value_of_t()
    ArgumentNotFound,

    /// Represents an [I/O error].
    /// Can occur when writing to `stderr` or `stdout` or reading a configuration file.
    ///
    /// [I/O error]: std::io::Error
    Io,

    /// Represents a [Format error] (which is a part of [`Display`]).
    /// Typically caused by writing to `stderr` or `stdout`.
    ///
    /// [`Display`]: std::fmt::Display
    /// [Format error]: std::fmt::Error
    Format,
}

/// Command Line Argument Parser Error
///
/// See [`App::error`] to create an error.
///
/// [`App::error`]: crate::App::error
#[derive(Debug)]
pub struct Error {
    /// Formatted error message, enhancing the cause message with extra information
    pub(crate) message: Message,
    /// The type of error
    pub kind: ErrorKind,
    /// Additional information depending on the error kind, like values and argument names.
    /// Useful when you want to render an error of your own.
    pub info: Vec<String>,
    pub(crate) source: Option<Box<dyn error::Error + Send + Sync>>,
    wait_on_exit: bool,
    backtrace: Option<Backtrace>,
}

impl Error {
    /// Create an unformatted error
    ///
    /// This is for you need to pass the error up to
    /// a place that has access to the `App` at which point you can call [`Error::format`].
    ///
    /// Prefer [`App::error`] for generating errors.
    ///
    /// [`App::error`]: crate::App::error
    pub fn raw(kind: ErrorKind, message: impl std::fmt::Display) -> Self {
        Self::new(message.to_string(), kind, false)
    }

    /// Format the existing message with the App's context
    #[must_use]
    pub fn format(mut self, app: &mut App) -> Self {
        app._build();
        let usage = app.render_usage();
        self.message.format(app, usage);
        self.wait_on_exit = app.settings.is_set(AppSettings::WaitOnError);
        self
    }

    /// Should the message be written to `stdout` or not?
    #[inline]
    pub fn use_stderr(&self) -> bool {
        !matches!(
            self.kind,
            ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
        )
    }

    /// Prints the error and exits.
    ///
    /// Depending on the error kind, this either prints to `stderr` and exits with a status of `1`
    /// or prints to `stdout` and exits with a status of `0`.
    pub fn exit(&self) -> ! {
        if self.use_stderr() {
            // Swallow broken pipe errors
            let _ = self.print();

            if self.wait_on_exit {
                wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                let mut s = String::new();
                let i = io::stdin();
                i.lock().read_line(&mut s).unwrap();
            }

            safe_exit(USAGE_CODE);
        }

        // Swallow broken pipe errors
        let _ = self.print();
        safe_exit(SUCCESS_CODE)
    }

    /// Prints formatted and colored error to `stdout` or `stderr` according to its error kind
    ///
    /// # Example
    /// ```no_run
    /// use clap::App;
    ///
    /// match App::new("App").try_get_matches() {
    ///     Ok(matches) => {
    ///         // do_something
    ///     },
    ///     Err(err) => {
    ///         err.print().expect("Error writing Error");
    ///         // do_something
    ///     },
    /// };
    /// ```
    pub fn print(&self) -> io::Result<()> {
        self.message.formatted().print()
    }

    /// Deprecated, replaced with [`App::error`]
    ///
    /// [`App::error`]: crate::App::error
    #[deprecated(since = "3.0.0", note = "Replaced with `App::error`")]
    pub fn with_description(description: String, kind: ErrorKind) -> Self {
        Error::raw(kind, description)
    }

    pub(crate) fn new(message: impl Into<Message>, kind: ErrorKind, wait_on_exit: bool) -> Self {
        Self {
            message: message.into(),
            kind,
            info: vec![],
            source: None,
            wait_on_exit,
            backtrace: Backtrace::new(),
        }
    }

    pub(crate) fn set_info(mut self, info: Vec<String>) -> Self {
        self.info = info;
        self
    }

    pub(crate) fn set_source(mut self, source: Box<dyn error::Error + Send + Sync>) -> Self {
        self.source = Some(source);
        self
    }

    pub(crate) fn argument_conflict(
        app: &App,
        arg: &Arg,
        others: Vec<String>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg);
        c.none("' cannot be used with");

        let mut info = vec![];
        match others.len() {
            0 => {
                c.none(" one or more of the other specified arguments");
            }
            1 => {
                let v = &others[0];
                c.none(" '");
                c.warning(v.clone());
                c.none("'");
                info.push(v.clone());
            }
            _ => {
                c.none(":");
                for v in others {
                    c.none("\n    ");
                    c.warning(v.to_string());
                    info.push(v.to_string());
                }
            }
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::ArgumentConflict,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(info)
    }

    pub(crate) fn empty_value(app: &App, arg: &Arg, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' requires a value but none was supplied");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::EmptyValue,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![arg])
    }

    pub(crate) fn no_equals(app: &App, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "Equal sign is needed when assigning values to '");
        c.warning(&arg);
        c.none("'.");

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::NoEquals,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![arg])
    }

    pub(crate) fn invalid_value<G>(
        app: &App,
        bad_val: String,
        good_vals: &[G],
        arg: &Arg,
        usage: String,
    ) -> Self
    where
        G: AsRef<str> + Display,
    {
        let mut c = Colorizer::new(true, app.get_color());
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
        try_help(app, &mut c);

        let mut info = vec![arg.to_string(), bad_val];
        info.extend(sorted);

        Self::new(
            c,
            ErrorKind::InvalidValue,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(info)
    }

    pub(crate) fn invalid_subcommand(
        app: &App,
        subcmd: String,
        did_you_mean: String,
        name: String,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

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
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::InvalidSubcommand,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![subcmd])
    }

    pub(crate) fn unrecognized_subcommand(app: &App, subcmd: String, name: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, " The subcommand '");
        c.warning(subcmd.clone());
        c.none("' wasn't recognized\n\n");
        c.warning("USAGE:");
        c.none(format!("\n    {} <subcommands>", name));
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::UnrecognizedSubcommand,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![subcmd])
    }

    pub(crate) fn missing_required_argument(
        app: &App,
        required: Vec<String>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

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
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::MissingRequiredArgument,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(info)
    }

    pub(crate) fn missing_subcommand(app: &App, name: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "'");
        c.warning(name);
        c.none("' requires a subcommand, but one was not provided");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::MissingSubcommand,
            app.settings.is_set(AppSettings::WaitOnError),
        )
    }

    pub(crate) fn invalid_utf8(app: &App, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(
            &mut c,
            "Invalid UTF-8 was detected in one or more arguments",
        );
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::InvalidUtf8,
            app.settings.is_set(AppSettings::WaitOnError),
        )
    }

    pub(crate) fn too_many_occurrences(
        app: &App,
        arg: &Arg,
        max_occurs: usize,
        curr_occurs: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let verb = Error::singular_or_plural(curr_occurs);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' allows at most ");
        c.warning(max_occurs.to_string());
        c.none(" occurrences, but ");
        c.warning(curr_occurs.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::TooManyOccurrences,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![
            arg.to_string(),
            curr_occurs.to_string(),
            max_occurs.to_string(),
        ])
    }

    pub(crate) fn too_many_values(app: &App, val: String, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "The value '");
        c.warning(val.clone());
        c.none("' was provided to '");
        c.warning(&arg);
        c.none("' but it wasn't expecting any more values");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::TooManyValues,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![arg, val])
    }

    pub(crate) fn too_few_values(
        app: &App,
        arg: &Arg,
        min_vals: usize,
        curr_vals: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let verb = Error::singular_or_plural(curr_vals);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' requires at least ");
        c.warning(min_vals.to_string());
        c.none(" values, but only ");
        c.warning(curr_vals.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::TooFewValues,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![
            arg.to_string(),
            curr_vals.to_string(),
            min_vals.to_string(),
        ])
    }

    pub(crate) fn value_validation(
        app: &App,
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
    ) -> Self {
        let mut err = Self::value_validation_with_color(
            arg,
            val,
            err,
            app.get_color(),
            app.settings.is_set(AppSettings::WaitOnError),
        );
        match &mut err.message {
            Message::Raw(_) => {
                unreachable!("`value_validation_with_color` only deals in formatted errors")
            }
            Message::Formatted(c) => try_help(app, c),
        }
        err
    }

    pub(crate) fn value_validation_without_app(
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
    ) -> Self {
        let mut err = Self::value_validation_with_color(arg, val, err, ColorChoice::Never, false);
        match &mut err.message {
            Message::Raw(_) => {
                unreachable!("`value_validation_with_color` only deals in formatted errors")
            }
            Message::Formatted(c) => {
                c.none("\n");
            }
        }
        err
    }

    fn value_validation_with_color(
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
        color: ColorChoice,
        wait_on_exit: bool,
    ) -> Self {
        let mut c = Colorizer::new(true, color);

        start_error(&mut c, "Invalid value");

        c.none(" for '");
        c.warning(arg.clone());
        c.none("'");

        c.none(format!(": {}", err));

        Self::new(c, ErrorKind::ValueValidation, wait_on_exit)
            .set_info(vec![arg, val, err.to_string()])
            .set_source(err)
    }

    pub(crate) fn wrong_number_of_values(
        app: &App,
        arg: &Arg,
        num_vals: usize,
        curr_vals: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let verb = Error::singular_or_plural(curr_vals);

        start_error(&mut c, "The argument '");
        c.warning(arg.to_string());
        c.none("' requires ");
        c.warning(num_vals.to_string());
        c.none(" values, but ");
        c.warning(curr_vals.to_string());
        c.none(format!(" {} provided", verb));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::WrongNumberOfValues,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![
            arg.to_string(),
            curr_vals.to_string(),
            num_vals.to_string(),
        ])
    }

    pub(crate) fn unexpected_multiple_usage(app: &App, arg: &Arg, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' was provided more than once, but cannot be used multiple times");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::UnexpectedMultipleUsage,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![arg])
    }

    pub(crate) fn unknown_argument(
        app: &App,
        arg: String,
        did_you_mean: Option<(String, Option<String>)>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

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
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::UnknownArgument,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![arg])
    }

    pub(crate) fn unnecessary_double_dash(app: &App, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color());

        start_error(&mut c, "Found argument '");
        c.warning(arg.clone());
        c.none("' which wasn't expected, or isn't valid in this context");

        c.none(format!(
            "\n\n\tIf you tried to supply `{}` as a subcommand, remove the '--' before it.",
            arg
        ));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::new(
            c,
            ErrorKind::UnknownArgument,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(vec![arg])
    }

    pub(crate) fn argument_not_found_auto(arg: String) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Never);

        start_error(&mut c, "The argument '");
        c.warning(arg.clone());
        c.none("' wasn't found\n");

        Self::new(c, ErrorKind::ArgumentNotFound, false).set_info(vec![arg])
    }

    /// Returns the singular or plural form on the verb to be based on the argument's value.
    fn singular_or_plural(n: usize) -> String {
        if n > 1 {
            String::from("were")
        } else {
            String::from("was")
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::raw(ErrorKind::Io, e)
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::raw(ErrorKind::Format, e)
    }
}

impl error::Error for Error {
    #[allow(trivial_casts)]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Assuming `self.message` already has a trailing newline, from `try_help` or similar
        write!(f, "{}", self.message.formatted())?;
        if let Some(backtrace) = self.backtrace.as_ref() {
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            writeln!(f, "{}", backtrace)?;
        }
        Ok(())
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

fn try_help(app: &App, c: &mut Colorizer) {
    if !app.settings.is_set(AppSettings::DisableHelpFlag) {
        c.none("\n\nFor more information try ");
        c.good("--help");
        c.none("\n");
    } else if app.has_subcommands() && !app.settings.is_set(AppSettings::DisableHelpSubcommand) {
        c.none("\n\nFor more information try ");
        c.good("help");
        c.none("\n");
    } else {
        c.none("\n");
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Message {
    Raw(String),
    Formatted(Colorizer),
}

impl Message {
    fn format(&mut self, app: &App, usage: String) {
        match self {
            Message::Raw(s) => {
                let mut c = Colorizer::new(true, app.get_color());

                let mut message = String::new();
                std::mem::swap(s, &mut message);
                start_error(&mut c, message);
                put_usage(&mut c, usage);
                try_help(app, &mut c);
                *self = Self::Formatted(c);
            }
            Message::Formatted(_) => {}
        }
    }

    fn formatted(&self) -> Cow<Colorizer> {
        match self {
            Message::Raw(s) => {
                let mut c = Colorizer::new(true, ColorChoice::Never);
                start_error(&mut c, s);
                Cow::Owned(c)
            }
            Message::Formatted(c) => Cow::Borrowed(c),
        }
    }
}

impl From<String> for Message {
    fn from(inner: String) -> Self {
        Self::Raw(inner)
    }
}

impl From<Colorizer> for Message {
    fn from(inner: Colorizer) -> Self {
        Self::Formatted(inner)
    }
}

#[cfg(feature = "debug")]
#[derive(Debug)]
struct Backtrace(backtrace::Backtrace);

#[cfg(feature = "debug")]
impl Backtrace {
    fn new() -> Option<Self> {
        Some(Self(backtrace::Backtrace::new()))
    }
}

#[cfg(feature = "debug")]
impl Display for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `backtrace::Backtrace` uses `Debug` instead of `Display`
        write!(f, "{:?}", self.0)
    }
}

#[cfg(not(feature = "debug"))]
#[derive(Debug)]
struct Backtrace;

#[cfg(not(feature = "debug"))]
impl Backtrace {
    fn new() -> Option<Self> {
        None
    }
}

#[cfg(not(feature = "debug"))]
impl Display for Backtrace {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /// Check `clap::Error` impls Send and Sync.
    mod clap_error_impl_send_sync {
        use crate::Error;
        trait Foo: std::error::Error + Send + Sync + 'static {}
        impl Foo for Error {}
    }
}
