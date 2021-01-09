// Std
use std::{
    ffi::{OsStr, OsString},
    iter::{Cloned, Flatten},
    slice::Iter,
};

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
    vals: Vec<Vec<OsString>>,
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

    pub(crate) fn indices(&self) -> Cloned<Iter<'_, usize>> {
        self.indices.iter().cloned()
    }

    pub(crate) fn get_index(&self, index: usize) -> Option<usize> {
        self.indices.get(index).cloned()
    }

    pub(crate) fn push_index(&mut self, index: usize) {
        self.indices.push(index)
    }

    pub(crate) fn vals(&self) -> Iter<Vec<OsString>> {
        self.vals.iter()
    }

    pub(crate) fn vals_flatten(&self) -> Flatten<Iter<Vec<OsString>>> {
        self.vals.iter().flatten()
    }

    pub(crate) fn get_val(&self, index: usize) -> Option<&OsString> {
        self.vals.get(0)?.get(index)
    }

    pub(crate) fn push_val(&mut self, val: OsString) {
        self.vals.push(vec![val])
    }

    pub(crate) fn append_val(&mut self, val: OsString) {
        if let Some(val_last) = self.vals.last_mut() {
            val_last.push(val);
        } else {
            // Fall back to push val when no val
            self.push_val(val);
        }
    }

    pub(crate) fn push_vals(&mut self, vals: Vec<OsString>) {
        self.vals.push(vals)
    }

    pub(crate) fn num_vals(&self) -> usize {
        self.vals.len()
    }

    pub(crate) fn no_val(&self) -> bool {
        self.vals.is_empty()
    }

    pub(crate) fn remove_vals(&mut self, len: usize) {
        self.vals.drain(0..len);
    }

    pub(crate) fn contains_val(&self, val: &str) -> bool {
        self.vals_flatten()
            .any(|v| OsString::as_os_str(v) == OsStr::new(val))
    }
}
