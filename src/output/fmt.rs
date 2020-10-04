#[cfg(not(feature = "color"))]
use crate::util::termcolor::{Color, ColorChoice};
#[cfg(feature = "color")]
use termcolor::{Color, ColorChoice};

use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

#[cfg(feature = "color")]
fn is_a_tty(stderr: bool) -> bool {
    debug!("is_a_tty: stderr={:?}", stderr);

    let stream = if stderr {
        atty::Stream::Stderr
    } else {
        atty::Stream::Stdout
    };

    atty::is(stream)
}

#[derive(Debug)]
pub(crate) struct Colorizer {
    use_stderr: bool,
    color_when: ColorChoice,
    pieces: Vec<(String, Option<Color>)>,
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
        self.pieces.push((msg.into(), Some(Color::Green)));
    }

    #[inline]
    pub(crate) fn warning(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Some(Color::Yellow)));
    }

    #[inline]
    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), Some(Color::Red)));
    }

    #[inline]
    pub(crate) fn none(&mut self, msg: impl Into<String>) {
        self.pieces.push((msg.into(), None));
    }

    pub(crate) fn extend(&mut self, other: Colorizer) {
        self.pieces.extend(other.pieces);
    }

    pub(crate) fn nest(&mut self, nesting_level: usize) {
        let line_seperator = String::from("\n") + &". ".repeat(nesting_level);
        let mut new_pieces = self
            .pieces
            .iter()
            .map(|(s, c)| (s.replace('\n', &line_seperator), *c))
            .collect();
        std::mem::swap(&mut self.pieces, &mut new_pieces);
    }
}

/// Printing methods.
impl Colorizer {
    #[cfg(feature = "color")]
    pub(crate) fn print(&self) -> io::Result<()> {
        use termcolor::{BufferWriter, ColorSpec, WriteColor};

        let color_when = if is_a_tty(self.use_stderr) {
            self.color_when
        } else {
            ColorChoice::Never
        };

        let writer = if self.use_stderr {
            BufferWriter::stderr(color_when)
        } else {
            BufferWriter::stdout(color_when)
        };

        let mut buffer = writer.buffer();

        for piece in &self.pieces {
            let mut color = ColorSpec::new();
            color.set_fg(piece.1);
            if piece.1 == Some(Color::Red) {
                color.set_bold(true);
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
