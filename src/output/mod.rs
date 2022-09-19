mod help;
mod help_template;
mod usage;

pub(crate) mod fmt;
pub(crate) mod textwrap;

pub(crate) use self::help::write_help;
pub(crate) use self::help_template::AutoHelp;
pub(crate) use self::help_template::HelpTemplate;
pub(crate) use self::textwrap::core::display_width;
pub(crate) use self::textwrap::wrap;
pub(crate) use self::usage::Usage;

pub(crate) const TAB: &str = "  ";
pub(crate) const TAB_WIDTH: usize = TAB.len();
