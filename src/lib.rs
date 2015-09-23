//! clap2

mod arg;
mod app;
mod macros;
mod error;

pub use arg::{Accumulator, Rule};
pub use app::{App, AppBuilder, CollectionMatcher};
pub use error::ClapError;
