// Std
use std::ffi::{OsStr, OsString};

#[derive(Debug, Clone)]
pub struct MatchedArg {
    pub(crate) occurs: u64,
    pub(crate) indices: Vec<usize>,
    pub(crate) vals: Vec<OsString>,
}

impl Default for MatchedArg {
    fn default() -> Self {
        MatchedArg {
            occurs: 1,
            indices: Vec::new(),
            vals: Vec::new(),
        }
    }
}

impl MatchedArg {
    pub fn new() -> Self {
        MatchedArg::default()
    }
    pub(crate) fn contains_val(&self, val: &str) -> bool {
        self.vals
            .iter()
            .map(OsString::as_os_str)
            .any(|v| v == OsStr::new(val))
    }
}
