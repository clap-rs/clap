use build::Arg;
use std::collections::hash_map;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::hash::Hash;
use std::slice;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct MKeyMap<T> {
    keys: HashMap<KeyType, usize>,
    value_index: Vec<T>,
    built: bool, // mutation isn't possible after being built
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyType {
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
    pub(crate) fn is_short(&self) -> bool {
        match *self {
            KeyType::Short(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_long(&self) -> bool {
        match *self {
            KeyType::Long(_) => true,
            _ => false,
        }
    }
}

impl<T> MKeyMap<T>
where
    T: Sized + Hash + PartialEq + Default + Eq,
{
    pub fn new() -> Self { MKeyMap::default() }
    //TODO ::from(x), ::with_capacity(n) etc
    //? set theory ops?

    pub fn insert(&mut self, key: KeyType, value: T) -> usize {
        let index = self.push(value);
        self.keys.insert(key, index);
        index
    }

    pub fn push(&mut self, value: T) -> usize {
        if self.built {
            panic!("Cannot add Args to the map after the map is built");
        }

        let index = self.value_index.len();
        self.value_index.push(value);

        index
    }
    //TODO ::push_many([x, y])

    pub fn insert_key(&mut self, key: KeyType, index: usize) {
        if index >= self.value_index.len() {
            panic!("Index out of bounds");
        }

        self.keys.insert(key, index);
    }
    //TODO ::insert_keyset([Long, Key2])

    // ! Arg mutation functionality

    pub fn get(&self, key: KeyType) -> Option<&T> {
        self.keys
            .get(&key)
            .and_then(|&idx| self.value_index.get(idx))
    }
    //TODO ::get_first([KeyA, KeyB])

    pub fn get_mut(&mut self, key: KeyType) -> Option<&mut T> {
        if let Some(&idx) = self.keys.get(&key) {
            self.value_index.get_mut(idx)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool { self.keys.is_empty() && self.value_index.is_empty() }

    pub fn remove_key(&mut self, key: KeyType) { self.keys.remove(&key); }
    //TODO ::remove_keys([KeyA, KeyB])

    pub fn keys(&self) -> Keys<usize> {
        Keys {
            iter: self.keys.keys(),
        }
    }

    pub fn longs(&self) -> impl Iterator<Item = &OsString> {
        self.keys.keys().filter_map(|x| match x {
            KeyType::Long(ref l) => Some(l),
            _ => None,
        })
    }

    pub fn shorts(&self) -> impl Iterator<Item = &KeyType> {
        self.keys.keys().filter(|x| x.is_short())
    }

    pub fn values(&self) -> Values<T> {
        Values {
            iter: self.value_index.iter(),
        }
    }

    pub fn values_mut(&mut self) -> ValuesMut<T> {
        ValuesMut {
            iter: self.value_index.iter_mut(),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            map: self,
            keys: self.keys(),
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

impl<'a, 'b> MKeyMap<Arg<'a, 'b>> {
    pub fn insert_key_by_name(&mut self, key: KeyType, name: &str) {
        let index = self.find_by_name(name);

        self.keys.insert(key, index);
    }

    pub fn _build(&mut self) {
        self.built = true;

        for (i, arg) in self.value_index.iter_mut().enumerate() {
            for k in _get_keys(arg) {
                self.keys.insert(k, i);
            }
        }
    }

    pub fn make_entries_by_index(&mut self, index: usize) {
        let short;
        let positional;
        let mut longs;

        {
            let arg = &self.value_index[index];
            short = arg.short.map(|c| KeyType::Short(c));
            positional = arg.index.map(|n| KeyType::Position(n));

            longs = arg
                .aliases
                .clone()
                .map(|v| {
                    v.iter()
                        .map(|(n, _)| KeyType::Long(OsString::from(n)))
                        .collect()
                }).unwrap_or(Vec::new());
            longs.extend(arg.long.map(|l| KeyType::Long(OsString::from(l))));
        }

        short.map(|s| self.insert_key(s, index));
        positional.map(|p| self.insert_key(p, index));
        longs.into_iter().map(|l| self.insert_key(l, index)).count();
    }

    pub fn find_by_name(&mut self, name: &str) -> usize {
        self.value_index
            .iter()
            .position(|x| x.name == name)
            .expect("No such name found")
    }

    pub fn remove(&mut self, key: KeyType) -> Option<Arg<'a, 'b>> {
        if self.built {
            panic!("Cannot remove args after being built");
        }
        let index = if let Some(index) = self.keys.get(&key) {
            index.clone()
        } else {
            return None;
        };
        let arg = self.value_index.swap_remove(index);
        for key in _get_keys(&arg) {
            let _ = self.keys.remove(&key);
        }
        Some(arg)
    }
    //TODO ::remove_many([KeyA, KeyB])
    //? probably shouldn't add a possibility for removal?
    //? or remove by replacement by some dummy object, so the order is preserved

    pub fn remove_by_name(&mut self, _name: &str) -> Option<Arg<'a, 'b>> {
        if self.built {
            panic!("Cannot remove args after being built");
        }
        let mut index = None;
        for (i, arg) in self.value_index.iter().enumerate() {
            if arg.name == _name {
                index = Some(i);
                break;
            }
        }
        if let Some(i) = index {
            Some(self.value_index.swap_remove(i))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Keys<'a, V: 'a> {
    iter: hash_map::Keys<'a, KeyType, V>,
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a KeyType;

    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

#[derive(Debug)]
pub struct Values<'a, V: 'a> {
    iter: slice::Iter<'a, V>,
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

#[derive(Debug)]
pub struct ValuesMut<'a, V: 'a> {
    iter: slice::IterMut<'a, V>,
}

impl<'a, V> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

#[derive(Debug)]
pub struct Iter<'c, T>
where
    T: 'c,
{
    map: &'c MKeyMap<T>,
    keys: Keys<'c, usize>,
}

impl<'c, T> Iterator for Iter<'c, T>
where
    T: 'c + Sized + Hash + PartialEq + Default + Eq,
{
    type Item = (&'c KeyType, &'c T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(key) = self.keys.next() {
            Some((key, self.map.get(key.clone()).unwrap()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use self::KeyType::*;
    use super::*;

    #[test]
    fn get_some_value() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.get(Long(OsString::from("One"))),
            Some(&Arg::with_name("Value1"))
        );
    }

    #[test]
    fn get_none_value() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));
        map.get(Long(OsString::from("Two")));

        assert_eq!(map.get(Long(OsString::from("Two"))), None);
    }

    //    #[test]
    //    fn insert_delete_value() {
    //        let mut map = MKeyMap::new();
    //        map.insert("One", clap::Arg::with_name("Value1"));
    //        assert_eq!(map.remove("One"), Some(clap::Arg::with_name("Value1")));
    //        assert!(map.is_empty());
    //    }

    #[test]
    fn insert_duplicate_key() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.insert(Long(OsString::from("One")), Arg::with_name("Value2")),
            1
        );
    }

    #[test]
    #[should_panic]
    fn insert_duplicate_value() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        let orig_len = map.value_index.len();

        map.insert(Long(OsString::from("Two")), Arg::with_name("Value1"));

        assert_eq!(map.value_index.len(), orig_len);
        assert_eq!(
            map.get(Long(OsString::from("One"))),
            map.get(Long(OsString::from("Two")))
        );
    }

    //    #[test]
    //    fn insert_delete_none() {
    //        let mut map = MKeyMap::new();
    //        map.insert("One", clap::Arg::with_name("Value1"));
    //        assert_eq!(map.remove("Two"), None);
    //        assert!(!map.is_empty());
    //        assert_eq!(map.get("One"), Some(clap::Arg::with_name("Value1")));
    //    }

    #[test]
    fn insert_multiple_keys() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();
        let index = map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        map.insert_key(Long(OsString::from("Two")), index);

        assert_eq!(
            map.get(Long(OsString::from("One"))),
            map.get(Long(OsString::from("Two")))
        );
        assert_eq!(map.value_index.len(), 1);
    }

    // #[test]
    // fn insert_by_name() {
    //     let mut map: MKeyMap<Arg> = MKeyMap::new();
    //     let index = map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

    //     map.insert_key_by_name(Long(OsString::from("Two")), "Value1");

    //     assert_eq!(
    //         map.get(Long(OsString::from("One"))),
    //         map.get(Long(OsString::from("Two")))
    //     );
    //     assert_eq!(map.values.len(), 1);
    // }

    #[test]
    fn get_mutable() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.get_mut(Long(OsString::from("One"))),
            Some(&mut Arg::with_name("Value1"))
        );
    }

    #[test]
    fn remove_key() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();
        let index = map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        map.insert_key(Long(OsString::from("Two")), index);
        map.remove_key(Long(OsString::from("One")));

        assert_eq!(map.keys.len(), 1);
        assert_eq!(map.value_index.len(), 1);
    }
}
