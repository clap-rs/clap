use { App, ArgMatches };

/// TODO
pub trait DefineApp {
    /// TODO
    fn app() -> App<'static, 'static>;
}

/// TODO
pub trait FromArgMatches {
    /// TODO
    fn from(matches: &ArgMatches) -> Self;
}

/// TODO
pub trait DefineSubCommands {
    /// TODO
    fn subcommands() -> Vec<App<'static, 'static>>;
}

/// TODO
pub trait SubCommandFromArgMatches {
    /// TODO
    fn from(name: &str, matches: &ArgMatches) -> Self;
}

/// TODO
pub trait ParseApp {
    /// TODO
    fn parse() -> Self;
}

impl<C> ParseApp for C where C: DefineApp + FromArgMatches {
    fn parse() -> Self {
        C::from(&App::get_matches(C::app()))
    }
}

impl<C> DefineSubCommands for Option<C> where C: DefineSubCommands {
    fn subcommands() -> Vec<App<'static, 'static>> {
        C::subcommands()
    }
}

impl<C> SubCommandFromArgMatches for Option<C> where C: SubCommandFromArgMatches {
    fn from(name: &str, matches: &ArgMatches) -> Self {
        Some(C::from(name, matches))
    }
}
