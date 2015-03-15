use app::App;
use argmatches::ArgMatches;

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
/// #                 .SubCommand(
/// SubCommand::new("conifg")
///				.about("Used for configuration")
///				.arg(Arg::new("config_file")
///						   .help("The configuration file to use")
///					       .index(1))
/// # ).get_matches();
pub struct SubCommand {
   	pub name: &'static str,
   	pub matches: ArgMatches 
}

impl SubCommand {
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
	pub fn new(name: &'static str) -> App {
		App::new(name)
	}
}