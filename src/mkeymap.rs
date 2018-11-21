use std::collections::HashMap;

use build::Arg;
use std::ffi::{OsStr, OsString};
use util::hash;

// LE
fn short_to_bytes(s: char) -> [u8; 8] {
    let bytes = ['-', s];
    let h = '-' as u32;
    let c = s as u32;

    let b1 : u8 = ((h >> 24) & 0xff) as u8;
    let b2 : u8 = ((h >> 16) & 0xff) as u8;
    let b3 : u8 = ((h >> 8) & 0xff) as u8;
    let b4 : u8 = (h & 0xff) as u8;
    let b5 : u8 = ((c >> 24) & 0xff) as u8;
    let b6 : u8 = ((c >> 16) & 0xff) as u8;
    let b7 : u8 = ((c >> 8) & 0xff) as u8;
    let b8 : u8 = (c & 0xff) as u8;

    [b1, b1, b3, b4, b5, b6, b7, b8]
}

// LE
fn u64_to_bytes(u: u64) -> [u8; 8] {
    let b1 : u8 = ((u >> 56) & 0xff) as u8;
    let b2 : u8 = ((u >> 48) & 0xff) as u8;
    let b3 : u8 = ((u >> 40) & 0xff) as u8;
    let b4 : u8 = ((u >> 32) & 0xff) as u8;
    let b5 : u8 = ((u >> 24) & 0xff) as u8;
    let b6 : u8 = ((u >> 16) & 0xff) as u8;
    let b7 : u8 = ((u >> 8) & 0xff) as u8;
    let b8 : u8 = (u & 0xff) as u8;

    [b1, b1, b3, b4, b5, b6, b7, b8]
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct MKeyMap<'help> {
    pub index_map: HashMap<u64, usize>,
    pub args: Vec<Arg<'help>>,
    built: bool, // mutation isn't possible after being built
}

impl<'help> MKeyMap<'help> {
    pub fn new() -> Self { MKeyMap::default() }
    //TODO ::from(x), ::with_capacity(n) etc

    pub fn contains_long(&self, l: &str) -> bool { self.index_map.get(&hash(l.as_bytes())).is_some() }

    pub fn contains_short(&self, c: char) -> bool { self.index_map.get(&hash(short_to_bytes(c))).is_some() }

    pub fn insert(&mut self, arg: Arg<'help>) -> usize {
        assert!(!self.built, "Cannot add Args to the map after the map is built");

        let index = self.args.len();
        self.insert_keys(&arg, index);
        self.args.push(arg);
        index
    }

    pub fn push(&mut self, arg: Arg<'help>) -> usize {
        assert!(!self.built, "Cannot add Args to the map after the map is built");

        let index = self.args.len();
        self.args.push(arg);
        index
    }

    //TODO ::push_many([x, y])

    pub fn insert_short_key(&mut self, key: char, index: usize) {
        self.index_map.insert(hash(short_to_bytes(key)), index);
    }

    pub fn insert_long_key(&mut self, key: &str, index: usize) {
        self.index_map.insert(hash(key.as_bytes()), index);
    }

    pub fn insert_positional_key(&mut self, key: u64, index: usize) {
        self.index_map.insert(hash(u64_to_bytes(key)), index);
    }

