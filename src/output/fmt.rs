#[cfg(not(feature = "color"))]
use crate::util::termcolor::{Color, ColorChoice};
#[cfg(feature = "color")]
use termcolor::{Color, ColorChoice, ColorSpec};

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
}

/// Printing methods.
impl Colorizer {
    #[cfg(feature = "color")]
    pub(crate) fn print(&self) -> io::Result<()> {
        use termcolor::{BufferWriter, WriteColor};

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

        for (string, color) in &self.pieces {
            if Colorizer::is_label(string) {
                let mut label = string.clone();
                label.pop(); // pops off the trailing colon ":".

                let mut label_spec = Colorizer::bold_with_color(color);
                buffer.set_color(&label_spec)?;
                buffer.write_all(label.as_bytes())?;

                label_spec.set_fg(None); // the colon should be plane (with no color).
                buffer.set_color(&label_spec)?;
                buffer.write_all(":".as_bytes())?;
            } else {
                let mut color_spec = ColorSpec::new();
                color_spec.set_fg(*color);

                if *color == Some(Color::Red) {
                    color_spec.set_bold(true);
                }

                buffer.set_color(&color_spec)?;
                buffer.write_all(string.as_bytes())?;
            }

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

    fn is_label(s: &String) -> bool {
        s.ends_with(":")
    }

    fn bold_with_color(color: &Option<termcolor::Color>) -> ColorSpec {
        let mut spec = ColorSpec::new();
        spec.set_fg(*color);
        spec.set_bold(true);
        spec
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
