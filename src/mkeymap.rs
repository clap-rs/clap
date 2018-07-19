#![feature(nll)]

use build::Arg;
use std::collections::hash_map;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::hash::{Hash, Hasher};
use std::slice;
// ! rustdoc

#[derive(Default, PartialEq, Debug)]
pub struct MKeyMap<'a, 'b>
where
    'a: 'b,
{
    keys: HashMap<KeyType<'a>, usize>,
    value_index: Vec<Arg<'a, 'b>>,
    values: HashMap<u64, HashSet<usize>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyType<'a> {
    Short(char),
    Long(&'a OsStr),
    Position(usize),
}

impl<'a, 'b> MKeyMap<'a, 'b> {
    pub fn new() -> Self { MKeyMap::default() }
    //TODO ::from(x), ::with_capacity(n) etc
    //? set theory ops?

    pub fn insert(&mut self, key: KeyType<'a>, value: Arg<'a, 'b>) -> usize {
        let index = self.push(value);
        self.keys.insert(key, index);
        index
    }

    pub fn push(&mut self, value: Arg<'a, 'b>) -> usize {
        let index;
        let mut hasher = DefaultHasher::new();

        value.hash(&mut hasher);

        let hash = hasher.finish();

        if let Some((idx, _)) = self.values.get(&hash).and_then(|ids| {
            ids.iter()
                .map(|&x| (x, &self.value_index[x]))
                .find(|(_i, x)| x == &&value)
        }) {
            index = idx;
        } else {
            self.value_index.push(value);
            index = self.value_index.len() - 1;
            self.values
                .entry(hash)
                .and_modify(|x| {
                    x.insert(index);
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(index);
                    set
                });
        }
        
        index
    }
    //TODO ::push_many([x, y])

    pub fn insert_key(&mut self, key: KeyType<'a>, index: usize) {
        if index >= self.values.len() {
            panic!("Index out of bounds");
        }

        self.keys.insert(key, index);
    }
    //TODO ::insert_keyset([Long, Key2])

    pub fn insert_key_by_name(&mut self, key: KeyType<'a>, name: &str) {
        let index = self
            .value_index
            .iter()
            .position(|x| x.name == name)
            .expect("No such name found");

        self.keys.insert(key, index);
    }

    pub fn get(&self, key: KeyType<'a>) -> Option<&Arg<'a, 'b>> {
        self.keys
            .get(&key)
            .and_then(|&idx| self.value_index.get(idx))
    }
    //TODO ::get_first([KeyA, KeyB])

    pub fn get_mut(&mut self, key: KeyType<'a>) -> Option<&mut Arg<'a, 'b>> {
        if let Some(&idx) = self
            .keys
            .get(&key) {
                self.value_index.get_mut(idx)
            } else {
                None
            }
    }

    pub fn is_empty(&self) -> bool { self.keys.is_empty() && self.values.is_empty() }

    pub fn remove(&mut self, key: KeyType) -> Option<Arg> { unimplemented!() }
    //TODO ::remove_many([KeyA, KeyB])
    //? probably shouldn't add a possibility for removal?
    //? or remove by replacement by some dummy object, so the order is preserved

    pub fn remove_key(&mut self, key: KeyType) { unimplemented!() }
    //TODO ::remove_keys([KeyA, KeyB])

    pub fn keys(&'a self) -> Keys<'a, usize> {
        Keys {
            iter: self.keys.keys(),
        }
    }

    pub fn values(&'a self) -> Values<'a, Arg> {
        Values {
            iter: self.value_index.iter(),
        }
    }

    pub fn values_mut(&'a mut self) -> ValuesMut<'a, Arg> {
        ValuesMut {
            iter: self.value_index.iter_mut(),
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            map: self,
            keys: self.keys(),
        }
    }
}

//TODO remove generics
pub struct Keys<'a, V: 'a> {
    iter: hash_map::Keys<'a, KeyType<'a>, V>,
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a KeyType<'a>;

    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

pub struct Values<'a, V: 'a> {
    iter: slice::Iter<'a, V>,
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

pub struct ValuesMut<'a, V: 'a> {
    iter: slice::IterMut<'a, V>,
}

impl<'a, V> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

pub struct Iter<'a, 'b, 'c>  where
'a: 'b,
'b: 'c {
    map: &'c MKeyMap<'a, 'b>,
    keys: Keys<'c, usize>,
}

impl <'a, 'b, 'c> Iterator for Iter<'a, 'b, 'c> 
{
    type Item = (&'c KeyType<'a>, &'c Arg<'a, 'b>);

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
    use std::ffi::OsStr;

    #[test]
    fn get_some_value() {
        let mut map: MKeyMap = MKeyMap::new();

        {
            map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));
        }

        assert_eq!(map.get(Long(&OsStr::new("One"))), &Arg::with_name("Value1"));
    }

    #[test]
    #[should_panic]
    fn get_none_value() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));
        map.get(Long(&OsStr::new("Two")));
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
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.insert(Long(&OsStr::new("One")), Arg::with_name("Value2")),
            1
        );
    }

    #[test]
    fn insert_duplicate_value() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));

        let orig_len = map.values.len();

        map.insert(Long(&OsStr::new("Two")), Arg::with_name("Value1"));

        assert_eq!(map.values.len(), orig_len);
        assert_eq!(
            map.get(Long(&OsStr::new("One"))),
            map.get(Long(&OsStr::new("Two")))
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
        let mut map: MKeyMap = MKeyMap::new();
        let index = map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));

        map.insert_key(Long(&OsStr::new("Two")), index);

        assert_eq!(
            map.get(Long(&OsStr::new("One"))),
            map.get(Long(&OsStr::new("Two")))
        );
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn insert_by_name() {
        let mut map: MKeyMap = MKeyMap::new();
        let index = map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));

        map.insert_key_by_name(Long(&OsStr::new("Two")), "Value1");

        assert_eq!(
            map.get(Long(&OsStr::new("One"))),
            map.get(Long(&OsStr::new("Two")))
        );
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn get_mutable() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.get_mut(Long(&OsStr::new("One"))),
            &mut Arg::with_name("Value1")
        );
    }

    #[test]
    fn remove_key() {
        let mut map: MKeyMap = MKeyMap::new();
        let index = map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));

        map.insert_key(Long(&OsStr::new("Two")), index);
        map.remove_key(Long(&OsStr::new("One")));

        assert_eq!(map.keys.len(), 1);
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn iter_keys() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(&OsStr::new("One")), Arg::with_name("Value1"));
        map.insert(Long(&OsStr::new("Two")), Arg::with_name("Value2"));
        map.insert(Position(1), Arg::with_name("Value1"));

        let iter = map.keys().cloned();
        let mut ground_truth = HashSet::new();

        ground_truth.insert(Long(&OsStr::new("One")));
        ground_truth.insert(Long(&OsStr::new("Two")));
        ground_truth.insert(Position(1));

        assert_eq!(
            ground_truth.symmetric_difference(&iter.collect()).count(),
            0
        );
    }
}
