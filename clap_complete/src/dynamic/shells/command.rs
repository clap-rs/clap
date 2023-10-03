use super::{bash, fish};
use crate::dynamic::registrar::Registrar;
use std::io::Write as _;

#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum CompleteCommand {
    /// Complete a command for a given shell, to be called from autocomplete
    /// scripts primarily.
    Complete(CompleteArgs),
    /// Generate shell completions for this program
    Generate(GenerateArgs),
}

#[allow(missing_docs)]
#[derive(clap::Args, Clone, Debug)]
pub struct CompleteArgs {
    #[command(subcommand)]
    command: CompleteShellCommands,
}

#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum CompleteShellCommands {
    Bash(bash::complete::BashCompleteArgs),
    Fish(fish::complete::FishCompleteArgs),
}

#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum GenerateShellCommands {
    Bash(bash::generate::BashGenerateArgs),
    Fish(fish::generate::FishGenerateArgs),
}

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct GenerateArgs {
    /// Path to write completion-registration to.
    #[arg(long, short = 'o', default_value = "-")]
    output: std::path::PathBuf,

    #[command(subcommand)]
    command: GenerateShellCommands,
}

impl CompleteCommand {
    /// Process the completion request
    pub fn complete(&self, cmd: &mut clap::Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_run: {self:?}");
        match self {
            CompleteCommand::Complete(args) => match args.command {
                CompleteShellCommands::Bash(ref args) => args.try_complete(cmd),
                CompleteShellCommands::Fish(ref args) => args.try_complete(cmd),
            },
            CompleteCommand::Generate(args) => {
                let mut buf = Vec::new();
                let name = cmd.get_name();
                let bin = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());

                if args.output.is_dir() {
                    return Err(clap::error::Error::raw(
                        clap::error::ErrorKind::InvalidValue,
                        "output is a directory",
                    ));
                }

                match args.command {
                    GenerateShellCommands::Bash(ref args) => {
                        // TODO Figure out what to pass for complter, just assuming bin now.
                        args.write_registration(name, bin, bin, &mut buf)?
                    }
                    GenerateShellCommands::Fish(ref args) => {
                        args.write_registration(name, bin, bin, &mut buf)?
                    }
                }

                if args.output == std::path::Path::new("-") {
                    std::io::stdout().write_all(&buf)?;
                } else {
                    std::fs::write(args.output.as_path(), buf)?;
                }

                Ok(())
            }
        }
    }
}
