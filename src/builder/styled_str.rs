use crate::output::display_width;

/// Terminal-styling container
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct StyledStr {
    pieces: Vec<(Option<Style>, String)>,
}

impl StyledStr {
    /// Create an empty buffer
    pub const fn new() -> Self {
        Self { pieces: Vec::new() }
    }

    pub(crate) fn good(&mut self, msg: impl Into<String>) {
        self.stylize(Some(Style::Good), msg.into());
    }

    pub(crate) fn warning(&mut self, msg: impl Into<String>) {
        self.stylize(Some(Style::Warning), msg.into());
    }

    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        self.stylize(Some(Style::Error), msg.into());
    }

    #[allow(dead_code)]
    pub(crate) fn hint(&mut self, msg: impl Into<String>) {
        self.stylize(Some(Style::Hint), msg.into());
    }

    pub(crate) fn none(&mut self, msg: impl Into<String>) {
        self.stylize(None, msg.into());
    }

    fn stylize(&mut self, style: Option<Style>, msg: String) {
        if !msg.is_empty() {
            self.pieces.push((style, msg));
        }
    }

    pub(crate) fn display_width(&self) -> usize {
        let mut width = 0;
        for (_, c) in &self.pieces {
            width += display_width(c);
        }
        width
    }

    /// HACK: Until call sites are updated to handle formatted text, extract the unformatted
    #[track_caller]
    pub(crate) fn unwrap_none(&self) -> &str {
        match self.pieces.len() {
            0 => "",
            1 => {
                if self.pieces[0].0 != None {
                    panic!("{}", crate::INTERNAL_ERROR_MSG)
                }
                self.pieces[0].1.as_str()
            }
            _ => panic!("{}", crate::INTERNAL_ERROR_MSG),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Option<Style>, &str)> {
        self.pieces.iter().map(|(s, c)| (*s, c.as_str()))
    }

    pub(crate) fn extend(
        &mut self,
        other: impl IntoIterator<Item = (impl Into<Option<Style>>, impl Into<String>)>,
    ) {
        for (style, content) in other {
            self.stylize(style.into(), content.into());
        }
    }

    #[cfg(feature = "color")]
    pub(crate) fn write_colored(&self, buffer: &mut termcolor::Buffer) -> std::io::Result<()> {
        use std::io::Write;
        use termcolor::WriteColor;

        for (style, content) in &self.pieces {
            let mut color = termcolor::ColorSpec::new();
            match style {
                Some(Style::Good) => {
                    color.set_fg(Some(termcolor::Color::Green));
                }
                Some(Style::Warning) => {
                    color.set_fg(Some(termcolor::Color::Yellow));
                }
                Some(Style::Error) => {
                    color.set_fg(Some(termcolor::Color::Red));
                    color.set_bold(true);
                }
                Some(Style::Hint) => {
                    color.set_dimmed(true);
                }
                None => {}
            }

            buffer.set_color(&color)?;
            buffer.write_all(content.as_bytes())?;
            buffer.reset()?;
        }

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
        let mut styled = StyledStr::new();
        styled.none(name);
        styled
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

/// Color-unaware printing. Never uses coloring.
impl std::fmt::Display for StyledStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (_, content) in &self.pieces {
            std::fmt::Display::fmt(content, f)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Style {
    Good,
    Warning,
    Error,
    Hint,
}
