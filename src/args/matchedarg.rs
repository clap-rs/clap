use std::collections::BTreeMap;

#[doc(hidden)]
pub struct MatchedArg {
	// #[doc(hidden)]
    // pub name: String,
	#[doc(hidden)]
    pub occurrences: u8,
	#[doc(hidden)]
    pub values: Option<BTreeMap<u8, String>>
}
