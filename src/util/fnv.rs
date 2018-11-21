use std::hash::{Hash, Hasher};

#[inline]
pub(crate) fn hash<T>(t: T) -> u64 where T:  Hash {
    let mut hasher = FnvHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

pub(crate) struct FnvHasher(u64);

impl FnvHasher {
    pub(crate) fn new() -> Self {
        FnvHasher(0x811C9DC5)
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        *self = FnvHasher(hash);
    }
}