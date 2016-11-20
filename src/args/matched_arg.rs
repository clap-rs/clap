// Std
use std::ffi::OsString;

// Third Party
use vec_map::VecMap;

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct MatchedArg {
    #[doc(hidden)]
    pub occurs: u64,
    #[doc(hidden)]
    pub vals: VecMap<OsString>,
}

impl Default for MatchedArg {
    fn default() -> Self {
        MatchedArg {
            occurs: 1,
            vals: VecMap::new(),
        }
    }
}

impl MatchedArg {
    pub fn new() -> Self { MatchedArg::default() }
}
