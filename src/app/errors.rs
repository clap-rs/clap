use std::process;
use std::error::Error;
use std::fmt as std_fmt;
use std::io::{self, Write};
use std::convert::From;

use fmt::Format;

pub type ClapResult<T> = Result<T, ClapError>;

#[doc(hidden)]
#[allow(non_snake_case)]
pub mod error_builder {
    use super::ClapError;
    use super::ClapErrorType as cet;
    use fmt::Format;

    /// Error occurs when clap find two ore more conflicting arguments
    pub fn ArgumentConflict<S>(arg: S, other: Option<S>, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The argument '{}' cannot be used with {}\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.as_ref()),
                           match other {
                               Some(name) => format!("'{}'", Format::Warning(name)),
                               None => "one or more of the other specified arguments".to_owned(),
                           },
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::ArgumentConflict,
        }
    }

    /// Error occurs when option does not allow empty values but some was found
    pub fn EmptyValue<S>(arg: S, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The argument '{}' requires a value but none was supplied\
                            \n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.as_ref()),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::InvalidValue,
        }
    }

    /// Error occurs when some possible values were set, but clap found unexpected value
    pub fn InvalidValue<S>(bad_val: S,
                           arg: S,
                           valid_vals: S,
                           did_you_mean: S,
                           usage: S)
                           -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} '{}' isn't a valid value for '{}'\n\t\
                            [valid values:{}]\n\
                            {}\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(bad_val.as_ref()),
                           Format::Warning(arg.as_ref()),
                           valid_vals.as_ref(),
                           did_you_mean.as_ref(),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::InvalidValue,
        }
    }

    /// Error occurs when clap found unexpected flag or option
    pub fn InvalidArgument<S>(arg: S, did_you_mean: Option<S>, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The argument '{}' isn't valid{}\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg),
                           if did_you_mean.is_some() {
                               format!("{}\n", did_you_mean.unwrap().as_ref())
                           } else {
                               "".to_owned()
                           },
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::InvalidArgument,
        }
    }

    /// Error occurs when clap found unexpected subcommand
    pub fn InvalidSubcommand<S>(subcmd: S, did_you_mean: S, name: S, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The subcommand '{}' isn't valid\n\t\
                            Did you mean '{}' ?\n\n\
                            If you received this message in error, try \
                            re-running with '{} {} {}'\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(subcmd.as_ref()),
                           Format::Good(did_you_mean.as_ref()),
                           name.as_ref(),
                           Format::Good("--"),
                           subcmd.as_ref(),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::InvalidSubcommand,
        }
    }

    /// Error occurs when one or more required arguments missing
    pub fn MissingRequiredArgument<S>(required: S, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The following required arguments were not supplied:{}\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           required.as_ref(),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::MissingRequiredArgument,
        }
    }

    /// Error occurs when required subcommand missing
    pub fn MissingSubcommand<S>(name: S, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} '{}' requires a subcommand but none was provided\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(name),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::MissingSubcommand,
        }
    }


    /// Error occurs when argument contains invalid unicode characters
    pub fn InvalidUnicode<S>(usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} Invalid unicode character in one or more arguments\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::InvalidUnicode,
        }
    }

    /// Error occurs when argument got more values then were expected
    pub fn TooManyValues<S>(val: S, arg: S, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The argument '{}' was found, but '{}' wasn't expecting \
                            any more values\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(val),
                           Format::Warning(arg),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::TooManyValues,
        }
    }

    /// Error occurs when argument got less values then were expected
    pub fn TooFewValues<S>(arg: S, min_vals: u8, curr_vals: usize, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The argument '{}' requires at least {} values, but {} w{} \
                            provided\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.as_ref()),
                           Format::Warning(min_vals.to_string()),
                           Format::Warning(curr_vals.to_string()),
                           if curr_vals > 1 {
                               "ere"
                           } else {
                               "as"
                           },
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::TooFewValues,
        }
    }

    /// Option fails validation of a custom validator
    pub fn ValueValidationError<S>(err: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} {}", Format::Error("error:"), err.as_ref()),
            error_type: cet::ValueValidationError,
        }
    }

    /// Error occurs when argument got a different number of values then were expected
    pub fn WrongNumValues<S>(arg: S,
                             num_vals: u8,
                             curr_vals: usize,
                             suffix: S,
                             usage: S)
                             -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The argument '{}' requires {} values, but {} w{} \
                            provided\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg.as_ref()),
                           Format::Warning(num_vals.to_string()),
                           Format::Warning(curr_vals.to_string()),
                           suffix.as_ref(),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::InvalidSubcommand,
        }
    }

    /// Error occurs when clap find argument while is was not expecting any
    pub fn UnexpectedArgument<S>(arg: S, name: S, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} Found argument '{}', but {} wasn't expecting any\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg),
                           Format::Warning(name),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::UnexpectedArgument,
        }
    }

    /// Error occurs when argument was used multiple times and was not set as multiple.
    pub fn UnexpectedMultipleUsage<S>(arg: S, usage: S) -> ClapError
        where S: AsRef<str>
    {
        ClapError {
            error: format!("{} The argument '{}' was supplied more than once, but does \
                            not support multiple values\n\n\
                            {}\n\n\
                            For more information try {}",
                           Format::Error("error:"),
                           Format::Warning(arg),
                           usage.as_ref(),
                           Format::Good("--help")),
            error_type: cet::UnexpectedMultipleUsage,
        }
    }

}

