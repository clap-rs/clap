use std::ffi::OsStr;
use std::ffi::OsString;

#[derive(Debug)]
pub(crate) struct Input {
    items: Vec<OsString>,
    cursor: usize,
}

impl<I, T> From<I> for Input
where
    I: Iterator<Item = T>,
    T: Into<OsString> + Clone,
{
    fn from(val: I) -> Self {
        Self {
            items: val.map(|x| x.into()).collect(),
            cursor: 0,
        }
    }
}

impl Input {
    pub(crate) fn next(&mut self) -> Option<(&OsStr, &[OsString])> {
        if self.cursor >= self.items.len() {
            None
        } else {
            let current = &self.items[self.cursor];
            self.cursor += 1;
            let remaining = &self.items[self.cursor..];
            Some((current, remaining))
        }
    }

    pub(crate) fn previous(&mut self) {
        self.cursor -= 1;
    }

    /// Insert some items to the Input items just after current parsing cursor.
    /// Usually used by replaced items recovering.
    pub(crate) fn insert(&mut self, insert_items: &[&str]) {
        self.items = insert_items
            .iter()
            .map(OsString::from)
            .chain(self.items.drain(self.cursor..))
            .collect();
        self.cursor = 0;
    }
}
