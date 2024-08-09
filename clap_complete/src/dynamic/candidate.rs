use std::ffi::OsStr;
use std::ffi::OsString;

use clap::builder::StyledStr;

/// A completion candidate definition
///
/// This makes it easier to add more fields to completion candidate,
/// rather than using `(OsString, Option<StyledStr>)` or `(String, Option<StyledStr>)` to represent a completion candidate
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompletionCandidate {
    /// Main completion candidate content
    content: OsString,

    /// Help message with a completion candidate
    help: Option<StyledStr>,

    /// Whether the completion candidate is hidden
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
    pub fn hide(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

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

    /// Add a prefix to the content of completion candidate
    pub fn add_prefix(mut self, prefix: impl Into<OsString>) -> Self {
        let suffix = self.content;
        let mut content = prefix.into();
        content.push(&suffix);
        self.content = content;
        self
    }
}
