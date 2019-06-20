// Std
use std::str::FromStr;

bitflags! {
    struct Flags: u32 {
        const REQUIRED         = 1;
        const MULTIPLE_OCC     = 1 << 1;
        const EMPTY_VALS       = 1 << 2 | Self::TAKES_VAL.bits;
        const GLOBAL           = 1 << 3;
        const HIDDEN           = 1 << 4;
        const TAKES_VAL        = 1 << 5;
        const USE_DELIM        = 1 << 6;
        const NEXT_LINE_HELP   = 1 << 7;
        const R_UNLESS_ALL     = 1 << 8;
        const REQ_DELIM        = 1 << 9 | Self::TAKES_VAL.bits | Self::USE_DELIM.bits;
        const DELIM_NOT_SET    = 1 << 10;
        const HIDE_POS_VALS    = 1 << 11 | Self::TAKES_VAL.bits;
        const ALLOW_TAC_VALS   = 1 << 12 | Self::TAKES_VAL.bits;
        const REQUIRE_EQUALS   = 1 << 13 | Self::TAKES_VAL.bits;
        const LAST             = 1 << 14 | Self::TAKES_VAL.bits;
        const HIDE_DEFAULT_VAL = 1 << 15 | Self::TAKES_VAL.bits;
        const CASE_INSENSITIVE = 1 << 16;
        const HIDE_ENV_VALS    = 1 << 17;
        const HIDDEN_SHORT_H   = 1 << 18;
        const HIDDEN_LONG_H    = 1 << 19;
        const MULTIPLE_VALS    = 1 << 20 | Self::TAKES_VAL.bits;
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ArgFlags(Flags);

impl ArgFlags {
    pub fn new() -> Self { ArgFlags::default() }

    // @TODO @p6 @internal: Reorder alphabetically
    impl_settings! {ArgSettings,
        Required => Flags::REQUIRED,
        MultipleOccurrences => Flags::MULTIPLE_OCC,
        MultipleValues => Flags::MULTIPLE_VALS,
        AllowEmptyValues => Flags::EMPTY_VALS,
        Hidden => Flags::HIDDEN,
        TakesValue => Flags::TAKES_VAL,
        UseValueDelimiter => Flags::USE_DELIM,
        NextLineHelp => Flags::NEXT_LINE_HELP,
        RequiredUnlessAll => Flags::R_UNLESS_ALL,
        RequireDelimiter => Flags::REQ_DELIM,
        ValueDelimiterNotSet => Flags::DELIM_NOT_SET,
        HidePossibleValues => Flags::HIDE_POS_VALS,
        AllowHyphenValues => Flags::ALLOW_TAC_VALS,
        RequireEquals => Flags::REQUIRE_EQUALS,
        Last => Flags::LAST,
        IgnoreCase => Flags::CASE_INSENSITIVE,
        HideEnvValues => Flags::HIDE_ENV_VALS,
        HideDefaultValue => Flags::HIDE_DEFAULT_VAL,
        HiddenShortHelp => Flags::HIDDEN_SHORT_H,
        HiddenLongHelp => Flags::HIDDEN_LONG_H
    }
}

impl Default for ArgFlags {
    fn default() -> Self { ArgFlags(Flags::DELIM_NOT_SET) }
}

/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods [`Arg::setting`], [`Arg::unset_setting`], and [`Arg::is_set`]. This is what the
/// [`Arg`] methods which accept a `bool` use internally.
///
/// [`Arg`]: ./struct.Arg.html
/// [`Arg::setting`]: ./struct.Arg.html#method.setting
/// [`Arg::unset_setting`]: ./struct.Arg.html#method.unset_setting
/// [`Arg::is_set`]: ./struct.Arg.html#method.is_set
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgSettings {
    /// Specifies that an arg must be used
    Required,
    /// Allows an arg to accept multiple values
    MultipleValues,
    /// Allows an arg to appear multiple times
    MultipleOccurrences,
    /// Allows an arg accept empty values such as `""`
    AllowEmptyValues,
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
    /// Requires that an equals be used to provide a value to an option such as `--option=value`
    RequireEquals,
    /// Says that a positional arg will be the last positional, and reuqires `--` to be accessed.
    /// It can also be accessed early (i.e. before other positionals) by providing `--`
    Last,
    /// Hides the default value from the help message
    HideDefaultValue,
    /// Possible values become case insensitive
    IgnoreCase,
    /// Hides any values currently assigned to ENV variables in the help message (good for sensitive
    /// information)
    HideEnvValues,
    /// The argument should **not** be shown in short help text
    HiddenShortHelp,
    /// The argument should **not** be shown in long help text
    HiddenLongHelp,
    #[doc(hidden)]
    RequiredUnlessAll,
    #[doc(hidden)]
    ValueDelimiterNotSet,
}

impl FromStr for ArgSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "required" => Ok(ArgSettings::Required),
            "allowemptyvalues" => Ok(ArgSettings::AllowEmptyValues),
            "hidden" => Ok(ArgSettings::Hidden),
            "takesvalue" => Ok(ArgSettings::TakesValue),
            "usevaluedelimiter" => Ok(ArgSettings::UseValueDelimiter),
            "nextlinehelp" => Ok(ArgSettings::NextLineHelp),
            "requiredunlessall" => Ok(ArgSettings::RequiredUnlessAll),
            "requiredelimiter" => Ok(ArgSettings::RequireDelimiter),
            "valuedelimiternotset" => Ok(ArgSettings::ValueDelimiterNotSet),
            "hidepossiblevalues" => Ok(ArgSettings::HidePossibleValues),
            "allowhyphenvalues" => Ok(ArgSettings::AllowHyphenValues),
            "requireequals" => Ok(ArgSettings::RequireEquals),
            "last" => Ok(ArgSettings::Last),
            "hidedefaultvalue" => Ok(ArgSettings::HideDefaultValue),
            "ignorecase" => Ok(ArgSettings::IgnoreCase),
            "hideenvvalues" => Ok(ArgSettings::HideEnvValues),
            "hiddenshorthelp" => Ok(ArgSettings::HiddenShortHelp),
            "hiddenlonghelp" => Ok(ArgSettings::HiddenLongHelp),
            _ => Err("unknown ArgSetting, cannot convert from str".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ArgSettings;

    #[test]
    fn arg_settings_fromstr() {
        assert_eq!(
            "allowhyphenvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowHyphenValues
        );
        assert_eq!(
            "allowemptyvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowEmptyValues
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
            "requiredunlessall".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequiredUnlessAll
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
            "valuedelimiternotset".parse::<ArgSettings>().unwrap(),
            ArgSettings::ValueDelimiterNotSet
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
        assert!("hahahaha".parse::<ArgSettings>().is_err());
    }
}
