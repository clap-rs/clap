use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

use clap::__macro_refs::once_cell::sync::Lazy;
use clap::builder::PossibleValue;
use clap::ValueEnum;

use crate::shells;
use crate::Generator;

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

fn shell_possible_value(name: &'static str, paths: &'static [String]) -> PossibleValue {
    let mut pv = PossibleValue::new(name);

    for path in paths {
        pv = pv.alias(path.as_str());
    }

    pv
}

// Hand-rolled so it can work even when `derive` feature is disabled
impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Zsh,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        static SHELL_PATHS: Lazy<ShellPaths> = Lazy::new(ShellPaths::new);

        Some(match self {
            Shell::Bash => shell_possible_value("bash", &SHELL_PATHS.bash),
            Shell::Elvish => shell_possible_value("elvish", &SHELL_PATHS.elvish),
            Shell::Fish => shell_possible_value("fish", &SHELL_PATHS.fish),
            Shell::PowerShell => shell_possible_value("powershell", &SHELL_PATHS.powershell),
            Shell::Zsh => shell_possible_value("zsh", &SHELL_PATHS.zsh),
        })
    }
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => shells::Bash.file_name(name),
            Shell::Elvish => shells::Elvish.file_name(name),
            Shell::Fish => shells::Fish.file_name(name),
            Shell::PowerShell => shells::PowerShell.file_name(name),
            Shell::Zsh => shells::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => shells::Bash.generate(cmd, buf),
            Shell::Elvish => shells::Elvish.generate(cmd, buf),
            Shell::Fish => shells::Fish.generate(cmd, buf),
            Shell::PowerShell => shells::PowerShell.generate(cmd, buf),
            Shell::Zsh => shells::Zsh.generate(cmd, buf),
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
    pub fn from_shell_path<P: AsRef<Path>>(path: P) -> Option<Shell> {
        parse_shell_from_path(path.as_ref())
    }

    /// Determine the user's current shell from the environment
    ///
    /// This will read the SHELL environment variable and try to determine which shell is in use
    /// from that.
    ///
    /// If SHELL is not set, then on windows, it will default to powershell, and on
    /// other OSes it will return `None`.
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
    pub fn from_env() -> Option<Shell> {
        if let Some(env_shell) = std::env::var_os("SHELL") {
            Shell::from_shell_path(env_shell)
        } else if cfg!(windows) {
            Some(Shell::PowerShell)
        } else {
            None
        }
    }
}

// use a separate function to avoid having to monomorphize the entire function due
// to from_shell_path being generic
fn parse_shell_from_path(path: &Path) -> Option<Shell> {
    let name = path.file_stem()?.to_str()?;
    match name {
        "bash" => Some(Shell::Bash),
        "zsh" => Some(Shell::Zsh),
        "fish" => Some(Shell::Fish),
        "elvish" => Some(Shell::Elvish),
        "powershell" | "powershell_ise" => Some(Shell::PowerShell),
        _ => None,
    }
}

#[derive(Default)]
struct ShellPaths {
    bash: Vec<String>,
    elvish: Vec<String>,
    fish: Vec<String>,
    powershell: Vec<String>,
    zsh: Vec<String>,
}

impl ShellPaths {
    fn new() -> Self {
        #[cfg(windows)]
        const SEPARATOR: char = ';';
        #[cfg(not(windows))]
        const SEPARATOR: char = ':';

        let mut this = Self::default();

        let path_var = match std::env::var("PATH") {
            Ok(path_var) => path_var,
            Err(_) => return this,
        };

        for base in path_var.split(SEPARATOR).map(Path::new) {
            add_shell_path(&mut this.bash, base, "bash");
            add_shell_path(&mut this.elvish, base, "elvish");
            add_shell_path(&mut this.fish, base, "fish");
            add_shell_path(&mut this.powershell, base, "powershell");
            add_shell_path(&mut this.zsh, base, "zsh");
        }

        this
    }
}

fn add_shell_path(shell_paths: &mut Vec<String>, base: &Path, name: &str) {
    let path = base.join(name);

    #[cfg(windows)]
    let path = &path.with_extension("exe");

    if path.exists() {
        if let Some(path) = path.to_str() {
            shell_paths.push(path.to_string());
        }
    }
}
