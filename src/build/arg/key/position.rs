pub struct Position(u64);

impl Default for Position {
    fn default() -> Self {
        Position(1)
    }
}

impl Position {
    pub fn new() -> Self {
        Position(1)
    }
    pub fn at(i: u64) -> Self {
        assert!(i>0, "Positional Index cannot be less than 1");
        Position(i)
    }
}