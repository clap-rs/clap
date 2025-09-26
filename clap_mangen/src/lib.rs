#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod render;

pub use roff;

use render::subcommand_heading;
use roff::{roman, Roff};
use std::io::Write;

/// A manpage writer
pub struct Man {
    cmd: clap::Command,
    title: String,
    section: String,
    date: String,
    source: String,
    manual: String,
}

/// Build a [`Man`]
impl Man {
    /// Create a new manual page.
    pub fn new(mut cmd: clap::Command) -> Self {
        cmd.build();
        let title = cmd
            .get_display_name()
            .unwrap_or_else(|| cmd.get_name())
            .to_owned();
        let section = "1".to_owned();
        let date = "".to_owned();
        let source = format!(
            "{} {}",
            cmd.get_name(),
            cmd.get_version().unwrap_or_default()
        );
        let manual = "".to_owned();
        Self {
            cmd,
            title,
            section,
            date,
            source,
            manual,
        }
    }

    /// Override the default man page title, written in all caps
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Override the default section this man page is placed in
    ///
    /// Common values:
    ///
    /// - `"1"`: User Commands
    /// - `"2"`: System Calls
    /// - `"3"`: C Library Functions
    /// - `"4"`: Devices and Special Files
    /// - `"5"`: File Formats and Conventions
    /// - `"6"`: Games et. al.
    /// - `"7"`: Miscellanea
    /// - `"8"`: System Administration tools and Daemons
    pub fn section(mut self, section: impl Into<String>) -> Self {
        self.section = section.into();
        self
    }

    /// Override the default date for the last non-trivial change to this man page
    ///
    /// Dates should be written in the form `YYYY-MM-DD`.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    /// Override the default source your command
    ///
    /// For those few man-pages pages in Sections 1 and 8, probably you just want to write GNU.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Override the default manual this page is a member of
    pub fn manual(mut self, manual: impl Into<String>) -> Self {
        self.manual = manual.into();
        self
    }
}

/// Handle [`Man`] in relation to files
impl Man {
    /// Generate the filename of the manual page
    #[must_use]
    pub fn get_filename(&self) -> String {
        format!(
            "{}.{}",
            self.cmd
                .get_display_name()
                .unwrap_or_else(|| self.cmd.get_name()),
            self.section
        )
    }

    /// [Renders](Man::render) the manual page and writes it to a file
    pub fn generate_to(
        &self,
        out_dir: impl AsRef<std::path::Path>,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        let filepath = out_dir.as_ref().join(self.get_filename());
        let mut file = std::fs::File::create(&filepath)?;
        self.render(&mut file)?;
        file.flush()?;
        Ok(filepath)
    }
}

/// Generate manual page files for the command with all subcommands
pub fn generate_to(
    cmd: clap::Command,
    out_dir: impl AsRef<std::path::Path>,
) -> Result<(), std::io::Error> {
    fn generate(cmd: clap::Command, out_dir: &std::path::Path) -> Result<(), std::io::Error> {
        for cmd in cmd.get_subcommands().filter(|s| !s.is_hide_set()).cloned() {
            generate(cmd, out_dir)?;
        }
        Man::new(cmd).generate_to(out_dir)?;
        Ok(())
    }

    let mut cmd = cmd.disable_help_subcommand(true);
    cmd.build();
    generate(cmd, out_dir.as_ref())
}

/// Generate ROFF output
impl Man {
    /// Render a full manual page into the writer.
    ///
    /// If customization is needed, you can call the individual sections you want and mix them into
    /// your own ROFF content.
    pub fn render(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_title(&mut roff);
        self._render_name_section(&mut roff);
        self._render_synopsis_section(&mut roff);
        self._render_description_section(&mut roff);

        if app_has_arguments(&self.cmd) {
            self._render_options_section(&mut roff);
        }

        if app_has_subcommands(&self.cmd) {
            self._render_subcommands_section(&mut roff);
        }

        if self.cmd.get_after_long_help().is_some() || self.cmd.get_after_help().is_some() {
            self._render_extra_section(&mut roff);
        }

        if app_has_version(&self.cmd) {
            self._render_version_section(&mut roff);
        }

        if self.cmd.get_author().is_some() {
            self._render_authors_section(&mut roff);
        }

        roff.to_writer(w)
    }

    /// Render the title into the writer.
    pub fn render_title(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_title(&mut roff);
        roff.to_writer(w)
    }

    fn _render_title(&self, roff: &mut Roff) {
        roff.control("TH", self.title_args());
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
        render::about(roff, &self.cmd);
    }

    /// Render the SYNOPSIS section into the writer.
    pub fn render_synopsis_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_synopsis_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_synopsis_section(&self, roff: &mut Roff) {
        roff.control("SH", ["SYNOPSIS"]);
        render::synopsis(roff, &self.cmd);
    }

    /// Render the DESCRIPTION section into the writer.
    pub fn render_description_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_description_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_description_section(&self, roff: &mut Roff) {
        roff.control("SH", ["DESCRIPTION"]);
        render::description(roff, &self.cmd);
    }

    /// Render the OPTIONS section into the writer.
    pub fn render_options_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_options_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_options_section(&self, roff: &mut Roff) {
        let help_headings = self
            .cmd
            .get_arguments()
            .filter(|a| !a.is_hide_set())
            .filter_map(|arg| arg.get_help_heading())
            .fold(vec![], |mut acc, header| {
                if !acc.contains(&header) {
                    acc.push(header);
                }

                acc
            });

        let (args, mut args_with_heading) = self
            .cmd
            .get_arguments_sorted()
            .filter(|a| !a.is_hide_set())
            .partition::<Vec<_>, _>(|a| a.get_help_heading().is_none());

        if !args.is_empty() {
            roff.control("SH", ["OPTIONS"]);
            render::options(roff, &args);
        }

        for heading in help_headings {
            let args;
            (args, args_with_heading) = args_with_heading
                .into_iter()
                .partition(|&a| a.get_help_heading() == Some(heading));

            roff.control("SH", [heading.to_uppercase().as_str()]);
            render::options(roff, &args);
        }
    }

    /// Render the SUBCOMMANDS section into the writer.
    pub fn render_subcommands_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_subcommands_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_subcommands_section(&self, roff: &mut Roff) {
        let heading = subcommand_heading(&self.cmd);
        roff.control("SH", [heading]);
        render::subcommands(roff, &self.cmd, &self.section);
    }

    /// Render the EXTRA section into the writer.
    pub fn render_extra_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_extra_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_extra_section(&self, roff: &mut Roff) {
        roff.control("SH", ["EXTRA"]);
        render::after_help(roff, &self.cmd);
    }

    /// Render the VERSION section into the writer.
    pub fn render_version_section(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut roff = Roff::default();
        self._render_version_section(&mut roff);
        roff.to_writer(w)
    }

    fn _render_version_section(&self, roff: &mut Roff) {
        let version = roman(render::version(&self.cmd));
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
        let author = roman(self.cmd.get_author().unwrap_or_default());
        roff.control("SH", ["AUTHORS"]);
        roff.text([author]);
    }
}

// Does the application have a version?
fn app_has_version(cmd: &clap::Command) -> bool {
    cmd.get_version()
        .or_else(|| cmd.get_long_version())
        .is_some()
}

// Does the application have any command line arguments?
fn app_has_arguments(cmd: &clap::Command) -> bool {
    cmd.get_arguments().any(|i| !i.is_hide_set())
}

// Does the application have any subcommands?
fn app_has_subcommands(cmd: &clap::Command) -> bool {
    cmd.get_subcommands().any(|i| !i.is_hide_set())
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
