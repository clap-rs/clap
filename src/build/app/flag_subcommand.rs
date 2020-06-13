use super::App;

use crate::util::Id;

/// Used to create a subcommand that can be used as if it were a Flag or a regular subcommand.
///
/// # Examples
/// ```no_run
/// # use clap::{App, FlagSubCommand};
///
/// let m = App::new("MyProgram")
///     .author("Me, me@mail.com")
///     .version("1.0.2")
///     .about("Explains in brief what the program does")
///     .subcommand(
///         FlagSubCommand::new('S', "sub")
///             .about("Explains purpose of subcommand")
///     )
///     .after_help("Longer explanation to appear after the options when \
///                  displaying the help information from --help or -h")
///     .get_matches();
///
/// // Your program logic starts here...
/// ```
///
/// The above example's subcommand can be executed with any one of the following
///
/// ```sh
/// $ MyProgram subcommand
/// ```
///
/// ```sh
/// $ MyProgram -S
/// ```
///
/// ```sh
/// $ MyProgram --sub
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FlagSubCommand {}

#[allow(clippy::new_ret_no_self)]
impl FlagSubCommand {
    /// Creates a Subcommand that can be used either as a short Flag or a full named subcommand
    ///
    /// # Examples
    /// ```no_run
    /// # use clap::{App, FlagSubCommand};
    ///
    /// let m = App::new("MyProgram")
    ///     .author("Me, me@mail.com")
    ///     .version("1.0.2")
    ///     .about("Explains in brief what the program does")
    ///     .subcommand(
    ///         FlagSubCommand::new_short("subcommand", 'S')
    ///             .about("Explains purpose of subcommand")
    ///     )
    ///     .after_help("Longer explanation to appear after the options when \
    ///                  displaying the help information from --help or -h")
    ///     .get_matches();
    ///
    /// // Your program logic starts here...
    /// ```
    ///
    /// The above example's subcommand can be executed with any one of the following
    ///
    /// ```sh
    /// $ MyProgram subcommand
    /// ```
    ///
    /// ```sh
    /// $ MyProgram -S
    /// ```
    pub fn new_short<'b, S: Into<String>>(n: S, short: char) -> App<'b> {
        let name = n.into();
        App {
            id: Id::from(&*name),
            name,
            short: Some(short),
            ..Default::default()
        }
    }

    /// Creates a Subcommand that can be used either as a long Flag or a full named subcommand
    ///
    /// # Examples
    /// ```no_run
    /// # use clap::{App, FlagSubCommand};
    ///
    /// let m = App::new("MyProgram")
    ///     .author("Me, me@mail.com")
    ///     .version("1.0.2")
    ///     .about("Explains in brief what the program does")
    ///     .subcommand(
    ///         FlagSubCommand::new_long("subcommand", "sub")
    ///             .about("Explains purpose of subcommand")
    ///     )
    ///     .after_help("Longer explanation to appear after the options when \
    ///                  displaying the help information from --help or -h")
    ///     .get_matches();
    ///
    /// // Your program logic starts here...
    /// ```
    ///
    /// The above example's subcommand can be executed with any one of the following
    ///
    /// ```sh
    /// $ MyProgram subcommand
    /// ```
    ///
    /// ```sh
    /// $ MyProgram --sub
    /// ```
    pub fn new_long<S: Into<String>>(n: S, long: &str) -> App {
        let name = n.into();
        App {
            id: Id::from(&*name),
            name,
            long: Some(long),
            ..Default::default()
        }
    }

    /// Used to create a subcommand that can be used as if it were a Flag or a regular subcommand.
    ///
    /// NOTE: `long` parameter is used as the name of the subcommand
    ///
    /// # Examples
    /// ```no_run
    /// # use clap::{App, FlagSubCommand};
    ///
    /// let m = App::new("MyProgram")
    ///     .author("Me, me@mail.com")
    ///     .version("1.0.2")
    ///     .about("Explains in brief what the program does")
    ///     .subcommand(
    ///         FlagSubCommand::new('S', "subcommand")
    ///             .about("Explains purpose of subcommand")
    ///     )
    ///     .after_help("Longer explanation to appear after the options when \
    ///                  displaying the help information from --help or -h")
    ///     .get_matches();
    ///
    /// // Your program logic starts here...
    /// ```
    ///
    /// The above example's subcommand can be executed with any one of the following
    ///
    /// ```sh
    /// $ MyProgram subcommand
    /// ```
    ///
    /// ```sh
    /// $ MyProgram -S
    /// ```
    ///
    /// ```sh
    /// $ MyProgram --subcommand
    /// ```
    pub fn new(short: char, long: &str) -> App {
        let name = long.to_string();
        App {
            id: Id::from(long),
            name,
            short: Some(short),
            long: Some(long),
            ..Default::default()
        }
    }

    /// Used to create a subcommand that can be used as if it were a Flag or a regular subcommand.
    /// Where the name differs from the long flag.
    ///
    /// # Examples
    /// ```no_run
    /// # use clap::{App, FlagSubCommand};
    ///
    /// let m = App::new("MyProgram")
    ///     .author("Me, me@mail.com")
    ///     .version("1.0.2")
    ///     .about("Explains in brief what the program does")
    ///     .subcommand(
    ///         FlagSubCommand::with_name("subcommand2", 'S', "subcommand")
    ///             .about("Explains purpose of subcommand")
    ///     )
    ///     .after_help("Longer explanation to appear after the options when \
    ///                  displaying the help information from --help or -h")
    ///     .get_matches();
    ///
    /// // Your program logic starts here...
    /// ```
    ///
    /// The above example's subcommand can be executed with any one of the following
    ///
    /// ```sh
    /// $ MyProgram subcommand2
    /// ```
    ///
    /// ```sh
    /// $ MyProgram -S
    /// ```
    ///
    /// ```sh
    /// $ MyProgram --subcommand
    /// ```
    pub fn with_name<S: Into<String>>(n: S, short: char, long: &str) -> App {
        let name = n.into();
        App {
            id: Id::from(&*name),
            name,
            short: Some(short),
            long: Some(long),
            ..Default::default()
        }
    }
}
