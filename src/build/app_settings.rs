#![allow(deprecated)]

// Std
use std::ops::BitOr;
#[cfg(feature = "yaml")]
use std::str::FromStr;

#[allow(unused)]
use crate::App;
#[allow(unused)]
use crate::Arg;

// Third party
use bitflags::bitflags;

#[doc(hidden)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppFlags(Flags);

impl Default for AppFlags {
    fn default() -> Self {
        AppFlags(Flags::COLOR_AUTO)
    }
}

/// Application level settings, which affect how [`App`] operates
///
/// **NOTE:** When these settings are used, they apply only to current command, and are *not*
/// propagated down or up through child or parent subcommands
///
/// [`App`]: crate::App
#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum AppSettings {
    /// Deprecated, replaced with [`App::ignore_errors`]
    #[deprecated(since = "3.1.0", note = "Replaced with `App::ignore_errors`")]
    IgnoreErrors,

    /// Deprecated, replace
    /// ```rust,no_run
    /// let app = clap::App::new("app")
    ///     .global_setting(clap::AppSettings::WaitOnError)
    ///     .arg(clap::arg!(--flag));
    /// let m = app.get_matches();
    /// ```
    /// with
    /// ```rust
    /// let app = clap::App::new("app")
    ///     .arg(clap::arg!(--flag));
    /// let m = match app.try_get_matches() {
    ///     Ok(m) => m,
    ///     Err(err) => {
    ///         if err.use_stderr() {
    ///             let _ = err.print();
    ///
    ///             eprintln!("\nPress [ENTER] / [RETURN] to continue...");
    ///             use std::io::BufRead;
    ///             let mut s = String::new();
    ///             let i = std::io::stdin();
    ///             i.lock().read_line(&mut s).unwrap();
    ///
    ///             std::process::exit(2);
    ///         } else {
    ///             let _ = err.print();
    ///             std::process::exit(0);
    ///         }
    ///     }
    /// };
    /// ```
    #[deprecated(
        since = "3.1.0",
        note = "See documentation for how to hand-implement this"
    )]
    WaitOnError,

    /// Deprecated, replaced with [`App::allow_hyphen_values`] and
    /// [`Arg::is_allow_hyphen_values_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::allow_hyphen_values` and `Arg::is_allow_hyphen_values_set`"
    )]
    AllowHyphenValues,

    /// Deprecated, replaced with [`App::allow_negative_numbers`] and
    /// [`App::is_allow_negative_numbers_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::allow_negative_numbers` and `App::is_allow_negative_numbers_set`"
    )]
    AllowNegativeNumbers,

    /// Deprecated, replaced with [`App::args_override_self`]
    #[deprecated(since = "3.1.0", note = "Replaced with `App::args_override_self`")]
    AllArgsOverrideSelf,

    /// Deprecated, replaced with [`App::allow_missing_positional`] and
    /// [`App::is_allow_missing_positional_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::allow_missing_positional` and `App::is_allow_missing_positional_set`"
    )]
    AllowMissingPositional,

    /// Deprecated, replaced with [`App::trailing_var_arg`] and [`App::is_trailing_var_arg_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::trailing_var_arg` and `App::is_trailing_var_arg_set`"
    )]
    TrailingVarArg,

    /// Deprecated, replaced with [`App::dont_delimit_trailing_values`] and
    /// [`App::is_dont_delimit_trailing_values_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::dont_delimit_trailing_values` and `App::is_dont_delimit_trailing_values_set`"
    )]
    DontDelimitTrailingValues,

    /// Deprecated, replaced with [`App::infer_long_args`]
    #[deprecated(since = "3.1.0", note = "Replaced with `App::infer_long_args`")]
    InferLongArgs,

    /// Deprecated, replaced with [`App::infer_subcommands`]
    #[deprecated(since = "3.1.0", note = "Replaced with `App::infer_subcommands`")]
    InferSubcommands,

    /// Deprecated, replaced with [`App::subcommand_required`] and
    /// [`App::is_subcommand_required_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::subcommand_required` and `App::is_subcommand_required_set`"
    )]
    SubcommandRequired,

    /// Deprecated, replaced with [`App::subcommand_required`] combined with
    /// [`App::arg_required_else_help`].
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::subcommand_required` combined with `App::arg_required_else_help`"
    )]
    SubcommandRequiredElseHelp,

    /// Deprecated, replaced with [`App::allow_external_subcommands`] and
    /// [`App::is_allow_external_subcommands_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::allow_external_subcommands` and `App::is_allow_external_subcommands_set`"
    )]
    AllowExternalSubcommands,

    /// Deprecated, replaced with [`App::multicall`] and [`App::is_multicall_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::multicall` and `App::is_multicall_set`"
    )]
    #[cfg(feature = "unstable-multicall")]
    Multicall,

    /// Deprecated, replaced with [`App::allow_invalid_utf8_for_external_subcommands`] and [`App::is_allow_invalid_utf8_for_external_subcommands_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::allow_invalid_utf8_for_external_subcommands` and `App::is_allow_invalid_utf8_for_external_subcommands_set`"
    )]
    AllowInvalidUtf8ForExternalSubcommands,

    /// Deprecated, this is now the default
    #[deprecated(since = "3.1.0", note = "This is now the default")]
    UseLongFormatForHelpSubcommand,

    /// Deprecated, replaced with [`App::subcommand_negates_reqs`] and
    /// [`App::is_subcommand_negates_reqs_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::subcommand_negates_reqs` and `App::is_subcommand_negates_reqs_set`"
    )]
    SubcommandsNegateReqs,

    /// Deprecated, replaced with [`App::args_conflicts_with_subcommands`] and
    /// [`App::is_args_conflicts_with_subcommands_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::args_conflicts_with_subcommands` and `App::is_args_conflicts_with_subcommands_set`"
    )]
    ArgsNegateSubcommands,

    /// Deprecated, replaced with [`App::subcommand_precedence_over_arg`] and
    /// [`App::is_subcommand_precedence_over_arg_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::subcommand_precedence_over_arg` and `App::is_subcommand_precedence_over_arg_set`"
    )]
    SubcommandPrecedenceOverArg,

    /// Deprecated, replaced with [`App::arg_required_else_help`] and
    /// [`App::is_arg_required_else_help_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::arg_required_else_help` and `App::is_arg_required_else_help_set`"
    )]
    ArgRequiredElseHelp,

    /// Displays the arguments and [`subcommands`] in the help message in the order that they were
    /// declared in, and not alphabetically which is the default.
    ///
    /// To override the declaration order, see [`Arg::display_order`] and [`App::display_order`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .global_setting(AppSettings::DeriveDisplayOrder)
    ///     .get_matches();
    /// ```
    ///
    /// [`subcommands`]: crate::App::subcommand()
    /// [`Arg::display_order`]: crate::Arg::display_order
    /// [`App::display_order`]: crate::App::display_order
    DeriveDisplayOrder,

    /// Deprecated, replaced with [`App::dont_collapse_args_in_usage`] and
    /// [`App::is_dont_collapse_args_in_usage_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::dont_collapse_args_in_usage` and `App::is_dont_collapse_args_in_usage_set`"
    )]
    DontCollapseArgsInUsage,

    /// Deprecated, replaced with [`App::next_line_help`] and [`App::is_next_line_help_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::next_line_help` and `App::is_next_line_help_set`"
    )]
    NextLineHelp,

    /// Deprecated, replaced with [`App::disable_colored_help`] and
    /// [`App::is_disable_colored_help_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::disable_colored_help` and `App::is_disable_colored_help_set`"
    )]
    DisableColoredHelp,

    /// Deprecated, replaced with [`App::disable_help_flag`] and [`App::is_disable_help_flag_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::disable_help_flag` and `App::is_disable_help_flag_set`"
    )]
    DisableHelpFlag,

    /// Deprecated, replaced with [`App::disable_help_subcommand`] and
    /// [`App::is_disable_help_subcommand_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::disable_help_subcommand` and `App::is_disable_help_subcommand_set`"
    )]
    DisableHelpSubcommand,

    /// Deprecated, replaced with [`App::disable_version_flag`] and
    /// [`App::is_disable_version_flag_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::disable_version_flag` and `App::is_disable_version_flag_set`"
    )]
    DisableVersionFlag,

    /// Deprecated, replaced with [`App::propagate_version`] and [`App::is_propagate_version_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::propagate_version` and `App::is_propagate_version_set`"
    )]
    PropagateVersion,

    /// Deprecated, replaced with [`App::hide`] and [`App::is_hide_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::hide` and `App::is_hide_set`"
    )]
    Hidden,

    /// Deprecated, replaced with [`App::hide_possible_values`] and
    /// [`Arg::is_hide_possible_values_set`]
    #[deprecated(
        since = "3.1.0",
        note = "Replaced with `App::hide_possible_values` and `Arg::is_hide_possible_values_set`"
    )]
    HidePossibleValues,

    /// Deprecated, replaced with [`App::help_expected`]
    #[deprecated(since = "3.1.0", note = "Replaced with `App::help_expected`")]
    HelpExpected,

    /// Deprecated, replaced with [`App::no_binary_name`]
    #[deprecated(since = "3.1.0", note = "Replaced with `App::no_binary_name`")]
    NoBinaryName,

    /// Treat the auto-generated `-h, --help` flags like any other flag, and *not* print the help
    /// message.
    ///
    /// This allows one to handle printing of the help message manually.
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// let result = App::new("myprog")
    ///     .setting(AppSettings::NoAutoHelp)
    ///     .try_get_matches_from("myprog --help".split(" "));
    ///
    /// // Normally, if `--help` is used clap prints the help message and returns an
    /// // ErrorKind::DisplayHelp
    /// //
    /// // However, `--help` was treated like a normal flag
    ///
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().is_present("help"));
    /// ```
    NoAutoHelp,

    /// Treat the auto-generated `-V, --version` flags like any other flag, and
    /// *not* print the version message.
    ///
    /// This allows one to handle printing of the version message manually.
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// let result = App::new("myprog")
    ///     .version("3.0")
    ///     .setting(AppSettings::NoAutoVersion)
    ///     .try_get_matches_from("myprog --version".split(" "));
    ///
    /// // Normally, if `--version` is used clap prints the version message and returns an
    /// // ErrorKind::DisplayVersion
    /// //
    /// // However, `--version` was treated like a normal flag
    ///
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().is_present("version"));
    /// ```
    NoAutoVersion,

    /// Deprecated, replaced with [`AppSettings::AllowHyphenValues`]
    #[deprecated(
        since = "3.0.0",
        note = "Replaced with `AppSettings::AllowHyphenValues`"
    )]
    #[doc(hidden)]
    AllowLeadingHyphen,

    /// Deprecated, this is now the default, see [`AppSettings::AllowInvalidUtf8ForExternalSubcommands`] and [`ArgSettings::AllowInvalidUtf8`][crate::ArgSettings::AllowInvalidUtf8] for the opposite.
    #[deprecated(
        since = "3.0.0",
        note = "This is now the default see `AppSettings::AllowInvalidUtf8ForExternalSubcommands` and `ArgSettings::AllowInvalidUtf8` for the opposite."
    )]
    #[doc(hidden)]
    StrictUtf8,

    /// Deprecated, this is now the default
    #[deprecated(since = "3.0.0", note = "This is now the default")]
    #[doc(hidden)]
    UnifiedHelpMessage,

    /// Deprecated, this is now the default
    #[deprecated(since = "3.0.0", note = "This is now the default")]
    #[doc(hidden)]
    ColoredHelp,

    /// Deprecated, see [`App::color`][crate::App::color]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::color`")]
    #[doc(hidden)]
    ColorAuto,

    /// Deprecated, replaced with [`App::color`][crate::App::color]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::color`")]
    #[doc(hidden)]
    ColorAlways,

    /// Deprecated, replaced with [`App::color`][crate::App::color]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::color`")]
    #[doc(hidden)]
    ColorNever,

    /// Deprecated, replaced with [`AppSettings::DisableHelpFlag`]
    #[deprecated(since = "3.0.0", note = "Replaced with `AppSettings::DisableHelpFlag`")]
    #[doc(hidden)]
    DisableHelpFlags,

    /// Deprecated, replaced with [`AppSettings::DisableVersionFlag`]
    #[deprecated(
        since = "3.0.0",
        note = "Replaced with `AppSettings::DisableVersionFlag`"
    )]
    #[doc(hidden)]
    DisableVersion,

    /// Deprecated, replaced with [`AppSettings::PropagateVersion`]
    #[deprecated(
        since = "3.0.0",
        note = "Replaced with `AppSettings::PropagateVersion`"
    )]
    #[doc(hidden)]
    GlobalVersion,

    /// Deprecated, replaced with [`AppSettings::HidePossibleValues`]
    #[deprecated(
        since = "3.0.0",
        note = "Replaced with AppSettings::HidePossibleValues"
    )]
    #[doc(hidden)]
    HidePossibleValuesInHelp,

    /// Deprecated, this is now the default
    #[deprecated(since = "3.0.0", note = "This is now the default")]
    #[doc(hidden)]
    UnifiedHelp,

    /// If the app is already built, used for caching.
    #[doc(hidden)]
    Built,

    /// If the app's bin name is already built, used for caching.
    #[doc(hidden)]
    BinNameBuilt,
}

