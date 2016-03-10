#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct AppMeta<'b> {
    pub name: String,
    pub bin_name: Option<String>,
    pub author: Option<&'b str>,
    pub version: Option<&'b str>,
    pub about: Option<&'b str>,
    pub more_help: Option<&'b str>,
    pub usage_str: Option<&'b str>,
    pub usage: Option<String>,
    pub help_str: Option<&'b str>,
    pub disp_ord: usize,
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
            disp_ord: 999,
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
