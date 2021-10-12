mod shells;

// Std
use std::io::Write;

// Internal
use clap::App;
pub use shells::*;

/// Generator trait which can be used to write generators
pub trait Generator {
    /// Returns the file name that is created when this generator is called during compile time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::Write;
    /// # use clap::App;
    /// use clap_generate::Generator;
    ///
    /// pub struct Fish;
    ///
    /// impl Generator for Fish {
    /// #   fn generate(&self, app: &App, buf: &mut dyn Write) {}
    ///     fn file_name(&self, name: &str) -> String {
    ///         format!("{}.fish", name)
    ///     }
    /// }
    /// ```
    fn file_name(&self, name: &str) -> String;

    /// Generates output out of [`clap::App`](App).
    ///
    /// # Examples
    ///
    /// The following example generator displays the [`clap::App`](App)
    /// as if it is printed using [`std::println`].
    ///
    /// ```
    /// use std::{io::Write, fmt::write};
    /// use clap::App;
    /// use clap_generate::Generator;
    ///
    /// pub struct ClapDebug;
    ///
    /// impl Generator for ClapDebug {
    ///     fn generate(&self, app: &App, buf: &mut dyn Write) {
    ///         write!(buf, "{}", app).unwrap();
    ///     }
    /// #   fn file_name(&self, name: &str) -> String {
    /// #    name.into()
    /// #   }
    /// }
    /// ```
    fn generate(&self, app: &App, buf: &mut dyn Write);
}
