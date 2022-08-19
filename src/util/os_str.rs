/// A UTF-8-encoded fixed string
#[derive(Default, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct OsStr {
    name: Inner,
}

impl OsStr {
    pub(crate) fn from_string(name: std::ffi::OsString) -> Self {
        Self {
            name: Inner::Owned(name.into_boxed_os_str()),
        }
    }

    pub(crate) fn from_ref(name: &std::ffi::OsStr) -> Self {
        Self {
            name: Inner::Owned(Box::from(name)),
        }
    }

    pub(crate) const fn from_static_ref(name: &'static std::ffi::OsStr) -> Self {
        Self {
            name: Inner::Static(name),
        }
    }

    /// Get the raw string as an `std::ffi::OsStr`
    pub fn as_os_str(&self) -> &std::ffi::OsStr {
        self.name.as_os_str()
    }

    /// Get the raw string as an `OsString`
    pub fn to_os_string(&self) -> std::ffi::OsString {
        self.as_os_str().to_owned()
    }
}

impl From<&'_ OsStr> for OsStr {
    fn from(id: &'_ OsStr) -> Self {
        id.clone()
    }
}

impl From<crate::Str> for OsStr {
    fn from(id: crate::Str) -> Self {
        match id.into_inner() {
            crate::util::StrInner::Static(s) => Self::from_static_ref(std::ffi::OsStr::new(s)),
            crate::util::StrInner::Owned(s) => Self::from_ref(std::ffi::OsStr::new(s.as_ref())),
        }
    }
}

impl From<&'_ crate::Str> for OsStr {
    fn from(id: &'_ crate::Str) -> Self {
        match id.clone().into_inner() {
            crate::util::StrInner::Static(s) => Self::from_static_ref(std::ffi::OsStr::new(s)),
            crate::util::StrInner::Owned(s) => Self::from_ref(std::ffi::OsStr::new(s.as_ref())),
        }
    }
}

impl From<std::ffi::OsString> for OsStr {
    fn from(name: std::ffi::OsString) -> Self {
        Self::from_string(name)
    }
}

impl From<&'_ std::ffi::OsString> for OsStr {
    fn from(name: &'_ std::ffi::OsString) -> Self {
        Self::from_ref(name.as_os_str())
    }
}

impl From<std::string::String> for OsStr {
    fn from(name: std::string::String) -> Self {
        Self::from_string(name.into())
    }
}

impl From<&'_ std::string::String> for OsStr {
    fn from(name: &'_ std::string::String) -> Self {
        Self::from_ref(name.as_str().as_ref())
    }
}

impl From<&'static std::ffi::OsStr> for OsStr {
    fn from(name: &'static std::ffi::OsStr) -> Self {
        Self::from_static_ref(name)
    }
}

impl From<&'_ &'static std::ffi::OsStr> for OsStr {
    fn from(name: &'_ &'static std::ffi::OsStr) -> Self {
        Self::from_static_ref(*name)
    }
}

impl From<&'static str> for OsStr {
    fn from(name: &'static str) -> Self {
        Self::from_static_ref(name.as_ref())
    }
}

impl From<&'_ &'static str> for OsStr {
    fn from(name: &'_ &'static str) -> Self {
        Self::from_static_ref((*name).as_ref())
    }
}

impl std::fmt::Debug for OsStr {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_os_str(), f)
    }
}

impl std::ops::Deref for OsStr {
    type Target = std::ffi::OsStr;

    #[inline]
    fn deref(&self) -> &std::ffi::OsStr {
        self.as_os_str()
    }
}

impl AsRef<std::ffi::OsStr> for OsStr {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_os_str()
    }
}

impl AsRef<std::path::Path> for OsStr {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self)
    }
}

impl std::borrow::Borrow<std::ffi::OsStr> for OsStr {
    #[inline]
    fn borrow(&self) -> &std::ffi::OsStr {
        self.as_os_str()
    }
}

impl PartialEq<str> for OsStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_os_str(), other)
    }
}

impl PartialEq<&'_ str> for OsStr {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_os_str(), *other)
    }
}

impl<'s> PartialEq<&'s std::ffi::OsStr> for OsStr {
    #[inline]
    fn eq(&self, other: &&std::ffi::OsStr) -> bool {
        PartialEq::eq(self.as_os_str(), *other)
    }
}

impl PartialEq<std::string::String> for OsStr {
    #[inline]
    fn eq(&self, other: &std::string::String) -> bool {
        PartialEq::eq(self.as_os_str(), other.as_str())
    }
}

impl PartialEq<std::ffi::OsString> for OsStr {
    #[inline]
    fn eq(&self, other: &std::ffi::OsString) -> bool {
        PartialEq::eq(self.as_os_str(), other.as_os_str())
    }
}

#[derive(Clone)]
enum Inner {
    Static(&'static std::ffi::OsStr),
    Owned(Box<std::ffi::OsStr>),
}

impl Inner {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s.as_ref(),
        }
    }
}

impl Default for Inner {
    fn default() -> Self {
        Self::Static(std::ffi::OsStr::new(""))
    }
}

impl PartialEq for Inner {
    fn eq(&self, other: &Inner) -> bool {
        self.as_os_str() == other.as_os_str()
    }
}

impl PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_os_str().partial_cmp(other.as_os_str())
    }
}

impl Ord for Inner {
    fn cmp(&self, other: &Inner) -> std::cmp::Ordering {
        self.as_os_str().cmp(other.as_os_str())
    }
}

impl Eq for Inner {}

impl std::hash::Hash for Inner {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_os_str().hash(state);
    }
}
