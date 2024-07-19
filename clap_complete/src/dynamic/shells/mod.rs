//! Shell support

mod bash;
mod elvish;
mod fish;
mod shell;
mod zsh;

pub use bash::*;
pub use elvish::*;
pub use fish::*;
pub use shell::*;
pub use zsh::*;

use std::ffi::OsString;
use std::io::Write as _;

use crate::dynamic::Completer as _;

/// A subcommand definition to `flatten` into your CLI
///
/// This provides a one-stop solution for integrating completions into your CLI
///
/// # Examples
///
/// The following example shows how to integrate completions into your CLI and generate completions.
///
/// 1. Build an application with derive API.
/// 2. Call `clap_complete::dynamic::shells::CompleteCommand::augment_subcommands` to add the `complete` subcommand into the application.
/// 3. Call `get_matches()`, or any of the other normal methods directly after.
///
/// For example:
///
/// ```no_run
/// // src/main.rs
/// use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
/// use clap_complete::dynamic::shells::CompleteCommand;
///
/// #[derive(Parser, Debug)]
/// #[clap(name = "dynamic", about = "A dynamic command line tool")]
/// struct Cli {
/// 	/// The subcommand to run complete
/// 	#[command(subcommand)]
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
///		}
///
/// 	// normal logic continues...
/// }
///```
///
/// # Usage for complete subcommand:
///
/// To generate shell completion scripts and source them, we can use the following command.
///
/// **NOTE**: If you have set a custom shell configuration file,
/// please remember to modify the redirection output file in the following command.
///
/// - Bash
/// 	```bash
/// 	echo "source <(your_program complete --shell bash --register -)" >> ~/.bashrc
/// 	```
///
/// - Fish
/// 	```fish
/// 	echo "source (your_program complete --shell fish --register - | psub)" >> ~/.config/fish/config.fish
/// 	```
///
/// - Zsh
/// 	```zsh
/// 	echo "source <(your_program complete --shell zsh --register -)" >> ~/.zshrc
/// 	```
///
/// - Elvish
/// 	```elvish
/// 	echo "eval (your_program complete --shell elvish --register -)" >> ~/.elvish/rc.elv
/// 	```
///
/// - Powershell
/// 	```powershell
/// 	echo "your_program complete --shell powershell --register - | Invoke-Expression" >> $PROFILE
/// 	```
///
#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[command(about = None, long_about = None)]
pub enum CompleteCommand {
    /// Register shell completions for this program
    #[command(hide = true)]
    Complete(CompleteArgs),
}

/// Generally used via [`CompleteCommand`]
#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
#[command(group = clap::ArgGroup::new("complete").multiple(true).conflicts_with("register"))]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[command(about = None, long_about = None)]
pub struct CompleteArgs {
    /// Specify shell to complete for
    #[arg(long)]
    shell: Shell,

    /// Path to write completion-registration to
    #[arg(long, required = true)]
    register: Option<std::path::PathBuf>,

    #[arg(raw = true, hide_short_help = true, group = "complete")]
    comp_words: Vec<OsString>,
}

impl CompleteCommand {
    /// Process the completion request
    pub fn complete(&self, cmd: &mut clap::Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_complete: {self:?}");
        let CompleteCommand::Complete(args) = self;
        if let Some(out_path) = args.register.as_deref() {
            let mut buf = Vec::new();
            let name = cmd.get_name();
            let bin = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
            args.shell.write_registration(name, bin, bin, &mut buf)?;
            if out_path == std::path::Path::new("-") {
                std::io::stdout().write_all(&buf)?;
            } else if out_path.is_dir() {
                let out_path = out_path.join(args.shell.file_name(name));
                std::fs::write(out_path, buf)?;
            } else {
                std::fs::write(out_path, buf)?;
            }
        } else {
            let current_dir = std::env::current_dir().ok();

            let mut buf = Vec::new();
            args.shell.write_complete(
                cmd,
                args.comp_words.clone(),
                current_dir.as_deref(),
                &mut buf,
            )?;
            std::io::stdout().write_all(&buf)?;
        }

        Ok(())
    }
}
