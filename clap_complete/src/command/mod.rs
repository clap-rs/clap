//! [`<bin> complete`][CompleteCommand] completion integration
//!
//! - If you aren't using a subcommand, see [`CompleteCommand`]
//! - If you are using subcommands, see [`CompleteArgs`]
//!
//! To source your completions:
//!
//! **WARNING:** We recommend re-sourcing your completions on upgrade.
//! These completions work by generating shell code that calls into `your_program` while completing.
//! That interface is unstable and a mismatch between the shell code and `your_program` may result
//! in either invalid completions or no completions being generated.
//! For this reason, we recommend generating the shell code anew on shell startup so that it is
//! "self-correcting" on shell launch, rather than writing the generated completions to a file.
//!
//! Bash
//! ```bash
//! echo "source <(your_program complete bash)" >> ~/.bashrc
//! ```
//!
//! Elvish
//! ```elvish
//! echo "eval (your_program complete elvish)" >> ~/.elvish/rc.elv
//! ```
//!
//! Fish
//! ```fish
//! echo "source (your_program complete fish | psub)" >> ~/.config/fish/config.fish
//! ```
//!
//! Powershell
//! ```powershell
//! echo "your_program complete powershell | Invoke-Expression" >> $PROFILE
//! ```
//!
//! Zsh
//! ```zsh
//! echo "source <(your_program complete zsh)" >> ~/.zshrc
//! ```

mod shells;

use std::ffi::OsString;
use std::io::Write as _;

pub use shells::*;

/// A completion subcommand to add to your CLI
///
/// To customize completions, see
/// - [`ValueHint`][crate::ValueHint]
/// - [`ValueEnum`][clap::ValueEnum]
/// - [`ArgValueCompleter`][crate::ArgValueCompleter]
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
/// use clap_complete::CompleteCommand;
///
/// #[derive(Parser, Debug)]
/// #[clap(name = "dynamic", about = "A dynamic command line tool")]
/// struct Cli {
///     /// The subcommand to run complete
///     #[command(subcommand)]
///     complete: Option<CompleteCommand>,
///
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
/// To customize completions, see
/// - [`ValueHint`][crate::ValueHint]
/// - [`ValueEnum`][clap::ValueEnum]
/// - [`ArgValueCompleter`][crate::ArgValueCompleter]
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
/// use clap_complete::CompleteArgs;
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

        let shell = self.shell.or_else(Shell::from_env).ok_or_else(|| {
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
    /// Register for completions
    ///
    /// Write the `buf` the logic needed for calling into `<cmd> complete`, passing needed
    /// arguments to [`CommandCompleter::write_complete`] through the environment.
    ///
    /// **WARNING:** There are no stability guarantees between the call to
    /// [`CommandCompleter::write_complete`] that this generates and actually calling [`CommandCompleter::write_complete`].
    /// Caching the results of this call may result in invalid or no completions to be generated.
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