bitflags! {
    struct Flags: u64 {
        const SC_NEGATE_REQS                 = 1;
        const SC_REQUIRED                    = 1 << 1;
        const ARG_REQUIRED_ELSE_HELP         = 1 << 2;
        const PROPAGATE_VERSION              = 1 << 3;
        const DISABLE_VERSION_FOR_SC         = 1 << 4;
        const WAIT_ON_ERROR                  = 1 << 6;
        const SC_REQUIRED_ELSE_HELP          = 1 << 7;
        const NO_AUTO_HELP                   = 1 << 8;
        const NO_AUTO_VERSION                = 1 << 9;
        const DISABLE_VERSION_FLAG           = 1 << 10;
        const HIDDEN                         = 1 << 11;
        const TRAILING_VARARG                = 1 << 12;
        const NO_BIN_NAME                    = 1 << 13;
        const ALLOW_UNK_SC                   = 1 << 14;
        const SC_UTF8_NONE                   = 1 << 15;
        const LEADING_HYPHEN                 = 1 << 16;
        const NO_POS_VALUES                  = 1 << 17;
        const NEXT_LINE_HELP                 = 1 << 18;
        const DERIVE_DISP_ORDER              = 1 << 19;
        const DISABLE_COLORED_HELP           = 1 << 20;
        const COLOR_ALWAYS                   = 1 << 21;
        const COLOR_AUTO                     = 1 << 22;
        const COLOR_NEVER                    = 1 << 23;
        const DONT_DELIM_TRAIL               = 1 << 24;
        const ALLOW_NEG_NUMS                 = 1 << 25;
        const DISABLE_HELP_SC                = 1 << 27;
        const DONT_COLLAPSE_ARGS             = 1 << 28;
        const ARGS_NEGATE_SCS                = 1 << 29;
        const PROPAGATE_VALS_DOWN            = 1 << 30;
        const ALLOW_MISSING_POS              = 1 << 31;
        const TRAILING_VALUES                = 1 << 32;
        const BUILT                          = 1 << 33;
        const BIN_NAME_BUILT                 = 1 << 34;
        const VALID_ARG_FOUND                = 1 << 35;
        const INFER_SUBCOMMANDS              = 1 << 36;
        const CONTAINS_LAST                  = 1 << 37;
        const ARGS_OVERRIDE_SELF             = 1 << 38;
        const HELP_REQUIRED                  = 1 << 39;
        const SUBCOMMAND_PRECEDENCE_OVER_ARG = 1 << 40;
        const DISABLE_HELP_FLAG              = 1 << 41;
        const USE_LONG_FORMAT_FOR_HELP_SC    = 1 << 42;
        const INFER_LONG_ARGS                = 1 << 43;
        const IGNORE_ERRORS                  = 1 << 44;
        #[cfg(feature = "unstable-multicall")]
        const MULTICALL                      = 1 << 45;
        const NO_OP                          = 0;
    }
}

