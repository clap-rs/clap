use crate::util::color::ColorChoice;

#[cfg(feature = "color")]
use termcolor::{Color, ColorSpec};

use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StyleSpec {
    #[cfg(feature = "color")]
    pub good_style: ColorSpec,

    #[cfg(feature = "color")]
    pub warning_style: ColorSpec,

    #[cfg(feature = "color")]
    pub error_style: ColorSpec,

    #[cfg(feature = "color")]
    pub hint_style: ColorSpec,

    #[cfg(feature = "color")]
    pub default_style: ColorSpec,
}

impl StyleSpec {
    #[cfg(not(feature = "color"))]
    pub(crate) fn new() -> StyleSpec {
        StyleSpec {}
    }

    #[cfg(feature = "color")]
    pub(crate) fn new() -> StyleSpec {
        StyleSpec {
            good_style: ColorSpec::new(),
            warning_style: ColorSpec::new(),
            error_style: ColorSpec::new(),
            hint_style: ColorSpec::new(),
            default_style: ColorSpec::new(),
        }
    }

    #[cfg(feature = "color")]
    pub(crate) fn get_style(&self, style: Style) -> &ColorSpec {
        match style {
            Style::Good => &self.good_style,
            Style::Warning => &self.warning_style,
            Style::Error => &self.error_style,
            Style::Hint => &self.hint_style,
            Style::Default => &self.default_style,
        }
    }

    #[cfg(feature = "color")]
    pub(crate) fn set_style(&mut self, style: Style, spec: ColorSpec) -> &mut Self {
        match style {
            Style::Good => self.good_style = spec,
            Style::Warning => self.warning_style = spec,
            Style::Error => self.error_style = spec,
            Style::Hint => self.hint_style = spec,
            Style::Default => self.default_style = spec,
        }
        self
    }

    #[cfg(feature = "color")]
    pub(crate) fn style(&mut self, style: Style) -> &mut ColorSpec {
        match style {
            Style::Good => &mut self.good_style,
            Style::Warning => &mut self.warning_style,
            Style::Error => &mut self.error_style,
            Style::Hint => &mut self.hint_style,
            Style::Default => &mut self.default_style,
        }
    }
}

impl Default for StyleSpec {
    #[cfg(not(feature = "color"))]
    fn default() -> StyleSpec {
        StyleSpec {}
    }

    #[cfg(feature = "color")]
    fn default() -> StyleSpec {
        // Declare the styles
        let mut good_style = ColorSpec::new();
        let mut warning_style = ColorSpec::new();
        let mut error_style = ColorSpec::new();
        let mut hint_style = ColorSpec::new();
        let default_style = ColorSpec::new();

        // Set the defaults
        good_style.set_fg(Some(Color::Green));
        warning_style.set_fg(Some(Color::Yellow));
        error_style.set_fg(Some(Color::Red)).set_bold(true);
        hint_style.set_dimmed(true);

        StyleSpec {
            good_style,
            warning_style,
            error_style,
            hint_style,
            default_style,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Colorizer {
    use_stderr: bool,
    #[allow(unused)]
    color_when: ColorChoice,
    pieces: Vec<(String, Style)>,

    // If color is not enabled, then style_spec is never used
    #[allow(dead_code)]
    style_spec: StyleSpec,
}

impl Colorizer {
    /// Get the `ColorSpec` used for a particular style
    #[cfg(feature = "color")]
    pub(crate) fn spec_for(&self, style: Style) -> &ColorSpec {
        self.style_spec.get_style(style)
    }

    #[inline(never)]
    pub(crate) fn new(use_stderr: bool, color_when: ColorChoice, style_spec: StyleSpec) -> Self {
        // Construct the Colorizer
        Colorizer {
            use_stderr,
            color_when,
            pieces: vec![],
            style_spec,
        }
    }

    #[inline(never)]
    pub(crate) fn good(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Good));
    }

    #[inline(never)]
    pub(crate) fn warning(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Warning));
    }

    #[inline(never)]
    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Error));
    }

    #[inline(never)]
    #[allow(dead_code)]
    pub(crate) fn hint(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Hint));
    }

    #[inline(never)]
    pub(crate) fn none(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Default));
    }
}

/// Printing methods.
impl Colorizer {
    /// Returns the color spec associated with a particular style

    #[cfg(feature = "color")]
    pub(crate) fn print(&self) -> io::Result<()> {
        use termcolor::{BufferWriter, ColorChoice as DepColorChoice, WriteColor};

        let color_when = match self.color_when {
            ColorChoice::Always => DepColorChoice::Always,
            ColorChoice::Auto if is_a_tty(self.use_stderr) => DepColorChoice::Auto,
            _ => DepColorChoice::Never,
        };

        let writer = if self.use_stderr {
            BufferWriter::stderr(color_when)
        } else {
            BufferWriter::stdout(color_when)
        };

        let mut buffer = writer.buffer();

        for piece in &self.pieces {
            buffer.set_color(self.spec_for(piece.1))?;
            buffer.write_all(piece.0.as_bytes())?;
            buffer.reset()?;
        }

        writer.print(&buffer)
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn print(&self) -> io::Result<()> {
        // [e]println can't be used here because it panics
        // if something went wrong. We don't want that.
        if self.use_stderr {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            write!(stderr, "{}", self)
        } else {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            write!(stdout, "{}", self)
        }
    }
}

/// Color-unaware printing. Never uses coloring.
impl Display for Colorizer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for piece in &self.pieces {
            Display::fmt(&piece.0, f)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]

/// Style categories for output
pub enum Style {
    /// Style for  cli flags and the name of the program
    Good,
    /// Style for warnings and section headers in Help
    Warning,
    /// Style for error messages
    Error,
    /// Style for user hints
    Hint,
    /// Default style for plain text
    Default,
}

impl Default for Style {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(feature = "color")]
fn is_a_tty(stderr: bool) -> bool {
    let stream = if stderr {
        atty::Stream::Stderr
    } else {
        atty::Stream::Stdout
    };

    atty::is(stream)
}
