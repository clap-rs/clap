use crate::build::Arg;
use std::ffi::{OsStr, OsString};

type Id = u64;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Key {
    pub(crate) key: KeyType,
    pub(crate) index: usize,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct MKeyMap<'b> {
    pub(crate) keys: Vec<Key>,
    pub args: Vec<Arg<'b>>,

    // FIXME (@CreepySkeleton): this seems useless
    built: bool, // mutation isn't possible after being built
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum KeyType {
    Short(char),
    Long(OsString),
    Position(u64),
}

impl KeyType {
    pub(crate) fn is_position(&self) -> bool {
        match *self {
            KeyType::Position(_) => true,
            _ => false,
        }
    }
}

impl PartialEq<&str> for KeyType {
    fn eq(&self, rhs: &&str) -> bool {
        match self {
            KeyType::Long(ref l) => l == OsStr::new(rhs),
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

impl<'b> MKeyMap<'b> {
    //TODO ::from(x), ::with_capacity(n) etc
    //? set theory ops?

    pub(crate) fn contains<K>(&self, key: K) -> bool
    where
        KeyType: PartialEq<K>,
    {
        self.keys.iter().any(|x| x.key == key)
    }

    pub(crate) fn push(&mut self, value: Arg<'b>) -> usize {
        if self.built {
            panic!("Cannot add Args to the map after the map is built");
        }

        let index = self.args.len();
        self.args.push(value);

        index
    }
    //TODO ::push_many([x, y])

    pub(crate) fn insert_key(&mut self, key: KeyType, index: usize) {
        if index >= self.args.len() {
            panic!("Index out of bounds");
        }

        self.keys.push(Key { key, index });
    }
    //TODO ::insert_keyset([Long, Key2])

    // ! Arg mutation functionality

    pub(crate) fn get(&self, key: &KeyType) -> Option<&Arg<'b>> {
        self.keys
            .iter()
            .find(|k| k.key == *key)
            .map(|k| &self.args[k.index])
    }
    //TODO ::get_first([KeyA, KeyB])

    pub(crate) fn is_empty(&self) -> bool {
        self.keys.is_empty() && self.args.is_empty()
    }

    pub(crate) fn _build(&mut self) {
        self.built = true;

        for (i, arg) in self.args.iter_mut().enumerate() {
            for k in _get_keys(arg) {
                self.keys.push(Key { key: k, index: i });
            }
        }
    }

    //TODO ::remove_many([KeyA, KeyB])
    //? probably shouldn't add a possibility for removal?
    //? or remove by replacement by some dummy object, so the order is preserved

    pub(crate) fn remove_by_name(&mut self, _name: Id) -> Option<Arg<'b>> {
        if self.built {
            panic!("Cannot remove args after being built");
        }

        self.args
            .iter()
            .position(|arg| arg.id == _name)
            .map(|i| self.args.swap_remove(i))
    }
}

fn _get_keys(arg: &Arg) -> Vec<KeyType> {
    if let Some(index) = arg.index {
        return vec![KeyType::Position(index)];
    }

    let mut keys = vec![];
    if let Some(c) = arg.short {
        keys.push(KeyType::Short(c));
    }
    if let Some(ref aliases) = arg.aliases {
        for long in aliases
            .iter()
            .map(|(a, _)| KeyType::Long(OsString::from(a)))
        {
            keys.push(long);
        }
    }
    if let Some(long) = arg.long {
        keys.push(KeyType::Long(OsString::from(long)));
    }

    keys
}
