use std::{ffi::OsString, io::Write};

use crate::dynamic::complete::complete;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct FishCompleteArgs {
    // TODO Add clap stuff for --
    args: Vec<OsString>,
}

impl FishCompleteArgs {
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        let index = self.args.len() - 1;
        let completions = complete(
            cmd,
            self.args.clone(),
            index,
            std::env::current_dir().ok().as_deref(),
        )?;

        let mut buf = Vec::new();
        for (completion, help) in completions {
            write!(buf, "{}", completion.to_string_lossy())?;
            if let Some(help) = help {
                write!(
                    buf,
                    "\t{}",
                    help.to_string().lines().next().unwrap_or_default()
                )?;
            }
            writeln!(buf)?;
        }
        std::io::stdout().write_all(&buf)?;

        Ok(())
    }
}
