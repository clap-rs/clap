use ArgMatches;

/// TODO
pub trait App {
    /// TODO
    fn app() -> ::App<'static, 'static>;
}

/// TODO
pub trait SubCommands {
    /// TODO
    fn subcommands() -> Vec<::App<'static, 'static>>;
}

/// TODO
pub trait SubCommandFromArgMatches {
    /// TODO
    fn from_matches(name: &str, matches: &ArgMatches) -> Self;
}

impl<C> SubCommands for Option<C> where C: SubCommands {
    fn subcommands() -> Vec<::App<'static, 'static>> {
        C::subcommands()
    }
}

impl<C> SubCommandFromArgMatches for Option<C> where C: SubCommandFromArgMatches {
    fn from_matches(name: &str, matches: &ArgMatches) -> Self {
        Some(C::from_matches(name, matches))
    }
}
