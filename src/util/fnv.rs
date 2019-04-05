use std::hash::{Hash, Hasher};

// precompute some common values
pub static HELP_HASH: u64 = 0x5963_6393_CFFB_FE5F;
pub static VERSION_HASH: u64 = 0x30FF_0B7C_4D07_9478;
pub static EMPTY_HASH: u64 = 0x1C9D_3ADB_639F_298E;
const MAGIC_INIT: u64 = 0x811C_9DC5;

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
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        *self = FnvHasher(hash);
    }
}
