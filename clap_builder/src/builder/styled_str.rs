/// Terminal-styling container
///
/// Styling may be encoded as [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)
///
/// # Examples
///
/// ```rust
/// # use clap_builder as clap;
/// // `cstr!` converts tags to ANSI codes
/// let after_help: &'static str = color_print::cstr!(
/// r#"<bold><underline>Examples</underline></bold>
///
///   <dim>$</dim> <bold>mybin --input file.toml</bold>
/// "#);
///
/// let cmd = clap::Command::new("mybin")
///     .after_help(after_help)  // The `&str` gets converted into a `StyledStr`
///     // ...
/// #   ;
/// ```
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyledStr(String);

impl StyledStr {
    /// Create an empty buffer
    pub const fn new() -> Self {
        Self(String::new())
    }

    /// Display using [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code) styling
    #[cfg(feature = "color")]
    pub fn ansi(&self) -> impl std::fmt::Display + '_ {
        self.0.as_str()
    }

    #[cfg(feature = "color")]
    pub(crate) fn stylize(&mut self, style: Style, msg: &str) {
        if !msg.is_empty() {
            use std::fmt::Write as _;

            let style = style.as_style();
            let _ = write!(self.0, "{}{}{}", style.render(), msg, style.render_reset());
        }
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn stylize(&mut self, _style: Style, msg: &str) {
        self.0.push_str(msg);
    }

    pub(crate) fn none(&mut self, msg: impl Into<String>) {
        self.0.push_str(&msg.into());
    }

    pub(crate) fn trim(&mut self) {
        self.0 = self.0.trim().to_owned()
    }

    #[cfg(feature = "help")]
    pub(crate) fn replace_newline_var(&mut self) {
        self.0 = self.0.replace("{n}", "\n");
    }

    #[cfg(feature = "help")]
    pub(crate) fn indent(&mut self, initial: &str, trailing: &str) {
        self.0.insert_str(0, initial);

        let mut line_sep = "\n".to_owned();
        line_sep.push_str(trailing);
        self.0 = self.0.replace('\n', &line_sep);
    }

    #[cfg(all(not(feature = "wrap_help"), feature = "help"))]
    pub(crate) fn wrap(&mut self, _hard_width: usize) {}

    #[cfg(feature = "wrap_help")]
    pub(crate) fn wrap(&mut self, hard_width: usize) {
        let mut new = String::with_capacity(self.0.len());

        let mut last = 0;
        let mut wrapper = crate::output::textwrap::wrap_algorithms::LineWrapper::new(hard_width);
        for content in self.iter_text() {
            // Preserve styling
            let current = content.as_ptr() as usize - self.0.as_str().as_ptr() as usize;
            if last != current {
                new.push_str(&self.0.as_str()[last..current]);
            }
            last = current + content.len();

            for (i, line) in content.split_inclusive('\n').enumerate() {
                if 0 < i {
                    // reset char count on newline, skipping the start as we might have carried
                    // over from a prior block of styled text
                    wrapper.reset();
                }
                let line = crate::output::textwrap::word_separators::find_words_ascii_space(line)
                    .collect::<Vec<_>>();
                new.extend(wrapper.wrap(line));
            }
        }
        if last != self.0.len() {
            new.push_str(&self.0.as_str()[last..]);
        }
        new = new.trim_end().to_owned();

        self.0 = new;
    }

    #[inline(never)]
    #[cfg(feature = "help")]
    pub(crate) fn display_width(&self) -> usize {
        let mut width = 0;
        for c in self.iter_text() {
            width += crate::output::display_width(c);
        }
        width
    }

    #[cfg(feature = "help")]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[cfg(feature = "help")]
    pub(crate) fn as_styled_str(&self) -> &str {
        &self.0
    }

    #[cfg(feature = "color")]
    pub(crate) fn iter_text(&self) -> impl Iterator<Item = &str> {
        anstream::adapter::strip_str(&self.0)
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn iter_text(&self) -> impl Iterator<Item = &str> {
        [self.0.as_str()].into_iter()
    }

    pub(crate) fn push_styled(&mut self, other: &Self) {
        self.0.push_str(&other.0);
    }

    pub(crate) fn write_to(&self, buffer: &mut dyn std::io::Write) -> std::io::Result<()> {
        ok!(buffer.write_all(self.0.as_bytes()));

        Ok(())
    }
}

impl Default for &'_ StyledStr {
    fn default() -> Self {
        static DEFAULT: StyledStr = StyledStr::new();
        &DEFAULT
    }
}

impl From<std::string::String> for StyledStr {
    fn from(name: std::string::String) -> Self {
        StyledStr(name)
    }
}

impl From<&'_ std::string::String> for StyledStr {
    fn from(name: &'_ std::string::String) -> Self {
        let mut styled = StyledStr::new();
        styled.none(name);
        styled
    }
}

impl From<&'static str> for StyledStr {
    fn from(name: &'static str) -> Self {
        let mut styled = StyledStr::new();
        styled.none(name);
        styled
    }
}

impl From<&'_ &'static str> for StyledStr {
    fn from(name: &'_ &'static str) -> Self {
        StyledStr::from(*name)
    }
}

impl std::fmt::Write for StyledStr {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.0.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error> {
        self.0.push(c);
        Ok(())
    }
}

/// Color-unaware printing. Never uses coloring.
impl std::fmt::Display for StyledStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for part in self.iter_text() {
            part.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Style {
    #[allow(dead_code)]
    Header,
    #[allow(dead_code)]
    Literal,
    #[allow(dead_code)]
    Placeholder,
    #[allow(dead_code)]
    Good,
    #[allow(dead_code)]
    Warning,
    #[allow(dead_code)]
    Error,
    #[allow(dead_code)]
    Hint,
}

impl Style {
    #[cfg(feature = "color")]
    fn as_style(&self) -> anstyle::Style {
        match self {
            Style::Header => (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE).into(),
            Style::Literal => anstyle::Effects::BOLD.into(),
            Style::Placeholder => anstyle::Style::default(),
            Style::Good => anstyle::AnsiColor::Green.on_default(),
            Style::Warning => anstyle::AnsiColor::Yellow.on_default(),
            Style::Error => anstyle::AnsiColor::Red.on_default() | anstyle::Effects::BOLD,
            Style::Hint => anstyle::Effects::DIMMED.into(),
        }
    }
}
