#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct AppMeta<'b> {
    pub name: String,
    pub bin_name: Option<String>,
    pub author: Option<&'b str>,
    pub version: Option<&'b str>,
    pub about: Option<&'b str>,
    pub more_help: Option<&'b str>,
    pub pre_help: Option<&'b str>,
    pub aliases: Option<Vec<(&'b str, bool)>>, // (name, visible)
    pub usage_str: Option<&'b str>,
    pub usage: Option<String>,
    pub help_str: Option<&'b str>,
    pub disp_ord: usize,
    pub term_w: Option<usize>,
    pub max_w: Option<usize>,
    pub template: Option<&'b str>,
}

impl<'b> Default for AppMeta<'b> {
    fn default() -> Self {
        AppMeta {
            name: String::new(),
            author: None,
            about: None,
            more_help: None,
            pre_help: None,
            version: None,
            usage_str: None,
            usage: None,
            bin_name: None,
            help_str: None,
            disp_ord: 999,
            template: None,
            aliases: None,
            term_w: None,
            max_w: None,
        }
    }
}

impl<'b> AppMeta<'b> {
    pub fn new() -> Self { Default::default() }
    pub fn with_name(s: String) -> Self { AppMeta { name: s, ..Default::default() } }
}

impl<'b> Clone for AppMeta<'b> {
    fn clone(&self) -> Self {
        AppMeta {
            name: self.name.clone(),
            author: self.author,
            about: self.about,
            more_help: self.more_help,
            pre_help: self.pre_help,
            version: self.version,
            usage_str: self.usage_str,
            usage: self.usage.clone(),
            bin_name: self.bin_name.clone(),
            help_str: self.help_str,
            disp_ord: self.disp_ord,
            template: self.template,
            aliases: self.aliases.clone(),
            term_w: self.term_w,
            max_w: self.max_w,
        }
    }
}
