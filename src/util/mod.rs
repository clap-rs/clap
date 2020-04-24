#![allow(clippy::single_component_path_imports)]

mod argstr;
mod fnv;
mod graph;
mod id;

pub use self::fnv::Key;

pub(crate) use self::{argstr::ArgStr, graph::ChildGraph, id::Id};
pub(crate) use vec_map::VecMap;

#[cfg(feature = "color")]
pub(crate) use termcolor;

#[cfg(not(feature = "color"))]
pub(crate) mod termcolor;
