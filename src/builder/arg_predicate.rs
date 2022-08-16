use crate::OsStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ArgPredicate {
    IsPresent,
    Equals(OsStr),
}

impl<'help> From<Option<&'help std::ffi::OsStr>> for ArgPredicate {
    fn from(other: Option<&'help std::ffi::OsStr>) -> Self {
        match other {
            Some(other) => Self::Equals(other.to_owned().into()),
            None => Self::IsPresent,
        }
    }
}
