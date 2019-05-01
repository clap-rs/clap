struct SwitchData {
    longs: Vec<Alias<'help>>,
    short: Option<char>,
}

pub(crate) enum Key<'help> {
    Index(usize),
    Switch(SwitchData),
}

impl<'help> Key {
    fn long(&mut self, l: &'help str) {
        self.switch_mut()
            .longs
            .push(Alias::visible(l.trim_left_matches(|c| c == '-')));
    }
    fn hidden_long(&mut self, l: &'help str) {
        self.switch_mut()
            .longs
            .push(Alias::hidden(l.trim_left_matches(|c| c == '-')));
    }

    fn switch_mut(&mut self) -> &mut SwitchData<'help> {
        match *self {
            Switch(s) => s,
            _ => unreachable!(),
        }
    }

    fn has_switch(&self) -> bool {
        use Key::*;
        match *self {
            Index(_) => false,
            Switch(_) => true,
        }
    }
}
