use std::ffi::OsStr;
use std::ffi::OsString;

pub use std::io::SeekFrom;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub(crate) struct RawArgs {
    items: Vec<OsString>,
}

impl RawArgs {
    pub fn cursor(&self) -> ArgCursor {
        ArgCursor::new()
    }

    pub fn next(&self, cursor: &mut ArgCursor) -> Option<&OsStr> {
        let next = self.items.get(cursor.cursor).map(|s| s.as_os_str());
        cursor.cursor = cursor.cursor.saturating_add(1);
        next
    }

    pub fn peek(&self, cursor: &ArgCursor) -> Option<&OsStr> {
        self.items.get(cursor.cursor).map(|s| s.as_os_str())
    }

    pub fn remaining(&self, cursor: &mut ArgCursor) -> impl Iterator<Item = &OsStr> {
        let remaining = self.items[cursor.cursor..].iter().map(|s| s.as_os_str());
        cursor.cursor = self.items.len();
        remaining
    }

    pub fn seek(&self, cursor: &mut ArgCursor, pos: SeekFrom) {
        let pos = match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::End(pos) => (self.items.len() as i64).saturating_add(pos).max(0) as u64,
            SeekFrom::Current(pos) => (cursor.cursor as i64).saturating_add(pos).max(0) as u64,
        };
        let pos = (pos as usize).min(self.items.len());
        cursor.cursor = pos;
    }

    /// Inject arguments before the [`RawArgs::next`]
    pub fn insert(&mut self, cursor: &ArgCursor, insert_items: &[&str]) {
        self.items.splice(
            cursor.cursor..cursor.cursor,
            insert_items.iter().map(OsString::from),
        );
    }
}

impl<I, T> From<I> for RawArgs
where
    I: Iterator<Item = T>,
    T: Into<OsString>,
{
    fn from(val: I) -> Self {
        Self {
            items: val.map(|x| x.into()).collect(),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ArgCursor {
    cursor: usize,
}

impl ArgCursor {
    fn new() -> Self {
        Default::default()
    }
}
