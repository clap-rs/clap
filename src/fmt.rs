#[cfg(all(feature = "color", not(target_os = "windows")))]
use termion::{color, style};

#[cfg(feature = "color")]
use libc;
use std::fmt;

#[cfg(all(feature = "color", not(target_os = "windows")))]
const STDERR: i32 = libc::STDERR_FILENO;
#[cfg(all(feature = "color", not(target_os = "windows")))]
const STDOUT: i32 = libc::STDOUT_FILENO;

#[cfg(any(not(feature = "color"), target_os = "windows"))]
const STDERR: i32 = 0;
#[cfg(any(not(feature = "color"), target_os = "windows"))]
const STDOUT: i32 = 0;

#[doc(hidden)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

#[cfg(feature = "color")]
pub fn is_a_tty(stderr: bool) -> bool {
    debugln!("fn=is_a_tty;");
    debugln!("Use stderr...{:?}", stderr);
    let fd = if stderr { STDERR } else { STDOUT };
    unsafe { libc::isatty(fd) != 0 }
}

#[cfg(not(feature = "color"))]
pub fn is_a_tty(_: bool) -> bool {
    debugln!("fn=is_a_tty;");
    false
}

#[doc(hidden)]
pub struct Colorizer {
    pub use_stderr: bool,
    pub when: ColorWhen,
}

macro_rules! color {
    ($_self:ident, $c:ident, $m:expr) => {
        match $_self.when {
            ColorWhen::Auto => if is_a_tty($_self.use_stderr) {
                Format::$c($m)
            } else {
                Format::none($m)
            },
            ColorWhen::Always => Format::$c($m),
            ColorWhen::Never => Format::none($m),
        }
    };
}

impl Colorizer {
    pub fn good<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=good;");
        color!(self, good, msg)
    }

    pub fn warning<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=warning;");
        color!(self, warning, msg)
    }

    pub fn error<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=error;");
        color!(self, error, msg)
    }

    pub fn none<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=none;");
        Format::none(msg)
    }
}

impl Default for Colorizer {
    fn default() -> Self {
        Colorizer {
            use_stderr: true,
            when: ColorWhen::Auto,
        }
    }
}

/// Defines styles for different types of error messages. Defaults to Error=Red, Warning=Yellow,
/// and Good=Green
#[derive(Debug, Copy, Clone)]
pub enum Kind {
    /// Defines the style used for errors, defaults to Red
    Error,
    /// Defines the style used for warnings, defaults to Yellow
    Warning,
    /// Defines the style used for good values, defaults to Green
    Good,
    /// Defines no formatting style
    None,
}

/// An log message.
#[doc(hidden)]
#[derive(Debug)]
pub struct Format<T> {
    kind: Kind,
    msg: T,
}

impl<T> Format<T> {
    /// An error message.
    pub fn error(msg: T) -> Format<T> {
        Format {
            kind: Kind::Error,
            msg: msg,
        }
    }

    /// A warning message.
    pub fn warning(msg: T) -> Format<T> {
        Format {
            kind: Kind::Warning,
            msg: msg,
        }
    }

    /// A "good" message.
    ///
    /// That is, it went well.
    pub fn good(msg: T) -> Format<T> {
        Format {
            kind: Kind::Good,
            msg: msg,
        }
    }

    /// A "neutral" message.
    ///
    /// That is, it is simply a log message.
    pub fn none(msg: T) -> Format<T> {
        Format {
            kind: Kind::None,
            msg: msg,
        }
    }
}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: fmt::Display> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::Error => write!(f, "{color}{style}{msg}{reset}",
                                    color = color::Fg(color::Red),
                                    style = style::Bold,
                                    msg   = self.msg,
                                    reset = style::Reset),
            Kind::Warning => write!(f, "{color}{msg}{reset}",
                                      color = color::Fg(color::Yellow),
                                      msg   = self.msg,
                                      reset = style::Reset),
            Kind::Good => write!(f, "{color}{msg}{reset}",
                                   color = color::Fg(color::Green),
                                   msg   = self.msg,
                                   reset = style::Reset),
            Kind::None => write!(f, "{}", self.msg),
        }
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: AsRef<str>> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, self.msg.as_ref())
    }
}

#[cfg(all(test, feature = "color", not(target_os = "windows")))]
mod test {
    use termion::{color, style};
    use super::*;

    #[test]
    fn colored_output() {
        let err = Format {
            kind: Kind::Error,
            msg: "error",
        };
        assert_eq!(&*format!("{}", err), &*format!("{}{}error{}", color::Fg(color::Red), style::Bold, style::Reset));
        let good = Format {
            kind: Kind::Good,
            msg: "good",
        };
        assert_eq!(&*format!("{}", good), &*format!("{}good{}", color::Fg(color::Green), style::Reset));
        let warn = Format {
            kind: Kind::Warning,
            msg: "warn",
        };
        assert_eq!(&*format!("{}", warn), &*format!("{}warn{}", color::Fg(color::Yellow), style::Reset));
        let none = Format {
            kind: Kind::None,
            msg: "none",
        };
        assert_eq!(&*format!("{}", none),
                   "none");
    }
}
