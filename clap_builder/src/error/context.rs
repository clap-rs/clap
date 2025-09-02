/// Semantics for a piece of error information
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg(feature = "error-context")]
pub enum ContextKind {
    /// The cause of the error
    InvalidSubcommand,
    /// The cause of the error
    InvalidArg,
    /// Existing arguments
    PriorArg,
    /// Accepted subcommands
    ValidSubcommand,
    /// Accepted values
    ValidValue,
    /// Rejected values
    InvalidValue,
    /// Number of values present
    ActualNumValues,
    /// Number of allowed values
    ExpectedNumValues,
    /// Minimum number of allowed values
    MinValues,
    /// Potential fix for the user
    SuggestedCommand,
    /// Potential fix for the user
    SuggestedSubcommand,
    /// Potential fix for the user
    SuggestedArg,
    /// Potential fix for the user
    SuggestedValue,
    /// Trailing argument
    TrailingArg,
    /// Potential fix for the user
    Suggested,
    /// A usage string
    Usage,
    /// An opaque message to the user
    Custom,
}

impl ContextKind {
    /// End-user description of the error case, where relevant
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Self::InvalidSubcommand => "Invalid Subcommand",
            Self::InvalidArg => "Invalid Argument",
            Self::PriorArg => "Prior Argument",
            Self::ValidSubcommand => "Valid Subcommand",
            Self::ValidValue => "Valid Value",
            Self::InvalidValue => "Invalid Value",
            Self::ActualNumValues => "Actual Number of Values",
            Self::ExpectedNumValues => "Expected Number of Values",
            Self::MinValues => "Minimum Number of Values",
            Self::SuggestedCommand => "Suggested Command",
            Self::SuggestedSubcommand => "Suggested Subcommand",
            Self::SuggestedArg => "Suggested Argument",
            Self::SuggestedValue => "Suggested Value",
            Self::TrailingArg => "Trailing Argument",
            Self::Suggested => "Suggested",
            Self::Usage | Self::Custom => return None,
        })
    }
}

impl std::fmt::Display for ContextKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().unwrap_or_default().fmt(f)
    }
}

/// A piece of error information
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[cfg(feature = "error-context")]
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
    StyledStr(crate::builder::StyledStr),
    /// many value
    StyledStrs(Vec<crate::builder::StyledStr>),
    /// A single value
    Number(isize),
}

impl std::fmt::Display for ContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => "".fmt(f),
            Self::Bool(v) => v.fmt(f),
            Self::String(v) => v.fmt(f),
            Self::Strings(v) => v.join(", ").fmt(f),
            Self::StyledStr(v) => v.fmt(f),
            Self::StyledStrs(v) => {
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        ", ".fmt(f)?;
                    }
                    v.fmt(f)?;
                }
                Ok(())
            }
            Self::Number(v) => v.fmt(f),
        }
    }
}
