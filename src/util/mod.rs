mod fnv;
mod graph;
mod id;
mod map;
mod osstringext;
mod strext;

pub use self::fnv::Key;

pub(crate) use self::{graph::ChildGraph, id::Id, map::VecMap, osstringext::OsStrExt2};

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
pub(crate) use self::osstringext::OsStrExt3;

#[cfg(feature = "color")]
pub(crate) use termcolor;

#[cfg(not(feature = "color"))]
pub(crate) mod termcolor;
