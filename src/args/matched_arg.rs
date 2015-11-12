use vec_map::VecMap;

#[doc(hidden)]
#[derive(Debug)]
pub struct MatchedArg {
    #[doc(hidden)]
    pub occurrences: u8,
    #[doc(hidden)]
    pub values: Option<VecMap<String>>,
}

impl MatchedArg {
    pub fn new() -> Self {
        MatchedArg {
            occurrences: 1,
            values: None
        }
    }
}
