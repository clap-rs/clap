// Std
use std::{ffi::{OsStr, OsString}, slice::Iter, iter::Cloned};

// TODO: Maybe make this public?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValueType {
    EnvVariable,
    CommandLine,
    DefaultValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MatchedArg {
    pub(crate) occurs: u64,
    pub(crate) ty: ValueType,
    indices: Vec<usize>,
    pub(crate) vals: Vec<OsString>,
}

impl MatchedArg {
    pub(crate) fn new(ty: ValueType) -> Self {
        MatchedArg {
            occurs: 0,
            ty,
            indices: Vec::new(),
            vals: Vec::new(),
        }
    }

    pub(crate) fn indices<'a>(&'a self) -> Cloned<Iter<'a, usize>> {
        self.indices.iter().cloned()
    }

    pub(crate) fn get_index(&self, index: usize) -> Option<usize> {
        self.indices.get(index).cloned()
    }

    pub(crate) fn push_index(&mut self, index: usize) {
        self.indices.push(index)
    }

    pub(crate) fn contains_val(&self, val: &str) -> bool {
        self.vals
            .iter()
            .any(|v| OsString::as_os_str(v) == OsStr::new(val))
    }
}
