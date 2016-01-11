pub struct AppMeta<'b> {
    // The name displayed to the user when showing version and help/usage information
    pub name: String,
    pub bin_name: Option<String>,
    // A string of author(s) if desired. Displayed when showing help/usage information
    pub author: Option<&'b str>,
    // The version displayed to the user
    pub version: Option<&'b str>,
    // A brief explanation of the program that gets displayed to the user when shown
    // help/usage
    // information
    pub about: Option<&'b str>,
    // Additional help information
    pub more_help: Option<&'b str>,
    pub usage_str: Option<&'b str>,
    pub usage: Option<String>,
    pub help_str: Option<&'b str>,
}

impl<'b> Default for AppMeta<'b> {
    fn default() -> Self {
        AppMeta {
            name: String::new(),
            author: None,
            about: None,
            more_help: None,
            version: None,
            usage_str: None,
            usage: None,
            bin_name: None,
            help_str: None,
        }
    }
}

impl<'b> AppMeta<'b> {
    pub fn new() -> Self { Default::default() }
    pub fn with_name(s: String) -> Self {
        AppMeta {
            name: s,
            ..Default::default()
        }
    }
}