    pub fn get_by_id(&self, key: u64) -> Option<&Arg<'help>> {
        self.args.get(*self.index_map.get(&key)?)
    }
    pub fn get_by_index(&self, key: usize) -> Option<&Arg<'help>> {
        self.args.get(key)
    }
    pub fn get_by_short(&self, key: char) -> Option<&Arg<'help>> {
        self.args.get(*self.index_map.get(&hash(short_to_bytes(key)))?)
    }
    // &[u8] better?
    pub fn get_by_short_with_hyphen(&self, key: [u8; 8]) -> Option<&Arg<'help>> {
        self.args.get(*self.index_map.get(&hash(&key))?)
    }
    pub fn get_by_long(&self, key: &str) -> Option<&Arg<'help>> {
        self.args.get(*self.index_map.get(&hash(key.as_bytes()))?)
    }
    pub fn get_by_long_with_hyphen(&self, key: &[u8]) -> Option<&Arg<'help>> {
        self.args.get(*self.index_map.get(&hash(key))?)
    }
    pub fn get_by_positional(&self, key: u64) -> Option<&Arg<'help>> {
        self.args.get(*self.index_map.get(&hash(u64_to_bytes(key)))?)
    }

    pub fn get_mut_by_id(&self, key: u64) -> Option<&mut Arg<'help>> {
        self.args.get_mut(*self.index_map.get(&key)?)
    }
    pub fn get_mut_by_index(&self, key: usize) -> Option<&mut Arg<'help>> {
        self.args.get_mut(key)
    }
    pub fn get_mut_by_short(&self, key: char) -> Option<&mut Arg<'help>> {
        self.args.get_mut(*self.index_map.get(&hash(short_to_bytes(key)))?)
    }
    // &[u8] better?
    pub fn get_mut_by_short_with_hyphen(&self, key: [u8; 8]) -> Option<&mut Arg<'help>> {
        self.args.get_mut(*self.index_map.get(&hash(&key))?)
    }
    pub fn get_mut_by_long(&self, key: &str) -> Option<&mut Arg<'help>> {
        self.args.get_mut(*self.index_map.get(&hash(key.as_bytes()))?)
    }
    pub fn get_mut_by_long_with_hyphen(&self, key: &[u8]) -> Option<&mut Arg<'help>> {
        self.args.get_mut(*self.index_map.get(&hash(key))?)
    }
    pub fn get_mut_by_positional(&self, key: u64) -> Option<&mut Arg<'help>> {
        self.args.get_mut(*self.index_map.get(&hash(u64_to_bytes(key)))?)
    }

    pub fn is_empty(&self) -> bool { self.args.is_empty() }

    // Remove Key
    pub fn remove_id_key(&mut self, key: u64) -> Option<usize> {
        self.index_map.remove(&key)
    }
    pub fn remove_short_key(&mut self, key: char) -> Option<usize> {
        self.index_map.remove(&hash(short_to_bytes(key)))
    }
    pub fn remove_short_key_with_hyphen(&mut self, key: [u8; 8]) -> Option<usize> {
        self.index_map.remove(&hash(&key))
    }
    pub fn remove_long_key(&mut self, key: &str) -> Option<usize> {
        self.index_map.remove(&hash(key.as_bytes()))
    }
    pub fn remove_long_key_with_hyphen(&mut self, key: &[u8]) -> Option<usize> {
        self.index_map.remove(&hash(key))
    }
    pub fn remove_positional_key(&mut self, key: u64) -> Option<usize> {
        self.index_map.remove(&hash(u64_to_bytes(key)))
    }

    pub fn remove_by_id(&mut self, id: u64) -> Option<Arg> {
        assert!(!self.built, "Cannot remove args once built");
        let i = self.index_map.remove(&id)?;
        Some(self.args.swap_remove(i))
    }

    //TODO ::remove_keys([KeyA, KeyB])

    fn insert_keys(&mut self, arg: &Arg, index: usize) {
        self.index_map.insert(arg.id, index);
        if let Some(p) = arg.index {
            self.index_map.insert(hash(u64_to_bytes(p)), index);
        } else {
            if let Some(s) = arg.short {
                self.index_map.insert(hash(short_to_bytes(s)), index);
            } 
            if let Some(l) = arg.long {
                self.index_map.insert(hash(l.as_bytes()), index);
            }
        }
    }

    pub fn _build(&mut self) {
        self.built = true;

        for (i, arg) in self.args.iter().enumerate() {
            self.insert_keys(arg, i);
        }
    }

    pub fn find_by_name(&mut self, name: u64) -> usize {
        self.args
            .iter()
            .position(|x| x.id == name)
            .expect("No such name found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::hash;

    #[test]
    fn get_some_value() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Arg::new("Value1").long("value"));

        assert_eq!(
            map.get_by_long_with_hyphen("--value".as_bytes()).unwrap().id,
            hash("Value1")
        );
    }

    #[test]
    fn get_none_value() {
        let mut map: MKeyMap = MKeyMap::new();

        map.insert(Arg::new("Value1").long("value"));

        assert_eq!(map.get_by_long("none"), None);
    }

    #[test]
    fn insert_multiple_keys() {
        let mut map: MKeyMap = MKeyMap::new();
        let index = map.insert(Arg::new("Value1").long("value"));

        map.insert_long_key("other", index);

        assert_eq!(
            map.get_by_long("value"),
            map.get_by_long("other"),
        );
        assert_eq!(map.args.len(), 1);
    }

    #[test]
    fn remove_key() {
        let mut map: MKeyMap = MKeyMap::new();
        let index = map.insert(Arg::new("Value1").long("value"));
        map.insert_long_key("other", index);

        map.remove_long_key("other");

        assert_eq!(map.index_map.len(), 1);
        assert_eq!(map.args.len(), 1);
    }
}
