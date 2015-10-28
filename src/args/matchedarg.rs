use std::collections::BTreeMap;

#[doc(hidden)]
#[derive(Debug)]
pub struct MatchedArg {
    // #[doc(hidden)]
    // pub name: String,
    #[doc(hidden)]
    pub occurrences: u8,
    #[doc(hidden)]
    // Consider VecMap<String> once stablized
    pub values: Option<BTreeMap<u8, String>>,
}
