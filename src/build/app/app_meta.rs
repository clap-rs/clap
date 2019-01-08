
#[derive(Default)]
pub struct AppMeta<'help> {
    // Used in the Help message title (typically the same as the binary file name used to call
    // the program). This can also be just a title, "My Awesome App" where the binary name is "maa".
    #[doc(hidden)]
    pub name: String,
    // The actual binary file name used to call this program as determined at runtime, OR as
    // overridden by the consumer. Displayed in usage strings and help message.
    #[doc(hidden)]
    pub bin_name: Option<String>,
    // A list of aliases this command could be called by
    #[doc(hidden)]
    pub aliases: Aliases<'help>,
    // Sets a way to manually override the order this App appears in, in the Help message
    #[doc(hidden)]
    pub disp_ord: usize,
    // Settings that change how the args are parsed, or App behaves
    #[doc(hidden)]
    pub settings: AppFlags,
    // Global settings (i.e. all subcommands)
    #[doc(hidden)]
    pub g_settings: AppFlags,
    // The terminal width as determined at runtime
    #[doc(hidden)]
    pub term_w: Option<usize>,
    // The overridden terminal width as set by the consumer
    #[doc(hidden)]
    pub max_w: Option<usize>,
}

impl<'help> AppMeta<'help> {
    pub fn new(name: String) -> Self {
        AppMeta {
            name,
            .. AppMeta::default()
        }
    }

    pub fn get_name(&self) -> &str {
        &*self.name
    }

    pub fn get_bin_name(&self) -> &str {
        self.bin_name.as_ref().map(|s| s.as_str())
    }
}