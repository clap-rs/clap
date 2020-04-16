#[cfg(feature = "vec_map")]
pub(crate) use vec_map::VecMap;

#[cfg(not(feature = "vec_map"))]
pub(crate) use self::vec_map::VecMap;

#[cfg(not(feature = "vec_map"))]
mod vec_map {
    use std::collections::btree_map;
    use std::collections::BTreeMap;
    use std::fmt::{self, Debug, Formatter};

    #[derive(Clone, Default, Debug)]
    pub(crate) struct VecMap<V> {
        inner: BTreeMap<usize, V>,
    }

    impl<V> VecMap<V> {
        pub(crate) fn new() -> Self {
            VecMap {
                inner: Default::default(),
            }
        }

        pub(crate) fn len(&self) -> usize {
            self.inner.len()
        }

        pub(crate) fn is_empty(&self) -> bool {
            self.inner.is_empty()
        }

        pub(crate) fn insert(&mut self, key: usize, value: V) -> Option<V> {
            self.inner.insert(key, value)
        }

        pub(crate) fn values(&self) -> Values<V> {
            self.inner.values()
        }

        pub(crate) fn keys(&self) -> btree_map::Keys<usize, V> {
            self.inner.keys()
        }

        pub(crate) fn iter(&self) -> Iter<V> {
            Iter {
                inner: self.inner.iter(),
            }
        }

        pub(crate) fn contains_key(&self, key: usize) -> bool {
            self.inner.contains_key(&key)
        }

        pub(crate) fn entry(&mut self, key: usize) -> Entry<V> {
            self.inner.entry(key)
        }

        pub(crate) fn get(&self, key: usize) -> Option<&V> {
            self.inner.get(&key)
        }
    }

    pub(crate) type Values<'a, V> = btree_map::Values<'a, usize, V>;

    pub(crate) type Entry<'a, V> = btree_map::Entry<'a, usize, V>;

    #[derive(Clone)]
    pub(crate) struct Iter<'a, V: 'a> {
        inner: btree_map::Iter<'a, usize, V>,
    }

    impl<'a, V: 'a + Debug> Debug for Iter<'a, V> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            f.debug_list().entries(self.inner.clone()).finish()
        }
    }

    impl<'a, V: 'a> Iterator for Iter<'a, V> {
        type Item = (usize, &'a V);

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|(k, v)| (*k, v))
        }
    }

    impl<'a, V: 'a> DoubleEndedIterator for Iter<'a, V> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.inner.next_back().map(|(k, v)| (*k, v))
        }
    }
}
