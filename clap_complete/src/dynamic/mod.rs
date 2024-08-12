//! Complete commands within shells
//!
//! To customize completions, see
//! - [`ValueHint`][crate::ValueHint]
//! - [`ValueEnum`][clap::ValueEnum]
//! - [`ArgValueCompleter`]

mod candidate;
mod complete;
mod custom;

#[cfg(feature = "unstable-command")]
pub mod command;
pub mod env;

pub use candidate::CompletionCandidate;
pub use complete::complete;
pub use custom::ArgValueCompleter;
pub use custom::CustomCompleter;

#[cfg(feature = "unstable-command")]
pub use command::CompleteArgs;
#[cfg(feature = "unstable-command")]
pub use command::CompleteCommand;
pub use env::CompleteEnv;
