use std::ffi::OsStr;
use std::ffi::OsString;

use clap::builder::StyledStr;

/// A shell-agnostic completion candidate
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompletionCandidate {
    content: OsString,
    help: Option<StyledStr>,
    hidden: bool,
}

impl CompletionCandidate {
    /// Create a new completion candidate
    pub fn new(content: impl Into<OsString>) -> Self {
        let content = content.into();
        Self {
            content,
            ..Default::default()
        }
    }

    /// Set the help message of the completion candidate
    pub fn help(mut self, help: Option<StyledStr>) -> Self {
        self.help = help;
        self
    }

    /// Set the visibility of the completion candidate
    ///
    /// Only shown when there is no visible candidate for completing the current argument.
    pub fn hide(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Add a prefix to the content of completion candidate
    ///
    /// This is generally used for post-process by [`complete`][crate::dynamic::complete()] for
    /// things like pre-pending flags, merging delimiter-separated values, etc.
    pub fn add_prefix(mut self, prefix: impl Into<OsString>) -> Self {
        let suffix = self.content;
        let mut content = prefix.into();
        content.push(&suffix);
        self.content = content;
        self
    }
}

/// Reflection API
impl CompletionCandidate {
    /// Get the content of the completion candidate
    pub fn get_content(&self) -> &OsStr {
        &self.content
    }

    /// Get the help message of the completion candidate
    pub fn get_help(&self) -> Option<&StyledStr> {
        self.help.as_ref()
    }

    /// Get the visibility of the completion candidate
    pub fn is_hide_set(&self) -> bool {
        self.hidden
    }
}
