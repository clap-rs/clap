#![allow(clippy::single_component_path_imports)]

mod argstr;
mod fnv;
mod graph;
mod id;

pub use self::fnv::Key;
pub use self::id::Id;

pub(crate) use self::{argstr::ArgStr, graph::ChildGraph};
pub(crate) use vec_map::VecMap;

#[cfg(feature = "color")]
pub(crate) use termcolor;

#[cfg(not(feature = "color"))]
pub(crate) mod termcolor;

pub(crate) fn safe_exit(code: i32) -> ! {
    use std::io::Write;

    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stderr().lock().flush();

    std::process::exit(code)
}
