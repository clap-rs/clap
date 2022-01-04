use crate::util::color::ColorChoice;

use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

#[derive(Clone, Debug)]
pub(crate) struct Colorizer {
    use_stderr: bool,
    #[allow(unused)]
    color_when: ColorChoice,
    pieces: Vec<(String, Style)>,
}

impl Colorizer {
    #[inline]
    pub(crate) fn new(use_stderr: bool, color_when: ColorChoice) -> Self {
        Colorizer {
            use_stderr,
            color_when,
            pieces: vec![],
        }
    }

    #[inline]
    pub(crate) fn good(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Good));
    }

    #[inline]
    pub(crate) fn warning(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Warning));
    }

    #[inline]
    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Error));
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn hint(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Hint));
    }

    #[inline]
    pub(crate) fn none(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Style::Default));
    }
}

/// Printing methods.
impl Colorizer {
    #[cfg(feature = "color")]
    pub(crate) fn print(&self) -> io::Result<()> {
        use termcolor::{BufferWriter, ColorChoice as DepColorChoice, ColorSpec, WriteColor};

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
            let mut color = ColorSpec::new();
            match piece.1 {
                Style::Good => {
                    color.set_fg(Some(termcolor::Color::Green));
                }
                Style::Warning => {
                    color.set_fg(Some(termcolor::Color::Yellow));
                }
                Style::Error => {
                    color.set_fg(Some(termcolor::Color::Red));
                    color.set_bold(true);
                }
                Style::Hint => {
                    color.set_dimmed(true);
                }
                Style::Default => {}
            }

            buffer.set_color(&color)?;
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
pub enum Style {
    Good,
    Warning,
    Error,
    Hint,
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
