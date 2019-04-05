use std::hash::{Hash, Hasher};

// precompute some common values
pub static HELP_HASH: u64 = 0x59636393CFFBFE5F;
pub static VERSION_HASH: u64 = 0x30FF0B7C4D079478;
pub static EMPTY_HASH: u64 = 0x1C9D3ADB639F298E;

const MAGIC_INIT: u64 = 0x811C9DC5;

pub trait Key: Hash {
    fn key(&self) -> u64;
}

impl<T> Key for T
where
    T: Hash,
{
    fn key(&self) -> u64 {
        let mut hasher = FnvHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub(crate) struct FnvHasher(u64);

impl FnvHasher {
    pub(crate) fn new() -> Self { FnvHasher(MAGIC_INIT) }
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
