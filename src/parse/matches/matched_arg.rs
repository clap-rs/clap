// Std
use std::{
    ffi::{OsStr, OsString},
    iter::{Cloned, Flatten},
    slice::Iter,
};

use crate::INTERNAL_ERROR_MSG;

// TODO: Maybe make this public?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValueType {
    Unknown,
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
    case_insensitive: bool,
}

impl Default for MatchedArg {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchedArg {
    pub(crate) fn new() -> Self {
        MatchedArg {
            occurs: 0,
            ty: ValueType::Unknown,
            indices: Vec::new(),
            vals: Vec::new(),
            case_insensitive: false,
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

    pub(crate) fn new_val_group(&mut self) {
        self.vals.push(vec![])
    }

    pub(crate) fn append_val(&mut self, val: OsString) {
        // We assume there is always a group created before.
        self.vals.last_mut().expect(INTERNAL_ERROR_MSG).push(val)
    }

    pub(crate) fn num_vals(&self) -> usize {
        self.vals.iter().flatten().count()
    }

    // Will be used later
    #[allow(dead_code)]
    pub(crate) fn num_vals_last_group(&self) -> usize {
        self.vals.last().map(|x| x.len()).unwrap_or(0)
    }

    pub(crate) fn is_vals_empty(&self) -> bool {
        self.vals.iter().flatten().count() == 0
    }

    // Will be used later
    #[allow(dead_code)]
    pub(crate) fn remove_vals(&mut self, len: usize) {
        let mut want_remove = len;
        let mut remove_group = None;
        let mut remove_val = None;
        for (i, g) in self.vals().enumerate() {
            if g.len() <= want_remove {
                want_remove -= g.len();
                remove_group = Some(i);
            } else {
                remove_val = Some(want_remove);
                break;
            }
        }
        if let Some(remove_group) = remove_group {
            self.vals.drain(0..=remove_group);
        }
        if let Some(remove_val) = remove_val {
            self.vals[0].drain(0..remove_val);
        }
    }

    pub(crate) fn override_vals(&mut self) {
        let len = self.vals.len();
        if len > 1 {
            self.vals.drain(0..len - 1);
        }
    }

    pub(crate) fn contains_val(&self, val: &str) -> bool {
        self.vals_flatten().any(|v| {
            if self.case_insensitive {
                // For rust v1.53.0 and above, can use
                // OsString.eq_ignore_ascii_case
                // (https://github.com/rust-lang/rust/pull/80193)
                v.to_string_lossy().to_lowercase() == val.to_lowercase()
            } else {
                OsString::as_os_str(v) == OsStr::new(val)
            }
        })
    }

    pub(crate) fn set_ty(&mut self, ty: ValueType) {
        self.ty = ty;
    }

    pub(crate) fn set_case_insensitive(&mut self, ci: bool) {
        self.case_insensitive = ci;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vals_override() {
        let mut m = MatchedArg::new();
        m.push_val("aaa".into());
        m.new_val_group();
        m.append_val("bbb".into());
        m.append_val("ccc".into());
        m.new_val_group();
        m.append_val("ddd".into());
        m.push_val("eee".into());
        m.new_val_group();
        m.append_val("fff".into());
        m.append_val("ggg".into());
        m.append_val("hhh".into());
        m.append_val("iii".into());

        m.override_vals();
        let vals: Vec<&Vec<OsString>> = m.vals().collect();
        assert_eq!(
            vals,
            vec![&vec![
                OsString::from("fff"),
                OsString::from("ggg"),
                OsString::from("hhh"),
                OsString::from("iii"),
            ]]
        );
        m.override_vals();

        let vals: Vec<&Vec<OsString>> = m.vals().collect();
        assert_eq!(
            vals,
            vec![&vec![
                OsString::from("fff"),
                OsString::from("ggg"),
                OsString::from("hhh"),
                OsString::from("iii"),
            ]]
        );
    }

    #[test]
    fn test_grouped_vals_push_and_append() {
        let mut m = MatchedArg::new();
        m.push_val("aaa".into());
        m.new_val_group();
        m.append_val("bbb".into());
        m.append_val("ccc".into());
        m.new_val_group();
        m.append_val("ddd".into());
        m.push_val("eee".into());
        m.new_val_group();
        m.append_val("fff".into());
        m.append_val("ggg".into());
        m.append_val("hhh".into());
        m.append_val("iii".into());

        let vals: Vec<&Vec<OsString>> = m.vals().collect();
        assert_eq!(
            vals,
            vec![
                &vec![OsString::from("aaa")],
                &vec![OsString::from("bbb"), OsString::from("ccc"),],
                &vec![OsString::from("ddd")],
                &vec![OsString::from("eee")],
                &vec![
                    OsString::from("fff"),
                    OsString::from("ggg"),
                    OsString::from("hhh"),
                    OsString::from("iii"),
                ]
            ]
        )
    }

    #[test]
    fn test_grouped_vals_removal() {
        let m = {
            let mut m = MatchedArg::new();
            m.push_val("aaa".into());
            m.new_val_group();
            m.append_val("bbb".into());
            m.append_val("ccc".into());
            m.new_val_group();
            m.append_val("ddd".into());
            m.push_val("eee".into());
            m.new_val_group();
            m.append_val("fff".into());
            m.append_val("ggg".into());
            m.append_val("hhh".into());
            m.append_val("iii".into());
            m
        };
        {
            let mut m = m.clone();
            m.remove_vals(0);
            let vals1 = m.vals().collect::<Vec<_>>();
            assert_eq!(
                vals1,
                vec![
                    &vec![OsString::from("aaa")],
                    &vec![OsString::from("bbb"), OsString::from("ccc"),],
                    &vec![OsString::from("ddd")],
                    &vec![OsString::from("eee")],
                    &vec![
                        OsString::from("fff"),
                        OsString::from("ggg"),
                        OsString::from("hhh"),
                        OsString::from("iii"),
                    ]
                ]
            );
        }

        {
            let mut m = m.clone();
            m.remove_vals(1);
            let vals0 = m.vals().collect::<Vec<_>>();
            assert_eq!(
                vals0,
                vec![
                    &vec![OsString::from("bbb"), OsString::from("ccc"),],
                    &vec![OsString::from("ddd")],
                    &vec![OsString::from("eee")],
                    &vec![
                        OsString::from("fff"),
                        OsString::from("ggg"),
                        OsString::from("hhh"),
                        OsString::from("iii"),
                    ]
                ]
            );
        }

        {
            let mut m = m.clone();
            m.remove_vals(2);
            let vals1 = m.vals().collect::<Vec<_>>();
            assert_eq!(
                vals1,
                vec![
                    &vec![OsString::from("ccc"),],
                    &vec![OsString::from("ddd")],
                    &vec![OsString::from("eee")],
                    &vec![
                        OsString::from("fff"),
                        OsString::from("ggg"),
                        OsString::from("hhh"),
                        OsString::from("iii"),
                    ]
                ]
            );
        }

        {
            let mut m = m.clone();
            m.remove_vals(3);
            let vals1 = m.vals().collect::<Vec<_>>();
            assert_eq!(
                vals1,
                vec![
                    &vec![OsString::from("ddd")],
                    &vec![OsString::from("eee")],
                    &vec![
                        OsString::from("fff"),
                        OsString::from("ggg"),
                        OsString::from("hhh"),
                        OsString::from("iii"),
                    ]
                ]
            );
        }

        {
            let mut m = m.clone();
            m.remove_vals(4);
            let vals1 = m.vals().collect::<Vec<_>>();
            assert_eq!(
                vals1,
                vec![
                    &vec![OsString::from("eee")],
                    &vec![
                        OsString::from("fff"),
                        OsString::from("ggg"),
                        OsString::from("hhh"),
                        OsString::from("iii"),
                    ]
                ]
            );
        }

        {
            let mut m = m.clone();
            m.remove_vals(5);
            let vals1 = m.vals().collect::<Vec<_>>();
            assert_eq!(
                vals1,
                vec![&vec![
                    OsString::from("fff"),
                    OsString::from("ggg"),
                    OsString::from("hhh"),
                    OsString::from("iii"),
                ]]
            );
        }

        {
            let mut m = m.clone();
            m.remove_vals(7);
            let vals1 = m.vals().collect::<Vec<_>>();
            assert_eq!(
                vals1,
                vec![&vec![OsString::from("hhh"), OsString::from("iii"),]]
            );
        }

        {
            let mut m = m.clone();
            m.remove_vals(9);
            let vals1: Vec<&Vec<OsString>> = m.vals().collect();
            assert!(vals1.is_empty());
        }
    }
}