impl_settings! { AppSettings, AppFlags,
    ArgRequiredElseHelp
        => Flags::ARG_REQUIRED_ELSE_HELP,
    SubcommandPrecedenceOverArg
        => Flags::SUBCOMMAND_PRECEDENCE_OVER_ARG,
    ArgsNegateSubcommands
        => Flags::ARGS_NEGATE_SCS,
    AllowExternalSubcommands
        => Flags::ALLOW_UNK_SC,
    StrictUtf8
        => Flags::NO_OP,
    AllowInvalidUtf8ForExternalSubcommands
        => Flags::SC_UTF8_NONE,
    AllowHyphenValues
        => Flags::LEADING_HYPHEN,
    AllowLeadingHyphen
        => Flags::LEADING_HYPHEN,
    AllowNegativeNumbers
        => Flags::ALLOW_NEG_NUMS,
    AllowMissingPositional
        => Flags::ALLOW_MISSING_POS,
    UnifiedHelpMessage
        => Flags::NO_OP,
    ColoredHelp
        => Flags::NO_OP,
    ColorAlways
        => Flags::COLOR_ALWAYS,
    ColorAuto
        => Flags::COLOR_AUTO,
    ColorNever
        => Flags::COLOR_NEVER,
    DontDelimitTrailingValues
        => Flags::DONT_DELIM_TRAIL,
    DontCollapseArgsInUsage
        => Flags::DONT_COLLAPSE_ARGS,
    DeriveDisplayOrder
        => Flags::DERIVE_DISP_ORDER,
    DisableColoredHelp
        => Flags::DISABLE_COLORED_HELP,
    DisableHelpSubcommand
        => Flags::DISABLE_HELP_SC,
    DisableHelpFlag
        => Flags::DISABLE_HELP_FLAG,
    DisableHelpFlags
        => Flags::DISABLE_HELP_FLAG,
    DisableVersionFlag
        => Flags::DISABLE_VERSION_FLAG,
    DisableVersion
        => Flags::DISABLE_VERSION_FLAG,
    PropagateVersion
        => Flags::PROPAGATE_VERSION,
    GlobalVersion
        => Flags::PROPAGATE_VERSION,
    HidePossibleValues
        => Flags::NO_POS_VALUES,
    HidePossibleValuesInHelp
        => Flags::NO_POS_VALUES,
    HelpExpected
        => Flags::HELP_REQUIRED,
    Hidden
        => Flags::HIDDEN,
    #[cfg(feature = "unstable-multicall")]
    Multicall
        => Flags::MULTICALL,
    NoAutoHelp
        => Flags::NO_AUTO_HELP,
    NoAutoVersion
        => Flags::NO_AUTO_VERSION,
    NoBinaryName
        => Flags::NO_BIN_NAME,
    SubcommandsNegateReqs
        => Flags::SC_NEGATE_REQS,
    SubcommandRequired
        => Flags::SC_REQUIRED,
    SubcommandRequiredElseHelp
        => Flags::SC_REQUIRED_ELSE_HELP,
    UseLongFormatForHelpSubcommand
        => Flags::USE_LONG_FORMAT_FOR_HELP_SC,
    TrailingVarArg
        => Flags::TRAILING_VARARG,
    UnifiedHelp => Flags::NO_OP,
    NextLineHelp
        => Flags::NEXT_LINE_HELP,
    IgnoreErrors
        => Flags::IGNORE_ERRORS,
    WaitOnError
        => Flags::WAIT_ON_ERROR,
    Built
        => Flags::BUILT,
    BinNameBuilt
        => Flags::BIN_NAME_BUILT,
    InferSubcommands
        => Flags::INFER_SUBCOMMANDS,
    AllArgsOverrideSelf
        => Flags::ARGS_OVERRIDE_SELF,
    InferLongArgs
        => Flags::INFER_LONG_ARGS
}

