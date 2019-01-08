#[derive(Default)]
pub struct HelpMsg<'help> {
    // Author string to add to help message
    #[doc(hidden)]
    pub author: Option<&'help str>,
    // Description of the program to be displayed with `-h`, or `--help` if `long_about` isn't
    // defined
    #[doc(hidden)]
    pub about: Option<&'help str>,
    // Description of the program to be displayed with `--help`
    #[doc(hidden)]
    pub long_about: Option<&'help str>,
    // Text to be displayed prior to any help message
    #[doc(hidden)]
    pub more_help: Option<&'help str>,
    // Text to be displayed after any help message
    #[doc(hidden)]
    pub pre_help: Option<&'help str>,
    // An override of the auto-generated usage string to be displayed in the help message or errors
    #[doc(hidden)]
    pub usage_str: Option<&'help str>,
    // The auto-generated usage string to be displayed in the help message or errors
    #[doc(hidden)]
    pub usage: Option<String>,
    // An override of the auto-generated help message to be displayed with `-h` or `--help`
    #[doc(hidden)]
    pub help_str: Option<&'help str>,
    // A template to use help messages with `-h` or `--help`
    #[doc(hidden)]
    pub template: Option<&'help str>,
    // Headings to apply for help message sections
    #[doc(hidden)]
    pub help_headings: Vec<Option<&'help str>>,
}

impl<'help> HelpMsg<'help> {
    pub fn new() -> Self {
        HelpMsg::default()
    }
}