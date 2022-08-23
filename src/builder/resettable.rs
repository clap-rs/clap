// Unlike `impl Into<Option<T>>` or `Option<impl Into<T>>`, this isn't ambiguous for the `None`
// case.

use crate::builder::OsStr;
use crate::builder::Str;

/// Clearable builder value
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resettable<T> {
    /// Overwrite builder value
    Value(T),
    /// Reset builder value
    Reset,
}

impl<T> Resettable<T> {
    pub(crate) fn into_option(self) -> Option<T> {
        match self {
            Self::Value(t) => Some(t),
            Self::Reset => None,
        }
    }
}

impl<T> From<T> for Resettable<T> {
    fn from(other: T) -> Self {
        Self::Value(other)
    }
}

impl<T> From<Option<T>> for Resettable<T> {
    fn from(other: Option<T>) -> Self {
        match other {
            Some(inner) => Self::Value(inner),
            None => Self::Reset,
        }
    }
}

/// Convert to the intended resettable type
pub trait IntoResettable<T> {
    /// Convert to the intended resettable type
    fn into_resettable(self) -> Resettable<T>;
}

impl IntoResettable<OsStr> for Option<&'static str> {
    fn into_resettable(self) -> Resettable<OsStr> {
        match self {
            Some(s) => Resettable::Value(s.into()),
            None => Resettable::Reset,
        }
    }
}

impl IntoResettable<Str> for Option<&'static str> {
    fn into_resettable(self) -> Resettable<Str> {
        match self {
            Some(s) => Resettable::Value(s.into()),
            None => Resettable::Reset,
        }
    }
}

impl<T> IntoResettable<T> for Resettable<T> {
    fn into_resettable(self) -> Resettable<T> {
        self
    }
}

impl<I: Into<OsStr>> IntoResettable<OsStr> for I {
    fn into_resettable(self) -> Resettable<OsStr> {
        Resettable::Value(self.into())
    }
}

impl<I: Into<Str>> IntoResettable<Str> for I {
    fn into_resettable(self) -> Resettable<Str> {
        Resettable::Value(self.into())
    }
}

impl<I: Into<crate::Id>> IntoResettable<crate::Id> for I {
    fn into_resettable(self) -> Resettable<crate::Id> {
        Resettable::Value(self.into())
    }
}
