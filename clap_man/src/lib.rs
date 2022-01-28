#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![doc = include_str!("../README.md")]
#![warn(missing_docs, trivial_casts, unused_allocation, trivial_numeric_casts)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod render;

pub use roff;

use render::subcommand_heading;
use roff::{roman, Roff};
use std::io::Write;

/// A manual page as constructed from a clap::App.
pub struct Man<'a> {
    app: clap::App<'a>,
    title: String,
    section: String,
    date: String,
    source: String,
    manual: String,
}

impl<'a> Man<'a> {
    /// Create a new manual page.
    pub fn new(mut app: clap::App<'a>) -> Self {
        app._build_all();
        let title = app.get_name().to_owned();
        let section = "1".to_owned();
        let date = "".to_owned();
        let source = format!(
            "{} {}",
            app.get_name(),
            app.get_version().unwrap_or_default()
        );
        let manual = "".to_owned();
        Self {
            app,
            title,
            section,
            date,
            source,
            manual,
        }
    }

    /// Override the default title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Override the default section
    pub fn section(mut self, section: impl Into<String>) -> Self {
        self.section = section.into();
        self
    }

    /// Override the default date
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    /// Override the default source
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Override the default manual
    pub fn manual(mut self, manual: impl Into<String>) -> Self {
        self.manual = manual.into();
        self
    }

    /// Render a manual page into writer.
    pub fn render(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        roff.control("TH", self.title_args());
        self._render_name_section(&mut roff);
        self._render_synopsis_section(&mut roff);
        self._render_description_section(&mut roff);

        if app_has_arguments(&self.app) {
            self._render_options_section(&mut roff);
        }

        if app_has_subcommands(&self.app) {
            self._render_subcommands_section(&mut roff);
        }

        if self.app.get_after_long_help().is_some() || self.app.get_after_help().is_some() {
            self._render_extra_section(&mut roff);
        }

        if app_has_version(&self.app) {
            self._render_version_section(&mut roff);
        }

        if self.app.get_author().is_some() {
            self._render_authors_section(&mut roff);
        }

        roff.to_writer(w)
    }

    // Turn metadata into arguments for a .TH macro.
    fn title_args(&self) -> Vec<&str> {
        vec![
            &self.title,
            &self.section,
            &self.date,
            &self.source,
            &self.manual,
        ]
    }

    /// Render the NAME section into the writer.
    pub fn render_name_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_name_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_name_section(&self, roff: &mut Roff) {
        roff.control("SH", ["NAME"]);
        render::about(roff, &self.app);
    }

    /// Render the SYNOPSIS section into the writer.
    pub fn render_synopsis_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_synopsis_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_synopsis_section(&self, roff: &mut Roff) {
        roff.control("SH", ["SYNOPSIS"]);
        render::synopsis(roff, &self.app);
    }

    /// Render the DESCRIPTION section into the writer.
    pub fn render_description_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_description_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_description_section(&self, roff: &mut Roff) {
        roff.control("SH", ["DESCRIPTION"]);
        render::description(roff, &self.app);
    }

    /// Render the OPTIONS section into the writer.
    pub fn render_options_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_options_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_options_section(&self, roff: &mut Roff) {
        roff.control("SH", ["OPTIONS"]);
        render::options(roff, &self.app);
    }

    /// Render the SUBCOMMANDS section into the writer.
    pub fn render_subcommands_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_subcommands_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_subcommands_section(&self, roff: &mut Roff) {
        let heading = subcommand_heading(&self.app);
        roff.control("SH", [heading.as_str()]);
        render::subcommands(roff, &self.app, &self.section);
    }

    /// Render the EXTRA section into the writer.
    pub fn render_extra_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_extra_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_extra_section(&self, roff: &mut Roff) {
        roff.control("SH", ["EXTRA"]);
        render::after_help(roff, &self.app);
    }

    /// Render the VERSION section into the writer.
    pub fn render_version_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_version_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_version_section(&self, roff: &mut Roff) {
        let version = roman(&render::version(&self.app));
        roff.control("SH", ["VERSION"]);
        roff.text([version]);
    }

    /// Render the AUTHORS section into the writer.
    pub fn render_authors_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_authors_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_authors_section(&self, roff: &mut Roff) {
        let author = roman(self.app.get_author().unwrap_or_default());
        roff.control("SH", ["AUTHORS"]);
        roff.text([author]);
    }
}

// Does the application have a version?
fn app_has_version(app: &clap::App) -> bool {
    app.get_version()
        .or_else(|| app.get_long_version())
        .is_some()
}

// Does the application have any command line arguments?
fn app_has_arguments(app: &clap::App) -> bool {
    app.get_arguments()
        .any(|i| !i.is_set(clap::ArgSettings::Hidden))
}

// Does the application have any subcommands?
fn app_has_subcommands(app: &clap::App) -> bool {
    app.get_subcommands()
        .any(|i| !i.is_set(clap::AppSettings::Hidden))
}
