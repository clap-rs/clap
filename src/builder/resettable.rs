// Unlike `impl Into<Option<T>>` or `Option<impl Into<T>>`, this isn't ambiguous for the `None`
// case.

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

/// Convert to the intended resettable type
pub trait IntoResettable<T> {
    /// Convert to the intended resettable type
    fn into_resettable(self) -> Resettable<T>;
}

impl IntoResettable<crate::OsStr> for Option<&'static str> {
    fn into_resettable(self) -> Resettable<crate::OsStr> {
        match self {
            Some(s) => Resettable::Value(s.into()),
            None => Resettable::Reset,
        }
    }
}

impl<I: Into<crate::OsStr>> IntoResettable<crate::OsStr> for I {
    fn into_resettable(self) -> Resettable<crate::OsStr> {
        Resettable::Value(self.into())
    }
}

impl<I: Into<crate::Str>> IntoResettable<crate::Str> for I {
    fn into_resettable(self) -> Resettable<crate::Str> {
        Resettable::Value(self.into())
    }
}

impl<I: Into<crate::Id>> IntoResettable<crate::Id> for I {
    fn into_resettable(self) -> Resettable<crate::Id> {
        Resettable::Value(self.into())
    }
}
