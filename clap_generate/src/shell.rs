use std::fmt::Display;
use std::str::FromStr;

use clap::{ArgEnum, ArgValue};

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
    pub fn arg_values() -> impl Iterator<Item = ArgValue<'static>> {
        Shell::value_variants()
            .iter()
            .filter_map(ArgEnum::to_arg_value)
    }
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_arg_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_arg_value().unwrap().matches(s, false) {
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

    fn to_arg_value<'a>(&self) -> Option<ArgValue<'a>> {
        let value = match self {
            Shell::Bash => ArgValue::new("bash"),
            Shell::Elvish => ArgValue::new("elvish"),
            Shell::Fish => ArgValue::new("fish"),
            Shell::PowerShell => ArgValue::new("powershell"),
            Shell::Zsh => ArgValue::new("zsh"),
        };
        Some(value)
    }
}

impl crate::Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => crate::generators::Bash.file_name(name),
            Shell::Elvish => crate::generators::Elvish.file_name(name),
            Shell::Fish => crate::generators::Fish.file_name(name),
            Shell::PowerShell => crate::generators::PowerShell.file_name(name),
            Shell::Zsh => crate::generators::Zsh.file_name(name),
        }
    }

    fn generate(&self, app: &clap::App, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => crate::generators::Bash.generate(app, buf),
            Shell::Elvish => crate::generators::Elvish.generate(app, buf),
            Shell::Fish => crate::generators::Fish.generate(app, buf),
            Shell::PowerShell => crate::generators::PowerShell.generate(app, buf),
            Shell::Zsh => crate::generators::Zsh.generate(app, buf),
        }
    }
}
