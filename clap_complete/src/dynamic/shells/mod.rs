//! Shell support

mod bash;
mod fish;
mod shell;

pub use bash::*;
pub use fish::*;
pub use shell::*;

use std::ffi::OsString;
use std::io::Write as _;

use crate::dynamic::Completer as _;

/// A subcommand definition to `flatten` into your CLI
///
/// This provides a one-stop solution for integrating completions into your CLI
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
