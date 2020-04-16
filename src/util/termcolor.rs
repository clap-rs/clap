use std::io::{stderr, stdout, Result, Write};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum ColorChoice {
    Auto,
    Always,
    Never,
}

pub(crate) type Buffer = Vec<u8>;

pub(crate) struct BufferWriter {
    use_stderr: bool,
}

impl BufferWriter {
    pub(crate) fn buffer(&self) -> Buffer {
        vec![]
    }

    pub(crate) fn stderr(_: ColorChoice) -> Self {
        Self { use_stderr: true }
    }

    pub(crate) fn stdout(_: ColorChoice) -> Self {
        Self { use_stderr: false }
    }

    pub(crate) fn print(&self, buf: &Buffer) -> Result<()> {
        if self.use_stderr {
            stderr().lock().write(buf)?;
        } else {
            stdout().lock().write(buf)?;
        }

        Ok(())
    }
}
