use std::fmt;

#[cfg(feature = "color")]
use ansi_term::Colour::{Red, Green, Yellow};
#[cfg(feature = "color")]
use ansi_term::ANSIString;


pub enum Format<T> {
     Error(T),
     Warning(T),
     Good(T),
}

#[cfg(feature = "color")]
impl<T: AsRef<str>> Format<T> {
    fn format(&self) -> ANSIString {
        match *self {
            Format::Error(ref e) => Red.bold().paint(e.as_ref()),
            Format::Warning(ref e) => Yellow.paint(e.as_ref()),
            Format::Good(ref e) => Green.paint(e.as_ref()),
        }
    }

}

#[cfg(feature = "color")]
impl<T: AsRef<str>> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}

#[cfg(not(feature = "color"))]
impl<T: fmt::Display> Format<T> {
    fn format(&self) -> &T {
        match *self {
            Format::Error(ref e) => e,
            Format::Warning(ref e) => e,
            Format::Good(ref e) => e,
        }
    }
}

#[cfg(not(feature = "color"))]
impl<T: fmt::Display> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}
