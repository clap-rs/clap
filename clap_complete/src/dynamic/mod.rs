//! Complete commands within shells
//!
//! For quick-start, see [`shells::CompleteCommand`]

mod candidate;
mod complete;
mod custom;

pub mod shells;

pub use candidate::CompletionCandidate;
pub use complete::complete;
pub use custom::ArgValueCompleter;
pub use custom::CustomCompleter;
