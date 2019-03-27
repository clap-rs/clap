// Std
use std::collections::HashSet;
#[allow(deprecated, unused_imports)]
use std::ascii::AsciiExt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Flags {
    Required,
    Multiple,
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
    HiddenLongH
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ArgFlags(HashSet<Flags>);

impl ArgFlags {
    pub fn new() -> Self { ArgFlags::default() }

    impl_settings!{ArgSettings,
        Required => Flags::Required,
        Multiple => Flags::Multiple,
        EmptyValues => Flags::EmptyVals,
        Global => Flags::Global,
        Hidden => Flags::Hidden,
        TakesValue => Flags::TakesVal,
        UseValueDelimiter => Flags::UseDelim,
        NextLineHelp => Flags::NextLineHelp,
        RequiredUnlessAll => Flags::RUnlessAll,
        RequireDelimiter => Flags::ReqDelim,
        ValueDelimiterNotSet => Flags::DelimNotSet,
        HidePossibleValues => Flags::HidePosVals,
        AllowLeadingHyphen => Flags::AllowTacVals,
        RequireEquals => Flags::RequireEquals,
        Last => Flags::Last,
        CaseInsensitive => Flags::CaseInsensitive,
        HideEnvValues => Flags::HideEnvVals,
        HideDefaultValue => Flags::HideDefaultVal,
        HiddenShortHelp => Flags::HiddenShortH,
        HiddenLongHelp => Flags::HiddenLongH
    }
}

impl Default for ArgFlags {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(Flags::EmptyVals);
        set.insert(Flags::DelimNotSet);
        ArgFlags(set)
    }
}

/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods [`Arg::set`], [`Arg::unset`], and [`Arg::is_set`]
///
/// [`Arg::set`]: ./struct.Arg.html#method.set
/// [`Arg::unset`]: ./struct.Arg.html#method.unset
/// [`Arg::is_set`]: ./struct.Arg.html#method.is_set
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgSettings {
    /// The argument must be used
    Required,
    /// The argument may be used multiple times such as `--flag --flag`
    Multiple,
    /// The argument allows empty values such as `--option ""`
    EmptyValues,
    /// The argument should be propagated down through all child [`SubCommand`]s
    ///
    /// [`SubCommand`]: ./struct.SubCommand.html
    Global,
    /// The argument should **not** be shown in help text
    Hidden,
    /// The argument accepts a value, such as `--option <value>`
    TakesValue,
    /// Determines if the argument allows values to be grouped via a delimiter
    UseValueDelimiter,
    /// Prints the help text on the line after the argument
    NextLineHelp,
    /// Requires the use of a value delimiter for all multiple values
    RequireDelimiter,
    /// Hides the possible values from the help string
    HidePossibleValues,
    /// Allows vals that start with a '-'
    AllowLeadingHyphen,
    /// Require options use `--option=val` syntax
    RequireEquals,
    /// Specifies that the arg is the last positional argument and may be accessed early via `--`
    /// syntax
    Last,
    /// Hides the default value from the help string
    HideDefaultValue,
    /// Makes `Arg::possible_values` case insensitive
    CaseInsensitive,
    /// Hides ENV values in the help message
    HideEnvValues,
    /// The argument should **not** be shown in short help text
    HiddenShortHelp,
    /// The argument should **not** be shown in long help text
    HiddenLongHelp,
    #[doc(hidden)] RequiredUnlessAll,
    #[doc(hidden)] ValueDelimiterNotSet,
}

impl FromStr for ArgSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "required" => Ok(ArgSettings::Required),
            "multiple" => Ok(ArgSettings::Multiple),
            "global" => Ok(ArgSettings::Global),
            "emptyvalues" => Ok(ArgSettings::EmptyValues),
            "hidden" => Ok(ArgSettings::Hidden),
            "takesvalue" => Ok(ArgSettings::TakesValue),
            "usevaluedelimiter" => Ok(ArgSettings::UseValueDelimiter),
            "nextlinehelp" => Ok(ArgSettings::NextLineHelp),
            "requiredunlessall" => Ok(ArgSettings::RequiredUnlessAll),
            "requiredelimiter" => Ok(ArgSettings::RequireDelimiter),
            "valuedelimiternotset" => Ok(ArgSettings::ValueDelimiterNotSet),
            "hidepossiblevalues" => Ok(ArgSettings::HidePossibleValues),
            "allowleadinghyphen" => Ok(ArgSettings::AllowLeadingHyphen),
            "requireequals" => Ok(ArgSettings::RequireEquals),
            "last" => Ok(ArgSettings::Last),
            "hidedefaultvalue" => Ok(ArgSettings::HideDefaultValue),
            "caseinsensitive" => Ok(ArgSettings::CaseInsensitive),
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
            "allowleadinghyphen".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowLeadingHyphen
        );
        assert_eq!(
            "emptyvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::EmptyValues
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
            "multiple".parse::<ArgSettings>().unwrap(),
            ArgSettings::Multiple
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
            "caseinsensitive".parse::<ArgSettings>().unwrap(),
            ArgSettings::CaseInsensitive
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
