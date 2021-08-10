#![allow(clippy::single_component_path_imports)]

mod argstr;
mod fnv;
mod graph;
mod id;
mod str_to_bool;

pub use self::fnv::Key;

pub(crate) use self::{argstr::ArgStr, graph::ChildGraph, id::Id, str_to_bool::str_to_bool};
pub(crate) use vec_map::VecMap;

#[cfg(feature = "color")]
pub(crate) use termcolor;

#[cfg(not(feature = "color"))]
pub(crate) mod termcolor;

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
