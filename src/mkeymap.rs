// Third Party
use bstr::{BStr, BString};

// Internal
use crate::build::arg::{Arg, Key};
use crate::util::FnvHash;
use crate::INTERNAL_ERROR_MSG;

type Id = u64;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct MKeyMap<'b> {
    pub keys: Vec<MapKey>,
    pub args: Vec<Arg<'b>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MapKey {
    pub key: MapKeyKind,
    pub index: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MapKeyKind {
    Id(Id),
    Short(char),
    Long(BString),
    Position(usize),
}

impl MapKeyKind {
    pub(crate) fn is_position(&self) -> bool {
        match *self {
            MapKeyKind::Position(_) => true,
            _ => false,
        }
    }
}

impl PartialEq<usize> for MapKeyKind {
    fn eq(&self, rhs: &usize) -> bool {
        match self {
            MapKeyKind::Position(i) => i == rhs,
            _ => false,
        }
    }
}

impl PartialEq<Id> for MapKeyKind {
    fn eq(&self, rhs: &Id) -> bool {
        match self {
            MapKeyKind::Id(i) => i == rhs,
            _ => false,
        }
    }
}

impl PartialEq<str> for MapKeyKind {
    fn eq(&self, rhs: &str) -> bool {
        match self {
            MapKeyKind::Long(ref l) => l == rhs,
            MapKeyKind::Id(i) => *i == rhs.fnv_hash(),
            _ => false,
        }
    }
}

impl PartialEq<char> for MapKeyKind {
    fn eq(&self, rhs: &char) -> bool {
        match self {
            MapKeyKind::Short(c) => c == rhs,
            _ => false,
        }
    }
}

impl From<usize> for MapKeyKind {
    fn from(us: usize) -> Self { MapKeyKind::Position(us) }
}

impl From<char> for MapKeyKind {
    fn from(c: char) -> Self { MapKeyKind::Short(c) }
}

impl From<Id> for MapKeyKind {
    fn from(i: Id) -> Self { MapKeyKind::Id(i) }
}

impl<'b> MKeyMap<'b> {
    pub fn new() -> Self { MKeyMap::default() }

    pub fn is_empty(&self) -> bool { self.args.is_empty() }

    pub fn push(&mut self, value: Arg<'b>) -> usize {
        let index = self.args.len();
        self.args.push(value);

        index
    }

    pub fn contains<K: Into<MapKeyKind>>(&self, k: K) -> bool {
        let key = k.into();
        self.keys.iter().any(|x| x.key == key)
    }

    pub fn find<K: Into<MapKeyKind>>(&self, k: K) -> Option<&Arg<'b>> {
        let key = k.into();
        self.keys
            .iter()
            .find(|x| x.key == key)
            .map(|mk| self.args.get(mk.index).expect(INTERNAL_ERROR_MSG))
    }

    pub fn remove_key<K: Into<MapKeyKind>>(&mut self, k: K) {
        let key = k.into();
        let mut idx = None;
        for k in self.keys.iter() {
            if k.key == key {
                idx = Some(k.index);
                break;
            }
        }
        if let Some(idx) = idx {
            self.keys.swap_remove(idx);
        }
    }

    pub fn remove<K: Into<MapKeyKind>>(&mut self, k: K) -> Option<Arg<'b>> {
        let key = k.into();
        let mut idx = None;
        for k in self.keys.iter() {
            if k.key == key {
                idx = Some(k.index);
                break;
            }
        }
        if let Some(idx) = idx {
            let arg = self.args.swap_remove(idx);
            for key in get_keys(&arg) {
                self.remove_key(key);
            }
            return Some(arg);
        }
        None
    }

    pub fn _build(&mut self) {
        let mut counter = 0;
        for (i, arg) in self.args.iter_mut().enumerate() {
            for k in get_keys(arg) {
                self.keys.push(MapKey { key: k, index: i });
            }
            if arg.key().kind == Key::Unset {
                arg.index(counter);
                counter += 1;
            }
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Arg<'b>> {
        let key: MapKeyKind = name.fnv_hash().into();
        self.keys
            .iter()
            .find(|x| x.key == key)
            .map(|mk| self.args.get(mk.index))
    }

    pub fn remove_by_name(&mut self, name: &str) -> Option<Arg<'b>> {
        let key: MapKeyKind = name.fnv_hash().into();
        let mut index = None;
        for k in self.keys.iter() {
            if k.key == key {
                index = Some(k.index);
                break;
            }
        }
        if let Some(i) = index {
            for key in get_keys(&arg) {
                self.remove_key(&key);
            }
            Some(self.args.swap_remove(i))
        } else {
            None
        }
    }
}

fn get_keys(arg: &Arg) -> Vec<MapKeyKind> {
    let mut keys = vec![arg.id.into()];
    if let Some(index) = arg.get_index() {
        keys.push(index.into());
        return keys;
    }

    let sd = arg.switch();
    if let Some(s) = sd.short() {
        keys.push(s.into());
    }
    for l in sd.all_longs() {
        keys.push(l.into());
    }
    keys
}

#[cfg(test)]
mod tests {
    use self::MapKeyKind::*;
    use super::*;

    #[test]
    fn get_some_value() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(BString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.get(&Long(BString::from("One"))),
            Some(&Arg::with_name("Value1"))
        );
    }

    #[test]
    fn get_none_value() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(BString::from("One")), Arg::with_name("Value1"));
        map.get(&Long(BString::from("Two")));

        assert_eq!(map.get(&Long(BString::from("Two"))), None);
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

        map.insert(Long(BString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.insert(Long(BString::from("One")), Arg::with_name("Value2")),
            1
        );
    }

    #[test]
    // #[should_panic(expected = "Len changed")]
    fn insert_duplicate_value() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(BString::from("One")), Arg::with_name("Value1"));

        let orig_len = map.args.len();

        map.insert(Long(BString::from("Two")), Arg::with_name("Value1"));

        assert_eq!(map.args.len(), orig_len + 1 /* , "Len changed" */);
        // assert_eq!(
        //     map.get(&Long(BString::from("One"))),
        //     map.get(&Long(BString::from("Two")))
        // );
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
        let index = map.insert(Long(BString::from("One")), Arg::with_name("Value1"));

        map.insert_key(Long(BString::from("Two")), index);

        assert_eq!(
            map.get(&Long(BString::from("One"))),
            map.get(&Long(BString::from("Two")))
        );
        assert_eq!(map.args.len(), 1);
    }

    // #[test]
    // fn insert_by_name() {
    //     let mut map: MKeyMap<Arg> = MKeyMap::new();
    //     let index = map.insert(Long(BString::from("One")), Arg::with_name("Value1"));

    //     map.insert_key_by_name(Long(BString::from("Two")), "Value1");

    //     assert_eq!(
    //         map.get(Long(BString::from("One"))),
    //         map.get(Long(BString::from("Two")))
    //     );
    //     assert_eq!(map.values.len(), 1);
    // }

    #[test]
    fn get_mutable() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Long(BString::from("One")), Arg::with_name("Value1"));

        assert_eq!(
            map.get_mut(&Long(BString::from("One"))),
            Some(&mut Arg::with_name("Value1"))
        );
    }

    #[test]
    fn remove_key() {
        let mut map: MKeyMap = MKeyMap::new();
        let index = map.insert(Long(BString::from("One")), Arg::with_name("Value1"));

        map.insert_key(Long(BString::from("Two")), index);
        map.remove_key(&Long(BString::from("One")));

        assert_eq!(map.keys.len(), 1);
        assert_eq!(map.args.len(), 1);
    }
}
