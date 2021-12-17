#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![doc = include_str!("../README.md")]
#![warn(missing_docs, trivial_casts, unused_allocation, trivial_numeric_casts)]
#![forbid(unsafe_code)]

mod render;

pub use roff;

use render::subcommand_heading;
use roff::{ManSection, Roff, Troffable};
use std::io::Write;

/// Man page generator
pub struct Man {
    section: Option<ManSection>,
    manual: Option<String>,
    sections: Vec<(String, String)>,
}

impl Default for Man {
    fn default() -> Self {
        Self {
            section: Some(ManSection::Executable),
            manual: Some("General Commands Manual".to_string()),
            sections: Vec::new(),
        }
    }
}

/// Generate manpage for your application using the most common default values.
pub fn generate_manpage<'a>(app: &mut clap::App<'a>, buf: &mut dyn Write) {
    let man = Man::default();
    man.render(app, buf);
}

impl Man {
    /// Create a new builder for man pages.
    pub fn new() -> Self {
        Man {
            section: None,
            manual: None,
            sections: Vec::new(),
        }
    }

    /// Add section for your man page, see [`Section`].
    pub fn section(mut self, section: ManSection) -> Self {
        self.section = Some(section);
        self
    }

    /// Set manual for where the document comes from, the most common being
    /// `General Commands Manual`.
    pub fn manual(mut self, manual: impl Into<String>) -> Self {
        self.manual = Some(manual.into());
        self
    }

    /// Add a custom section to the man pages.
    pub fn custom_section<'a, I, C>(mut self, title: impl Into<String>, content: I) -> Self
    where
        I: IntoIterator<Item = &'a C>,
        C: Troffable + 'a,
    {
        self.sections.push((
            title.into(),
            content.into_iter().map(Troffable::render).collect(),
        ));
        self
    }

    /// Write the manpage to a buffer.
    pub fn render(self, app: &mut clap::App, buf: &mut dyn std::io::Write) {
        app._build_all();

        let mut page = Roff::new(app.get_name(), self.get_section())
            .section("Name", [&render::about(app)])
            .section("Synopsis", [&render::synopsis(app)])
            .section("Description", &render::description(app));

        if app_has_arguments(app) {
            page = page.section("Options", &render::options(app));
        }

        if app_has_subcommands(app) {
            page = page.section(
                &subcommand_heading(app),
                &render::subcommands(app, self.get_section().value()),
            )
        }

        if app.get_after_long_help().is_some() || app.get_after_help().is_some() {
            page = page.section("Extra", &render::after_help(app))
        }

        for (title, section) in self.sections {
            page = page.section(&title, &[section]);
        }

        if app.get_version().is_some() {
            page = page.section("Version", &[render::version(app)]);
        }

        if app.get_author().is_some() {
            page = page.section("Author(s)", &[app.get_author().unwrap_or_default()]);
        }

        buf.write_all(page.render().as_bytes()).unwrap();
    }

    fn get_section(&self) -> ManSection {
        self.section.unwrap_or(ManSection::Executable)
    }
}

fn app_has_arguments(app: &clap::App) -> bool {
    app.get_arguments()
        .any(|i| !i.is_set(clap::ArgSettings::Hidden))
}

fn app_has_subcommands(app: &clap::App) -> bool {
    app.get_subcommands()
        .any(|i| !i.is_set(clap::AppSettings::Hidden))
}
