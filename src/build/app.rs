mod settings;

pub use self::settings::{AppFlags, AppSettings};

// Std
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::hash::Hash;
use std::io::{self, BufRead, BufWriter, Write};
use std::iter::Peekable;
use std::path::Path;
use std::process;

// Third Party
#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

// Internal
use crate::build::app::settings::AppSettings::Propagated;
use crate::build::args::ArgsVec;
use crate::build::{Aliases, Arg, ArgGroup, ArgSettings, HelpMsg, Terminal, VersionMsg};
use crate::output::fmt::ColorWhen;
use crate::output::{Help, Usage};
use crate::parse::errors::Result as ClapResult;
use crate::parse::{ArgMatcher, ArgMatches, Parser};
use crate::util::{hash, HELP_HASH, VERSION_HASH};
use crate::ArgV;
use crate::ErrorKind;
use INTERNAL_ERROR_MSG;

// @TODO @perf @maybe @p4
// We could add cache (Option<BuiltCmd>) if building takes too long for custom terminal style
// CLIs, although I suspect it'll be fine.
/// Used to create a representation of a command line program and all possible command line
/// arguments. Application settings are set using the "builder pattern" with the
/// [`App::get_matches`] family of methods being the terminal methods that starts the
/// runtime-parsing process. These methods then return information about the user supplied
/// arguments (or lack there of).
///
/// **NOTE:** There aren't any mandatory "options" that one must set. The "options" may
/// also appear in any order (so long as one of the [`App::get_matches`] methods is the last method
/// called).
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg};
/// let m = App::new("My Program")
///     .author("Me, me@mail.com")
///     .version("1.0.2")
///     .about("Explains in brief what the program does")
///     .arg(
///         Arg::new("in_file").index(1)
///     )
///     .after_help("Longer explanation to appear after the options when \
///                  displaying the help information from --help or -h")
///     .get_matches();
///
/// // Your program logic starts here...
/// ```
/// [`App::get_matches`]: ./struct.App.html#method.get_matches
#[derive(Default, Debug, Clone)]
pub struct App<'help> {
    #[doc(hidden)]
    root: Cmd<'help>,
    #[doc(hidden)]
    pub term: Terminal,
    #[doc(hidden)]
    pub ext_argv: Vec<Box<ArgV>>,
}

impl<'help> App<'help> {
    /// Creates a new instance of an application requiring a name. The name may be, but doesn't
    /// have to be same as the binary. The name will be displayed to the user when they request to
    /// print version or help and usage information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let app = App::new(Cmd::new("my_prog"));
    /// ```
    pub fn new(cmd: Cmd<'help>) -> Self {
        App {
            root: cmd,
            ..Default::default()
        }
    }
    /// Enables a single command, or [``], level settings.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .setting(AppSettings::WaitOnError)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`AppSettings`]: ./enum.AppSettings.html
    pub fn setting(mut self, setting: AppSettings) -> Self {
        self.root._set(setting);
        self
    }

    /// Disables a single command, or [``], level setting.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .unset_setting(AppSettings::ColorAuto)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`AppSettings`]: ./enum.AppSettings.html
    /// [global]: ./struct.App.html#method.global_setting
    pub fn unset_setting(mut self, setting: AppSettings) -> Self {
        self.root._unset(setting);
        self
    }

    /// Sets the terminal width at which to wrap help messages. Defaults to `120`. Using `0` will
    /// ignore terminal widths and use source formatting.
    ///
    /// `clap` automatically tries to determine the terminal width on Unix, Linux, OSX and Windows
    /// if the `wrap_help` cargo "feature" has been used while compiling. If the terminal width
    /// cannot be determined, `clap` defaults to `120`.
    ///
    /// **NOTE:** This setting applies globally and *not* on a per-command basis.
    ///
    /// **NOTE:** This setting must be set **before** any subcommands are added!
    ///
    /// # Platform Specific
    ///
    /// Only Unix, Linux, OSX and Windows support automatic determination of terminal width.
    /// Even on those platforms, this setting is useful if for any reason the terminal width
    /// cannot be determined.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .set_term_width(80)
    /// # ;
    /// ```
    pub fn set_term_width(mut self, width: usize) -> Self {
        self.term.width = Some(width);
        self
    }

    /// Sets the max terminal width at which to wrap help messages. Using `0` will ignore terminal
    /// widths and use source formatting.
    ///
    /// `clap` automatically tries to determine the terminal width on Unix, Linux, OSX and Windows
    /// if the `wrap_help` cargo "feature" has been used while compiling, but one might want to
    /// limit the size (e.g. when the terminal is running fullscreen).
    ///
    /// **NOTE:** This setting applies globally and *not* on a per-command basis.
    ///
    /// **NOTE:** This setting must be set **before** any subcommands are added!
    ///
    /// # Platform Specific
    ///
    /// Only Unix, Linux, OSX and Windows support automatic determination of terminal width.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .max_term_width(100)
    /// # ;
    /// ```
    pub fn max_term_width(mut self, w: usize) -> Self {
        self.term.max_width = Some(w);
        self
    }

    /// Starts the parsing process, upon a failed parse an error will be displayed to the user and
    /// the process will exit with the appropriate error code. By default this method gets all user
    /// provided arguments from [`env::args_os`] in order to allow for invalid UTF-8 code points,
    /// which are legal on many platforms.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, Cmd};
    /// let matches = App::new(
    ///     Cmd::new("myprog")
    ///         // Args and options go here...
    ///     ).parse();
    /// ```
    /// [`env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
    pub fn parse(self) -> ArgMatches { self.parse_from(&mut env::args_os()) }

