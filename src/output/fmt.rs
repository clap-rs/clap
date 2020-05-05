use crate::util::termcolor::{Buffer, BufferWriter, ColorChoice};
#[cfg(feature = "color")]
use crate::util::termcolor::{Color, ColorSpec, WriteColor};

use std::{
    fmt::{self, Debug, Formatter},
    io::{Result, Write},
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

#[cfg(not(feature = "color"))]
fn is_a_tty(_: bool) -> bool {
    debug!("is_a_tty");
    false
}

pub(crate) struct Colorizer {
    writer: BufferWriter,
    buffer: Buffer,
}

impl Debug for Colorizer {
    #[cfg(feature = "color")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.buffer.as_slice()))
    }

    #[cfg(not(feature = "color"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buffer))
    }
}

impl Colorizer {
    pub(crate) fn new(use_stderr: bool, when: ColorChoice) -> Self {
        let checked_when = if is_a_tty(use_stderr) {
            when
        } else {
            ColorChoice::Never
        };

        let writer = if use_stderr {
            BufferWriter::stderr(checked_when)
        } else {
            BufferWriter::stdout(checked_when)
        };

        let buffer = writer.buffer();

        Self { writer, buffer }
    }

    pub(crate) fn print(&self) -> Result<()> {
        self.writer.print(&self.buffer)
    }

    #[cfg(feature = "color")]
    pub(crate) fn good(&mut self, msg: &str) -> Result<()> {
        self.buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        self.write_all(msg.as_bytes())?;
        self.buffer.reset()
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn good(&mut self, msg: &str) -> Result<()> {
        self.none(msg)
    }

    #[cfg(feature = "color")]
    pub(crate) fn warning(&mut self, msg: &str) -> Result<()> {
        self.buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        self.write_all(msg.as_bytes())?;
        self.buffer.reset()
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn warning(&mut self, msg: &str) -> Result<()> {
        self.none(msg)
    }

    #[cfg(feature = "color")]
    pub(crate) fn error(&mut self, msg: &str) -> Result<()> {
        self.buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
        self.write_all(msg.as_bytes())?;
        self.buffer.reset()
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn error(&mut self, msg: &str) -> Result<()> {
        self.none(msg)
    }

    pub(crate) fn none(&mut self, msg: &str) -> Result<()> {
        self.write_all(msg.as_bytes())?;
        Ok(())
    }
}

impl Write for Colorizer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.buffer.flush()
    }
}
