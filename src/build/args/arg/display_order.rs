pub struct DisplayOrder(usize);

impl Default for DisplayOrder {
    fn default() -> Self {
        DisplayOrder(999)
    }
}

impl DisplayOrder {
    pub fn new() -> Self {
        DisplayOrder::default()
    }
}