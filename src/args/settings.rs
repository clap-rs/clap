use std::str::FromStr;
use std::ascii::AsciiExt;

bitflags! {
    flags Flags: u8 {
        const REQUIRED   = 0b00001,
        const MULTIPLE   = 0b00010,
        const EMPTY_VALS = 0b00100,
        const GLOBAL     = 0b01000,
        const HIDDEN     = 0b10000,
    }
}

#[derive(Debug)]
pub struct ArgFlags(Flags);

impl ArgFlags {
    pub fn new() -> Self {
        ArgFlags(EMPTY_VALS)
    }

    pub fn set(&mut self, s: &ArgSettings) {
        match *s {
            ArgSettings::Required => self.0.insert(REQUIRED),
            ArgSettings::Multiple => self.0.insert(MULTIPLE),
            ArgSettings::EmptyValues => self.0.insert(EMPTY_VALS),
            ArgSettings::Global => self.0.insert(GLOBAL),
            ArgSettings::Hidden => self.0.insert(HIDDEN),
        }
    }

    pub fn unset(&mut self, s: &ArgSettings) {
        match *s {
            ArgSettings::Required => self.0.remove(REQUIRED),
            ArgSettings::Multiple => self.0.remove(MULTIPLE),
            ArgSettings::EmptyValues => self.0.remove(EMPTY_VALS),
            ArgSettings::Global => self.0.remove(GLOBAL),
            ArgSettings::Hidden => self.0.remove(HIDDEN),
        }
    }

    pub fn is_set(&self, s: &ArgSettings) -> bool {
        match *s {
            ArgSettings::Required => self.0.contains(REQUIRED),
            ArgSettings::Multiple => self.0.contains(MULTIPLE),
            ArgSettings::EmptyValues => self.0.contains(EMPTY_VALS),
            ArgSettings::Global => self.0.contains(GLOBAL),
            ArgSettings::Hidden => self.0.contains(HIDDEN),
        }
    }
}

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub enum ArgSettings {
    Required,
    Multiple,
    EmptyValues,
    Global,
    Hidden,
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
            _ => Err("unknown ArgSetting, cannot convert from str".to_owned()),
        }
    }
}
