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
pub fn generate_manpage<'a>(app: &mut clap::App<'a>, buf: &mut dyn Write) -> Result<(), std::io::Error> {
    let meta = Meta::from_clap("1", "", app);
    let man = Man::new(&meta, app);
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
pub struct Man {
    roff: Roff,
}

impl Man {
    /// Create a new manual page.
    pub fn new(meta: &Meta, app: &mut clap::App) -> Self {
        app._build_all();

        let mut roff = Roff::default();
        roff.control("TH", meta.to_args());
        roff.control("SH", ["NAME"]);
        render::about(&mut roff, app);
        roff.control("SH", ["SYNOPSIS"]);
        render::synopsis(&mut roff, app);
        roff.control("SH", ["DESCRIPTION"]);
        render::description(&mut roff, app);

        if app_has_arguments(app) {
            roff.control("SH", ["OPTIONS"]);
            render::options(&mut roff, app);
        }

        if app_has_subcommands(app) {
            let heading = subcommand_heading(app);
            roff.control("SH", [heading.as_str()]);
            render::subcommands(&mut roff, app, &meta.section);
        }

        if app.get_after_long_help().is_some() || app.get_after_help().is_some() {
            roff.control("SH", ["EXTRA"]);
            render::after_help(&mut roff, app);
        }

        if app_has_version(app) {
            let version = roman(&render::version(app));
            roff.control("SH", ["VERSION"]);
            roff.text([version]);
        }

        if app.get_author().is_some() {
            let author = roman(app.get_author().unwrap_or_default());
            roff.control("SH", ["AUTHORS"]);
            roff.text([author]);
        }

        Self { roff }
    }

    /// Render a manual page into writer.
    pub fn render(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        self.roff.to_writer(w)
    }
}

// Does the application have a version?
fn app_has_version(app: &clap::App) -> bool {
    app.get_long_version()
        .or_else(|| app.get_version())
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
