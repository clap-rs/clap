//! `clap`-native completion system
//!
//! See [`complete()`]

mod candidate;
mod complete;
mod custom;

pub use candidate::CompletionCandidate;
pub use complete::complete;
pub use custom::ArgValueCompleter;
pub use custom::CustomCompleter;
