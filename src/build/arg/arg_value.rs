use std::fmt::Display;

/// TODO
#[derive(Debug, Default, Clone)]
pub struct ArgValue<'help> {
    pub(crate) name: &'help str,
    pub(crate) about: Option<&'help str>,
    pub(crate) hidden: bool,
}

impl Display for ArgValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.contains(char::is_whitespace) {
            write!(f, "{:?}", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl<'help> From<&'help str> for ArgValue<'help> {
    fn from(s: &'help str) -> Self {
        Self::new(s)
    }
}

impl<'help> From<&&'help str> for ArgValue<'help> {
    fn from(s: &&'help str) -> Self {
        Self::new(*s)
    }
}

/// Getters
impl<'help> ArgValue<'help> {
    /// TODO
    #[inline]
    pub fn get_name(&self) -> &str {
        self.name
    }

    /// TODO
    #[inline]
    pub fn get_about(&self) -> Option<&str> {
        self.about
    }

    /// TODO
    #[inline]
    pub fn get_hidden(&self) -> bool {
        self.hidden
    }
}

impl<'help> ArgValue<'help> {
    /// TODO
    pub fn new<S: Into<&'help str>>(n: S) -> Self {
        let name = n.into();
        ArgValue {
            name,
            ..Default::default()
        }
    }

    /// TODO
    #[inline]
    pub fn about<S: Into<&'help str>>(mut self, a: S) -> Self {
        self.about = Some(a.into());
        self
    }

    /// TODO
    #[inline]
    pub fn hidden(mut self, h: bool) -> Self {
        self.hidden = h;
        self
    }
}
