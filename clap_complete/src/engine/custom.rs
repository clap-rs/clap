use std::any::type_name;
use std::sync::Arc;

use clap::builder::ArgExt;

use super::CompletionCandidate;

/// Extend [`Arg`][clap::Arg] with a [`ValueCandidates`]
///
/// # Example
///
/// ```rust
/// use clap::Parser;
/// use clap_complete::engine::{ArgValueCandidates, CompletionCandidate};
///
/// #[derive(Debug, Parser)]
/// struct Cli {
///     #[arg(long, add = ArgValueCandidates::new(|| { vec![
///         CompletionCandidate::new("foo"),
///         CompletionCandidate::new("bar"),
///         CompletionCandidate::new("baz")] }))]
///     custom: Option<String>,
/// }
/// ```
#[derive(Clone)]
pub struct ArgValueCandidates(Arc<dyn ValueCandidates>);

impl ArgValueCandidates {
    /// Create a new `ArgValueCandidates` with a custom completer
    pub fn new<C>(completer: C) -> Self
    where
        C: ValueCandidates + 'static,
    {
        Self(Arc::new(completer))
    }

    /// All potential candidates for an argument.
    ///
    /// See [`CompletionCandidate`] for more information.
    pub fn candidates(&self) -> Vec<CompletionCandidate> {
        self.0.candidates()
    }
}

impl std::fmt::Debug for ArgValueCandidates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(type_name::<Self>())
    }
}

impl ArgExt for ArgValueCandidates {}

/// User-provided completion candidates for an [`Arg`][clap::Arg], see [`ArgValueCandidates`]
///
/// This is useful when predefined value hints are not enough.
pub trait ValueCandidates: Send + Sync {
    /// All potential candidates for an argument.
    ///
    /// See [`CompletionCandidate`] for more information.
    fn candidates(&self) -> Vec<CompletionCandidate>;
}

impl<F> ValueCandidates for F
where
    F: Fn() -> Vec<CompletionCandidate> + Send + Sync,
{
    fn candidates(&self) -> Vec<CompletionCandidate> {
        self()
    }
}
