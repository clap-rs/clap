use std::process;
use std::error::Error as StdError;
use std::fmt as std_fmt;
use std::fmt::Display;
use std::io::{self, Write};
use std::convert::From;
use std::result::Result as StdResult;

use fmt::Format;
use suggestions;
use args::any_arg::AnyArg;

/// Short hand for result type
pub type Result<T> = StdResult<T, Error>;

/// Command line argument parser kind of error
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    /// Occurs when an `Arg` has a set of possible values, and the user provides a value which
    /// isn't in that set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("speed")
    ///         .possible_value("fast")
    ///         .possible_value("slow"))
    ///     .get_matches_from_safe(vec!["myprog", "other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidValue);
    /// ```
    InvalidValue,
    /// Occurs when a user provides a flag, option, or argument which wasn't defined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .arg(Arg::from_usage("--flag 'some flag'"))
    ///     .get_matches_from_safe(vec!["myprog", "--other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    UnknownArgument,
    /// Occurs when the user provids an unrecognized subcommand which meets the threshold for being
    /// similar enough to an existing subcommand so as to not cause the more general
    /// `UnknownArgument` error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, SubCommand};
    /// let result = App::new("myprog")
    ///     .subcommand(SubCommand::with_name("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::with_name("config_file")
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .get_matches_from_safe(vec!["myprog", "confi"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidSubcommand);
    /// ```
    InvalidSubcommand,
    /// Occurs when the user provides an empty value for an option that does not allow empty
    /// values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("myprog")
    ///     .arg(Arg::with_name("color")
    ///          .long("color")
    ///          .empty_values(false))
    ///     .get_matches_from_safe(vec!["myprog", "--color="]);
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
    /// fn is_numeric(val: String) -> Result<(), String> {
    ///     match val.parse::<i64>() {
    ///         Ok(..) => Ok(()),
    ///         Err(..) => Err(String::from("Value wasn't a number!")),
    ///     }
    /// }
    ///
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("num")
    ///          .validator(is_numeric))
    ///     .get_matches_from_safe(vec!["myprog", "NotANumber"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::ValueValidation);
    /// ```
    ValueValidation,
    /// Occurs when a user provides more values for an argument than were defined by setting
    /// `Arg::max_values`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("arg")
    ///         .multiple(true)
    ///         .max_values(2))
    ///     .get_matches_from_safe(vec!["myprog", "too", "many", "values"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooManyValues);
    /// ```
    TooManyValues,
    /// Occurs when the user provides fewer values for an argument than were defined by setting
    /// `Arg::min_values`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("some_opt")
    ///         .long("opt")
    ///         .min_values(3))
    ///     .get_matches_from_safe(vec!["myprog", "--opt", "too", "few"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::TooFewValues);
    /// ```
    TooFewValues,
    /// Occurs when the user provides a different number of values for an argument than what's
    /// been defined by setting `Arg::number_of_values` or than was implicitly set by
    /// `Arg::value_names`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("some_opt")
    ///         .long("opt")
    ///         .takes_value(true)
    ///         .number_of_values(2))
    ///     .get_matches_from_safe(vec!["myprog", "--opt", "wrong"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::WrongNumberOfValues);
    /// ```
    WrongNumberOfValues,
    /// Occurs when the user provides two values which conflict with each other and can't be used
    /// together.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .long("debug")
    ///         .conflicts_with("color"))
    ///     .arg(Arg::with_name("color")
    ///         .long("color"))
    ///     .get_matches_from_safe(vec!["myprog", "--debug", "--color"]);
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
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .required(true))
    ///     .get_matches_from_safe(vec!["myprog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    MissingRequiredArgument,
    /// Occurs when a subcommand is required (as defined by `AppSettings::SubcommandRequired`), but
    /// the user does not provide one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, SubCommand, ErrorKind};
    /// let err = App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches_from_safe(vec![
    ///         "myprog",
    ///     ]);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err().kind, ErrorKind::MissingSubcommand);
    /// # ;
    /// ```
    MissingSubcommand,
    /// Occurs when either an argument or subcommand is required, as defined by
    /// `AppSettings::ArgRequiredElseHelp` but the user did not provide one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, ErrorKind, SubCommand};
    /// let result = App::new("myprog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    ///     .subcommand(SubCommand::with_name("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::with_name("config_file")
    ///             .help("The configuration file to use")))
    ///     .get_matches_from_safe(vec!["myprog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::MissingArgumentOrSubcommand);
    /// ```
    MissingArgumentOrSubcommand,
    /// Occurs when the user provides an argument multiple times which has not been set to allow
    /// multiple uses.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .long("debug")
    ///         .multiple(false))
    ///     .get_matches_from_safe(vec!["myprog", "--debug", "--debug"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::UnexpectedMultipleUsage);
    /// ```
    UnexpectedMultipleUsage,
    /// Occurs when the user provides a value containing invalid UTF-8 for an argument and
    /// `AppSettings::StrictUtf8` is set.
    ///
    /// # Platform Speicific
    ///
    /// Non-Windows platforms only (such as Linux, Unix, OSX, etc.)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg, ErrorKind, AppSettings};
    /// # use std::os::unix::ffi::OsStringExt;
    /// # use std::ffi::OsString;
    /// let result = App::new("myprog")
    ///     .setting(AppSettings::StrictUtf8)
    ///     .arg(Arg::with_name("utf8")
    ///         .short("u")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![OsString::from("myprog"),
    ///                                 OsString::from("-u")
    ///                                 OsString::from_vec(vec![0xE9])]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidUtf8);
    /// ```
    InvalidUtf8,
    /// Not a true "error" as it means `--help` or similar was used. The help message will be sent
    /// to `stdout`.
    ///
    /// **Note**: If the help is displayed due to an error (such as missing subcommands) it will
    /// be sent to `stderr` instead of `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .get_matches_from_safe(vec!["myprog", "--help"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::HelpDisplayed);
    /// ```
    HelpDisplayed,
    /// Not a true "error" as it means `--version` or similar was used. The message will be sent
    /// to `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("myprog")
    ///     .get_matches_from_safe(vec!["myprog", "--version"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind, ErrorKind::VersionDisplayed);
    /// ```
    VersionDisplayed,
    /// Occurs when using the `value_t!` and `values_t!` macros to convert an argument value into
    /// type `T`, but the argument you requested wasn't used. I.e. you asked for an argument with
    /// name `config` to be converted, but `config` wasn't used by the user.
    ArgumentNotFound,
    /// Represents an I/O error, typically while writing to `stderr` or `stdout`.
    Io,
    /// Represents an Rust Display Format error, typically white writing to `stderr` or `stdout`.
    Format,
}

