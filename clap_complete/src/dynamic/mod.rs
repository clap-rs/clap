//! Complete commands within shells
//!
//! For quick-start, see [`CompleteCommand`]
//!
//! To customize completions, see
//! - [`ValueHint`][crate::ValueHint]
//! - [`ValueEnum`][clap::ValueEnum]
//! - [`ArgValueCompleter`]

mod candidate;
mod complete;
mod custom;

pub mod shells;

pub use candidate::CompletionCandidate;
pub use complete::complete;
pub use custom::ArgValueCompleter;
pub use custom::CustomCompleter;

// These live in `shells` because they are tightly coupled with the `ShellCompleter`s
#[cfg(feature = "unstable-command")]
pub use shells::CompleteArgs;
#[cfg(feature = "unstable-command")]
pub use shells::CompleteCommand;
