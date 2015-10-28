use std::process;
use std::error::Error;
use std::fmt;

/// Command line argument parser error types
#[derive(PartialEq, Debug)]
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
    ///     .subcommand(SubCommand::with_name("conifg")
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
    ///     .subcommand(SubCommand::with_name("conifg")
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
    ///     .subcommand(SubCommand::with_name("conifg")
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
    VersionDisplayed
}

/// Command line argument parser error
#[derive(Debug)]
pub struct ClapError {
    /// Formated error message
    pub error: String,
    /// Command line argument parser error type
    pub error_type: ClapErrorType,
}

impl ClapError {
    /// Prints the error to `stderr` and exits with a status of `1`
    pub fn exit(&self) -> ! {
        wlnerr!("{}", self.error);
        process::exit(1);
    }
}

impl Error for ClapError {
    fn description(&self) -> &str {
        &*self.error
    }
}

impl fmt::Display for ClapError {
    fn fmt(&self,
           f: &mut fmt::Formatter)
           -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
