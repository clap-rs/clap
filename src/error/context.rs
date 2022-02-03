/// Semantics for a piece of error information
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContextKind {
    /// An opaque message to the user
    Custom,
}

/// A piece of error information
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContextValue {
    /// [`ContextKind`] is self-sufficient, no additional information needed
    None,
    /// A single value
    Value(String),
}
