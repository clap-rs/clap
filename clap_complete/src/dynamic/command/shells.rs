use std::ffi::OsString;
use std::fmt::Display;
use std::str::FromStr;

use clap::builder::PossibleValue;
use clap::ValueEnum;
use unicode_xid::UnicodeXID as _;

use super::CommandCompleter;

/// Completion support for built-in shells
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
    Powershell,
    /// Z `SHell` (zsh)
    Zsh,
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
    pub fn from_shell_path(path: impl AsRef<std::path::Path>) -> Option<Shell> {
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
    /// let shell = Shell::from_env();
    /// println!("{shell:?}");
    /// ```
    pub fn from_env() -> Option<Shell> {
        if let Some(env_shell) = std::env::var_os("SHELL") {
            Shell::from_shell_path(env_shell)
        } else {
            None
        }
    }

    fn completer(&self) -> &dyn CommandCompleter {
        match self {
            Self::Bash => &Bash,
            Self::Elvish => &Elvish,
            Self::Fish => &Fish,
            Self::Powershell => &Powershell,
            Self::Zsh => &Zsh,
        }
    }
}

impl CommandCompleter for Shell {
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
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        self.completer().write_complete(cmd, args, current_dir, buf)
    }
}

// use a separate function to avoid having to monomorphize the entire function due
// to from_shell_path being generic
fn parse_shell_from_path(path: &std::path::Path) -> Option<Shell> {
    let name = path.file_stem()?.to_str()?;
    match name {
        "bash" => Some(Shell::Bash),
        "elvish" => Some(Shell::Elvish),
        "fish" => Some(Shell::Fish),
        "powershell" | "powershell_ise" => Some(Shell::Powershell),
        "zsh" => Some(Shell::Zsh),
        _ => None,
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
        Err(format!("invalid variant: {s}"))
    }
}

// Hand-rolled so it can work even when `derive` feature is disabled
impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::Powershell,
            Shell::Zsh,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Shell::Bash => PossibleValue::new("bash"),
            Shell::Elvish => PossibleValue::new("elvish"),
            Shell::Fish => PossibleValue::new("fish"),
            Shell::Powershell => PossibleValue::new("powershell"),
            Shell::Zsh => PossibleValue::new("zsh"),
        })
    }
}

/// Bash completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Bash;

impl CommandCompleter for Bash {
    fn write_registration(
        &self,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let escaped_name = name.replace('-', "_");
        debug_assert!(
            escaped_name.chars().all(|c| c.is_xid_continue()),
            "`name` must be an identifier, got `{escaped_name}`"
        );
        let mut upper_name = escaped_name.clone();
        upper_name.make_ascii_uppercase();

        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        let script = r#"
_clap_complete_NAME() {
    export IFS=$'\013'
    export _CLAP_COMPLETE_INDEX=${COMP_CWORD}
    export _CLAP_COMPLETE_COMP_TYPE=${COMP_TYPE}
    if compopt +o nospace 2> /dev/null; then
        export _CLAP_COMPLETE_SPACE=false
    else
        export _CLAP_COMPLETE_SPACE=true
    fi
    COMPREPLY=( $("COMPLETER" complete bash -- "${COMP_WORDS[@]}") )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -o nospace -o bashdefault -o nosort -F _clap_complete_NAME BIN
else
    complete -o nospace -o bashdefault -F _clap_complete_NAME BIN
fi
"#
        .replace("NAME", &escaped_name)
        .replace("BIN", bin)
        .replace("COMPLETER", &completer)
        .replace("UPPER", &upper_name);

        writeln!(buf, "{script}")?;
        Ok(())
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index: usize = std::env::var("_CLAP_COMPLETE_INDEX")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let _comp_type: CompType = std::env::var("_CLAP_COMPLETE_COMP_TYPE")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let _space: Option<bool> = std::env::var("_CLAP_COMPLETE_SPACE")
            .ok()
            .and_then(|i| i.parse().ok());
        let ifs: Option<String> = std::env::var("IFS").ok().and_then(|i| i.parse().ok());
        let completions = crate::dynamic::complete(cmd, args, index, current_dir)?;

        for (i, candidate) in completions.iter().enumerate() {
            if i != 0 {
                write!(buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(buf, "{}", candidate.get_content().to_string_lossy())?;
        }
        Ok(())
    }
}

/// Type of completion attempted that caused a completion function to be called
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
enum CompType {
    /// Normal completion
    Normal,
    /// List completions after successive tabs
    Successive,
    /// List alternatives on partial word completion
    Alternatives,
    /// List completions if the word is not unmodified
    Unmodified,
    /// Menu completion
    Menu,
}

impl FromStr for CompType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "9" => Ok(Self::Normal),
            "63" => Ok(Self::Successive),
            "33" => Ok(Self::Alternatives),
            "64" => Ok(Self::Unmodified),
            "37" => Ok(Self::Menu),
            _ => Err(format!("unsupported COMP_TYPE `{}`", s)),
        }
    }
}

