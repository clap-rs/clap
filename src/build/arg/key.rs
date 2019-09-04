use crate::INTERNAL_ERR_MSG;

#[derive(Default)]
struct SwitchData<'help> {
    longs: Vec<Alias<'help>>,
    short: Option<char>,
}

pub(crate) enum ArgKey<'help> {
    Unset,
    Index(usize),
    Switch(SwitchData),
}

impl Default for ArgKey<'help> {
    fn default() -> Self {
        ArgKey::Unset
    }
}

impl<'help> ArgKey {
    fn long(&mut self, l: &'help str) {
        match *self {
            ArgKey::Unset => *self = ArgKey::Switch(SwitchData::default()),
            ArgKey::Index => panic!("You cannot add both an index and switch (short or long) to an Arg"),
            _ => (),
        }
        self.switch_mut()
            .expect(INTERNAL_ERR_MSG)
            .longs
            .push(Alias::visible(l.trim_left_matches(|c| c == '-')));
    }
    fn longs(&mut self, longs: &[&'help str]) {
        for l in longs {
            self.long(l);
        }
    }
    fn hidden_long(&mut self, l: &'help str) {
        self.switch_mut()
            .longs
            .push(Alias::hidden(l.trim_left_matches(|c| c == '-')));
    }
    fn hidden_longs(&mut self, longs: &[&'help str]) {
        for l in longs {
            self.switch_mut()
                .longs
                .push(Alias::hidden(l.trim_left_matches(|c| c == '-')));
        }
    }
    fn has_index(&self) -> bool {
        use ArgKey::*;
        match *self {
            Index(_) => true,
            Switch(_) => false,
        }
    }
    fn index(&self) -> Option<&usize>> {
        match *self {
            Index(i) => Some(i),
            _ => None,
        }
    }
    fn index_mut(&mut self) -> Option<&mut usize> {
        match *self {
            Index(i) => Some(i),
            _ => None,
        }
    }
    fn has_switch(&self) -> bool {
        use ArgKey::*;
        match *self {
            Index(_) => false,
            Switch(_) => true,
        }
    }
    fn switch(&self) -> Option<&SwitchData<'help>> {
        match *self {
            Switch(s) => Some(s),
            _ => None,
        }
    }
    fn switch_mut(&mut self) -> Option<&mut SwitchData<'help>> {
        match *self {
            Switch(s) => Some(s),
            _ => None,
        }
    }
}
