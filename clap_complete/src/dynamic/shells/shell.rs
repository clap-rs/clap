use std::fmt::Display;
use std::str::FromStr;

use clap::builder::PossibleValue;
use clap::ValueEnum;

/// Completion support for built-in shells
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again `SHell` (bash)
    Bash,
    /// Friendly Interactive `SHell` (fish)
    Fish,
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

// Hand-rolled so it can work even when `derive` feature is disabled
impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[Shell::Bash, Shell::Fish]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Shell::Bash => PossibleValue::new("bash"),
            Shell::Fish => PossibleValue::new("fish"),
        })
    }
}

impl Shell {
    fn completer(&self) -> &dyn crate::dynamic::Completer {
        match self {
            Self::Bash => &super::Bash,
            Self::Fish => &super::Fish,
        }
    }
}

impl crate::dynamic::Completer for Shell {
    fn file_name(&self, name: &str) -> String {
        self.completer().file_name(name)
    }
    fn write_registration(
        &self,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        self.completer()
            .write_registration(name, bin, completer, buf)
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<std::ffi::OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        self.completer().write_complete(cmd, args, current_dir, buf)
    }
}
