use std::fmt::Display;

/// TODO
#[derive(Debug, Default, Clone)]
pub struct ArgValue<'help> {
    pub(crate) value: &'help str,
    pub(crate) about: Option<&'help str>,
    pub(crate) hidden: bool,
}

impl Display for ArgValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.contains(char::is_whitespace) {
            write!(f, "{:?}", self.value)
        } else {
            write!(f, "{}", self.value)
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
    pub fn get_value(&self) -> &str {
        self.value
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
    pub fn new(name: &'help str) -> Self {
        let name = name;
        ArgValue {
            value: name,
            ..Default::default()
        }
    }

    /// TODO
    #[inline]
    pub fn about(mut self, about: &'help str) -> Self {
        self.about = Some(about);
        self
    }

    /// TODO
    #[inline]
    pub fn hidden(mut self, yes: bool) -> Self {
        self.hidden = yes;
        self
    }
}