impl Default for CompType {
    fn default() -> Self {
        Self::Normal
    }
}

/// Elvish completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Elvish;

impl CommandCompleter for Elvish {
    fn write_registration(
        &self,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::try_quote(bin).unwrap_or(std::borrow::Cow::Borrowed(bin));
        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        let script = r#"
set edit:completion:arg-completer[BIN] = { |@words|
    set E:_CLAP_IFS = "\n"

    var index = (count $words)
    set index = (- $index 1)
    set E:_CLAP_COMPLETE_INDEX = (to-string $index)

    put (COMPLETER complete elvish -- $@words) | to-lines
}
"#
        .replace("COMPLETER", &completer)
        .replace("BIN", &bin);

        writeln!(buf, "{script}")?;
        Ok(())
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index: usize = std::env::var("_CLAP_COMPLETE_INDEX")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let ifs: Option<String> = std::env::var("_CLAP_IFS").ok().and_then(|i| i.parse().ok());
        let completions = crate::dynamic::complete(cmd, args, index, current_dir)?;

        for (i, candidate) in completions.iter().enumerate() {
            if i != 0 {
                write!(buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(buf, "{}", candidate.get_content().to_string_lossy())?;
        }
        Ok(())
    }
}

/// Fish completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Fish;

impl CommandCompleter for Fish {
    fn write_registration(
        &self,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::try_quote(bin).unwrap_or(std::borrow::Cow::Borrowed(bin));
        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        writeln!(
            buf,
            r#"complete -x -c {bin} -a "("'{completer}'" complete fish -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))""#
        )
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index = args.len() - 1;
        let completions = crate::dynamic::complete(cmd, args, index, current_dir)?;

        for candidate in completions {
            write!(buf, "{}", candidate.get_content().to_string_lossy())?;
            if let Some(help) = candidate.get_help() {
                write!(
                    buf,
                    "\t{}",
                    help.to_string().lines().next().unwrap_or_default()
                )?;
            }
            writeln!(buf)?;
        }
        Ok(())
    }
}

/// Powershell completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Powershell;

impl CommandCompleter for Powershell {
    fn write_registration(
        &self,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::try_quote(bin).unwrap_or(std::borrow::Cow::Borrowed(bin));
        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        writeln!(
            buf,
            r#"
Register-ArgumentCompleter -Native -CommandName {bin} -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $results = Invoke-Expression "&{completer} complete powershell -- $($commandAst.ToString())";
    $results | ForEach-Object {{
        $split = $_.Split("`t");
        $cmd = $split[0];

        if ($split.Length -eq 2) {{
            $help = $split[1];
        }}
        else {{
            $help = $split[0];
        }}

        [System.Management.Automation.CompletionResult]::new($cmd, $cmd, 'ParameterValue', $help)
    }}
}};
        "#
        )
    }

    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index = args.len() - 1;
        let completions = crate::dynamic::complete(cmd, args, index, current_dir)?;

        for candidate in completions {
            write!(buf, "{}", candidate.get_content().to_string_lossy())?;
            if let Some(help) = candidate.get_help() {
                write!(
                    buf,
                    "\t{}",
                    help.to_string().lines().next().unwrap_or_default()
                )?;
            }
            writeln!(buf)?;
        }
        Ok(())
    }
}

/// Zsh completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Zsh;

impl CommandCompleter for Zsh {
    fn write_registration(
        &self,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::try_quote(bin).unwrap_or(std::borrow::Cow::Borrowed(bin));
        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        let script = r#"#compdef BIN
function _clap_dynamic_completer() {
    export _CLAP_COMPLETE_INDEX=$(expr $CURRENT - 1)
    export _CLAP_IFS=$'\n'

    local completions=("${(@f)$(COMPLETER complete zsh -- ${words} 2>/dev/null)}")

    if [[ -n $completions ]]; then
        compadd -a completions
    fi
}

compdef _clap_dynamic_completer BIN"#
            .replace("COMPLETER", &completer)
            .replace("BIN", &bin);

        writeln!(buf, "{script}")?;
        Ok(())
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index: usize = std::env::var("_CLAP_COMPLETE_INDEX")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let ifs: Option<String> = std::env::var("_CLAP_IFS").ok().and_then(|i| i.parse().ok());

        // If the current word is empty, add an empty string to the args
        let mut args = args.clone();
        if args.len() == index {
            args.push("".into());
        }
        let completions = crate::dynamic::complete(cmd, args, index, current_dir)?;

        for (i, candidate) in completions.iter().enumerate() {
            if i != 0 {
                write!(buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(buf, "{}", candidate.get_content().to_string_lossy())?;
        }
        Ok(())
    }
}
