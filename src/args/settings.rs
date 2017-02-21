// Std
use std::ascii::AsciiExt;
use std::str::FromStr;

bitflags! {
    flags Flags: u16 {
        const REQUIRED       = 0b00000000000001,
        const MULTIPLE       = 0b00000000000010,
        const EMPTY_VALS     = 0b00000000000100,
        const GLOBAL         = 0b00000000001000,
        const HIDDEN         = 0b00000000010000,
        const TAKES_VAL      = 0b00000000100000,
        const USE_DELIM      = 0b00000001000000,
        const NEXT_LINE_HELP = 0b00000010000000,
        const R_UNLESS_ALL   = 0b00000100000000,
        const REQ_DELIM      = 0b00001000000000,
        const DELIM_NOT_SET  = 0b00010000000000,
        const HIDE_POS_VALS  = 0b00100000000000,
        const ALLOW_TAC_VALS = 0b01000000000000,
        const REQUIRE_EQUALS = 0b10000000000000,
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ArgFlags(Flags);

impl ArgFlags {
    pub fn new() -> Self { ArgFlags::default() }

    impl_settings!{ArgSettings,
        Required => REQUIRED,
        Multiple => MULTIPLE,
        EmptyValues => EMPTY_VALS,
        Global => GLOBAL,
        Hidden => HIDDEN,
        TakesValue => TAKES_VAL,
        UseValueDelimiter => USE_DELIM,
        NextLineHelp => NEXT_LINE_HELP,
        RequiredUnlessAll => R_UNLESS_ALL,
        RequireDelimiter => REQ_DELIM,
        ValueDelimiterNotSet => DELIM_NOT_SET,
        HidePossibleValues => HIDE_POS_VALS,
        AllowLeadingHyphen => ALLOW_TAC_VALS,
        RequireEquals => REQUIRE_EQUALS 
    }
}

impl Default for ArgFlags {
    fn default() -> Self { ArgFlags(EMPTY_VALS | DELIM_NOT_SET) }
}

/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods [`Arg::set`], [`Arg::unset`], and [`Arg::is_set`]
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
    /// The argument should be propagated down through all child [`SubCommands`]
    /// [`SubCommand`]: ./struct.SubCommand.html
    Global,
    /// The argument should **not** be shown in help text
    Hidden,
    /// The argument accepts a value, such as `--option <value>`
    TakesValue,
    /// Determines if the argument allows values to be grouped via a delimter
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
            _ => Err("unknown ArgSetting, cannot convert from str".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ArgSettings;

    #[test]
    fn arg_settings_fromstr() {
        assert_eq!("allowleadinghyphen".parse::<ArgSettings>().unwrap(),
                   ArgSettings::AllowLeadingHyphen);
        assert_eq!("emptyvalues".parse::<ArgSettings>().unwrap(),
                   ArgSettings::EmptyValues);
        assert_eq!("global".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Global);
        assert_eq!("hidepossiblevalues".parse::<ArgSettings>().unwrap(),
                   ArgSettings::HidePossibleValues);
        assert_eq!("hidden".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Hidden);
        assert_eq!("multiple".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Multiple);
        assert_eq!("nextlinehelp".parse::<ArgSettings>().unwrap(),
                   ArgSettings::NextLineHelp);
        assert_eq!("requiredunlessall".parse::<ArgSettings>().unwrap(),
                   ArgSettings::RequiredUnlessAll);
        assert_eq!("requiredelimiter".parse::<ArgSettings>().unwrap(),
                   ArgSettings::RequireDelimiter);
        assert_eq!("required".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Required);
        assert_eq!("takesvalue".parse::<ArgSettings>().unwrap(),
                   ArgSettings::TakesValue);
        assert_eq!("usevaluedelimiter".parse::<ArgSettings>().unwrap(),
                   ArgSettings::UseValueDelimiter);
        assert_eq!("valuedelimiternotset".parse::<ArgSettings>().unwrap(),
                   ArgSettings::ValueDelimiterNotSet);
        assert_eq!("requireequals".parse::<ArgSettings>().unwrap(),
                   ArgSettings::RequireEquals);
        assert!("hahahaha".parse::<ArgSettings>().is_err());
    }
}
