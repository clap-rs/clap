pub enum Key {
    Short,
    Long,
    Index,
    Id,
}

pub struct SeenArg {
    id: u64,
    key: Key
}