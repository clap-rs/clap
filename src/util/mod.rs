#![allow(clippy::single_component_path_imports)]

mod graph;
mod id;
mod str_to_bool;

#[cfg(not(feature = "o1"))]
pub(crate) mod flat_map;
#[cfg(feature = "o1")]
pub(crate) mod flat_map {
    pub(crate) use indexmap::map::Entry;
    pub(crate) use indexmap::map::Iter;
    pub(crate) use indexmap::map::Keys;
    pub(crate) use indexmap::IndexMap as FlatMap;
}
#[cfg(not(feature = "o1"))]
pub(crate) mod flat_set;
#[cfg(feature = "o1")]
pub(crate) mod flat_set {
    pub(crate) use indexmap::IndexSet as FlatSet;
}

pub use self::id::Id;

pub(crate) use self::graph::ChildGraph;
pub(crate) use self::str_to_bool::str_to_bool;
pub(crate) use self::str_to_bool::FALSE_LITERALS;
pub(crate) use self::str_to_bool::TRUE_LITERALS;

pub(crate) use self::flat_map::Entry;
pub(crate) use self::flat_map::FlatMap;
pub(crate) use self::flat_set::FlatSet;

pub(crate) mod color;

pub(crate) const SUCCESS_CODE: i32 = 0;
// While sysexists.h defines EX_USAGE as 64, this doesn't seem to be used much in practice but
// instead 2 seems to be frequently used.
// Examples
// - GNU `ls` returns 2
// - Python's `argparse` returns 2
pub(crate) const USAGE_CODE: i32 = 2;

pub(crate) fn safe_exit(code: i32) -> ! {
    use std::io::Write;

    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stderr().lock().flush();

    std::process::exit(code)
}

#[cfg(not(feature = "unicode"))]
pub(crate) fn eq_ignore_case(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

#[cfg(feature = "unicode")]
pub(crate) use unicase::eq as eq_ignore_case;