/// Deprecated in [Issue #3087](https://github.com/clap-rs/clap/issues/3087), maybe [`clap::Parser`][crate::Parser] would fit your use case?
#[cfg(feature = "yaml")]
impl FromStr for AppSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        #[allow(deprecated)]
        #[allow(unreachable_patterns)]
        match &*s.to_ascii_lowercase() {
            "argrequiredelsehelp" => Ok(AppSettings::ArgRequiredElseHelp),
            "subcommandprecedenceoverarg" => Ok(AppSettings::SubcommandPrecedenceOverArg),
            "argsnegatesubcommands" => Ok(AppSettings::ArgsNegateSubcommands),
            "allowexternalsubcommands" => Ok(AppSettings::AllowExternalSubcommands),
            "strictutf8" => Ok(AppSettings::StrictUtf8),
            "allowinvalidutf8forexternalsubcommands" => {
                Ok(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
            }
            "allowhyphenvalues" => Ok(AppSettings::AllowHyphenValues),
            "allowleadinghyphen" => Ok(AppSettings::AllowLeadingHyphen),
            "allownegativenumbers" => Ok(AppSettings::AllowNegativeNumbers),
            "allowmissingpositional" => Ok(AppSettings::AllowMissingPositional),
            "unifiedhelpmessage" => Ok(AppSettings::UnifiedHelpMessage),
            "coloredhelp" => Ok(AppSettings::ColoredHelp),
            "coloralways" => Ok(AppSettings::ColorAlways),
            "colorauto" => Ok(AppSettings::ColorAuto),
            "colornever" => Ok(AppSettings::ColorNever),
            "dontdelimittrailingvalues" => Ok(AppSettings::DontDelimitTrailingValues),
            "dontcollapseargsinusage" => Ok(AppSettings::DontCollapseArgsInUsage),
            "derivedisplayorder" => Ok(AppSettings::DeriveDisplayOrder),
            "disablecoloredhelp" => Ok(AppSettings::DisableColoredHelp),
            "disablehelpsubcommand" => Ok(AppSettings::DisableHelpSubcommand),
            "disablehelpflag" => Ok(AppSettings::DisableHelpFlag),
            "disablehelpflags" => Ok(AppSettings::DisableHelpFlags),
            "disableversionflag" => Ok(AppSettings::DisableVersionFlag),
            "disableversion" => Ok(AppSettings::DisableVersion),
            "propagateversion" => Ok(AppSettings::PropagateVersion),
            "propagateversion" => Ok(AppSettings::GlobalVersion),
            "hidepossiblevalues" => Ok(AppSettings::HidePossibleValues),
            "hidepossiblevaluesinhelp" => Ok(AppSettings::HidePossibleValuesInHelp),
            "helpexpected" => Ok(AppSettings::HelpExpected),
            "hidden" => Ok(AppSettings::Hidden),
            "noautohelp" => Ok(AppSettings::NoAutoHelp),
            "noautoversion" => Ok(AppSettings::NoAutoVersion),
            "nobinaryname" => Ok(AppSettings::NoBinaryName),
            "subcommandsnegatereqs" => Ok(AppSettings::SubcommandsNegateReqs),
            "subcommandrequired" => Ok(AppSettings::SubcommandRequired),
            "subcommandrequiredelsehelp" => Ok(AppSettings::SubcommandRequiredElseHelp),
            "uselongformatforhelpsubcommand" => Ok(AppSettings::UseLongFormatForHelpSubcommand),
            "trailingvararg" => Ok(AppSettings::TrailingVarArg),
            "unifiedhelp" => Ok(AppSettings::UnifiedHelp),
            "nextlinehelp" => Ok(AppSettings::NextLineHelp),
            "ignoreerrors" => Ok(AppSettings::IgnoreErrors),
            "waitonerror" => Ok(AppSettings::WaitOnError),
            "built" => Ok(AppSettings::Built),
            "binnamebuilt" => Ok(AppSettings::BinNameBuilt),
            "infersubcommands" => Ok(AppSettings::InferSubcommands),
            "allargsoverrideself" => Ok(AppSettings::AllArgsOverrideSelf),
            "inferlongargs" => Ok(AppSettings::InferLongArgs),
            _ => Err(format!("unknown AppSetting: `{}`", s)),
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(clippy::cognitive_complexity)]
    #[test]
    #[cfg(feature = "yaml")]
    fn app_settings_fromstr() {
        use super::AppSettings;

        assert_eq!(
            "disablehelpflag".parse::<AppSettings>().unwrap(),
            AppSettings::DisableHelpFlag
        );
        assert_eq!(
            "argsnegatesubcommands".parse::<AppSettings>().unwrap(),
            AppSettings::ArgsNegateSubcommands
        );
        assert_eq!(
            "argrequiredelsehelp".parse::<AppSettings>().unwrap(),
            AppSettings::ArgRequiredElseHelp
        );
        assert_eq!(
            "subcommandprecedenceoverarg"
                .parse::<AppSettings>()
                .unwrap(),
            AppSettings::SubcommandPrecedenceOverArg
        );
        assert_eq!(
            "allowexternalsubcommands".parse::<AppSettings>().unwrap(),
            AppSettings::AllowExternalSubcommands
        );
        assert_eq!(
            "allowinvalidutf8forexternalsubcommands"
                .parse::<AppSettings>()
                .unwrap(),
            AppSettings::AllowInvalidUtf8ForExternalSubcommands
        );
        assert_eq!(
            "allowhyphenvalues".parse::<AppSettings>().unwrap(),
            AppSettings::AllowHyphenValues
        );
        assert_eq!(
            "allownegativenumbers".parse::<AppSettings>().unwrap(),
            AppSettings::AllowNegativeNumbers
        );
        assert_eq!(
            "disablehelpsubcommand".parse::<AppSettings>().unwrap(),
            AppSettings::DisableHelpSubcommand
        );
        assert_eq!(
            "disableversionflag".parse::<AppSettings>().unwrap(),
            AppSettings::DisableVersionFlag
        );
        assert_eq!(
            "dontcollapseargsinusage".parse::<AppSettings>().unwrap(),
            AppSettings::DontCollapseArgsInUsage
        );
        assert_eq!(
            "dontdelimittrailingvalues".parse::<AppSettings>().unwrap(),
            AppSettings::DontDelimitTrailingValues
        );
        assert_eq!(
            "derivedisplayorder".parse::<AppSettings>().unwrap(),
            AppSettings::DeriveDisplayOrder
        );
        assert_eq!(
            "disablecoloredhelp".parse::<AppSettings>().unwrap(),
            AppSettings::DisableColoredHelp
        );
        assert_eq!(
            "propagateversion".parse::<AppSettings>().unwrap(),
            AppSettings::PropagateVersion
        );
        assert_eq!(
            "hidden".parse::<AppSettings>().unwrap(),
            AppSettings::Hidden
        );
        assert_eq!(
            "hidepossiblevalues".parse::<AppSettings>().unwrap(),
            AppSettings::HidePossibleValues
        );
        assert_eq!(
            "helpexpected".parse::<AppSettings>().unwrap(),
            AppSettings::HelpExpected
        );
        assert_eq!(
            "nobinaryname".parse::<AppSettings>().unwrap(),
            AppSettings::NoBinaryName
        );
        assert_eq!(
            "nextlinehelp".parse::<AppSettings>().unwrap(),
            AppSettings::NextLineHelp
        );
        assert_eq!(
            "subcommandsnegatereqs".parse::<AppSettings>().unwrap(),
            AppSettings::SubcommandsNegateReqs
        );
        assert_eq!(
            "subcommandrequired".parse::<AppSettings>().unwrap(),
            AppSettings::SubcommandRequired
        );
        assert_eq!(
            "subcommandrequiredelsehelp".parse::<AppSettings>().unwrap(),
            AppSettings::SubcommandRequiredElseHelp
        );
        assert_eq!(
            "uselongformatforhelpsubcommand"
                .parse::<AppSettings>()
                .unwrap(),
            AppSettings::UseLongFormatForHelpSubcommand
        );
        assert_eq!(
            "trailingvararg".parse::<AppSettings>().unwrap(),
            AppSettings::TrailingVarArg
        );
        assert_eq!(
            "waitonerror".parse::<AppSettings>().unwrap(),
            AppSettings::WaitOnError
        );
        assert_eq!("built".parse::<AppSettings>().unwrap(), AppSettings::Built);
        assert_eq!(
            "binnamebuilt".parse::<AppSettings>().unwrap(),
            AppSettings::BinNameBuilt
        );
        assert_eq!(
            "infersubcommands".parse::<AppSettings>().unwrap(),
            AppSettings::InferSubcommands
        );
        assert!("hahahaha".parse::<AppSettings>().is_err());
    }
}
