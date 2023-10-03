use crate::dynamic::complete::complete;

use super::comp_type::CompType;
use std::ffi::OsString;
use std::io::Write;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct BashCompleteArgs {
    /// `COMP_CWORD` environment variable from Bash.
    ///
    /// An index into ${COMP_WORDS} of the word containing the current cursor
    /// position.  This variable is available only in shell functions invoked by
    /// the programmable completion facilities.
    ///
    /// Quoted from the bash man pages:
    /// https://man7.org/linux/man-pages/man1/bash.1.html.
    #[arg(
        long,
        required = true,
        value_name = "COMP_CWORD",
        hide_short_help = true
    )]
    index: Option<usize>,

    /// `IFS` environment variable from Bash.
    ///
    /// The Internal Field Separator that is used for word splitting after
    /// expansion and to split lines into words with the read builtin command.
    /// The default value is `<space><tab><newline>`.
    ///
    /// Quoted from the bash man pages:
    /// https://man7.org/linux/man-pages/man1/bash.1.html.
    #[arg(long, hide_short_help = true)]
    ifs: Option<String>,

    /// `COMP_TYPE` environment variable from Bash.
    ///
    /// Set to an integer value corresponding to the type of completion
    /// attempted that caused a completion function to be called: TAB, for
    /// normal completion, ?, for listing completions after successive tabs, !,
    /// for listing alternatives on partial word completion, @, to list
    /// completions if the word is not unmodified, or %, for menu completion.
    /// This variable is available only in shell functions and external commands
    /// invoked by the programmable completion facilities.
    ///
    /// Quoted from the bash man pages:
    /// https://man7.org/linux/man-pages/man1/bash.1.html.
    #[arg(long = "type", required = true, hide_short_help = true)]
    comp_type: Option<CompType>,

    /// Disable the `nospace` options from `complete` in Bash.
    ///
    /// See https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion-Builtins.html.
    #[arg(long, hide_short_help = true)]
    space: bool,

    /// Enable the `nospace` options from `complete` in Bash.
    ///
    /// See https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion-Builtins.html.
    #[arg(long, conflicts_with = "space", hide_short_help = true)]
    no_space: bool,

    /// `COMP_WORDS` environment variable from Bash.
    ///
    /// An array variable (see Arrays below) consisting of the individual words
    /// in the current command line.  The line is split into words as readline
    /// would split it, using COMP_WORDBREAKS as described above.  This variable
    /// is available only in shell functions invoked by the programmable
    /// completion facilities.
    ///
    /// Quoted from the bash man pages:
    /// https://man7.org/linux/man-pages/man1/bash.1.html.
    #[arg(raw = true, hide_short_help = true)]
    comp_words: Vec<OsString>,
}

impl BashCompleteArgs {
    /// Process the completion request
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        let index = self.index.unwrap_or_default();
        // let _comp_type = self.comp_type.unwrap_or_default();
        // let _space = match (self.space, self.no_space) {
        //     (true, false) => Some(true),
        //     (false, true) => Some(false),
        //     (true, true) => {
        //         unreachable!("`--space` and `--no-space` set, clap should prevent this")
        //     }
        //     (false, false) => None,
        // }
        // .unwrap();
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
