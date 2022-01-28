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

/// Generate a manual page and write it out.
pub fn generate_manpage(app: clap::App<'_>, buf: &mut dyn Write) -> Result<(), std::io::Error> {
    let meta = Meta::from_clap("1", "", &app);
    let man = Man::new(meta, app);
    man.render(buf)
}

/// Metadata about a manual page.
pub struct Meta {
    title: String,
    section: String,
    date: String,
    source: String,
    manual: String,
}

impl Meta {
    /// Create metadata from a clap::App.
    pub fn from_clap(section: &str, manual: &str, app: &clap::App) -> Self {
        Self {
            title: app.get_name().to_string(),
            section: section.to_string(),
            date: "".to_string(), // FIXME
            source: format!(
                "{} {}",
                app.get_name(),
                app.get_version().unwrap_or_default()
            ),
            manual: manual.to_string(),
        }
    }

    // Turn metadata into arguments for a .TH macro.
    fn to_args(&self) -> Vec<&str> {
        vec![
            &self.title,
            &self.section,
            &self.date,
            &self.source,
            &self.manual,
        ]
    }
}

/// A manual page as constructed from a clap::App.
pub struct Man<'a> {
    meta: Meta,
    app: clap::App<'a>,
}

impl<'a> Man<'a> {
    /// Create a new manual page.
    pub fn new(meta: Meta, mut app: clap::App<'a>) -> Self {
        app._build_all();
        Self { meta, app }
    }

    /// Render a manual page into writer.
    pub fn render(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        roff.control("TH", self.meta.to_args());
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
        render::subcommands(roff, &self.app, &self.meta.section);
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
