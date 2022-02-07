/// Semantics for a piece of error information
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContextKind {
    /// The cause of the error
    InvalidSubcommand,
    /// Existing subcommand
    ValidSubcommand,
    /// The cause of the error
    InvalidArg,
    /// Existing arguments
    ValidArg,
    /// Accepted values
    ValidValue,
    /// Rejected values
    InvalidValue,
    /// Highest allowed
    MaxValue,
    /// Lowest allowed
    MinValue,
    /// Potential fix for the user
    SuggestedCommand,
    /// Trailing argument
    TrailingArg,
    /// A usage string
    Usage,
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
    Bool(bool),
    /// A single value
    String(String),
    /// Many values
    Strings(Vec<String>),
    /// A single value
    Number(isize),
}
