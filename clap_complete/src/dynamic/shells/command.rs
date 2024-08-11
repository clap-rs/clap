use std::ffi::OsString;
use std::io::Write as _;

use unicode_xid::UnicodeXID as _;

use super::Shell;

/// A completion subcommand to add to your CLI
///
/// If you aren't using a subcommand, you can annotate a field with this type as `#[command(subcommand)]`.
///
/// If you are using subcommands, see [`CompleteArgs`].
///
/// **Warning:** `stdout` should not be written to before [`CompleteCommand::complete`] has had a
/// chance to run.
///
/// # Examples
///
/// To integrate completions into an application without subcommands:
/// ```no_run
/// // src/main.rs
/// use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
/// use clap_complete::dynamic::CompleteCommand;
///
/// #[derive(Parser, Debug)]
/// #[clap(name = "dynamic", about = "A dynamic command line tool")]
/// struct Cli {
///     /// The subcommand to run complete
///     #[command(subcommand)]
///     complete: Option<CompleteCommand>,
///     /// Input file path
///     #[clap(short, long, value_hint = clap::ValueHint::FilePath)]
///     input: Option<String>,
///     /// Output format
///     #[clap(short = 'F', long, value_parser = ["json", "yaml", "toml"])]
///     format: Option<String>,
/// }
///
/// fn main() {
///     let cli = Cli::parse();
///     if let Some(completions) = cli.complete {
///         completions.complete(&mut Cli::command());
///     }
///
///     // normal logic continues...
/// }
///```
///
/// To source your completions:
///
/// Bash
/// ```bash
/// echo "source <(your_program complete bash --register -)" >> ~/.bashrc
/// ```
///
/// Elvish
/// ```elvish
/// echo "eval (your_program complete elvish --register -)" >> ~/.elvish/rc.elv
/// ```
///
/// Fish
/// ```fish
/// echo "source (your_program complete fish --register - | psub)" >> ~/.config/fish/config.fish
/// ```
///
/// Powershell
/// ```powershell
/// echo "your_program complete powershell --register - | Invoke-Expression" >> $PROFILE
/// ```
///
/// Zsh
/// ```zsh
/// echo "source <(your_program complete zsh --register -)" >> ~/.zshrc
/// ```
#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[command(about = None, long_about = None)]
pub enum CompleteCommand {
    /// Register shell completions for this program
    #[command(hide = true)]
    Complete(CompleteArgs),
}

impl CompleteCommand {
    /// Process the completion request and exit
    ///
    /// **Warning:** `stdout` should not be written to before this has had a
    /// chance to run.
    pub fn complete(&self, cmd: &mut clap::Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request
    ///
    /// **Warning:** `stdout` should not be written to before or after this has run.
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_complete: {self:?}");
        let CompleteCommand::Complete(args) = self;
        args.try_complete(cmd)
    }
}

/// A completion subcommand to add to your CLI
///
/// If you are using subcommands, add a `Complete(CompleteArgs)` variant.
///
/// If you aren't using subcommands, generally you will want [`CompleteCommand`].
///
/// **Warning:** `stdout` should not be written to before [`CompleteArgs::complete`] has had a
/// chance to run.
///
/// # Examples
///
/// To integrate completions into an application without subcommands:
/// ```no_run
/// // src/main.rs
/// use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
/// use clap_complete::dynamic::CompleteArgs;
///
/// #[derive(Parser, Debug)]
/// #[clap(name = "dynamic", about = "A dynamic command line tool")]
/// struct Cli {
///     #[command(subcommand)]
///     complete: Command,
/// }
///
/// #[derive(Subcommand, Debug)]
/// enum Command {
///     Complete(CompleteArgs),
///     Print,
/// }
///
/// fn main() {
///     let cli = Cli::parse();
///     match cli.complete {
///         Command::Complete(completions) => {
///             completions.complete(&mut Cli::command());
///         },
///         Command::Print => {
///             println!("Hello world!");
///         }
///     }
/// }
///```
///
/// To source your completions:
///
/// Bash
/// ```bash
/// echo "source <(your_program complete bash)" >> ~/.bashrc
/// ```
///
/// Elvish
/// ```elvish
/// echo "eval (your_program complete elvish)" >> ~/.elvish/rc.elv
/// ```
///
/// Fish
/// ```fish
/// echo "source (your_program complete fish | psub)" >> ~/.config/fish/config.fish
/// ```
///
/// Powershell
/// ```powershell
/// echo "your_program complete powershell | Invoke-Expression" >> $PROFILE
/// ```
///
/// Zsh
/// ```zsh
/// echo "source <(your_program complete zsh)" >> ~/.zshrc
/// ```
#[derive(clap::Args, Clone, Debug)]
#[command(about = None, long_about = None)]
pub struct CompleteArgs {
    /// Specify shell to complete for
    #[arg(value_name = "NAME")]
    shell: Option<Shell>,

    #[arg(raw = true, value_name = "ARG", hide = true)]
    comp_words: Option<Vec<OsString>>,
}

impl CompleteArgs {
    /// Process the completion request and exit
    ///
    /// **Warning:** `stdout` should not be written to before this has had a
    /// chance to run.
    pub fn complete(&self, cmd: &mut clap::Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request
    ///
    /// **Warning:** `stdout` should not be written to before or after this has run.
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_complete: {self:?}");

        let shell = self.shell.or_else(|| Shell::from_env()).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "unknown shell, please specify the name of your shell",
            )
        })?;

        if let Some(comp_words) = self.comp_words.as_ref() {
            let current_dir = std::env::current_dir().ok();

            let mut buf = Vec::new();
            shell.write_complete(cmd, comp_words.clone(), current_dir.as_deref(), &mut buf)?;
            std::io::stdout().write_all(&buf)?;
        } else {
            let name = cmd.get_name();
            let bin = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());

            let mut buf = Vec::new();
            shell.write_registration(name, bin, bin, &mut buf)?;
            std::io::stdout().write_all(&buf)?;
        }

        Ok(())
    }
}

/// Shell-integration for completions
///
/// This will generally be called by [`CompleteCommand`] or [`CompleteArgs`].
///
/// This handles adapting between the shell and [`completer`][crate::dynamic::complete()].
/// A `CommandCompleter` can choose how much of that lives within the registration script and or
/// lives in [`CommandCompleter::write_complete`].
pub trait CommandCompleter {
    /// The recommended file name for the registration code
    fn file_name(&self, name: &str) -> String;
    /// Register for completions
    ///
    /// Write the `buf` the logic needed for calling into `<cmd> complete`, passing needed
    /// arguments to [`CommandCompleter::write_complete`] through the environment.
    fn write_registration(
        &self,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error>;
    /// Complete the given command
    ///
    /// Adapt information from arguments and [`CommandCompleter::write_registration`]-defined env
    /// variables to what is needed for [`completer`][crate::dynamic::complete()].
    ///
    /// Write out the [`CompletionCandidate`][crate::dynamic::CompletionCandidate]s in a way the shell will understand.
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error>;
}

impl CommandCompleter for super::Bash {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.bash")
    }
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
if [[ \"${{BASH_VERSINFO[0]}}\" -eq 4 && \"${{BASH_VERSINFO[1]}}\" -ge 4 || \"${{BASH_VERSINFO[0]}}\" -gt 4 ]]; then
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

impl std::str::FromStr for CompType {
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
impl CommandCompleter for super::Elvish {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.elv")
    }
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

impl CommandCompleter for super::Fish {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.fish")
    }
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

impl CommandCompleter for super::Powershell {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.ps1")
    }

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

impl CommandCompleter for super::Zsh {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.zsh")
    }
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
