// Std
use std::ffi::{OsStr, OsString};

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct MatchedArg {
    #[doc(hidden)]
    pub occurs: u64,
    #[doc(hidden)]
    pub indices: Vec<usize>,
    #[doc(hidden)]
    pub vals: Vec<OsString>,
    // This contains the positions in `vals` at which each occurrence starts.  The first occurrence
    // is implicitely starting at position `0` so that `occurrences` is empty in the common case of
    // a single occurrence.
    #[doc(hidden)]
    pub occurrences: Vec<usize>,
}

impl Default for MatchedArg {
    fn default() -> Self {
        MatchedArg {
            occurs: 1,
            indices: Vec::new(),
            vals: Vec::new(),
            occurrences: Vec::new(),
        }
    }
}

impl MatchedArg {
    pub fn new() -> Self { MatchedArg::default() }
    pub(crate) fn contains_val(&self, val: &str) -> bool {
        self.vals
            .iter()
            .map(|v| v.as_os_str())
            .any(|v| v == OsStr::new(val))
    }
}
