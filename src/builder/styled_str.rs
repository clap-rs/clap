use crate::output::display_width;
use crate::output::textwrap;

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
        self.stylize_(Some(Style::Good), msg.into());
    }

    pub(crate) fn warning(&mut self, msg: impl Into<String>) {
        self.stylize_(Some(Style::Warning), msg.into());
    }

    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        self.stylize_(Some(Style::Error), msg.into());
    }

    #[allow(dead_code)]
    pub(crate) fn hint(&mut self, msg: impl Into<String>) {
        self.stylize_(Some(Style::Hint), msg.into());
    }

    pub(crate) fn none(&mut self, msg: impl Into<String>) {
        self.stylize_(None, msg.into());
    }

    pub(crate) fn stylize(&mut self, style: impl Into<Option<Style>>, msg: impl Into<String>) {
        self.stylize_(style.into(), msg.into());
    }

    pub(crate) fn replace_newline(&mut self) {
        for (_, content) in &mut self.pieces {
            *content = content.replace("{n}", "\n");
        }
    }

    pub(crate) fn indent(&mut self, initial: &str, trailing: &str) {
        if let Some((_, first)) = self.pieces.first_mut() {
            first.insert_str(0, initial);
        }
        let mut line_sep = "\n".to_owned();
        line_sep.push_str(trailing);
        for (_, content) in &mut self.pieces {
            *content = content.replace('\n', &line_sep);
        }
    }

    pub(crate) fn wrap(&mut self, hard_width: usize) {
        let mut wrapper = textwrap::wrap_algorithms::LineWrapper::new(hard_width);
        for (_, content) in &mut self.pieces {
            let mut total = Vec::new();
            for (i, line) in content.split_inclusive('\n').enumerate() {
                if 0 < i {
                    // start of a section does not imply newline
                    wrapper.reset();
                }
                let line =
                    textwrap::word_separators::find_words_ascii_space(line).collect::<Vec<_>>();
                total.extend(wrapper.wrap(line));
            }
            let total = total.join("");
            *content = total;
        }
        if let Some((_, last)) = self.pieces.last_mut() {
            *last = last.trim_end().to_owned();
        }
    }

    fn stylize_(&mut self, style: Option<Style>, msg: String) {
        if !msg.is_empty() {
            self.pieces.push((style, msg));
        }
    }

    #[inline(never)]
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

    pub(crate) fn into_iter(self) -> impl Iterator<Item = (Option<Style>, String)> {
        self.pieces.into_iter()
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
