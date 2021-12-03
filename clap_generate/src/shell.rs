use std::fmt::Display;
use std::str::FromStr;

use clap::{ArgEnum, PossibleValue};

use crate::{generators, Generator};

/// Shell with auto-generated completion script available.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,
    /// Elvish shell
    Elvish,
    /// Friendly Interactive SHell (fish)
    Fish,
    /// PowerShell
    PowerShell,
    /// Z SHell (zsh)
    Zsh,
}

impl Shell {
    /// Report all `possible_values`
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        Shell::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
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
        Err(format!("Invalid variant: {}", s))
    }
}

// Hand-rolled so it can work even when `derive` feature is disabled
impl ArgEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Zsh,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        Some(match self {
            Shell::Bash => PossibleValue::new("bash"),
            Shell::Elvish => PossibleValue::new("elvish"),
            Shell::Fish => PossibleValue::new("fish"),
            Shell::PowerShell => PossibleValue::new("powershell"),
            Shell::Zsh => PossibleValue::new("zsh"),
        })
    }
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => generators::Bash.file_name(name),
            Shell::Elvish => generators::Elvish.file_name(name),
            Shell::Fish => generators::Fish.file_name(name),
            Shell::PowerShell => generators::PowerShell.file_name(name),
            Shell::Zsh => generators::Zsh.file_name(name),
        }
    }

    fn generate(&self, app: &clap::App, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => generators::Bash.generate(app, buf),
            Shell::Elvish => generators::Elvish.generate(app, buf),
            Shell::Fish => generators::Fish.generate(app, buf),
            Shell::PowerShell => generators::PowerShell.generate(app, buf),
            Shell::Zsh => generators::Zsh.generate(app, buf),
        }
    }
}
