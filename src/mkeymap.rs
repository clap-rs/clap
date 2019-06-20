use crate::build::Arg;
use std::ffi::{OsStr, OsString};

type Id = u64;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Key {
    pub(crate) key: KeyType,
    pub(crate) index: usize,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub(crate) struct MKeyMap<'b> {
    pub(crate) keys: Vec<Key>,
    pub(crate) args: Vec<Arg<'b>>,
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
    pub(crate) fn contains_short(&self, c: char) -> bool { self.keys.iter().any(|x| x.key == c) }

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
        for k in &self.keys {
            if &k.key == key {
                return Some(&self.args[k.index]);
            }
        }
        None
    }
    //TODO ::get_first([KeyA, KeyB])

    pub(crate) fn is_empty(&self) -> bool { self.keys.is_empty() && self.args.is_empty() }

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
        let mut index = None;
        for (i, arg) in self.args.iter().enumerate() {
            if arg.id == _name {
                index = Some(i);
                break;
            }
        }
        if let Some(i) = index {
            Some(self.args.swap_remove(i))
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use self::KeyType::*;
    use super::*;

    impl<'b> MKeyMap<'b> {
        fn insert(&mut self, key: KeyType, value: Arg<'b>) -> usize {
            let index = self.push(value);
            self.insert_key(key, index);
            index
        }
    }

    #[test]
    fn get_some_value() {
        let mut map: MKeyMap = MKeyMap::default();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.get(&Long(OsString::from("One"))),
            Some(&Arg::with_name("Value1"))
        );
    }

    #[test]
    fn get_none_value() {
        let mut map: MKeyMap = MKeyMap::default();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));
        map.get(&Long(OsString::from("Two")));

        assert_eq!(map.get(&Long(OsString::from("Two"))), None);
    }

    //    #[test]
    //    fn insert_delete_value() {
    //        let mut map = MKeyMap::default();
    //        map.insert("One", clap::Arg::with_name("Value1"));
    //        assert_eq!(map.remove("One"), Some(clap::Arg::with_name("Value1")));
    //        assert!(map.is_empty());
    //    }

    #[test]
    fn insert_duplicate_key() {
        let mut map: MKeyMap = MKeyMap::default();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.insert(Long(OsString::from("One")), Arg::with_name("Value2")),
            1
        );
    }

    #[test]
    #[should_panic]
    fn insert_duplicate_value() {
        let mut map: MKeyMap = MKeyMap::default();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        let orig_len = map.args.len();

        map.insert(Long(OsString::from("Two")), Arg::with_name("Value1"));

        assert_eq!(map.args.len(), orig_len);
        assert_eq!(
            map.get(&Long(OsString::from("One"))),
            map.get(&Long(OsString::from("Two")))
        );
    }

    //    #[test]
    //    fn insert_delete_none() {
    //        let mut map = MKeyMap::default();
    //        map.insert("One", clap::Arg::with_name("Value1"));
    //        assert_eq!(map.remove("Two"), None);
    //        assert!(!map.is_empty());
    //        assert_eq!(map.get("One"), Some(clap::Arg::with_name("Value1")));
    //    }

    #[test]
    fn insert_multiple_keys() {
        let mut map: MKeyMap = MKeyMap::default();
        let index = map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        map.insert_key(Long(OsString::from("Two")), index);

        assert_eq!(
            map.get(&Long(OsString::from("One"))),
            map.get(&Long(OsString::from("Two")))
        );
        assert_eq!(map.args.len(), 1);
    }

    // #[test]
    // fn insert_by_name() {
    //     let mut map: MKeyMap<Arg> = MKeyMap::default();
    //     let index = map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

    //     map.insert_key_by_name(Long(OsString::from("Two")), "Value1");

    //     assert_eq!(
    //         map.get(Long(OsString::from("One"))),
    //         map.get(Long(OsString::from("Two")))
    //     );
    //     assert_eq!(map.values.len(), 1);
    // }

    /*
    #[test]
    fn get_mutable() {
        let mut map: MKeyMap = MKeyMap::default();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.get_mut(&Long(OsString::from("One"))),
            Some(&mut Arg::with_name("Value1"))
        );
    }
    */

    /*
    #[test]
    fn remove_key() {
        let mut map: MKeyMap = MKeyMap::default();
        let index = map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        map.insert_key(Long(OsString::from("Two")), index);
        map.remove_key(&Long(OsString::from("One")));

        assert_eq!(map.keys.len(), 1);
        assert_eq!(map.args.len(), 1);
    }
    */
}
