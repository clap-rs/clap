use std::error::Error;
use std::fmt;

/// Command line argument parser error types
#[derive(PartialEq, Debug)]
pub enum ClapErrorType {
    /// Error occurs when some possible values were set, but clap found unexpected value
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .possible_value("fast")
    /// .possible_value("slow")
    /// # ).get_matches_from_safe(vec!["", "other"]);
    /// ```
    InvalidValue,
    /// Error occurs when clap found unexpected flag or option
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::from_usage("-f, --flag 'some flag'")
    /// # ).get_matches_from_safe(vec!["", "--other"]);
    /// ```
    InvalidArgument,
    /// Error occurs when clap found unexpected subcommand
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let result = App::new("myprog")
    /// #                    .subcommand(
    /// SubCommand::with_name("conifg")
    ///                .about("Used for configuration")
    ///                .arg(Arg::with_name("config_file")
    ///                           .help("The configuration file to use")
    ///                           .index(1))
    /// # ).get_matches_from_safe(vec!["", "other"]);
    /// ```
    InvalidSubcommand,
    /// Error occurs when option does not allow empty values but some was found
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .empty_values(false))
    /// #                 .arg(
    /// # Arg::with_name("color")
    /// # ).get_matches_from_safe(vec!["", "--debug", "--color"]);
    /// ```
    EmptyValue,
    /// Parser inner error
    OptionError,
    /// Parser inner error
    ArgumentError,
    /// Parser inner error
    ValueError,
    /// Error occurs when argument got more values then were expected
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .max_values(2)
    /// # ).get_matches_from_safe(vec!["", "too", "much", "values"]);
    /// ```
    TooMuchValues,
    /// Error occurs when argument got less values then were expected
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug").index(1)
    /// .min_values(3)
    /// # ).get_matches_from_safe(vec!["", "too", "few"]);
    /// ```
    TooFewValues,
    /// Error occurs when clap find two ore more conflicting arguments
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .conflicts_with("color")
    /// # ).get_matches_from_safe(vec!["", "--debug", "--color"]);
    /// ```
    ArgumentConflict,
    /// Error occurs when one or more required arguments missing
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .required(true)
    /// # ).get_matches_from_safe(vec![""]);
    /// ```
    MissingRequiredArgument,
    /// Error occurs when required subcommand missing
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings, SubCommand};
    /// # let result = App::new("myprog")
    /// #                    .setting(AppSettings::SubcommandRequired)
    /// #                    .subcommand(
    /// SubCommand::with_name("conifg")
    ///                .about("Used for configuration")
    ///                .arg(Arg::with_name("config_file")
    ///                           .help("The configuration file to use")
    ///                           .index(1))
    /// # ).get_matches_from_safe(vec![""]);
    /// ```
    MissingSubcommand,
    /// Error occurs when clap find argument while is was not expecting any
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App};
    /// # let result = App::new("myprog").get_matches_from_safe(vec!["", "--arg"]);
    /// ```
    UnexpectedArgument,
    /// Error occurs when argument was used multiple times and was not set as multiple.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let result = App::new("myprog")
    /// #                 .arg(
    /// # Arg::with_name("debug")
    /// .multiple(false)
    /// # ).get_matches_from_safe(vec!["", "--debug", "--debug"]);
    /// ```
    UnexpectedMultipleUsage
}

/// Command line argument parser error
#[derive(Debug)]
pub struct ClapError {
    /// Formated error message
    pub error: String,
    /// Command line argument parser error type
    pub error_type: ClapErrorType
}

impl Error for ClapError {
    fn description(&self) -> &str {
        &*self.error
    }
}

impl fmt::Display for ClapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}