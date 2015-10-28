use std::fmt;

#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::Colour::{Green, Red, Yellow};
#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::ANSIString;


/// Defines styles for different types of error messages. Defaults to Error=Red, Warning=Yellow,
/// and Good=Green
#[derive(Debug)]
pub enum Format<T> {
    /// Defines the style used for errors, defaults to Red
    Error(T),
    /// Defines the style used for warnings, defaults to Yellow
    Warning(T),
    /// Defines the style used for good values, defaults to Green
    Good(T),
}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> Format<T> {
    fn format(&self) -> ANSIString {
        match *self {
            Format::Error(ref e) => Red.bold().paint(e.as_ref()),
            Format::Warning(ref e) => Yellow.paint(e.as_ref()),
            Format::Good(ref e) => Green.paint(e.as_ref()),
        }
    }

}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: fmt::Display> Format<T> {
    fn format(&self) -> &T {
        match *self {
            Format::Error(ref e) => e,
            Format::Warning(ref e) => e,
            Format::Good(ref e) => e,
        }
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: fmt::Display> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}

#[cfg(test)]
mod test {
    use super::Format;
    use ansi_term::Colour::{Green, Red, Yellow};

    #[test]
    fn colored_output() {
        let err = Format::Error("error");
        assert_eq!(&*format!("{}", err),
                   &*format!("{}", Red.bold().paint("error")));
        let good = Format::Good("good");
        assert_eq!(&*format!("{}", good), &*format!("{}", Green.paint("good")));
        let warn = Format::Warning("warn");
        assert_eq!(&*format!("{}", warn), &*format!("{}", Yellow.paint("warn")));
    }
}
