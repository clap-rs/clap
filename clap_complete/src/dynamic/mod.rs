//! Complete commands within shells

pub mod bash;

use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use clap::{Args, Command, Parser, Subcommand, ValueEnum};

use bash::Bash;

#[derive(Parser, Clone, Debug)]
#[command(hide = true)]
/// Subcommand to trigger completions
///
/// To add to a [`Command`] either:
/// - use [`Subcommand::augment_subcommands`]
/// - use `#[command(flatten)]` when adding to an enum deriving [`Subcommand`]
///
/// Afterwards completions can be manually triggered by calling [`CompleteCommand::complete`].
pub enum CompleteCommand {
    /// Register shell completions for this program
    #[command(subcommand)]
    Complete(CompleteShell),
}

impl CompleteCommand {
    /// Process the completion request, exit on errors
    pub fn complete(self, cmd: &mut Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request, return errors
    pub fn try_complete(self, cmd: &mut Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_complete: {:?}", self);
        let CompleteCommand::Complete(complete) = self;
        match complete {
            CompleteShell::Bash(args) => <Bash as Completer>::try_complete(args, cmd),
            CompleteShell::Register(RegisterArgs { path, shell }) => {
                let script = shell.completion_script(cmd);
                if path == Path::new("-") {
                    io::stdout().write_all(&script)?;
                } else if path.is_dir() {
                    let path = path.join(shell.file_name(cmd.get_name()));
                    std::fs::write(path, script)?;
                } else {
                    std::fs::write(path, script)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
#[command(hide = true)]
#[allow(missing_docs)]
// Subcommand for all the shells, so each can have their own options
pub enum CompleteShell {
    Bash(bash::CompleteArgs),
    /// Only exception is Register, which outputs the completion script for a shell
    Register(RegisterArgs),
}

#[derive(Args, Clone, Debug)]
/// Arguments for registering dynamic completions
pub struct RegisterArgs {
    /// Path to write completion-registration to
    #[arg(long, short)]
    path: PathBuf,
    /// Shell to generate completions for
    #[arg(long, short)]
    shell: Shell,
}

/// Shell with dynamic completion available.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,
}

impl Shell {
    /// Return completion script
    fn completion_script(&self, cmd: &mut Command) -> Vec<u8> {
        match self {
            Shell::Bash => Bash::completion_script(cmd),
        }
    }
    /// The recommended file name for the registration code
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => Bash::file_name(name),
        }
    }
}

/// dynamic completions
pub trait Completer {
    /// Arguments used by the shells dynamic completions
    type CompleteArgs: Args;
    /// Return completion script
    fn completion_script(cmd: &mut Command) -> Vec<u8>;
    /// The recommended file name for the registration code
    fn file_name(name: &str) -> String;
    // TODO maybe also have a function returning the expected file path for SYSTEM/USER
    // installation e.g. for fish /etc/fish/completions and ~/.config/fish/completions/
    /// Process the completion request
    fn try_complete(args: Self::CompleteArgs, cmd: &mut Command) -> clap::error::Result<()>;
}
