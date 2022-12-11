/// Behavior of arguments when they are encountered while parsing
///
/// # Examples
///
#[cfg_attr(not(feature = "help"), doc = " ```ignore")]
#[cfg_attr(feature = "help", doc = " ```")]
/// # use clap::Command;
/// # use clap::Arg;
/// let cmd = Command::new("mycmd")
///     .arg(
///         Arg::new("special-help")
///             .short('?')
///             .action(clap::ArgAction::Help)
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
    /// **NOTE:** If the argument has previously been seen, it will result in a
    /// [`ArgumentConflict`][crate::error::ErrorKind::ArgumentConflict] unless
    /// [`Command::args_override_self(true)`][crate::Command::args_override_self] is set.
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
    ///             .action(clap::ArgAction::Set)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd", "--flag", "value"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_many::<String>("flag").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
    ///     vec!["value"]
    /// );
    /// ```
    Set,
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
    ///             .action(clap::ArgAction::Append)
    ///     );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd", "--flag", "value1", "--flag", "value2"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_many::<String>("flag").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
    ///     vec!["value1", "value2"]
    /// );
    /// ```
    Append,
    /// When encountered, act as if `"true"` was encountered on the command-line
    ///
    /// If no [`default_value`][super::Arg::default_value] is set, it will be `false`.
    ///
    /// No value is allowed. To optionally accept a value, see
    /// [`Arg::default_missing_value`][super::Arg::default_missing_value]
    ///
    /// **NOTE:** If the argument has previously been seen, it will result in a
    /// [`ArgumentConflict`][crate::error::ErrorKind::ArgumentConflict] unless
    /// [`Command::args_override_self(true)`][crate::Command::args_override_self] is set.
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
    ///             .action(clap::ArgAction::SetTrue)
    ///     );
    ///
    /// let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_one::<bool>("flag").copied(),
    ///     Some(true)
    /// );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_one::<bool>("flag").copied(),
    ///     Some(false)
    /// );
    /// ```
    ///
    /// You can use [`TypedValueParser::map`][crate::builder::TypedValueParser::map] to have the
    /// flag control an application-specific type:
    /// ```rust
    /// # use clap::Command;
    /// # use clap::Arg;
    /// # use clap::builder::TypedValueParser as _;
    /// # use clap::builder::BoolishValueParser;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("flag")
    ///             .long("flag")
    ///             .action(clap::ArgAction::SetTrue)
    ///             .value_parser(
    ///                 BoolishValueParser::new()
    ///                 .map(|b| -> usize {
    ///                     if b { 10 } else { 5 }
    ///                 })
    ///             )
    ///     );
    ///
    /// let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_one::<usize>("flag").copied(),
    ///     Some(10)
    /// );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_one::<usize>("flag").copied(),
    ///     Some(5)
    /// );
    /// ```
    SetTrue,
    /// When encountered, act as if `"false"` was encountered on the command-line
    ///
    /// If no [`default_value`][super::Arg::default_value] is set, it will be `true`.
    ///
    /// No value is allowed. To optionally accept a value, see
    /// [`Arg::default_missing_value`][super::Arg::default_missing_value]
    ///
    /// **NOTE:** If the argument has previously been seen, it will result in a
    /// [`ArgumentConflict`][crate::error::ErrorKind::ArgumentConflict] unless
    /// [`Command::args_override_self(true)`][crate::Command::args_override_self] is set.
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
    ///             .action(clap::ArgAction::SetFalse)
    ///     );
    ///
    /// let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_one::<bool>("flag").copied(),
    ///     Some(false)
    /// );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_one::<bool>("flag").copied(),
    ///     Some(true)
    /// );
    /// ```
    SetFalse,
    /// When encountered, increment a `u8` counter
    ///
    /// If no [`default_value`][super::Arg::default_value] is set, it will be `0`.
    ///
    /// No value is allowed. To optionally accept a value, see
    /// [`Arg::default_missing_value`][super::Arg::default_missing_value]
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
    ///             .action(clap::ArgAction::Count)
    ///     );
    ///
    /// let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag", "--flag"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_count("flag"),
    ///     2
    /// );
    ///
    /// let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
    /// assert!(matches.contains_id("flag"));
    /// assert_eq!(
    ///     matches.get_count("flag"),
    ///     0
    /// );
    /// ```
    Count,
    /// When encountered, display [`Command::print_help`][super::Command::print_help]
    ///
    /// Depending on the flag, [`Command::print_long_help`][super::Command::print_long_help] may be shown
    ///
    /// # Examples
    ///
    #[cfg_attr(not(feature = "help"), doc = " ```ignore")]
    #[cfg_attr(feature = "help", doc = " ```")]
    /// # use clap::Command;
    /// # use clap::Arg;
    /// let cmd = Command::new("mycmd")
    ///     .arg(
    ///         Arg::new("special-help")
    ///             .short('?')
    ///             .action(clap::ArgAction::Help)
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
    /// When encountered, display [`Command::version`][super::Command::version]
    ///
    /// Depending on the flag, [`Command::long_version`][super::Command::long_version] may be shown
    ///
    /// When using `#[derive(clap::Parser)]` you need to add `#[command(version)]` to enable automatic `--version`
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
    ///             .action(clap::ArgAction::Version)
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
    /// Returns whether this action accepts values on the command-line
    ///
    /// [`default_values`][super::Arg::default_values] and [`env`][super::Arg::env] may still be
    /// processed.
    pub fn takes_values(&self) -> bool {
        match self {
            Self::Set => true,
            Self::Append => true,
            Self::SetTrue => false,
            Self::SetFalse => false,
            Self::Count => false,
            Self::Help => false,
            Self::Version => false,
        }
    }

    pub(crate) fn default_value(&self) -> Option<&'static std::ffi::OsStr> {
        match self {
            Self::Set => None,
            Self::Append => None,
            Self::SetTrue => Some(std::ffi::OsStr::new("false")),
            Self::SetFalse => Some(std::ffi::OsStr::new("true")),
            Self::Count => Some(std::ffi::OsStr::new("0")),
            Self::Help => None,
            Self::Version => None,
        }
    }

    pub(crate) fn default_missing_value(&self) -> Option<&'static std::ffi::OsStr> {
        match self {
            Self::Set => None,
            Self::Append => None,
            Self::SetTrue => Some(std::ffi::OsStr::new("true")),
            Self::SetFalse => Some(std::ffi::OsStr::new("false")),
            Self::Count => None,
            Self::Help => None,
            Self::Version => None,
        }
    }

    pub(crate) fn default_value_parser(&self) -> Option<super::ValueParser> {
        match self {
            Self::Set => None,
            Self::Append => None,
            Self::SetTrue => Some(super::ValueParser::bool()),
            Self::SetFalse => Some(super::ValueParser::bool()),
            Self::Count => Some(crate::value_parser!(u8).into()),
            Self::Help => None,
            Self::Version => None,
        }
    }

    #[cfg(debug_assertions)]
    pub(crate) fn value_type_id(&self) -> Option<crate::parser::AnyValueId> {
        use crate::parser::AnyValueId;

        match self {
            Self::Set => None,
            Self::Append => None,
            Self::SetTrue => None,
            Self::SetFalse => None,
            Self::Count => Some(AnyValueId::of::<CountType>()),
            Self::Help => None,
            Self::Version => None,
        }
    }
}

pub(crate) type CountType = u8;
