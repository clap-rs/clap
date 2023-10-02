use crate::dynamic::complete::complete;

use super::comp_type::CompType;
use std::ffi::OsString;
use std::io::Write;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct BashCompleteArgs {
    #[arg(
        long,
        required = true,
        value_name = "COMP_CWORD",
        hide_short_help = true,
        group = "complete"
    )]
    index: Option<usize>,

    #[arg(long, hide_short_help = true, group = "complete")]
    ifs: Option<String>,

    #[arg(
        long = "type",
        required = true,
        hide_short_help = true,
        group = "complete"
    )]
    comp_type: Option<CompType>,

    #[arg(long, hide_short_help = true, group = "complete")]
    space: bool,

    #[arg(
        long,
        conflicts_with = "space",
        hide_short_help = true,
        group = "complete"
    )]
    no_space: bool,

    #[arg(raw = true, hide_short_help = true, group = "complete")]
    comp_words: Vec<OsString>,
}

impl BashCompleteArgs {
    /// Process the completion request
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        let index = self.index.unwrap_or_default();
        let _comp_type = self.comp_type.unwrap_or_default();
        let _space = match (self.space, self.no_space) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (true, true) => {
                unreachable!("`--space` and `--no-space` set, clap should prevent this")
            }
            (false, false) => None,
        }
        .unwrap();
        let current_dir = std::env::current_dir().ok();
        let completions = complete(cmd, self.comp_words.clone(), index, current_dir.as_deref())?;

        let mut buf = Vec::new();
        for (i, (suggestion, _)) in completions.iter().enumerate() {
            if i != 0 {
                write!(&mut buf, "{}", self.ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(&mut buf, "{}", suggestion.to_string_lossy())?;
        }
        std::io::stdout().write_all(&buf)?;

        Ok(())
    }
}