/// Command line argument parser error types
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClapErrorType {
    /// Error occurs when some possible values were set, but clap found unexpected value
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug").index(1)
    ///         .possible_value("fast")
    ///         .possible_value("slow"))
    ///     .get_matches_from_safe(vec!["", "other"]);
    /// ```
    InvalidValue,
    /// Error occurs when clap found unexpected flag or option
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::from_usage("-f, --flag 'some flag'"))
    ///     .get_matches_from_safe(vec!["", "--other"]);
    /// ```
    InvalidArgument,
    /// Error occurs when clap found unexpected subcommand
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// let result = App::new("myprog")
    ///     .subcommand(SubCommand::with_name("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::with_name("config_file")
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .get_matches_from_safe(vec!["", "other"]);
    /// ```
    InvalidSubcommand,
    /// Error occurs when option does not allow empty values but some was found
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///          .empty_values(false))
    ///     .arg(Arg::with_name("color"))
    ///     .get_matches_from_safe(vec!["", "--debug", "--color"]);
    /// ```
    EmptyValue,
    /// Option fails validation of a custom validator
    ValueValidationError,
    /// Parser inner error
    ArgumentError,
    /// Error occurs when an application got more arguments then were expected
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug").index(1)
    ///         .max_values(2))
    ///     .get_matches_from_safe(vec!["", "too", "much", "values"]);
    /// ```
    TooManyArgs,
    /// Error occurs when argument got more values then were expected
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug").index(1)
    ///         .max_values(2))
    ///     .get_matches_from_safe(vec!["", "too", "much", "values"]);
    /// ```
    TooManyValues,
    /// Error occurs when argument got less values then were expected
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug").index(1)
    ///         .min_values(3))
    ///     .get_matches_from_safe(vec!["", "too", "few"]);
    /// ```
    TooFewValues,
    /// Error occurs when argument got a different number of values then were expected
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug").index(1)
    ///         .max_values(2))
    ///     .get_matches_from_safe(vec!["", "too", "much", "values"]);
    /// ```
    WrongNumValues,
    /// Error occurs when clap find two ore more conflicting arguments
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .conflicts_with("color"))
    ///     .get_matches_from_safe(vec!["", "--debug", "--color"]);
    /// ```
    ArgumentConflict,
    /// Error occurs when one or more required arguments missing
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .required(true))
    ///     .get_matches_from_safe(vec![""]);
    /// ```
    MissingRequiredArgument,
    /// Error occurs when required subcommand missing
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings, SubCommand};
    /// let result = App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .subcommand(SubCommand::with_name("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::with_name("config_file")
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .get_matches_from_safe(vec![""]);
    /// ```
    MissingSubcommand,
    /// Occurs when no argument or subcommand has been supplied and
    /// `AppSettings::ArgRequiredElseHelp` was used
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings, SubCommand};
    /// let result = App::new("myprog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    ///     .subcommand(SubCommand::with_name("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::with_name("config_file")
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .get_matches_from_safe(vec![""]);
    /// ```
    MissingArgumentOrSubcommand,
    /// Error occurs when clap find argument while is was not expecting any
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App};
    /// let result = App::new("myprog")
    ///     .get_matches_from_safe(vec!["", "--arg"]);
    /// ```
    UnexpectedArgument,
    /// Error occurs when argument was used multiple times and was not set as multiple.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .multiple(false))
    ///     .get_matches_from_safe(vec!["", "--debug", "--debug"]);
    /// ```
    UnexpectedMultipleUsage,
    /// Error occurs when argument contains invalid unicode characters
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # use std::os::unix::ffi::OsStringExt;
    /// # use std::ffi::OsString;
    /// let result = App::new("myprog")
    ///     .arg(Arg::with_name("debug")
    ///         .short("u")
    ///         .takes_value(true))
    ///     .get_matches_from_safe(vec![OsString::from_vec(vec![0x20]),
    ///                                 OsString::from_vec(vec![0xE9])]);
    /// assert!(result.is_err());
    /// ```
    InvalidUnicode,
    /// Not a true 'error' as it means `--help` or similar was used. The help message will be sent
    /// to `stdout` unless the help is displayed due to an error (such as missing subcommands) at
    /// which point it will be sent to `stderr`
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # use clap::ClapErrorType;
    /// let result = App::new("myprog")
    ///     .get_matches_from_safe(vec!["", "--help"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().error_type, ClapErrorType::HelpDisplayed);
    /// ```
    HelpDisplayed,
    /// Not a true 'error' as it means `--version` or similar was used. The message will be sent
    /// to `stdout`
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # use clap::ClapErrorType;
    /// let result = App::new("myprog")
    ///     .get_matches_from_safe(vec!["", "--version"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().error_type, ClapErrorType::VersionDisplayed);
    /// ```
    VersionDisplayed,
    /// Represents an internal error, please consider filing a bug report if this happens!
    InternalError,
    /// Represents an I/O error, typically white writing to stderr or stdout
    Io,
    /// Represents an Rust Display Format error, typically white writing to stderr or stdout
    Format,
}

