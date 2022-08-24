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
        self.pieces.push((style, msg));
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
