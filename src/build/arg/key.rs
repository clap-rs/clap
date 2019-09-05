// Third party
use bstr::{ByteSlice, B};

// Internal
use crate::build::Alias;

#[derive(Default)]
pub struct Key<'help> {
    kind: KeyKind<'help>,
}

#[derive(Default)]
pub struct SwitchData<'help> {
    l: Vec<Alias<'help>>,
    s: Option<char>,
}

impl<'help> SwitchData<'help> {
    pub fn short(&self) -> Option<char> { self.s }
    pub fn all_longs(&self) -> impl Iterator<Item = &str> { self.l.iter().map(|a| a.name) }
    pub fn has_longs(&self) -> bool { !self.l.is_empty() }
}

pub(crate) enum KeyKind<'help> {
    Unset,
    Index(usize),
    Switch(SwitchData<'help>),
}

impl<'help> Default for KeyKind<'help> {
    fn default() -> Self { KeyKind::Unset }
}

impl<'help> Key<'help> {
    pub fn has_long(&self) -> bool { self.has_switch() && self.switch().has_longs() }
    pub fn has_short(&self) -> bool { self.has_switch() && self.switch().short().is_some() }
    pub fn set_short(&mut self, c: char) { self.switch_mut().s = Some(c); }
    pub fn add_long(&mut self, l: &'help str) {
        self.switch_mut()
            .l
            .push(Alias::visible(B(l).trim_start_with(|c| c == '-')));
    }
    pub fn add_longs(&mut self, longs: &[&'help str]) {
        for l in longs {
            self.long(l);
        }
    }
    pub fn add_hidden_long(&mut self, l: &'help str) {
        self.switch_mut()
            .l
            .push(Alias::hidden(l.trim_left_matches(|c| c == '-')));
    }
    pub fn add_hidden_longs(&mut self, longs: &[&'help str]) {
        for l in longs {
            self.switch_mut()
                .l
                .push(Alias::hidden(l.trim_left_matches(|c| c == '-')));
        }
    }
    pub fn set_index(&mut self, i: usize) {
        assert!(i > 0, "Argument index cannot be zero (0)");
        self.kind = KeyKind::Index(i);
    }
    pub fn has_index(&self) -> bool { self.index().is_some() }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Index`
    pub fn index(&self) -> usize {
        use KeyKind::*;
        match self.kind {
            Index(i) => i,
            _ => panic!("Argument is not positional"),
        }
    }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Index`
    pub fn index_mut(&mut self) -> usize {
        use KeyKind::*;
        match *self {
            Index(i) => i,
            _ => panic!("Argument is not positional"),
        }
    }
    pub fn has_switch(&self) -> bool {
        use KeyKind::*;
        match *self {
            Switch(_) => true,
            _ => false,
        }
    }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Switch`
    pub fn switch(&self) -> &SwitchData<'help> {
        use KeyKind::*;
        match *self {
            Switch(s) => s,
            _ => panic!("Argument has no switch"),
        }
    }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Switch`
    pub fn switch_mut(&mut self) -> &mut SwitchData<'help> {
        use KeyKind::*;
        match *self {
            Switch(s) => s,
            _ => panic!("Argument has no switch"),
        }
    }
}
