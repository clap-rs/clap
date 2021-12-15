#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![doc = include_str!("../README.md")]
#![warn(missing_docs, trivial_casts, unused_allocation, trivial_numeric_casts)]
#![forbid(unsafe_code)]

mod man;
mod render;

use std::io::Write;

/// Man page generator
#[derive(Debug, Clone)]
pub struct Man {
    section: Option<i8>,
    manual: Option<String>,
    sections: Vec<(String, Vec<String>)>,
}

impl Default for Man {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate manpage for your application using sane defaults.
pub fn generate_manpage<'a>(app: &mut clap::App<'a>, buf: &mut dyn Write) {
    let man = Man::new().section(1).manual("General Commands Manual");
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

    /// Add section for your man page, the most common being 1 for programs.
    ///
    /// 1. Executable programs or shell commands
    /// 2. System calls (functions provided by the kernel)
    /// 3. Library calls (functions within program libraries)
    /// 4. Special files (usually found in /dev)
    /// 5. File formats and conventions, e.g. /etc/passwd
    /// 6. Games
    /// 7. Miscellaneous (including macro packages and conventions), e.g. man(7), groff(7)
    /// 8. System administration commands (usually only for root)
    /// 9. Kernel routines [Non standard]
    pub fn section(mut self, section: i8) -> Self {
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
    pub fn custom_section(
        mut self,
        title: impl Into<String>,
        body: Vec<impl Into<String>>,
    ) -> Self {
        self.sections
            .push((title.into(), body.into_iter().map(|s| s.into()).collect()));
        self
    }

    /// Write the manpage to a buffer.
    pub fn render(self, app: &mut clap::App, buf: &mut dyn std::io::Write) {
        app._build_all();
        render::header(app, self.get_section(), self.manual.clone(), buf);

        // Set sentence_space_size to 0 to prevent extra space between sentences separated
        // by a newline the alternative is to add \& at the end of the line
        writeln!(buf, ".ss \\n[.ss] 0").unwrap();
        // Disable hyphenation
        writeln!(buf, ".nh").unwrap();
        // Disable justification (adjust text to the left margin only)
        writeln!(buf, ".ad l").unwrap();

        render::about(app, buf);
        render::description(app, buf);
        render::synopsis(app, buf);

        render::options(app, buf);
        render::subcommands(app, self.get_section(), buf);

        render::after_help(app, buf);
        render::custom_sections(self.sections, buf);

        render::version(app, buf);
        render::authors(app, buf);
    }

    fn get_section(&self) -> i8 {
        self.section.unwrap_or(1)
    }
}
