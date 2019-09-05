use crate::build::Alias;

#[derive(Default)]
pub struct Key<'help> {
    kind: KeyKind<'help>,
}

#[derive(Default)]
pub struct SwitchData<'help> {
    longs: Vec<Alias<'help>>,
    short: Option<char>,
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
    fn set_short(&mut self, c: char) { self.switch_mut().short = Some(c); }
    fn add_long(&mut self, l: &'help str) {
        self.switch_mut()
            .longs
            .push(Alias::visible(l.trim_left_matches(|c| c == '-')));
    }
    fn add_longs(&mut self, longs: &[&'help str]) {
        for l in longs {
            self.long(l);
        }
    }
    fn add_hidden_long(&mut self, l: &'help str) {
        self.switch_mut()
            .longs
            .push(Alias::hidden(l.trim_left_matches(|c| c == '-')));
    }
    fn add_hidden_longs(&mut self, longs: &[&'help str]) {
        for l in longs {
            self.switch_mut()
                .longs
                .push(Alias::hidden(l.trim_left_matches(|c| c == '-')));
        }
    }
    fn set_index(&mut self, i: usize) {
        assert!(i > 0, "Argument index cannot be zero (0)");
        self.kind = KeyKind::Index(i);
    }
    fn has_index(&self) -> bool { self.index().is_some() }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Index`
    fn index(&self) -> usize {
        use KeyKind::*;
        match self.kind {
            Index(i) => i,
            _ => panic!("Argument is not positional"),
        }
    }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Index`
    fn index_mut(&mut self) -> usize {
        use KeyKind::*;
        match *self {
            Index(i) => i,
            _ => panic!("Argument is not positional"),
        }
    }
    fn has_switch(&self) -> bool {
        use KeyKind::*;
        match *self {
            Switch(_) => true,
            _ => false,
        }
    }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Switch`
    fn switch(&self) -> &SwitchData<'help> {
        use KeyKind::*;
        match *self {
            Switch(s) => s,
            _ => panic!("Argument has no switch"),
        }
    }
    /// # Panics
    ///
    /// Panics if `*self != KeyKind::Switch`
    fn switch_mut(&mut self) -> &mut SwitchData<'help> {
        use KeyKind::*;
        match *self {
            Switch(s) => s,
            _ => panic!("Argument has no switch"),
        }
    }
}
