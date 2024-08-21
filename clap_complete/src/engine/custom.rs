use std::any::type_name;
use std::ffi::OsStr;
use std::sync::Arc;

use clap::builder::ArgExt;
use clap_lex::OsStrExt as _;

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

pub(crate) fn complete_path(
    value_os: &OsStr,
    current_dir: Option<&std::path::Path>,
    is_wanted: &dyn Fn(&std::path::Path) -> bool,
) -> Vec<CompletionCandidate> {
    let mut completions = Vec::new();

    let value_path = std::path::Path::new(value_os);
    let (prefix, current) = split_file_name(value_path);
    let current = current.to_string_lossy();
    let search_root = if prefix.is_absolute() {
        prefix.to_owned()
    } else {
        let current_dir = match current_dir {
            Some(current_dir) => current_dir,
            None => {
                // Can't complete without a `current_dir`
                return Vec::new();
            }
        };
        current_dir.join(prefix)
    };
    debug!("complete_path: search_root={search_root:?}, prefix={prefix:?}");

    for entry in std::fs::read_dir(&search_root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
    {
        let raw_file_name = entry.file_name();
        if !raw_file_name.starts_with(&current) {
            continue;
        }

        if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            let mut suggestion = prefix.join(raw_file_name);
            suggestion.push(""); // Ensure trailing `/`
            completions.push(CompletionCandidate::new(suggestion.as_os_str().to_owned()));
        } else {
            let path = entry.path();
            if is_wanted(&path) {
                let suggestion = prefix.join(raw_file_name);
                completions.push(CompletionCandidate::new(suggestion.as_os_str().to_owned()));
            }
        }
    }

    completions
}

fn split_file_name(path: &std::path::Path) -> (&std::path::Path, &OsStr) {
    // Workaround that `Path::new("name/").file_name()` reports `"name"`
    if path_has_name(path) {
        (
            path.parent().unwrap_or_else(|| std::path::Path::new("")),
            path.file_name().expect("not called with `..`"),
        )
    } else {
        (path, Default::default())
    }
}

fn path_has_name(path: &std::path::Path) -> bool {
    let path_bytes = path.as_os_str().as_encoded_bytes();
    let Some(trailing) = path_bytes.last() else {
        return false;
    };
    let trailing = *trailing as char;
    !std::path::is_separator(trailing) && path.file_name().is_some()
}
