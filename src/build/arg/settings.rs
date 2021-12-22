// Std
use std::ops::BitOr;
#[cfg(feature = "yaml")]
use std::str::FromStr;

// Third party
use bitflags::bitflags;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgFlags(Flags);

impl Default for ArgFlags {
    fn default() -> Self {
        Self::empty()
    }
}

/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods [`Arg::setting`], [`Arg::unset_setting`], and [`Arg::is_set`]. This is what the
/// [`Arg`] methods which accept a `bool` use internally.
///
/// [`Arg`]: crate::Arg
/// [`Arg::setting`]: crate::Arg::setting()
/// [`Arg::unset_setting`]: crate::Arg::unset_setting()
/// [`Arg::is_set`]: crate::Arg::is_set()
#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum ArgSettings {
    /// Specifies that an arg must be used
    Required,
    /// Allows an arg to accept multiple values
    MultipleValues,
    /// Allows an arg to appear multiple times
    MultipleOccurrences,
    /// Deprecated, see [`ArgSettings::MultipleOccurrences`] (most likely what you want) and
    /// [`ArgSettings::MultipleValues`]
    #[deprecated(
        since = "3.0.0",
        note = "Split into `ArgSettings::MultipleOccurrences` (most likely what you want)  and `ArgSettings::MultipleValues`"
    )]
    Multiple,
    /// Forbids an arg from accepting empty values such as `""`
    ForbidEmptyValues,
    /// Sets an arg to be global (i.e. exist in all subcommands)
    Global,
    /// Hides an arg from the help message
    Hidden,
    /// Allows an argument to take a value (such as `--option value`)
    TakesValue,
    /// Enables a delimiter to break up arguments `--option val1,val2,val3` becomes three values
    /// (`val1`, `val2`, and `val3`) instead of the default one (`val1,val2,val3`)
    UseValueDelimiter,
    /// Tells an arg to display it's help on the line below the arg itself in the help message
    NextLineHelp,
    /// Says that arg *must* use a delimiter to separate values
    RequireDelimiter,
    /// Hides the possible values from the help message
    HidePossibleValues,
    /// Allows values that start with a hyphen
    AllowHyphenValues,
    /// Deprecated, replaced with [`ArgSettings::AllowHyphenValues`]
    #[deprecated(
        since = "3.0.0",
        note = "Replaced with `ArgSettings::AllowHyphenValues`"
    )]
    AllowLeadingHyphen,
    /// Requires that an equals be used to provide a value to an option such as `--option=value`
    RequireEquals,
    /// Says that a positional arg will be the last positional, and requires `--` to be accessed.
    /// It can also be accessed early (i.e. before other positionals) by providing `--`
    Last,
    /// Hides the default value from the help message
    HideDefaultValue,
    /// Possible values become case insensitive
    IgnoreCase,
    /// Deprecated, replaced with [`ArgSettings::IgnoreCase`]
    #[deprecated(since = "3.0.0", note = "Replaced with `ArgSettings::IgnoreCase`")]
    CaseInsensitive,
    /// Hides environment variable arguments from the help message
    #[cfg(feature = "env")]
    HideEnv,
    /// Hides any values currently assigned to ENV variables in the help message (good for sensitive
    /// information)
    #[cfg(feature = "env")]
    HideEnvValues,
    /// The argument should **not** be shown in short help text
    HiddenShortHelp,
    /// The argument should **not** be shown in long help text
    HiddenLongHelp,
    /// Specifies that option values that are invalid UTF-8 should *not* be treated as an error.
    AllowInvalidUtf8,
    /// Specifies that option should exist on its own.
    /// Having any other arguments present at runtime is an error.
    Exclusive,
}

