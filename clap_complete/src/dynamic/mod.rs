//! Complete commands within shells
//!
//! To customize completions, see
//! - [`ValueHint`][crate::ValueHint]
//! - [`ValueEnum`][clap::ValueEnum]
//! - [`ArgValueCompleter`]

mod candidate;
mod complete;
mod custom;

pub use candidate::CompletionCandidate;
pub use complete::complete;
pub use custom::ArgValueCompleter;
pub use custom::CustomCompleter;
