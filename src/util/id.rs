#[derive(Default, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id {
    name: String,
}

impl Id {
    pub(crate) const HELP: &'static str = "help";
    pub(crate) const VERSION: &'static str = "version";
    pub(crate) const EXTERNAL: &'static str = "";

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
        Self { name }
    }
}

impl<'s> From<&'s String> for Id {
    fn from(name: &'s String) -> Self {
        name.to_owned().into()
    }
}

impl From<&'static str> for Id {
    fn from(name: &'static str) -> Self {
        name.to_owned().into()
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
