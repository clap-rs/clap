use App;
use ArgMatches;

/// The abstract representation of a command line subcommand used by the consumer of the library.
/// 
///
/// This struct is used by the library consumer and describes all the valid options of the subcommand for 
/// their program. SubCommands are treated like "sub apps" and contain all the same possibilities (such as
/// their own arguments and subcommands).
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg, SubCommand};
/// # let matches = App::new("myprog")
/// #                    .subcommand(
/// SubCommand::new("conifg")
///                .about("Used for configuration")
///                .arg(Arg::new("config_file")
///                           .help("The configuration file to use")
///                           .index(1))
/// # ).get_matches();
pub struct SubCommand<'n, 'a> {
       pub name: &'n str,
       pub matches: ArgMatches<'n, 'a>
}

impl<'n, 'a> SubCommand<'n, 'a> {
    /// Creates a new instance of a subcommand requiring a name. Will be displayed
    /// to the user when they print version or help and usage information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let prog = App::new("myprog").subcommand(
    /// SubCommand::new("config")
    /// # ).get_matches();
    /// ```
    pub fn new<'au, 'v, 'ab, 'u, 'h, 'ar>(name: &'ar str) -> App<'au, 'v, 'ab, 'u, 'h, 'ar> {
        App::new(name)
    }
}