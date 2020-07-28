// Std
use std::ffi::{OsStr, OsString};

// TODO: Maybe make this public?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValueType {
    EnvVariable,
    CommandLine,
    DefaultValue,
}

#[derive(Debug, Clone)]
pub(crate) struct MatchedArg {
    pub(crate) occurs: u64,
    pub(crate) ty: ValueType,
    pub(crate) indices: Vec<usize>,
    pub(crate) vals: Vec<OsString>,
}

impl Default for MatchedArg {
    fn default() -> Self {
        MatchedArg {
            occurs: 1,
            ty: ValueType::CommandLine,
            indices: Vec::new(),
            vals: Vec::new(),
        }
    }
}

impl MatchedArg {
    pub(crate) fn new() -> Self {
        MatchedArg::default()
    }
    pub(crate) fn contains_val(&self, val: &str) -> bool {
        self.vals
            .iter()
            .any(|v| OsString::as_os_str(v) == OsStr::new(val))
    }
}
