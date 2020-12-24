use crate::{build::Arg, util::Id, INTERNAL_ERROR_MSG};

use std::{ffi::OsString, ops::Index};

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Key {
    pub(crate) key: KeyType,
    pub(crate) index: usize,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub(crate) struct MKeyMap<'help> {
    pub(crate) keys: Vec<Key>,
    pub(crate) args: Vec<Arg<'help>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum KeyType {
    Short(char),
    Long(OsString),
    Position(u64),
}

impl KeyType {
    pub(crate) fn is_position(&self) -> bool {
        matches!(self, KeyType::Position(_))
    }
}

impl PartialEq<&str> for KeyType {
    fn eq(&self, rhs: &&str) -> bool {
        match self {
            KeyType::Long(l) => l == rhs,
            _ => false,
        }
    }
}

impl PartialEq<OsString> for KeyType {
    fn eq(&self, rhs: &OsString) -> bool {
        match self {
            KeyType::Long(l) => l == rhs,
            _ => false,
        }
    }
}

impl PartialEq<char> for KeyType {
    fn eq(&self, rhs: &char) -> bool {
        match self {
            KeyType::Short(c) => c == rhs,
            _ => false,
        }
    }
}

impl<'help> MKeyMap<'help> {
    //TODO ::from(x), ::with_capacity(n) etc.
    //? set theory ops?

    pub(crate) fn contains<K>(&self, key: K) -> bool
    where
        KeyType: PartialEq<K>,
    {
        self.keys.iter().any(|x| x.key == key)
    }

    pub(crate) fn push(&mut self, value: Arg<'help>) -> usize {
        let index = self.args.len();
        self.args.push(value);
        index
    }
    //TODO ::push_many([x, y])

    // ! Arg mutation functionality

    pub(crate) fn get<K>(&self, key: &K) -> Option<&Arg<'help>>
    where
        KeyType: PartialEq<K>,
    {
        self.keys
            .iter()
            .find(|k| &k.key == key)
            .map(|k| &self.args[k.index])
    }
    //TODO ::get_first([KeyA, KeyB])

    pub(crate) fn is_empty(&self) -> bool {
        self.keys.is_empty() && self.args.is_empty()
    }

    pub(crate) fn _build(&mut self) {
        for (i, arg) in self.args.iter_mut().enumerate() {
            for k in _get_keys(arg) {
                self.keys.push(Key { key: k, index: i });
            }
        }
    }

    //TODO ::remove_many([KeyA, KeyB])
    //? probably shouldn't add a possibility for removal?
    //? or remove by replacement by some dummy object, so the order is preserved

    pub(crate) fn remove_by_name(&mut self, name: &Id) -> Option<Arg<'help>> {
        self.args
            .iter()
            .position(|arg| &arg.id == name)
            .map(|i| self.args.swap_remove(i))
    }
}

impl<'help> Index<&'_ KeyType> for MKeyMap<'help> {
    type Output = Arg<'help>;

    fn index(&self, key: &KeyType) -> &Self::Output {
        self.get(key).expect(INTERNAL_ERROR_MSG)
    }
}

fn _get_keys(arg: &Arg) -> Vec<KeyType> {
    if let Some(index) = arg.index {
        return vec![KeyType::Position(index)];
    }

    let mut keys = vec![];
    for short in arg.short_aliases.iter().map(|(c, _)| KeyType::Short(*c)) {
        keys.push(short);
    }
    if let Some(c) = arg.short {
        keys.push(KeyType::Short(c));
    }

    for long in arg
        .aliases
        .iter()
        .map(|(a, _)| KeyType::Long(OsString::from(a)))
    {
        keys.push(long);
    }

    if let Some(long) = arg.long {
        keys.push(KeyType::Long(OsString::from(long)));
    }

    keys
}
