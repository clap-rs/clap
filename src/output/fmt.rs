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
    values: [ColorSpec; NUM_STYLES],
}

impl StyleSpec {
    #[cfg(not(feature = "color"))]
    pub(crate) fn new() -> StyleSpec {
        StyleSpec {}
    }

    #[cfg(feature = "color")]
    pub(crate) fn new() -> StyleSpec {
        StyleSpec {
            values: Default::default(),
        }
    }
}

#[cfg(feature = "color")]
impl core::ops::Index<Style> for StyleSpec {
    type Output = ColorSpec;

    fn index(&self, style: Style) -> &Self::Output {
        &self.values[style as usize]
    }
}

#[cfg(feature = "color")]
impl core::ops::IndexMut<Style> for StyleSpec {
    fn index_mut(&mut self, style: Style) -> &mut Self::Output {
        &mut self.values[style as usize]
    }
}

impl Default for StyleSpec {
    #[cfg(not(feature = "color"))]
    fn default() -> StyleSpec {
        StyleSpec {}
    }

    #[cfg(feature = "color")]
    fn default() -> StyleSpec {
        use Style::*;
        let mut spec = StyleSpec::new();
        // Declare the styles
        spec[Good].set_fg(Some(Color::Green));
        spec[Warning].set_fg(Some(Color::Yellow));
        spec[Error].set_fg(Some(Color::Red)).set_bold(true);
        spec[Hint].set_dimmed(true);

        spec
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
            buffer.set_color(&self.style_spec[piece.1])?;
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
    Good = 0,
    /// Style for warnings and section headers in Help
    Warning,
    /// Style for error messages
    Error,
    /// Style for user hints
    Hint,
    /// Default style for plain text
    Default,
}

/// Used to store the number of different styles for use by StyleSpec.
/// Assumes Style::Default is the last style
#[cfg(feature = "color")]
const NUM_STYLES: usize = (Style::Default as usize) + 1;

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
