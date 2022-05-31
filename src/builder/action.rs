use crate::parser::AnyValueId;

/// Behavior of arguments when they are encountered while parsing
///
/// # Examples
///
/// ```rust
/// # use clap::Command;
/// # use clap::Arg;
/// let cmd = Command::new("mycmd")
///     .arg(
///         Arg::new("special-help")
///             .short('?')
///             .action(clap::builder::ArgAction::Help)
///     );
///
/// // Existing help still exists
/// let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
/// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
///
/// // New help available
/// let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
/// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
#[allow(missing_copy_implementations)] // In the future, we may accept `Box<dyn ...>`
pub enum ArgAction {
    /// When encountered, store the associated value(s) in [`ArgMatches`][crate::ArgMatches]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("flag")
    ///             .long("flag")
    ///             .action(clap::builder::ArgAction::StoreValue)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd", "--flag", "value"]).unwrap();
    /// assert!(matches.is_present("flag"));
    /// assert_eq!(matches.occurrences_of("flag"), 1);
    /// assert_eq!(
    ///     matches.get_many::<String>("flag").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
    ///     vec!["value"]
    /// );
    /// ```
    StoreValue,
    /// When encountered, increment [`ArgMatches::occurrences_of`][crate::ArgMatches::occurrences_of]
    ///
    /// No value is allowed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("flag")
    ///             .long("flag")
    ///             .multiple_occurrences(true)
    ///             .action(clap::builder::ArgAction::IncOccurrence)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd", "--flag", "--flag"]).unwrap();
    /// assert!(matches.is_present("flag"));
    /// assert_eq!(matches.occurrences_of("flag"), 2);
    /// assert_eq!(matches.get_many::<String>("flag").unwrap_or_default().count(), 0);
    /// ```
    IncOccurrence,
    /// When encountered, act as if `"true"` was encountered on the command-line
    ///
    /// No value is allowed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("flag")
    ///             .long("flag")
    ///             .action(clap::builder::ArgAction::SetTrue)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd", "--flag", "--flag"]).unwrap();
    /// assert!(matches.is_present("flag"));
    /// assert_eq!(matches.occurrences_of("flag"), 0);
    /// assert_eq!(
    ///     matches.get_one::<bool>("flag").copied(),
    ///     Some(true)
    /// );
    /// ```
    SetTrue,
    /// When encountered, act as if `"false"` was encountered on the command-line
    ///
    /// No value is allowed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("flag")
    ///             .long("flag")
    ///             .action(clap::builder::ArgAction::SetFalse)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd", "--flag", "--flag"]).unwrap();
    /// assert!(matches.is_present("flag"));
    /// assert_eq!(matches.occurrences_of("flag"), 0);
    /// assert_eq!(
    ///     matches.get_one::<bool>("flag").copied(),
    ///     Some(false)
    /// );
    /// ```
    SetFalse,
    /// When encountered, increment a counter
    ///
    /// No value is allowed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("flag")
    ///             .long("flag")
    ///             .action(clap::builder::ArgAction::Count)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd", "--flag", "--flag"]).unwrap();
    /// assert!(matches.is_present("flag"));
    /// assert_eq!(matches.occurrences_of("flag"), 0);
    /// assert_eq!(
    ///     matches.get_one::<u64>("flag").copied(),
    ///     Some(2)
    /// );
    /// ```
    Count,
    /// When encountered, display [`Command::print_help`][super::App::print_help]
    ///
    /// Depending on the flag, [`Command::print_long_help`][super::App::print_long_help] may be shown
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("special-help")
    ///             .short('?')
    ///             .action(clap::builder::ArgAction::Help)
    ///     );
    ///
    /// // Existing help still exists
    /// let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
    /// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    ///
    /// // New help available
    /// let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
    /// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    /// ```
    Help,
    /// When encountered, display [`Command::version`][super::App::version]
    ///
    /// Depending on the flag, [`Command::long_version`][super::App::long_version] may be shown
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .version("1.0.0")
    ///     .arg(
    ///         Arg::new("special-version")
    ///             .long("special-version")
    ///             .action(clap::builder::ArgAction::Version)
    ///     );
    ///
    /// // Existing help still exists
    /// let err = cmd.clone().try_get_matches_from(["mycmd", "--version"]).unwrap_err();
    /// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
    ///
    /// // New help available
    /// let err = cmd.try_get_matches_from(["mycmd", "--special-version"]).unwrap_err();
    /// assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
    /// ```
    Version,
}

impl ArgAction {
    pub(crate) fn takes_value(&self) -> bool {
        match self {
            Self::StoreValue => true,
            Self::IncOccurrence => false,
            Self::SetTrue => false,
            Self::SetFalse => false,
            Self::Count => false,
            Self::Help => false,
            Self::Version => false,
        }
    }

    pub(crate) fn default_value_parser(&self) -> Option<super::ValueParser> {
        match self {
            Self::StoreValue => None,
            Self::IncOccurrence => None,
            Self::SetTrue => Some(super::ValueParser::bool()),
            Self::SetFalse => Some(super::ValueParser::bool()),
            Self::Count => Some(crate::value_parser!(u64)),
            Self::Help => None,
            Self::Version => None,
        }
    }

    pub(crate) fn value_type_id(&self) -> Option<AnyValueId> {
        match self {
            Self::StoreValue => None,
            Self::IncOccurrence => None,
            Self::SetTrue => Some(AnyValueId::of::<bool>()),
            Self::SetFalse => Some(AnyValueId::of::<bool>()),
            Self::Count => Some(AnyValueId::of::<CountType>()),
            Self::Help => None,
            Self::Version => None,
        }
    }
}

pub(crate) type CountType = u64;
