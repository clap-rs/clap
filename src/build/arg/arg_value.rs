/// The representation of a possible value of an argument.
///
/// This is used for specifying [possible values] of [Args].
///
/// **NOTE:** This struct is likely not needed for most usecases as it is only required to
/// [hide] single values from help messages and shell completions or to attach [about] to possible values.
///
/// # Examples
///
/// ```rust
/// # use clap::{Arg, ArgValue};
/// let cfg = Arg::new("config")
///       .takes_value(true)
///       .value_name("FILE")
///       .possible_value(ArgValue::new("fast"))
///       .possible_value(ArgValue::new("slow").about("slower than fast"))
///       .possible_value(ArgValue::new("secret speed").hidden(true));
/// ```
/// [Args]: crate::Arg
/// [possible values]: crate::Arg::possible_value()
/// [hide]: ArgValue::hidden()
/// [about]: ArgValue::about()
#[derive(Debug, Default, Clone)]
pub struct ArgValue<'help> {
    pub(crate) name: &'help str,
    pub(crate) about: Option<&'help str>,
    pub(crate) hidden: bool,
}

impl<'help> From<&'help str> for ArgValue<'help> {
    fn from(s: &'help str) -> Self {
        Self::new(s)
    }
}

/// Getters
impl<'help> ArgValue<'help> {
    /// Get the name of the argument value
    #[inline]
    pub fn get_name(&self) -> &str {
        self.name
    }

    /// Get the help specified for this argument, if any
    #[inline]
    pub fn get_about(&self) -> Option<&str> {
        self.about
    }

    /// Should the value be hidden from help messages and completion
    #[inline]
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Get the name if argument value is not hidden, `None` otherwise
    pub fn get_visible_name(&self) -> Option<&str> {
        if self.hidden {
            None
        } else {
            Some(self.name)
        }
    }
}

impl<'help> ArgValue<'help> {
    /// Creates a new instance of [`ArgValue`] using a string name. The name will be used to
    /// decide wether this value was provided by the user to an argument.
    ///
    /// **NOTE:** In case it is not [hidden] it will also be shown in help messages for arguments
    /// that use it as a [possible value] and have not hidden them through [`Arg::hide_possible_values(true)`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::ArgValue;
    /// ArgValue::new("fast")
    /// # ;
    /// ```
    /// [hidden]: ArgValue::hide
    /// [possible value]: crate::Arg::possible_values
    /// [`Arg::hide_possible_values(true)`]: crate::Arg::hide_possible_values()
    pub fn new(name: &'help str) -> Self {
        ArgValue {
            name,
            ..Default::default()
        }
    }

    /// Sets the help text of the value that will be displayed to the user when completing the
    /// value in a compatible shell. Typically, this is a short description of the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::ArgValue;
    /// ArgValue::new("slow")
    ///     .about("not fast")
    /// # ;
    /// ```
    #[inline]
    pub fn about(mut self, about: &'help str) -> Self {
        self.about = Some(about);
        self
    }

    /// Hides this value from help text and shell completions.
    ///
    /// This is an alternative to hiding through [`Arg::hide_possible_values(true)`], if you only
    /// want to hide some values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::ArgValue;
    /// ArgValue::new("secret")
    ///     .hidden(true)
    /// # ;
    /// ```
    /// [`Arg::hide_possible_values(true)`]: crate::Arg::hide_possible_values()
    #[inline]
    pub fn hidden(mut self, yes: bool) -> Self {
        self.hidden = yes;
        self
    }
}
