use build::Arg;
use std::collections::hash_map;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::hash::{Hash, Hasher};
use std::slice;
// ! rustdoc

#[derive(Default, PartialEq, Debug, Clone)]
pub struct MKeyMap<T> {
    keys: HashMap<KeyType, usize>,
    value_index: Vec<T>,
    values: HashMap<u64, HashSet<usize>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KeyType {
    Short(char),
    Long(OsString),
    Position(u64),
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

    pub fn insert_key(&mut self, key: KeyType, index: usize) {
        if index >= self.values.len() {
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

    pub fn is_empty(&self) -> bool { self.keys.is_empty() && self.values.is_empty() }

    pub fn remove_by_name(&mut self, _name: &str) -> Option<T> { unimplemented!() }

    pub fn remove(&mut self, _key: KeyType) -> Option<T> { unimplemented!() }
    //TODO ::remove_many([KeyA, KeyB])
    //? probably shouldn't add a possibility for removal?
    //? or remove by replacement by some dummy object, so the order is preserved

    pub fn remove_key(&mut self, key: KeyType) { self.keys.remove(&key); }
    //TODO ::remove_keys([KeyA, KeyB])

    pub fn keys(&self) -> Keys<usize> {
        Keys {
            iter: self.keys.keys(),
        }
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

impl<'a, 'b> MKeyMap<Arg<'a, 'b>> {
    pub fn insert_key_by_name(&mut self, key: KeyType, name: &str) {
        let index = self.find_by_name(name);

        self.keys.insert(key, index);
    }

    pub fn make_entries(&mut self, arg: Arg<'a, 'b>) -> usize {
        let short = arg.short.map(|c| KeyType::Short(c));
        let positional = arg.index.map(|n| KeyType::Position(n));

        let mut longs = arg
            .aliases
            .clone()
            .map(|v| {
                v.iter()
                    .map(|(n, _)| KeyType::Long(OsString::from(n)))
                    .collect()
            })
            .unwrap_or(Vec::new());

        longs.extend(arg.long.map(|l| KeyType::Long(OsString::from(l))));

        let index = self.push(arg);
        short.map(|s| self.insert_key(s, index));
        positional.map(|p| self.insert_key(p, index));
        longs.into_iter().map(|l| self.insert_key(l, index)).count();

        index
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
                })
                .unwrap_or(Vec::new());
            longs.extend(arg.long.map(|l| KeyType::Long(OsString::from(l))));
        }

        short.map(|s| self.insert_key(s, index));
        positional.map(|p| self.insert_key(p, index));
        longs.into_iter().map(|l| self.insert_key(l, index)).count();
    }

    pub fn mut_arg<F>(&mut self, name: &str, f: F)
    where
        F: FnOnce(Arg<'a, 'b>) -> Arg<'a, 'b>,
    {
        let index = self.find_by_name(name);
        let new_arg = f(self.value_index[index].clone());

        let value_key = self
            .values
            .iter()
            .filter(|(_, v)| v.contains(&index))
            .map(|(k, _)| k)
            .next()
            .map(|&x| x);
        value_key.map(|k| {
            self.values.entry(k).and_modify(|v| {
                v.remove(&index);
            })
        });

        let mut hasher = DefaultHasher::new();

        new_arg.hash(&mut hasher);

        let hash = hasher.finish();
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

        self.value_index.push(new_arg);
        self.value_index.swap_remove(index);
        self.make_entries_by_index(index);
    }

    pub fn find_by_name(&mut self, name: &str) -> usize {
        self.value_index
            .iter()
            .position(|x| x.name == name)
            .expect("No such name found")
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
    fn insert_duplicate_value() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));

        let orig_len = map.values.len();

        map.insert(Long(OsString::from("Two")), Arg::with_name("Value1"));

        assert_eq!(map.values.len(), orig_len);
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
        assert_eq!(map.values.len(), 1);
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
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn iter_keys() {
        let mut map: MKeyMap<Arg> = MKeyMap::new();

        map.insert(Long(OsString::from("One")), Arg::with_name("Value1"));
        map.insert(Long(OsString::from("Two")), Arg::with_name("Value2"));
        map.insert(Position(1), Arg::with_name("Value1"));

        let iter = map.keys().cloned();
        let mut ground_truth = HashSet::new();

        ground_truth.insert(Long(OsString::from("One")));
        ground_truth.insert(Long(OsString::from("Two")));
        ground_truth.insert(Position(1));

        assert_eq!(
            ground_truth.symmetric_difference(&iter.collect()).count(),
            0
        );
    }
}