/// Command Line Argument Parser Error
#[derive(Debug)]
pub struct Error {
    /// Formated error message
    pub message: String,
    /// The type of error
    pub kind: ErrorKind,
    /// Any additional information passed along, such as the argument name that caused the error
    pub info: Option<Vec<String>>,
}

impl Error {
    /// Should the message be written to `stdout` or not
    pub fn use_stderr(&self) -> bool {
        match self.kind {
            ErrorKind::HelpDisplayed | ErrorKind::VersionDisplayed => false,
            _ => true,
        }
    }

    /// Prints the error to `stderr` and exits with a status of `1`
    pub fn exit(&self) -> ! {
        if self.use_stderr() {
            wlnerr!("{}", self.message);
            process::exit(1);
        }
        let out = io::stdout();
        writeln!(&mut out.lock(), "{}", self.message).expect("Error writing Error to stdout");
        process::exit(0);
    }

    #[doc(hidden)]
    pub fn argument_conflict<'a, 'b, A, O, U>(arg: &A, other: Option<O>, usage: U) -> Self
        where A: AnyArg<'a, 'b>,
              O: Into<String>,
              U: Display
    {
        let mut v = vec![arg.name().to_owned()];
        Error {
            message: format!("{} The argument '{}' cannot be used with {}\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.to_string()),
                           match other {
                               Some(name) => {
                                   let n = name.into();
                                   v.push(n.clone());
                                   format!("'{}'", Format::Warning(n))
                               },
                               None => "one or more of the other specified arguments".to_owned(),
                           },
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::ArgumentConflict,
            info: Some(v),
        }
    }

    #[doc(hidden)]
    pub fn empty_value<'a, 'b, A, U>(arg: &A, usage: U) -> Self
        where A: AnyArg<'a, 'b>,
              U: Display
    {
        Error {
            message: format!("{} The argument '{}' requires a value but none was supplied\
                            \n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.to_string()),
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::EmptyValue,
            info: Some(vec![arg.name().to_owned()]),
        }
    }

    #[doc(hidden)]
    pub fn invalid_value<'a, 'b, B, G, A, U>(bad_val: B, good_vals: &[G], arg: &A, usage: U) -> Self
        where B: AsRef<str>,
              G: AsRef<str> + Display,
              A: AnyArg<'a, 'b>,
              U: Display
    {
        let suffix = suggestions::did_you_mean_suffix(bad_val.as_ref(),
                                                      good_vals.iter(),
                                                      suggestions::DidYouMeanMessageStyle::EnumValue);

        let mut sorted = vec![];
        for v in good_vals {
            sorted.push(v.as_ref());
        }
        sorted.sort();
        let valid_values = sorted.iter()
                                 .fold(String::new(), |a, name| a + &format!( " {}", name)[..]);
        Error {
            message: format!("{} '{}' isn't a valid value for '{}'\n\t\
                            [values:{}]\n\
                            {}\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(bad_val.as_ref()),
                           Format::Warning(arg.to_string()),
                           valid_values,
                           suffix.0,
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::InvalidValue,
            info: Some(vec![arg.name().to_owned(), bad_val.as_ref().to_owned()]),
        }
    }

    #[doc(hidden)]
    pub fn invalid_subcommand<S, D, N, U>(subcmd: S, did_you_mean: D, name: N, usage: U) -> Self
        where S: Into<String>,
              D: AsRef<str> + Display,
              N: Display,
              U: Display
    {
        let s = subcmd.into();
        Error {
            message: format!("{} The subcommand '{}' wasn't recognized\n\t\
                            Did you mean '{}' ?\n\n\
                            If you believe you received this message in error, try \
                            re-running with '{} {} {}'\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(&*s),
                           Format::Good(did_you_mean.as_ref()),
                           name,
                           Format::Good("--"),
                           &*s,
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::InvalidSubcommand,
            info: Some(vec![s]),
        }
    }

    #[doc(hidden)]
    pub fn missing_required_argument<R, U>(required: R, usage: U) -> Self
        where R: Display,
              U: Display
    {
        Error {
            message: format!("{} The following required arguments were not provided:{}\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           required,
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::MissingRequiredArgument,
            info: None,
        }
    }

    #[doc(hidden)]
    pub fn missing_subcommand<N, U>(name: N, usage: U) -> Self
        where N: AsRef<str> + Display,
              U: Display
    {
        Error {
            message: format!("{} '{}' requires a subcommand, but one was not provided\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(name),
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::MissingSubcommand,
            info: None,
        }
    }


    #[doc(hidden)]
    pub fn invalid_utf8<U>(usage: U) -> Self
        where U: Display
    {
        Error {
            message: format!("{} Invalid UTF-8 was detected in one or more arguments\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::InvalidUtf8,
            info: None,
        }
    }

    #[doc(hidden)]
    pub fn too_many_values<'a, 'b, V, A, U>(val: V, arg: &A, usage: U) -> Self
        where V: AsRef<str> + Display + ToOwned,
              A: AnyArg<'a, 'b>,
              U: Display
    {
        let v = val.as_ref();
        Error {
            message: format!("{} The value '{}' was provided to '{}', but it wasn't expecting \
                            any more values\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(v),
                           Format::Warning(arg.to_string()),
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::TooManyValues,
            info: Some(vec![arg.name().to_owned(), v.to_owned()]),
        }
    }

    #[doc(hidden)]
    pub fn too_few_values<'a, 'b, A, U>(arg: &A, min_vals: u8, curr_vals: usize, usage: U) -> Self
        where A: AnyArg<'a, 'b>,
              U: Display
    {
        Error {
            message: format!("{} The argument '{}' requires at least {} values, but only {} w{} \
                            provided\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.to_string()),
                           Format::Warning(min_vals.to_string()),
                           Format::Warning(curr_vals.to_string()),
                           if curr_vals > 1 {
                               "ere"
                           } else {
                               "as"
                           },
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::TooFewValues,
            info: Some(vec![arg.name().to_owned()]),
        }
    }

    #[doc(hidden)]
    pub fn value_validation(err: String) -> Self {
        Error {
            message: format!("{} {}", Format::Error("error:"), err),
            kind: ErrorKind::ValueValidation,
            info: None,
        }
    }

    #[doc(hidden)]
    pub fn wrong_number_of_values<'a, 'b, A, S, U>(arg: &A, num_vals: u8, curr_vals: usize, suffix: S, usage: U) -> Self
        where A: AnyArg<'a, 'b>,
              S: Display,
              U: Display
    {
        Error {
            message: format!("{} The argument '{}' requires {} values, but {} w{} \
                            provided\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.to_string()),
                           Format::Warning(num_vals.to_string()),
                           Format::Warning(curr_vals.to_string()),
                           suffix,
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::WrongNumberOfValues,
            info: Some(vec![arg.name().to_owned()]),
        }
    }

    #[doc(hidden)]
    pub fn unexpected_multiple_usage<'a, 'b, A, U>(arg: &A, usage: U) -> Self
        where A: AnyArg<'a, 'b>,
              U: Display
    {
        Error {
            message: format!("{} The argument '{}' was provided more than once, but cannot \
                            be used multiple times\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.to_string()),
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::UnexpectedMultipleUsage,
            info: Some(vec![arg.name().to_owned()]),
        }
    }

    #[doc(hidden)]
    pub fn unknown_argument<A, U>(arg: A, did_you_mean: &str, usage: U) -> Self
        where A: Into<String>,
              U: Display
    {
        let a = arg.into();
        Error {
            message: format!("{} Found argument '{}' which wasn't expected, or isn't valid in this context{}\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(&*a),
                           if !did_you_mean.is_empty() {
                               format!("{}\n", did_you_mean)
                           } else {
                               "\n".to_owned()
                           },
                           usage,
                           Format::Good("--help")),
            kind: ErrorKind::UnknownArgument,
            info: Some(vec![a]),
        }
    }

    #[doc(hidden)]
    pub fn argument_not_found<A>(arg: A) -> Self
        where A: Into<String>,
    {
        let a = arg.into();
        Error {
            message: format!("{} The argument '{}' wasn't found", Format::Error("error:"), a.clone()),
            kind: ErrorKind::ArgumentNotFound,
            info: Some(vec![a]),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &*self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std_fmt::Formatter) -> std_fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            message: format!("{} {}", Format::Error("error:"), e.description()),
            kind: ErrorKind::Io,
            info: None,
        }
    }
}

impl From<std_fmt::Error> for Error {
    fn from(e: std_fmt::Error) -> Self {
        Error {
            message: format!("{} {}", Format::Error("error:"), e),
            kind: ErrorKind::Format,
            info: None,
        }
    }
}
