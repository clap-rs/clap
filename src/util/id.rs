/// [`Arg`][crate::Arg] or [`ArgGroup`][crate::ArgGroup] identifier
///
/// This is used for accessing the value in [`ArgMatches`][crate::ArgMatches] or defining
/// relationships between `Arg`s and `ArgGroup`s with functions like
/// [`Arg::conflicts_with`][crate::Arg::conflicts_with].
#[derive(Default, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Id {
    name: Inner,
}

impl Id {
    pub(crate) const HELP: Self = Self::from_static_ref("help");
    pub(crate) const VERSION: Self = Self::from_static_ref("version");
    pub(crate) const EXTERNAL: Self = Self::from_static_ref("");

    fn from_string(name: String) -> Self {
        Self {
            name: Inner::Owned(name.into_boxed_str()),
        }
    }

    fn from_ref(name: &str) -> Self {
        Self {
            name: Inner::Owned(Box::from(name)),
        }
    }

    const fn from_static_ref(name: &'static str) -> Self {
        Self {
            name: Inner::Static(name),
        }
    }

    /// Get the raw string of the `Id`
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

impl<'s> From<&'s Id> for Id {
    fn from(id: &'s Id) -> Self {
        id.clone()
    }
}

impl From<String> for Id {
    fn from(name: String) -> Self {
        Self::from_string(name)
    }
}

impl From<&'_ String> for Id {
    fn from(name: &'_ String) -> Self {
        Self::from_ref(name.as_str())
    }
}

impl From<&'static str> for Id {
    fn from(name: &'static str) -> Self {
        Self::from_static_ref(name)
    }
}

impl From<&'_ &'static str> for Id {
    fn from(name: &'_ &'static str) -> Self {
        Self::from_static_ref(*name)
    }
}

impl std::fmt::Display for Id {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}

impl std::fmt::Debug for Id {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_str(), f)
    }
}

impl AsRef<str> for Id {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::borrow::Borrow<str> for Id {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for Id {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl<'s> PartialEq<&'s str> for Id {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl PartialEq<String> for Id {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

#[derive(Clone)]
enum Inner {
    Static(&'static str),
    Owned(Box<str>),
}

impl Inner {
    fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s.as_ref(),
        }
    }
}

impl Default for Inner {
    fn default() -> Self {
        Self::Static("")
    }
}

impl PartialEq for Inner {
    fn eq(&self, other: &Inner) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for Inner {
    fn cmp(&self, other: &Inner) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Eq for Inner {}

impl std::hash::Hash for Inner {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