    /// Starts the parsing process, just like [`App::parse`] but doesn't consume the `App`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, Cmd};
    /// let mut app = App::new(Cmd::new("myprog")
    ///     // Args and options go here...
    /// );
    /// let matches = app.parse_mut();
    /// ```
    /// [`env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
    /// [`App::parse`]: ./struct.App.html#method.parse
    pub fn parse_mut(&mut self) -> ArgMatches {
        self.try_parse_from_mut(&mut env::args_os())
            .unwrap_or_else(|e| self.maybe_wait_on_error(e))
    }

    /// Starts the parsing process. This method will return a [`clap::Result`] type instead of exiting
    /// the process on failed parse. By default this method gets matches from [`env::args_os`]
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return a [`clap::Error`], where the [`kind`] is a
    /// [`ErrorKind::HelpDisplayed`] or [`ErrorKind::VersionDisplayed`] respectively. You must call
    /// [`Error::exit`] or perform a [`std::process::exit`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .try_get_matches()
    ///     .unwrap_or_else( |e| e.exit() );
    /// ```
    /// [`env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
    /// [`ErrorKind::HelpDisplayed`]: ./enum.ErrorKind.html#variant.HelpDisplayed
    /// [`ErrorKind::VersionDisplayed`]: ./enum.ErrorKind.html#variant.VersionDisplayed
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`std::process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
    /// [`clap::Result`]: ./type.Result.html
    /// [`clap::Error`]: ./struct.Error.html
    /// [`kind`]: ./struct.Error.html
    pub fn try_parse(self) -> ClapResult<ArgMatches> {
        // Start the parsing
        self.try_parse_from(&mut env::args_os())
    }

    /// Starts the parsing process. Like [`App::get_matches`] this method does not return a [`clap::Result`]
    /// and will automatically exit with an error message. This method, however, lets you specify
    /// what iterator to use when performing matches, such as a [`Vec`] of your making.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches_from(arg_vec);
    /// ```
    /// [`App::get_matches`]: ./struct.App.html#method.get_matches
    /// [`clap::Result`]: ./type.Result.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [`AppSettings::NoBinaryName`]: ./enum.AppSettings.html#variant.NoBinaryName
    pub fn parse_from<T: ArgV>(mut self, argv: T) -> ArgMatches {
        self.try_parse_from_mut(itr)
            .unwrap_or_else(|e| self.maybe_wait_on_error(e))
    }

    /// Starts the parsing process. A combination of [`Cmd::parse_from`], and
    /// [`Cmd::try_parse`]
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return a [`clap::Error`], where the [`kind`] is a [`ErrorKind::HelpDisplayed`]
    /// or [`ErrorKind::VersionDisplayed`] respectively. You must call [`Error::exit`] or
    /// perform a [`std::process::exit`] yourself.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .try_get_matches_from(arg_vec)
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    /// [`App::get_matches_from`]: ./struct.App.html#method.get_matches_from
    /// [`App::try_get_matches`]: ./struct.App.html#method.try_get_matches
    /// [`ErrorKind::HelpDisplayed`]: ./enum.ErrorKind.html#variant.HelpDisplayed
    /// [`ErrorKind::VersionDisplayed`]: ./enum.ErrorKind.html#variant.VersionDisplayed
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`std::process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
    /// [`clap::Error`]: ./struct.Error.html
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`kind`]: ./struct.Error.html
    /// [`AppSettings::NoBinaryName`]: ./enum.AppSettings.html#variant.NoBinaryName
    pub fn try_parse_from<T: ArgV>(mut self, argv: T) -> ClapResult<ArgMatches> {
        self.try_parse_from_mut(itr)
    }

    /// Starts the parsing process without consuming the [`App`] struct `self`. This is normally not
    /// the desired functionality, instead prefer [`App::try_get_matches_from`] which *does*
    /// consume `self`.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let mut app = App::new("myprog");
    ///     // Args and options go here...
    /// let matches = app.try_get_matches_from_mut(arg_vec)
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    /// [`App`]: ./struct.App.html
    /// [`App::try_get_matches_from`]: ./struct.App.html#method.try_get_matches_from
    /// [`AppSettings::NoBinaryName`]: ./enum.AppSettings.html#variant.NoBinaryName
    pub fn try_parse_from_mut<T: ArgV>(&mut self, argv: T) -> ClapResult<ArgMatches> {
        let mut it = itr.into_iter();
        // Get the name of the program (argument 1 of env::args())
        if !self.is_set(AppSettings::NoBinaryName) {
            if let Some(name) = it.next() {
                self.root._actual_bin_name(name);
            }
        }

        self.root.build(Propagation::None).parse(&mut it.peekable())
    }

    // Checks if we should wait before exiting, then exits
    fn maybe_wait_on_error(&self, e: ClapError) -> ! {
        // Otherwise, write to stderr and exit
        if e.use_stderr() {
            wlnerr!("{}", e.message);
            if self.is_set(AppSettings::WaitOnError) {
                wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                let mut s = String::new();
                let i = io::stdin();
                i.lock().read_line(&mut s).unwrap();
            }
            drop(self);
            drop(e);
            process::exit(1);
        }

        drop(self);
        e.exit()
    }
}
