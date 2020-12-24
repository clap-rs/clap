use crate::{build::Arg, util::Id, INTERNAL_ERROR_MSG};
use std::collections::HashMap;

use std::{ffi::OsString, ops::Index};

#[derive(Default, PartialEq, Debug, Clone)]
pub(crate) struct MKeyMap<'help> {
    pub(crate) key_map: HashMap<KeyType, usize>,
    pub(crate) id_map: HashMap<Id, usize>,
    pub(crate) args: Vec<Arg<'help>>,

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

    pub(crate) fn contains(&self, key: &KeyType) -> bool {
        self.key_map.contains_key(key)
    }

    pub(crate) fn push(&mut self, value: Arg<'help>) -> usize {
        if self.built {
            panic!("Cannot add Args to the map after the map is built");
        }

        let index = self.args.len();
        self.args.push(value);

        index
    }
    //TODO ::push_many([x, y])

    // ! Arg mutation functionality

    pub(crate) fn get(&self, key: &KeyType) -> Option<&Arg<'help>> {
        let index = *self.key_map.get(key)?;
        let result = self.args.get(index);
        debug_assert!(result.is_some());
        result
    }
    //TODO ::get_first([KeyA, KeyB])

    pub(crate) fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub(crate) fn _build(&mut self) {
        self.built = true;

        for (i, arg) in self.args.iter().enumerate() {
            for k in _get_keys(arg) {
                self.key_map.insert(k, i);
            }
        }
    }

    //TODO ::remove_many([KeyA, KeyB])
    //? probably shouldn't add a possibility for removal?
    //? or remove by replacement by some dummy object, so the order is preserved

    pub(crate) fn remove_by_name(&mut self, name: &Id) -> Option<Arg<'help>> {
        if self.built {
            panic!("Cannot remove args after being built");
        }

        self.args
            .iter()
            .position(|arg| arg.id == *name)
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
    for (c, _) in arg.short_aliases.iter() {
        keys.push(KeyType::Short(*c));
    }
    if let Some(c) = arg.short {
        keys.push(KeyType::Short(c));
    }
    for (long, _) in arg.aliases.iter() {
        keys.push(KeyType::Long(OsString::from(long)));
    }
    if let Some(long) = arg.long {
        keys.push(KeyType::Long(OsString::from(long)));
    }
    keys
}
