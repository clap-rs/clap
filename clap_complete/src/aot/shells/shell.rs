use std::fmt::Display;
use std::io::Error;
use std::path::Path;
use std::str::FromStr;

use clap::builder::PossibleValue;
use clap::ValueEnum;

use crate::shells;
use crate::Generator;

/// Shell with auto-generated completion script available.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again `SHell` (bash)
    Bash,
    /// Elvish shell
    Elvish,
    /// Friendly Interactive `SHell` (fish)
    Fish,
    /// `PowerShell`
    PowerShell,
    /// Z `SHell` (zsh)
    Zsh,
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
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::PowerShell,
            Self::Zsh,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(match self {
            Self::Bash => "bash",
            Self::Elvish => "elvish",
            Self::Fish => "fish",
            Self::PowerShell => "powershell",
            Self::Zsh => "zsh",
        }))
    }
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => shells::Bash.file_name(name),
            Self::Elvish => shells::Elvish.file_name(name),
            Self::Fish => shells::Fish.file_name(name),
            Self::PowerShell => shells::PowerShell.file_name(name),
            Self::Zsh => shells::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        self.try_generate(cmd, buf)
            .expect("failed to write completion file");
    }

    fn try_generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) -> Result<(), Error> {
        match self {
            Self::Bash => shells::Bash.try_generate(cmd, buf),
            Self::Elvish => shells::Elvish.try_generate(cmd, buf),
            Self::Fish => shells::Fish.try_generate(cmd, buf),
            Self::PowerShell => shells::PowerShell.try_generate(cmd, buf),
            Self::Zsh => shells::Zsh.try_generate(cmd, buf),
        }
    }
}

impl Shell {
    /// Parse a shell from a path to the executable for the shell
    ///
    /// # Examples
    ///
    /// ```
    /// use clap_complete::shells::Shell;
    ///
    /// assert_eq!(Shell::from_shell_path("/bin/bash"), Some(Shell::Bash));
    /// assert_eq!(Shell::from_shell_path("/usr/bin/zsh"), Some(Shell::Zsh));
    /// assert_eq!(Shell::from_shell_path("/opt/my_custom_shell"), None);
    /// ```
    pub fn from_shell_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        parse_shell_from_path(path.as_ref())
    }

    /// Determine the user's current shell from the environment
    ///
    /// This will read the SHELL environment variable and try to determine which shell is in use
    /// from that.
    ///
    /// If SHELL is not set, then on windows, it will default to powershell, and on
    /// other operating systems it will return `None`.
    ///
    /// If SHELL is set, but contains a value that doesn't correspond to one of the supported shell
    /// types, then return `None`.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use clap::Command;
    /// use clap_complete::{generate, shells::Shell};
    /// # fn build_cli() -> Command {
    /// #     Command::new("compl")
    /// # }
    /// let mut cmd = build_cli();
    /// generate(Shell::from_env().unwrap_or(Shell::Bash), &mut cmd, "myapp", &mut std::io::stdout());
    /// ```
    pub fn from_env() -> Option<Self> {
        if let Some(env_shell) = std::env::var_os("SHELL") {
            Self::from_shell_path(env_shell)
        } else if cfg!(windows) {
            Some(Self::PowerShell)
        } else {
            None
        }
    }
}

// use a separate function to avoid having to monomorphize the entire function due
// to from_shell_path being generic
fn parse_shell_from_path(path: &Path) -> Option<Shell> {
    let name = path.file_stem()?.to_str()?;
    Some(match name {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "elvish" => Shell::Elvish,
        "powershell" | "powershell_ise" => Shell::PowerShell,
        _ => return None,
    })
}
