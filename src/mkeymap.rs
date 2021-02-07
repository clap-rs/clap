use crate::{build::Arg, util::Id, INTERNAL_ERROR_MSG};

use std::{ffi::OsString, iter::Iterator, ops::Index};

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct Key {
    key: KeyType,
    index: usize,
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub(crate) struct MKeyMap<'help> {
    /// All of the arguments.
    args: Vec<Arg<'help>>,

    // Cache part:
    /// Will be set after `_build()`.
    keys: Vec<Key>,
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

impl PartialEq<u64> for KeyType {
    fn eq(&self, rhs: &u64) -> bool {
        match self {
            KeyType::Position(x) => x == rhs,
            _ => false,
        }
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
    /// If any arg has corresponding key in this map, we can search the key with
    /// u64(for positional argument), char(for short flag), &str and OsString
    /// (for long flag)
    pub(crate) fn contains<K>(&self, key: K) -> bool
    where
        KeyType: PartialEq<K>,
    {
        self.keys.iter().any(|x| x.key == key)
    }

    /// Push an argument in the map.
    pub(crate) fn push(&mut self, new_arg: Arg<'help>) {
        self.args.push(new_arg);
    }

    /// Find the arg have corresponding key in this map, we can search the key
    /// with u64(for positional argument), char(for short flag), &str and
    /// OsString (for long flag)
    pub(crate) fn get<K>(&self, key: &K) -> Option<&Arg<'help>>
    where
        KeyType: PartialEq<K>,
    {
        self.keys
            .iter()
            .find(|k| &k.key == key)
            .map(|k| &self.args[k.index])
    }

    /// Find out if the map have no arg.
    pub(crate) fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    /// Return iterators of all keys.
    pub(crate) fn keys(&self) -> impl Iterator<Item = &KeyType> {
        self.keys.iter().map(|x| &x.key)
    }

    /// Return iterators of all args.
    pub(crate) fn args(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.args.iter()
    }

    /// Return mutable iterators of all args.
    pub(crate) fn args_mut<'map>(&'map mut self) -> impl Iterator<Item = &'map mut Arg<'help>> {
        self.args.iter_mut()
    }

    /// We need a lazy build here since some we may change args after creating
    /// the map, you can checkout who uses `args_mut`.
    pub(crate) fn _build(&mut self) {
        for (i, arg) in self.args.iter_mut().enumerate() {
            for k in _get_keys(arg) {
                self.keys.push(Key { key: k, index: i });
            }
        }
    }

    /// Remove an arg in the graph by Id, usually used by `mut_arg`. Return
    /// `Some(arg)` if removed.
    pub(crate) fn remove_by_name(&mut self, name: &Id) -> Option<Arg<'help>> {
        self.args
            .iter()
            .position(|arg| &arg.id == name)
            // since it's a cold function, using this wouldn't hurt much
            .map(|i| self.args.remove(i))
    }

    /// Remove an arg based on index
    pub(crate) fn remove(&mut self, index: usize) -> Arg<'help> {
        self.args.remove(index)
    }
}

impl<'help> Index<&'_ KeyType> for MKeyMap<'help> {
    type Output = Arg<'help>;

    fn index(&self, key: &KeyType) -> &Self::Output {
        self.get(key).expect(INTERNAL_ERROR_MSG)
    }
}

/// Generate key types for an specific Arg.
fn _get_keys(arg: &Arg) -> Vec<KeyType> {
    if let Some(index) = arg.index {
        return vec![KeyType::Position(index)];
    }

    let mut keys = vec![];

    if let Some(short) = arg.short {
        keys.push(KeyType::Short(short));
    }
    if let Some(long) = arg.long {
        keys.push(KeyType::Long(OsString::from(long)));
    }

    for (short, _) in arg.short_aliases.iter() {
        keys.push(KeyType::Short(*short));
    }
    for (long, _) in arg.aliases.iter() {
        keys.push(KeyType::Long(OsString::from(long)));
    }
    keys
}
