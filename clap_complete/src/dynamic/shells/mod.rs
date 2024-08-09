//! Shell completion support, see [`CompleteCommand`] for more details

mod bash;
mod elvish;
mod fish;
mod powershell;
mod shell;
mod zsh;

pub use bash::*;
pub use elvish::*;
pub use fish::*;
pub use powershell::*;
pub use shell::*;
pub use zsh::*;

use std::ffi::OsString;
use std::io::Write as _;

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
/// echo "source <(your_program complete --shell bash --register -)" >> ~/.bashrc
/// ```
///
/// Elvish
/// ```elvish
/// echo "eval (your_program complete --shell elvish --register -)" >> ~/.elvish/rc.elv
/// ```
///
/// Fish
/// ```fish
/// echo "source (your_program complete --shell fish --register - | psub)" >> ~/.config/fish/config.fish
/// ```
///
/// Powershell
/// ```powershell
/// echo "your_program complete --shell powershell --register - | Invoke-Expression" >> $PROFILE
/// ```
///
/// Zsh
/// ```zsh
/// echo "source <(your_program complete --shell zsh --register -)" >> ~/.zshrc
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
/// echo "source <(your_program complete --shell bash --register -)" >> ~/.bashrc
/// ```
///
/// Elvish
/// ```elvish
/// echo "eval (your_program complete --shell elvish --register -)" >> ~/.elvish/rc.elv
/// ```
///
/// Fish
/// ```fish
/// echo "source (your_program complete --shell fish --register - | psub)" >> ~/.config/fish/config.fish
/// ```
///
/// Powershell
/// ```powershell
/// echo "your_program complete --shell powershell --register - | Invoke-Expression" >> $PROFILE
/// ```
///
/// Zsh
/// ```zsh
/// echo "source <(your_program complete --shell zsh --register -)" >> ~/.zshrc
/// ```
#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
#[command(group = clap::ArgGroup::new("complete").multiple(true).conflicts_with("register"))]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[command(about = None, long_about = None)]
pub struct CompleteArgs {
    /// Specify shell to complete for
    #[arg(long)]
    shell: Option<Shell>,

    /// Path to write completion-registration to
    #[arg(long, required = true)]
    register: Option<std::path::PathBuf>,

    #[arg(raw = true, hide_short_help = true, group = "complete")]
    comp_words: Vec<OsString>,
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

        let shell = self
            .shell
            .or_else(|| Shell::from_env())
            .unwrap_or(Shell::Bash);

        if let Some(out_path) = self.register.as_deref() {
            let mut buf = Vec::new();
            let name = cmd.get_name();
            let bin = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
            shell.write_registration(name, bin, bin, &mut buf)?;
            if out_path == std::path::Path::new("-") {
                std::io::stdout().write_all(&buf)?;
            } else if out_path.is_dir() {
                let out_path = out_path.join(shell.file_name(name));
                std::fs::write(out_path, buf)?;
            } else {
                std::fs::write(out_path, buf)?;
            }
        } else {
            let current_dir = std::env::current_dir().ok();

            let mut buf = Vec::new();
            shell.write_complete(
                cmd,
                self.comp_words.clone(),
                current_dir.as_deref(),
                &mut buf,
            )?;
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
/// A `ShellCompleter` can choose how much of that lives within the registration script and or
/// lives in [`ShellCompleter::write_complete`].
pub trait ShellCompleter {
    /// The recommended file name for the registration code
    fn file_name(&self, name: &str) -> String;
    /// Register for completions
    ///
    /// Write the `buf` the logic needed for calling into `<cmd> complete`, passing needed
    /// arguments to [`ShellCompleter::write_complete`] through the environment.
    fn write_registration(
        &self,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error>;
    /// Complete the given command
    ///
    /// Adapt information from arguments and [`ShellCompleter::write_registration`]-defined env
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
