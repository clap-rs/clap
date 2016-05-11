use std::ffi::OsString;
use std::ffi::OsStr;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

use App;
use ArgMatches;

/// The abstract representation of a command line subcommand.
///
/// This struct describes all the valid options of the subcommand for the program. Subcommands are
/// essentially "sub-[`App`]s" and contain all the same possibilities (such as their own
/// [arguments], subcommands, and settings).
///
/// # Examples
///
/// ```rust
/// # use clap::{App, Arg, SubCommand};
/// App::new("myprog")
///     .subcommand(
///         SubCommand::with_name("config")
///             .about("Used for configuration")
///             .arg(Arg::with_name("config_file")
///                 .help("The configuration file to use")
///                 .index(1)))
/// # ;
/// ```
/// [`App`]: ./struct.App.html
/// [arguments]: ./struct.Arg.html
#[derive(Debug, Clone)]
pub struct SubCommand<'a> {
    #[doc(hidden)]
    pub name: OsString,
    #[doc(hidden)]
    pub matches: ArgMatches<'a>,
}

impl<'a> SubCommand<'a> {
    /// Creates a new instance of a subcommand requiring a name. The name will be displayed
    /// to the user when they print version or help and usage information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .subcommand(
    ///         SubCommand::with_name("config"))
    /// # ;
    /// ```
    pub fn with_name<'b, T>(t: T) -> App<'a, 'b>
        where T: AsRef<str> {
        App::new(t.as_ref())
    }

    /// Creates a new instance of a subcommand from a YAML (.yml) document
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg, SubCommand};
    /// let sc_yaml = load_yaml!("test_subcommand.yml");
    /// let sc = SubCommand::from_yaml(sc_yaml);
    /// ```
    #[cfg(feature = "yaml")]
    pub fn from_yaml<'y>(yaml: &'y Yaml) -> App<'y, 'y> {
        App::from_yaml(yaml)
    }
}

// Users shouldn't have to worry about this, used internally to give a from_str which doesn't
// return a `Result`
#[doc(hidden)]
pub trait SubCommandKey {
    fn from_os_str(s: &OsStr) -> Self;
    fn external(args: Vec<OsString>) -> Option<Self> where Self: Sized;
}

impl<'a> SubCommandKey for &'a str {
    fn from_os_str(s: &OsStr) -> Self {
        use std::mem;
        unsafe { mem::transmute(s) } // should not actually be called in the real world
    }
    fn external(_: Vec<OsString>) -> Option<Self> {
        None
    }
}
