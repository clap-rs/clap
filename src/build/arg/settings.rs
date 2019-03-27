// Std
use std::collections::HashSet;
#[allow(unused_imports)]
use std::ascii::AsciiExt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Flags {
    Required,
    MultipleOcc,
    EmptyVals,
    Global,
    Hidden,
    TakesVal,
    UseDelim,
    NextLineHelp,
    RUnlessAll,
    ReqDelim,
    DelimNotSet,
    HidePosVals,
    AllowTacVals,
    RequireEquals,
    Last,
    HideDefaultVal,
    CaseInsensitive,
    HideEnvVals,
    HiddenShortH,
    HiddenLongH,
    MultipleVals,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ArgFlags(HashSet<Flags>);

impl ArgFlags {
    pub fn new() -> Self { ArgFlags::default() }

    // @TODO @p6 @internal: Reorder alphabetically
    impl_settings!{ArgSettings,
        Required => Flags::Required,
        MultipleOccurrences => Flags::MultipleOcc,
        MultipleValues => Flags::MultipleVals | Flags::TakesVal,
        AllowEmptyValues => Flags::EmptyVals | Flags::TakesVal,
        Global => Flags::Global,
        Hidden => Flags::Hidden,
        TakesValue => Flags::TakesVal,
        UseValueDelimiter => Flags::UseDelim,
        NextLineHelp => Flags::NextLineHelp,
        RequiredUnlessAll => Flags::RUnlessAll,
        RequireDelimiter => Flags::ReqDelim | Flags::TakesVal | Flags::UseDelim,
        ValueDelimiterNotSet => Flags::DelimNotSet,
        HidePossibleValues => Flags::HidePosVals | Flags::TakesVal,
        AllowHyphenValues => Flags::AllowTacVals | Flags::TakesVal,
        RequireEquals => Flags::RequireEquals | Flags::TakesVal,
        Last => Flags::Last | Flags::TakesVal,
        IgnoreCase => Flags::CaseInsensitive,
        HideEnvValues => Flags::HideEnvVals,
        HideDefaultValue => Flags::HideDefaultVal | Flags::TakesVal,
        HiddenShortHelp => Flags::HiddenShortH,
        HiddenLongHelp => Flags::HiddenLongH
    }
}

impl Default for ArgFlags {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(Flags::DelimNotSet);
        ArgFlags(set)
    }
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
    /// Sets an arg to be global (i.e. exist in all subcommands)
    /// **DEPRECATED**
    #[deprecated(since = "2.32.0", note = "Use `App::global_arg` instead")]
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
            "global" => Ok(ArgSettings::Global),
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
            "global".parse::<ArgSettings>().unwrap(),
            ArgSettings::Global
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