/// Command line argument parser error
#[derive(Debug)]
pub struct ClapError {
    /// Formated error message
    pub error: String,
    /// The type of error
    pub error_type: ClapErrorType,
}

impl ClapError {
    /// Should the message be written to `stdout` or not
    pub fn use_stderr(&self) -> bool {
        match self.error_type {
            ClapErrorType::HelpDisplayed | ClapErrorType::VersionDisplayed => false,
            _ => true,
        }
    }
    /// Prints the error to `stderr` and exits with a status of `1`
    pub fn exit(&self) -> ! {
        if self.use_stderr() {
            werr!("{}", self.error);
            process::exit(1);
        }
        let out = io::stdout();
        writeln!(&mut out.lock(), "{}", self.error).expect("Error writing ClapError to stdout");
        process::exit(0);
    }
}

impl Error for ClapError {
    fn description(&self) -> &str {
        &*self.error
    }

    fn cause(&self) -> Option<&Error> {
        match self.error_type {
            _ => None,
        }
    }
}

impl std_fmt::Display for ClapError {
    fn fmt(&self, f: &mut std_fmt::Formatter) -> std_fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl From<io::Error> for ClapError {
    fn from(e: io::Error) -> Self {
        ClapError {
            error: format!("{} {}", Format::Error("error:"), e.description()),
            error_type: ClapErrorType::Io,
        }
    }
}

impl From<std_fmt::Error> for ClapError {
    fn from(e: std_fmt::Error) -> Self {
        ClapError {
            error: format!("{} {}", Format::Error("error:"), e),
            error_type: ClapErrorType::Format,
        }
    }
}
