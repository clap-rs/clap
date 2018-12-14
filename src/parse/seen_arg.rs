use parse::KeyType;

pub struct SeenArg {
    id: u64,
    key: Key
}

impl SeenArg {
    pub(crate) fn new(id: u64, key: Key) -> Self {
        SeenArg {
            id,
            key
        }
    }
}