bitflags! {
    struct Flags: u32 {
        const REQUIRED         = 1;
        const MULTIPLE_OCC     = 1 << 1;
        const NO_EMPTY_VALS    = 1 << 2;
        const GLOBAL           = 1 << 3;
        const HIDDEN           = 1 << 4;
        const TAKES_VAL        = 1 << 5;
        const USE_DELIM        = 1 << 6;
        const NEXT_LINE_HELP   = 1 << 7;
        const REQ_DELIM        = 1 << 9;
        const DELIM_NOT_SET    = 1 << 10;
        const HIDE_POS_VALS    = 1 << 11;
        const ALLOW_TAC_VALS   = 1 << 12;
        const REQUIRE_EQUALS   = 1 << 13;
        const LAST             = 1 << 14;
        const HIDE_DEFAULT_VAL = 1 << 15;
        const CASE_INSENSITIVE = 1 << 16;
        #[cfg(feature = "env")]
        const HIDE_ENV_VALS    = 1 << 17;
        const HIDDEN_SHORT_H   = 1 << 18;
        const HIDDEN_LONG_H    = 1 << 19;
        const MULTIPLE_VALS    = 1 << 20;
        const MULTIPLE         = Self::MULTIPLE_OCC.bits | Self::MULTIPLE_VALS.bits;
        #[cfg(feature = "env")]
        const HIDE_ENV         = 1 << 21;
        const UTF8_NONE        = 1 << 22;
        const EXCLUSIVE        = 1 << 23;
        const NO_OP            = 0;
    }
}

impl_settings! { ArgSettings, ArgFlags,
    Required => Flags::REQUIRED,
    MultipleOccurrences => Flags::MULTIPLE_OCC,
    MultipleValues => Flags::MULTIPLE_VALS,
    Multiple => Flags::MULTIPLE,
    ForbidEmptyValues => Flags::NO_EMPTY_VALS,
    Global => Flags::GLOBAL,
    Hidden => Flags::HIDDEN,
    TakesValue => Flags::TAKES_VAL,
    UseValueDelimiter => Flags::USE_DELIM,
    NextLineHelp => Flags::NEXT_LINE_HELP,
    RequireDelimiter => Flags::REQ_DELIM,
    HidePossibleValues => Flags::HIDE_POS_VALS,
    AllowHyphenValues => Flags::ALLOW_TAC_VALS,
    AllowLeadingHyphen => Flags::ALLOW_TAC_VALS,
    RequireEquals => Flags::REQUIRE_EQUALS,
    Last => Flags::LAST,
    IgnoreCase => Flags::CASE_INSENSITIVE,
    CaseInsensitive => Flags::CASE_INSENSITIVE,
    #[cfg(feature = "env")]
    HideEnv => Flags::HIDE_ENV,
    #[cfg(feature = "env")]
    HideEnvValues => Flags::HIDE_ENV_VALS,
    HideDefaultValue => Flags::HIDE_DEFAULT_VAL,
    HiddenShortHelp => Flags::HIDDEN_SHORT_H,
    HiddenLongHelp => Flags::HIDDEN_LONG_H,
    AllowInvalidUtf8 => Flags::UTF8_NONE,
    Exclusive => Flags::EXCLUSIVE
}

