use std::cmp::PartialEq;

use parse::KeyType;

pub struct SeenArg {
    id: u64,
    key: KeyType
}

impl SeenArg {
    pub(crate) fn new(id: u64, key: KeyType) -> Self {
        SeenArg {
            id,
            key
        }
    }
}

impl PartialEq<u64> for SeenArg {
    fn eq(&self, rhs: &u64) -> bool {
        self.id == *rhs
    }
}