/// Deprecated in [Issue #3087](https://github.com/clap-rs/clap/issues/3087), maybe [`clap::Parser`][crate::Parser] would fit your use case?
#[cfg(feature = "yaml")]
impl FromStr for ArgSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        #[allow(deprecated)]
        #[allow(unreachable_patterns)]
        match &*s.to_ascii_lowercase() {
            "required" => Ok(ArgSettings::Required),
            "multipleoccurrences" => Ok(ArgSettings::MultipleOccurrences),
            "multiplevalues" => Ok(ArgSettings::MultipleValues),
            "multiple" => Ok(ArgSettings::Multiple),
            "forbidemptyvalues" => Ok(ArgSettings::ForbidEmptyValues),
            "global" => Ok(ArgSettings::Global),
            "hidden" => Ok(ArgSettings::Hidden),
            "takesvalue" => Ok(ArgSettings::TakesValue),
            "usevaluedelimiter" => Ok(ArgSettings::UseValueDelimiter),
            "nextlinehelp" => Ok(ArgSettings::NextLineHelp),
            "requiredelimiter" => Ok(ArgSettings::RequireDelimiter),
            "hidepossiblevalues" => Ok(ArgSettings::HidePossibleValues),
            "allowhyphenvalues" => Ok(ArgSettings::AllowHyphenValues),
            "allowleadinghypyhen" => Ok(ArgSettings::AllowLeadingHyphen),
            "requireequals" => Ok(ArgSettings::RequireEquals),
            "last" => Ok(ArgSettings::Last),
            "ignorecase" => Ok(ArgSettings::IgnoreCase),
            "caseinsensitive" => Ok(ArgSettings::CaseInsensitive),
            #[cfg(feature = "env")]
            "hideenv" => Ok(ArgSettings::HideEnv),
            #[cfg(feature = "env")]
            "hideenvvalues" => Ok(ArgSettings::HideEnvValues),
            "hidedefaultvalue" => Ok(ArgSettings::HideDefaultValue),
            "hiddenshorthelp" => Ok(ArgSettings::HiddenShortHelp),
            "hiddenlonghelp" => Ok(ArgSettings::HiddenLongHelp),
            "allowinvalidutf8" => Ok(ArgSettings::AllowInvalidUtf8),
            "exclusive" => Ok(ArgSettings::Exclusive),
            _ => Err(format!("unknown AppSetting: `{}`", s)),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "yaml")]
    fn arg_settings_fromstr() {
        use super::ArgSettings;

        assert_eq!(
            "allowhyphenvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowHyphenValues
        );
        assert_eq!(
            "forbidemptyvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::ForbidEmptyValues
        );
        assert_eq!(
            "hidepossiblevalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::HidePossibleValues
        );
        assert_eq!(
            "hidden".parse::<ArgSettings>().unwrap(),
            ArgSettings::Hidden
        );
        assert_eq!(
            "nextlinehelp".parse::<ArgSettings>().unwrap(),
            ArgSettings::NextLineHelp
        );
        assert_eq!(
            "requiredelimiter".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequireDelimiter
        );
        assert_eq!(
            "required".parse::<ArgSettings>().unwrap(),
            ArgSettings::Required
        );
        assert_eq!(
            "takesvalue".parse::<ArgSettings>().unwrap(),
            ArgSettings::TakesValue
        );
        assert_eq!(
            "usevaluedelimiter".parse::<ArgSettings>().unwrap(),
            ArgSettings::UseValueDelimiter
        );
        assert_eq!(
            "requireequals".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequireEquals
        );
        assert_eq!("last".parse::<ArgSettings>().unwrap(), ArgSettings::Last);
        assert_eq!(
            "hidedefaultvalue".parse::<ArgSettings>().unwrap(),
            ArgSettings::HideDefaultValue
        );
        assert_eq!(
            "ignorecase".parse::<ArgSettings>().unwrap(),
            ArgSettings::IgnoreCase
        );
        #[cfg(feature = "env")]
        assert_eq!(
            "hideenv".parse::<ArgSettings>().unwrap(),
            ArgSettings::HideEnv
        );
        #[cfg(feature = "env")]
        assert_eq!(
            "hideenvvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::HideEnvValues
        );
        assert_eq!(
            "hiddenshorthelp".parse::<ArgSettings>().unwrap(),
            ArgSettings::HiddenShortHelp
        );
        assert_eq!(
            "hiddenlonghelp".parse::<ArgSettings>().unwrap(),
            ArgSettings::HiddenLongHelp
        );
        assert_eq!(
            "allowinvalidutf8".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowInvalidUtf8
        );
        assert_eq!(
            "exclusive".parse::<ArgSettings>().unwrap(),
            ArgSettings::Exclusive
        );
        assert!("hahahaha".parse::<ArgSettings>().is_err());
    }
}
