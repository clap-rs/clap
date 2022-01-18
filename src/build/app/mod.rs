#[cfg(debug_assertions)]
mod debug_asserts;
mod settings;
#[cfg(test)]
mod tests;

pub use self::settings::{AppFlags, AppSettings};

// Std
use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fmt,
    io::{self, Write},
    ops::Index,
    path::Path,
};

// Third Party
use os_str_bytes::RawOsStr;
#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

// Internal
use crate::{
    build::{arg::ArgProvider, Arg, ArgGroup, ArgSettings},
    mkeymap::MKeyMap,
    output::{fmt::Colorizer, Help, HelpWriter, Usage},
    parse::{ArgMatcher, ArgMatches, Input, Parser},
    util::{color::ColorChoice, Id, Key},
    Error, ErrorKind, Result as ClapResult, INTERNAL_ERROR_MSG,
};

/// Build a command-line interface.
///
/// This includes defining arguments, subcommands, parser behavior, and help output.
/// Once all configuration is complete,
/// the [`App::get_matches`] family of methods starts the runtime-parsing
/// process. These methods then return information about the user supplied
/// arguments (or lack thereof).
///
/// When deriving a [`Parser`][crate::Parser], you can use
/// [`IntoApp::into_app`][crate::IntoApp::into_app] to access the
/// `App`.
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
/// [`App::get_matches`]: App::get_matches()
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct App<'help> {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) long_flag: Option<&'help str>,
    pub(crate) short_flag: Option<char>,
    pub(crate) bin_name: Option<String>,
    pub(crate) author: Option<&'help str>,
    pub(crate) version: Option<&'help str>,
    pub(crate) long_version: Option<&'help str>,
    pub(crate) about: Option<&'help str>,
    pub(crate) long_about: Option<&'help str>,
    pub(crate) before_help: Option<&'help str>,
    pub(crate) before_long_help: Option<&'help str>,
    pub(crate) after_help: Option<&'help str>,
    pub(crate) after_long_help: Option<&'help str>,
    pub(crate) aliases: Vec<(&'help str, bool)>, // (name, visible)
    pub(crate) short_flag_aliases: Vec<(char, bool)>, // (name, visible)
    pub(crate) long_flag_aliases: Vec<(&'help str, bool)>, // (name, visible)
    pub(crate) usage_str: Option<&'help str>,
    pub(crate) usage: Option<String>,
    pub(crate) help_str: Option<&'help str>,
    pub(crate) disp_ord: Option<usize>,
    pub(crate) term_w: Option<usize>,
    pub(crate) max_w: Option<usize>,
    pub(crate) template: Option<&'help str>,
    pub(crate) settings: AppFlags,
    pub(crate) g_settings: AppFlags,
    pub(crate) args: MKeyMap<'help>,
    pub(crate) subcommands: Vec<App<'help>>,
    pub(crate) replacers: HashMap<&'help str, &'help [&'help str]>,
    pub(crate) groups: Vec<ArgGroup<'help>>,
    pub(crate) current_help_heading: Option<&'help str>,
    pub(crate) subcommand_value_name: Option<&'help str>,
    pub(crate) subcommand_heading: Option<&'help str>,
}

impl<'help> App<'help> {
    /// Creates a new instance of an `App`.
    ///
    /// It is common, but not required, to use binary name as the `name`. This
    /// name will only be displayed to the user when they request to print
    /// version or help and usage information.
    ///
    /// See also [`app_from_crate!!`](crate::app_from_crate!) and [`crate_name!`](crate::crate_name!).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("My Program")
    /// # ;
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();

        App {
            id: Id::from(&*name),
            name,
            ..Default::default()
        }
        .arg(
            Arg::new("help")
                .long("help")
                .help("Print help information")
                .global(true)
                .generated(),
        )
        .arg(
            Arg::new("version")
                .long("version")
                .help("Print version information")
                .global(true)
                .generated(),
        )
    }

    /// Adds an [argument] to the list of valid possibilities.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, arg, Arg};
    /// App::new("myprog")
    ///     // Adding a single "flag" argument with a short and help text, using Arg::new()
    ///     .arg(
    ///         Arg::new("debug")
    ///            .short('d')
    ///            .help("turns on debugging mode")
    ///     )
    ///     // Adding a single "option" argument with a short, a long, and help text using the less
    ///     // verbose Arg::from()
    ///     .arg(
    ///         arg!(-c --config <CONFIG> "Optionally sets a config file to use")
    ///     )
    /// # ;
    /// ```
    /// [argument]: Arg
    #[must_use]
    pub fn arg<A: Into<Arg<'help>>>(mut self, a: A) -> Self {
        let mut arg = a.into();
        arg.help_heading.get_or_insert(self.current_help_heading);
        self.args.push(arg);
        self
    }

    /// Adds multiple [arguments] to the list of valid possibilities.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, arg, Arg};
    /// App::new("myprog")
    ///     .args(&[
    ///         arg!("[debug] -d 'turns on debugging info'"),
    ///         Arg::new("input").index(1).help("the input file to use")
    ///     ])
    /// # ;
    /// ```
    /// [arguments]: Arg
    #[must_use]
    pub fn args<I, T>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Arg<'help>>,
    {
        let args = args.into_iter();
        let (lower, _) = args.size_hint();
        self.args.reserve(lower);

        for arg in args {
            self = self.arg(arg);
        }
        self
    }

    /// Allows one to mutate an [`Arg`] after it's been added to an [`App`].
    ///
    /// This can be useful for modifying the auto-generated help or version arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    ///
    /// let mut app = App::new("foo")
    ///     .arg(Arg::new("bar")
    ///         .short('b'))
    ///     .mut_arg("bar", |a| a.short('B'));
    ///
    /// let res = app.try_get_matches_from_mut(vec!["foo", "-b"]);
    ///
    /// // Since we changed `bar`'s short to "B" this should err as there
    /// // is no `-b` anymore, only `-B`
    ///
    /// assert!(res.is_err());
    ///
    /// let res = app.try_get_matches_from_mut(vec!["foo", "-B"]);
    /// assert!(res.is_ok());
    /// ```
    #[must_use]
    pub fn mut_arg<T, F>(mut self, arg_id: T, f: F) -> Self
    where
        F: FnOnce(Arg<'help>) -> Arg<'help>,
        T: Key + Into<&'help str>,
    {
        let arg_id: &str = arg_id.into();
        let id = Id::from(arg_id);

        let mut a = self.args.remove_by_name(&id).unwrap_or_else(|| Arg {
            id,
            name: arg_id,
            ..Arg::default()
        });

        if a.provider == ArgProvider::Generated {
            a.provider = ArgProvider::GeneratedMutated;
        }

        self.args.push(f(a));
        self
    }

    /// Adds an [`ArgGroup`] to the application.
    ///
    /// [`ArgGroup`]s are a family of related arguments.
    /// By placing them in a logical group, you can build easier requirement and exclusion rules.
    ///
    /// Example use cases:
    /// - Make an entire [`ArgGroup`] required, meaning that one (and *only*
    ///   one) argument from that group must be present at runtime.
    /// - Name an [`ArgGroup`] as a conflict to another argument.
    ///   Meaning any of the arguments that belong to that group will cause a failure if present with
    ///   the conflicting argument.
    /// - Ensure exclusion between arguments.
    /// - Extract a value from a group instead of determining exactly which argument was used.
    ///
    /// # Examples
    ///
    /// The following example demonstrates using an [`ArgGroup`] to ensure that one, and only one,
    /// of the arguments from the specified group is present at runtime.
    ///
    /// ```no_run
    /// # use clap::{App, arg, ArgGroup};
    /// App::new("app")
    ///     .arg(arg!("--set-ver [ver] 'set the version manually'"))
    ///     .arg(arg!("--major 'auto increase major'"))
    ///     .arg(arg!("--minor 'auto increase minor'"))
    ///     .arg(arg!("--patch 'auto increase patch'"))
    ///     .group(ArgGroup::new("vers")
    ///          .args(&["set-ver", "major", "minor","patch"])
    ///          .required(true))
    /// # ;
    /// ```
    #[inline]
    #[must_use]
    pub fn group<G: Into<ArgGroup<'help>>>(mut self, group: G) -> Self {
        self.groups.push(group.into());
        self
    }

    /// Adds multiple [`ArgGroup`]s to the [`App`] at once.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, arg, ArgGroup};
    /// App::new("app")
    ///     .arg(arg!("--set-ver [ver] 'set the version manually'"))
    ///     .arg(arg!("--major         'auto increase major'"))
    ///     .arg(arg!("--minor         'auto increase minor'"))
    ///     .arg(arg!("--patch         'auto increase patch'"))
    ///     .arg(arg!("-c [FILE]       'a config file'"))
    ///     .arg(arg!("-i [IFACE]      'an interface'"))
    ///     .groups(&[
    ///         ArgGroup::new("vers")
    ///             .args(&["set-ver", "major", "minor","patch"])
    ///             .required(true),
    ///         ArgGroup::new("input")
    ///             .args(&["c", "i"])
    ///     ])
    /// # ;
    /// ```
    #[must_use]
    pub fn groups<I, T>(mut self, groups: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ArgGroup<'help>>,
    {
        for g in groups.into_iter() {
            self = self.group(g.into());
        }
        self
    }

    /// Adds a subcommand to the list of valid possibilities.
    ///
    /// Subcommands are effectively sub-[`App`]s, because they can contain their own arguments,
    /// subcommands, version, usage, etc. They also function just like [`App`]s, in that they get
    /// their own auto generated help, version, and usage.
    ///
    /// A subcommand's [`App::name`] will be used for:
    /// - The argument the user passes in
    /// - Programmatically looking up the subcommand
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, arg};
    /// App::new("myprog")
    ///     .subcommand(App::new("config")
    ///         .about("Controls configuration features")
    ///         .arg(arg!("<config> 'Required configuration file to use'")))
    /// # ;
    /// ```
    #[inline]
    #[must_use]
    pub fn subcommand<S: Into<App<'help>>>(mut self, subcmd: S) -> Self {
        self.subcommands.push(subcmd.into());
        self
    }

    /// Adds multiple subcommands to the list of valid possibilities.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// # App::new("myprog")
    /// .subcommands( vec![
    ///        App::new("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::new("config_file").index(1)),
    ///        App::new("debug").about("Controls debug functionality")])
    /// # ;
    /// ```
    /// [`IntoIterator`]: std::iter::IntoIterator
    #[must_use]
    pub fn subcommands<I, T>(mut self, subcmds: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<App<'help>>,
    {
        for subcmd in subcmds.into_iter() {
            self.subcommands.push(subcmd.into());
        }
        self
    }

    /// Catch problems earlier in the development cycle.
    ///
    /// Most error states are handled as asserts under the assumption they are programming mistake
    /// and not something to handle at runtime.  Rather than relying on tests (manual or automated)
    /// that exhaustively test your CLI to ensure the asserts are evaluated, this will run those
    /// asserts in a way convenient for running as a test.
    ///
    /// **Note::** This will not help with asserts in [`ArgMatches`], those will need exhaustive
    /// testing of your CLI.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// fn app() -> App<'static> {
    ///     App::new("foo")
    ///         .arg(Arg::new("bar").short('b')
    ///     )
    /// }
    ///
    /// #[test]
    /// fn verify_app() {
    ///     app().debug_assert();
    /// }
    ///
    /// fn main() {
    ///     let m = app().get_matches_from(vec!["foo", "-b"]);
    ///     println!("{}", m.is_present("bar"));
    /// }
    /// ```
    pub fn debug_assert(mut self) {
        self._build_all();
    }

    /// Custom error message for post-parsing validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, ErrorKind};
    /// let mut app = App::new("myprog");
    /// let err = app.error(ErrorKind::InvalidValue, "Some failure case");
    /// ```
    pub fn error(&mut self, kind: ErrorKind, message: impl std::fmt::Display) -> Error {
        Error::raw(kind, message).format(self)
    }

    /// Parse [`env::args_os`], exiting on failure.
    ///
    /// # Panics
    ///
    /// If contradictory arguments or settings exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches();
    /// ```
    /// [`env::args_os`]: std::env::args_os()
    /// [`App::try_get_matches_from_mut`]: App::try_get_matches_from_mut()
    #[inline]
    pub fn get_matches(self) -> ArgMatches {
        self.get_matches_from(&mut env::args_os())
    }

    /// Parse [`env::args_os`], exiting on failure.
    ///
    /// Like [`App::get_matches`] but doesn't consume the `App`.
    ///
    /// # Panics
    ///
    /// If contradictory arguments or settings exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let mut app = App::new("myprog")
    ///     // Args and options go here...
    ///     ;
    /// let matches = app.get_matches_mut();
    /// ```
    /// [`env::args_os`]: std::env::args_os()
    /// [`App::get_matches`]: App::get_matches()
    pub fn get_matches_mut(&mut self) -> ArgMatches {
        self.try_get_matches_from_mut(&mut env::args_os())
            .unwrap_or_else(|e| e.exit())
    }

    /// Parse [`env::args_os`], returning a [`clap::Result`] on failure.
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return a [`clap::Error`], where the [`kind`] is a
    /// [`ErrorKind::DisplayHelp`] or [`ErrorKind::DisplayVersion`] respectively. You must call
    /// [`Error::exit`] or perform a [`std::process::exit`].
    ///
    /// # Panics
    ///
    /// If contradictory arguments or settings exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .try_get_matches()
    ///     .unwrap_or_else(|e| e.exit());
    /// ```
    /// [`env::args_os`]: std::env::args_os()
    /// [`Error::exit`]: crate::Error::exit()
    /// [`std::process::exit`]: std::process::exit()
    /// [`clap::Result`]: Result
    /// [`clap::Error`]: crate::Error
    /// [`kind`]: crate::Error
    /// [`ErrorKind::DisplayHelp`]: crate::ErrorKind::DisplayHelp
    /// [`ErrorKind::DisplayVersion`]: crate::ErrorKind::DisplayVersion
    #[inline]
    pub fn try_get_matches(self) -> ClapResult<ArgMatches> {
        // Start the parsing
        self.try_get_matches_from(&mut env::args_os())
    }

    /// Parse the specified arguments, exiting on failure.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used.
    ///
    /// # Panics
    ///
    /// If contradictory arguments or settings exist.
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
    /// [`App::get_matches`]: App::get_matches()
    /// [`clap::Result`]: Result
    /// [`Vec`]: std::vec::Vec
    pub fn get_matches_from<I, T>(mut self, itr: I) -> ArgMatches
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from_mut(itr).unwrap_or_else(|e| {
            drop(self);
            e.exit()
        })
    }

    /// Parse the specified arguments, returning a [`clap::Result`] on failure.
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return a [`clap::Error`], where the [`kind`] is a [`ErrorKind::DisplayHelp`]
    /// or [`ErrorKind::DisplayVersion`] respectively. You must call [`Error::exit`] or
    /// perform a [`std::process::exit`] yourself.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used.
    ///
    /// # Panics
    ///
    /// If contradictory arguments or settings exist.
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
    ///     .unwrap_or_else(|e| e.exit());
    /// ```
    /// [`App::get_matches_from`]: App::get_matches_from()
    /// [`App::try_get_matches`]: App::try_get_matches()
    /// [`Error::exit`]: crate::Error::exit()
    /// [`std::process::exit`]: std::process::exit()
    /// [`clap::Error`]: crate::Error
    /// [`Error::exit`]: crate::Error::exit()
    /// [`kind`]: crate::Error
    /// [`ErrorKind::DisplayHelp`]: crate::ErrorKind::DisplayHelp
    /// [`ErrorKind::DisplayVersion`]: crate::ErrorKind::DisplayVersion
    /// [`clap::Result`]: Result
    pub fn try_get_matches_from<I, T>(mut self, itr: I) -> ClapResult<ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from_mut(itr)
    }

    /// Parse the specified arguments, returning a [`clap::Result`] on failure.
    ///
    /// Like [`App::try_get_matches_from`] but doesn't consume the `App`.
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return a [`clap::Error`], where the [`kind`] is a [`ErrorKind::DisplayHelp`]
    /// or [`ErrorKind::DisplayVersion`] respectively. You must call [`Error::exit`] or
    /// perform a [`std::process::exit`] yourself.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used.
    ///
    /// # Panics
    ///
    /// If contradictory arguments or settings exist.
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
    ///     .unwrap_or_else(|e| e.exit());
    /// ```
    /// [`App::try_get_matches_from`]: App::try_get_matches_from()
    /// [`clap::Result`]: Result
    /// [`clap::Error`]: crate::Error
    /// [`kind`]: crate::Error
    pub fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut it = Input::from(itr.into_iter());

        #[cfg(feature = "unstable-multicall")]
        if self.settings.is_set(AppSettings::Multicall) {
            if let Some((argv0, _)) = it.next() {
                let argv0 = Path::new(&argv0);
                if let Some(command) = argv0.file_stem().and_then(|f| f.to_str()) {
                    // Stop borrowing command so we can get another mut ref to it.
                    let command = command.to_owned();
                    debug!(
                        "App::try_get_matches_from_mut: Parsed command {} from argv",
                        command
                    );

                    debug!("App::try_get_matches_from_mut: Reinserting command into arguments so subcommand parser matches it");
                    it.insert(&[&command]);
                    debug!("App::try_get_matches_from_mut: Clearing name and bin_name so that displayed command name starts with applet name");
                    self.name.clear();
                    self.bin_name = None;
                    return self._do_parse(&mut it);
                }
            }
        };

        // Get the name of the program (argument 1 of env::args()) and determine the
        // actual file
        // that was used to execute the program. This is because a program called
        // ./target/release/my_prog -a
        // will have two arguments, './target/release/my_prog', '-a' but we don't want
        // to display
        // the full path when displaying help messages and such
        if !self.settings.is_set(AppSettings::NoBinaryName) {
            if let Some((name, _)) = it.next() {
                let p = Path::new(name);

                if let Some(f) = p.file_name() {
                    if let Some(s) = f.to_str() {
                        if self.bin_name.is_none() {
                            self.bin_name = Some(s.to_owned());
                        }
                    }
                }
            }
        }

        self._do_parse(&mut it)
    }

    /// Prints the short help message (`-h`) to [`io::stdout()`].
    ///
    /// See also [`App::print_long_help`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// let mut app = App::new("myprog");
    /// app.print_help();
    /// ```
    /// [`io::stdout()`]: std::io::stdout()
    pub fn print_help(&mut self) -> io::Result<()> {
        self._build();
        let color = self.get_color();

        let p = Parser::new(self);
        let mut c = Colorizer::new(false, color);
        Help::new(HelpWriter::Buffer(&mut c), &p, false).write_help()?;
        c.print()
    }

    /// Prints the long help message (`--help`) to [`io::stdout()`].
    ///
    /// See also [`App::print_help`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// let mut app = App::new("myprog");
    /// app.print_long_help();
    /// ```
    /// [`io::stdout()`]: std::io::stdout()
    /// [`BufWriter`]: std::io::BufWriter
    /// [`-h` (short)]: Arg::help()
    /// [`--help` (long)]: Arg::long_help()
    pub fn print_long_help(&mut self) -> io::Result<()> {
        self._build();
        let color = self.get_color();

        let p = Parser::new(self);
        let mut c = Colorizer::new(false, color);
        Help::new(HelpWriter::Buffer(&mut c), &p, true).write_help()?;
        c.print()
    }

    /// Writes the short help message (`-h`) to a [`io::Write`] object.
    ///
    /// See also [`App::write_long_help`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_help(&mut out).expect("failed to write to stdout");
    /// ```
    /// [`io::Write`]: std::io::Write
    /// [`-h` (short)]: Arg::help()
    /// [`--help` (long)]: Arg::long_help()
    pub fn write_help<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        self._build();

        let p = Parser::new(self);
        Help::new(HelpWriter::Normal(w), &p, false).write_help()?;
        w.flush()
    }

    /// Writes the long help message (`--help`) to a [`io::Write`] object.
    ///
    /// See also [`App::write_help`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_long_help(&mut out).expect("failed to write to stdout");
    /// ```
    /// [`io::Write`]: std::io::Write
    /// [`-h` (short)]: Arg::help()
    /// [`--help` (long)]: Arg::long_help()
    pub fn write_long_help<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        self._build();

        let p = Parser::new(self);
        Help::new(HelpWriter::Normal(w), &p, true).write_help()?;
        w.flush()
    }

    /// Version message rendered as if the user ran `-V`.
    ///
    /// See also [`App::render_long_version`].
    ///
    /// ### Coloring
    ///
    /// This function does not try to color the message nor it inserts any [ANSI escape codes].
    ///
    /// ### Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// use std::io;
    /// let app = App::new("myprog");
    /// println!("{}", app.render_version());
    /// ```
    /// [`io::Write`]: std::io::Write
    /// [`-V` (short)]: App::version()
    /// [`--version` (long)]: App::long_version()
    /// [ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
    pub fn render_version(&self) -> String {
        self._render_version(false)
    }

    /// Version message rendered as if the user ran `--version`.
    ///
    /// See also [`App::render_version`].
    ///
    /// ### Coloring
    ///
    /// This function does not try to color the message nor it inserts any [ANSI escape codes].
    ///
    /// ### Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// use std::io;
    /// let app = App::new("myprog");
    /// println!("{}", app.render_long_version());
    /// ```
    /// [`io::Write`]: std::io::Write
    /// [`-V` (short)]: App::version()
    /// [`--version` (long)]: App::long_version()
    /// [ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
    pub fn render_long_version(&self) -> String {
        self._render_version(true)
    }

    /// Usage statement
    ///
    /// ### Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// use std::io;
    /// let mut app = App::new("myprog");
    /// println!("{}", app.render_usage());
    /// ```
    pub fn render_usage(&mut self) -> String {
        // If there are global arguments, or settings we need to propagate them down to subcommands
        // before parsing incase we run into a subcommand
        self._build();

        let mut parser = Parser::new(self);
        parser._build();
        Usage::new(&parser).create_usage_with_title(&[])
    }
}

/// App Settings
impl<'help> App<'help> {
    /// (Re)Sets the program's name.
    ///
    /// See [`App::new`] for more details.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, load_yaml};
    /// let yaml = load_yaml!("app.yaml");
    /// let app = App::from(yaml)
    ///     .name(crate_name!());
    ///
    /// // continued logic goes here, such as `app.get_matches()` etc.
    /// ```
    #[must_use]
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Overrides the runtime-determined name of the binary for help and error messages.
    ///
    /// This should only be used when absolutely necessary, such as when the binary name for your
    /// application is misleading, or perhaps *not* how the user should invoke your program.
    ///
    /// **Pro-tip:** When building things such as third party `cargo`
    /// subcommands, this setting **should** be used!
    ///
    /// **NOTE:** This *does not* change or set the name of the binary file on
    /// disk. It only changes what clap thinks the name is for the purposes of
    /// error or help messages.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("My Program")
    ///      .bin_name("my_binary")
    /// # ;
    /// ```
    #[must_use]
    pub fn bin_name<S: Into<String>>(mut self, name: S) -> Self {
        self.bin_name = Some(name.into());
        self
    }

    /// Sets the author(s) for the help message.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_authors!`] to
    /// automatically set your application's author(s) to the same thing as your
    /// crate at compile time.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///      .author("Me, me@mymain.com")
    /// # ;
    /// ```
    /// [`crate_authors!`]: ./macro.crate_authors!.html
    #[must_use]
    pub fn author<S: Into<&'help str>>(mut self, author: S) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sets the program's description for the short help (`-h`).
    ///
    /// If [`App::long_about`] is not specified, this message will be displayed for `--help`.
    ///
    /// **NOTE:** Only `App::about` (short format) is used in completion
    /// script generation in order to be concise.
    ///
    /// See also [`crate_description!`](crate::crate_description!).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .about("Does really amazing things for great people")
    /// # ;
    /// ```
    #[must_use]
    pub fn about<O: Into<Option<&'help str>>>(mut self, about: O) -> Self {
        self.about = about.into();
        self
    }

    /// Sets the program's description for the long help (`--help`).
    ///
    /// If [`App::about`] is not specified, this message will be displayed for `-h`.
    ///
    /// **NOTE:** Only [`App::about`] (short format) is used in completion
    /// script generation in order to be concise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .long_about(
    /// "Does really amazing things to great people. Now let's talk a little
    ///  more in depth about how this subcommand really works. It may take about
    ///  a few lines of text, but that's ok!")
    /// # ;
    /// ```
    /// [`App::about`]: App::about()
    #[must_use]
    pub fn long_about<O: Into<Option<&'help str>>>(mut self, long_about: O) -> Self {
        self.long_about = long_about.into();
        self
    }

    /// Free-form help text for after auto-generated short help (`-h`).
    ///
    /// This is often used to describe how to use the arguments, caveats to be noted, or license
    /// and contact information.
    ///
    /// If [`App::after_long_help`] is not specified, this message will be displayed for `--help`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .after_help("Does really amazing things for great people... but be careful with -R!")
    /// # ;
    /// ```
    ///
    #[must_use]
    pub fn after_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.after_help = Some(help.into());
        self
    }

    /// Free-form help text for after auto-generated long help (`--help`).
    ///
    /// This is often used to describe how to use the arguments, caveats to be noted, or license
    /// and contact information.
    ///
    /// If [`App::after_help`] is not specified, this message will be displayed for `-h`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .after_long_help("Does really amazing things to great people... but be careful with -R, \
    ///                      like, for real, be careful with this!")
    /// # ;
    /// ```
    #[must_use]
    pub fn after_long_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.after_long_help = Some(help.into());
        self
    }

    /// Free-form help text for before auto-generated short help (`-h`).
    ///
    /// This is often used for header, copyright, or license information.
    ///
    /// If [`App::before_long_help`] is not specified, this message will be displayed for `--help`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .before_help("Some info I'd like to appear before the help info")
    /// # ;
    /// ```
    #[must_use]
    pub fn before_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.before_help = Some(help.into());
        self
    }

    /// Free-form help text for before auto-generated long help (`--help`).
    ///
    /// This is often used for header, copyright, or license information.
    ///
    /// If [`App::before_help`] is not specified, this message will be displayed for `-h`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .before_long_help("Some verbose and long info I'd like to appear before the help info")
    /// # ;
    /// ```
    #[must_use]
    pub fn before_long_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.before_long_help = Some(help.into());
        self
    }

    /// Sets the version for the short version (`-V`) and help messages.
    ///
    /// If [`App::long_version`] is not specified, this message will be displayed for `--version`.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to
    /// automatically set your application's version to the same thing as your
    /// crate at compile time.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .version("v0.1.24")
    /// # ;
    /// ```
    /// [`crate_version!`]: ./macro.crate_version!.html
    #[must_use]
    pub fn version<S: Into<&'help str>>(mut self, ver: S) -> Self {
        self.version = Some(ver.into());
        self
    }

    /// Sets the version for the long version (`--version`) and help messages.
    ///
    /// If [`App::version`] is not specified, this message will be displayed for `-V`.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to
    /// automatically set your application's version to the same thing as your
    /// crate at compile time.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .long_version(
    /// "v0.1.24
    ///  commit: abcdef89726d
    ///  revision: 123
    ///  release: 2
    ///  binary: myprog")
    /// # ;
    /// ```
    /// [`crate_version!`]: ./macro.crate_version!.html
    #[must_use]
    pub fn long_version<S: Into<&'help str>>(mut self, ver: S) -> Self {
        self.long_version = Some(ver.into());
        self
    }

    /// Overrides the `clap` generated usage string for help and error messages.
    ///
    /// **NOTE:** Using this setting disables `clap`s "context-aware" usage
    /// strings. After this setting is set, this will be *the only* usage string
    /// displayed to the user!
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .override_usage("myapp [-clDas] <some_file>")
    /// # ;
    /// ```
    /// [`ArgMatches::usage`]: ArgMatches::usage()
    #[must_use]
    pub fn override_usage<S: Into<&'help str>>(mut self, usage: S) -> Self {
        self.usage_str = Some(usage.into());
        self
    }

    /// Overrides the `clap` generated help message (both `-h` and `--help`).
    ///
    /// This should only be used when the auto-generated message does not suffice.
    ///
    /// **NOTE:** This **only** replaces the help message for the current
    /// command, meaning if you are using subcommands, those help messages will
    /// still be auto-generated unless you specify a [`App::override_help`] for
    /// them as well.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myapp")
    ///     .override_help("myapp v1.0\n\
    ///            Does awesome things\n\
    ///            (C) me@mail.com\n\n\
    ///
    ///            USAGE: myapp <opts> <command>\n\n\
    ///
    ///            Options:\n\
    ///            -h, --help       Display this message\n\
    ///            -V, --version    Display version info\n\
    ///            -s <stuff>       Do something with stuff\n\
    ///            -v               Be verbose\n\n\
    ///
    ///            Commands:\n\
    ///            help             Print this message\n\
    ///            work             Do some work")
    /// # ;
    /// ```
    #[must_use]
    pub fn override_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.help_str = Some(help.into());
        self
    }

    /// Sets the help template to be used, overriding the default format.
    ///
    /// **NOTE:** The template system is by design very simple. Therefore, the
    /// tags have to be written in the lowercase and without spacing.
    ///
    /// Tags are given inside curly brackets.
    ///
    /// Valid tags are:
    ///
    ///   * `{bin}`                 - Binary name.
    ///   * `{version}`             - Version number.
    ///   * `{author}`              - Author information.
    ///   * `{author-with-newline}` - Author followed by `\n`.
    ///   * `{author-section}`      - Author preceded and followed by `\n`.
    ///   * `{about}`               - General description (from [`App::about`] or
    ///                               [`App::long_about`]).
    ///   * `{about-with-newline}`  - About followed by `\n`.
    ///   * `{about-section}`       - About preceded and followed by '\n'.
    ///   * `{usage-heading}`       - Automatically generated usage heading.
    ///   * `{usage}`               - Automatically generated or given usage string.
    ///   * `{all-args}`            - Help for all arguments (options, flags, positional
    ///                               arguments, and subcommands) including titles.
    ///   * `{options}`             - Help for options.
    ///   * `{positionals}`         - Help for positional arguments.
    ///   * `{subcommands}`         - Help for subcommands.
    ///   * `{after-help}`          - Help from [`App::after_help`] or [`App::after_long_help`].
    ///   * `{before-help}`         - Help from [`App::before_help`] or [`App::before_long_help`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .version("1.0")
    ///     .help_template("{bin} ({version}) - {usage}")
    /// # ;
    /// ```
    /// [`App::about`]: App::about()
    /// [`App::long_about`]: App::long_about()
    /// [`App::after_help`]: App::after_help()
    /// [`App::after_long_help`]: App::after_long_help()
    /// [`App::before_help`]: App::before_help()
    /// [`App::before_long_help`]: App::before_long_help()
    #[must_use]
    pub fn help_template<S: Into<&'help str>>(mut self, s: S) -> Self {
        self.template = Some(s.into());
        self
    }

    /// Apply a setting for the current command or subcommand.
    ///
    /// See [`App::global_setting`] to apply a setting to this command and all subcommands.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .setting(AppSettings::AllowLeadingHyphen)
    /// # ;
    /// ```
    /// or
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired | AppSettings::AllowLeadingHyphen)
    /// # ;
    /// ```
    #[inline]
    #[must_use]
    pub fn setting<F>(mut self, setting: F) -> Self
    where
        F: Into<AppFlags>,
    {
        self.settings.insert(setting.into());
        self
    }

    /// Remove a setting for the current command or subcommand.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .unset_setting(AppSettings::SubcommandRequired)
    ///     .setting(AppSettings::AllowLeadingHyphen)
    /// # ;
    /// ```
    /// or
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .unset_setting(AppSettings::SubcommandRequired | AppSettings::AllowLeadingHyphen)
    /// # ;
    /// ```
    #[inline]
    #[must_use]
    pub fn unset_setting<F>(mut self, setting: F) -> Self
    where
        F: Into<AppFlags>,
    {
        self.settings.remove(setting.into());
        self
    }

    /// Apply a setting for the current command and all subcommands.
    ///
    /// See [`App::setting`] to apply a setting only to this command.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .global_setting(AppSettings::AllowNegativeNumbers)
    /// # ;
    /// ```
    #[inline]
    #[must_use]
    pub fn global_setting(mut self, setting: AppSettings) -> Self {
        self.settings.set(setting);
        self.g_settings.set(setting);
        self
    }

    /// Remove a setting and stop propagating down to subcommands.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .unset_global_setting(AppSettings::AllowNegativeNumbers)
    /// # ;
    /// ```
    /// [global]: App::global_setting()
    #[inline]
    #[must_use]
    pub fn unset_global_setting(mut self, setting: AppSettings) -> Self {
        self.settings.unset(setting);
        self.g_settings.unset(setting);
        self
    }

    /// Sets when to color output.
    ///
    /// **NOTE:** This choice is propagated to all child subcommands.
    ///
    /// **NOTE:** Default behaviour is [`ColorChoice::Auto`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, ColorChoice};
    /// App::new("myprog")
    ///     .color(ColorChoice::Never)
    ///     .get_matches();
    /// ```
    /// [`ColorChoice::Auto`]: crate::ColorChoice::Auto
    #[cfg(feature = "color")]
    #[inline]
    #[must_use]
    pub fn color(self, color: ColorChoice) -> Self {
        #[allow(deprecated)]
        match color {
            ColorChoice::Auto => self.global_setting(AppSettings::ColorAuto),
            ColorChoice::Always => self.global_setting(AppSettings::ColorAlways),
            ColorChoice::Never => self.global_setting(AppSettings::ColorNever),
        }
    }

    /// Set the default section heading for future args.
    ///
    /// This will be used for any arg that hasn't had [`Arg::help_heading`] called.
    ///
    /// This is useful if the default `OPTIONS` or `ARGS` headings are
    /// not specific enough for one's use case.
    ///
    /// For subcommands, see [`App::subcommand_help_heading`]
    ///
    /// [`App::arg`]: App::arg()
    /// [`Arg::help_heading`]: crate::Arg::help_heading()
    #[inline]
    #[must_use]
    pub fn help_heading<O>(mut self, heading: O) -> Self
    where
        O: Into<Option<&'help str>>,
    {
        self.current_help_heading = heading.into();
        self
    }

    /// Sets the terminal width at which to wrap help messages.
    ///
    /// Using `0` will ignore terminal widths and use source formatting.
    ///
    /// Defaults to current terminal width when `wrap_help` feature flag is enabled.  If the flag
    /// is disabled or it cannot be determined, the default is 100.
    ///
    /// **NOTE:** This setting applies globally and *not* on a per-command basis.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .term_width(80)
    /// # ;
    /// ```
    #[inline]
    #[must_use]
    pub fn term_width(mut self, width: usize) -> Self {
        self.term_w = Some(width);
        self
    }

    /// Sets the maximum terminal width at which to wrap help messages.
    ///
    /// This only applies when setting the current terminal width.  See [`App::term_width`] for
    /// more details.
    ///
    /// Using `0` will ignore terminal widths and use source formatting.
    ///
    /// **NOTE:** This setting applies globally and *not* on a per-command basis.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .max_term_width(100)
    /// # ;
    /// ```
    #[inline]
    #[must_use]
    pub fn max_term_width(mut self, w: usize) -> Self {
        self.max_w = Some(w);
        self
    }

    /// Replaces an argument or subcommand used on the CLI at runtime with other arguments or subcommands.
    ///
    /// **Note:** This is gated behind [`unstable-replace`](https://github.com/clap-rs/clap/issues/2836)
    ///
    /// When this method is used, `name` is removed from the CLI, and `target`
    /// is inserted in its place. Parsing continues as if the user typed
    /// `target` instead of `name`.
    ///
    /// This can be used to create "shortcuts" for subcommands, or if a
    /// particular argument has the semantic meaning of several other specific
    /// arguments and values.
    ///
    /// # Examples
    ///
    /// We'll start with the "subcommand short" example. In this example, let's
    /// assume we have a program with a subcommand `module` which can be invoked
    /// via `app module`. Now let's also assume `module` also has a subcommand
    /// called `install` which can be invoked `app module install`. If for some
    /// reason users needed to be able to reach `app module install` via the
    /// short-hand `app install`, we'd have several options.
    ///
    /// We *could* create another sibling subcommand to `module` called
    /// `install`, but then we would need to manage another subcommand and manually
    /// dispatch to `app module install` handling code. This is error prone and
    /// tedious.
    ///
    /// We could instead use [`App::replace`] so that, when the user types `app
    /// install`, `clap` will replace `install` with `module install` which will
    /// end up getting parsed as if the user typed the entire incantation.
    ///
    /// ```rust
    /// # use clap::App;
    /// let m = App::new("app")
    ///     .subcommand(App::new("module")
    ///         .subcommand(App::new("install")))
    ///     .replace("install", &["module", "install"])
    ///     .get_matches_from(vec!["app", "install"]);
    ///
    /// assert!(m.subcommand_matches("module").is_some());
    /// assert!(m.subcommand_matches("module").unwrap().subcommand_matches("install").is_some());
    /// ```
    ///
    /// Now let's show an argument example!
    ///
    /// Let's assume we have an application with two flags `--save-context` and
    /// `--save-runtime`. But often users end up needing to do *both* at the
    /// same time. We can add a third flag `--save-all` which semantically means
    /// the same thing as `app --save-context --save-runtime`. To implement that,
    /// we have several options.
    ///
    /// We could create this third argument and manually check if that argument
    /// and in our own consumer code handle the fact that both `--save-context`
    /// and `--save-runtime` *should* have been used. But again this is error
    /// prone and tedious. If we had code relying on checking `--save-context`
    /// and we forgot to update that code to *also* check `--save-all` it'd mean
    /// an error!
    ///
    /// Luckily we can use [`App::replace`] so that when the user types
    /// `--save-all`, `clap` will replace that argument with `--save-context
    /// --save-runtime`, and parsing will continue like normal. Now all our code
    /// that was originally checking for things like `--save-context` doesn't
    /// need to change!
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("app")
    ///     .arg(Arg::new("save-context")
    ///         .long("save-context"))
    ///     .arg(Arg::new("save-runtime")
    ///         .long("save-runtime"))
    ///     .replace("--save-all", &["--save-context", "--save-runtime"])
    ///     .get_matches_from(vec!["app", "--save-all"]);
    ///
    /// assert!(m.is_present("save-context"));
    /// assert!(m.is_present("save-runtime"));
    /// ```
    ///
    /// This can also be used with options, for example if our application with
    /// `--save-*` above also had a `--format=TYPE` option. Let's say it
    /// accepted `txt` or `json` values. However, when `--save-all` is used,
    /// only `--format=json` is allowed, or valid. We could change the example
    /// above to enforce this:
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("app")
    ///     .arg(Arg::new("save-context")
    ///         .long("save-context"))
    ///     .arg(Arg::new("save-runtime")
    ///         .long("save-runtime"))
    ///     .arg(Arg::new("format")
    ///         .long("format")
    ///         .takes_value(true)
    ///         .possible_values(["txt", "json"]))
    ///     .replace("--save-all", &["--save-context", "--save-runtime", "--format=json"])
    ///     .get_matches_from(vec!["app", "--save-all"]);
    ///
    /// assert!(m.is_present("save-context"));
    /// assert!(m.is_present("save-runtime"));
    /// assert_eq!(m.value_of("format"), Some("json"));
    /// ```
    ///
    /// [`App::replace`]: App::replace()
    #[inline]
    #[cfg(feature = "unstable-replace")]
    #[must_use]
    pub fn replace(mut self, name: &'help str, target: &'help [&'help str]) -> Self {
        self.replacers.insert(name, target);
        self
    }
}

/// Subcommand-specific Settings
impl<'help> App<'help> {
    /// Sets the short version of the subcommand flag without the preceding `-`.
    ///
    /// Allows the subcommand to be used as if it were an [`Arg::short`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use clap::{App, Arg};
    /// let matches = App::new("pacman")
    ///     .subcommand(
    ///         App::new("sync").short_flag('S').arg(
    ///             Arg::new("search")
    ///                 .short('s')
    ///                 .long("search")
    ///                 .help("search remote repositories for matching strings"),
    ///         ),
    ///     )
    ///     .get_matches_from(vec!["pacman", "-Ss"]);
    ///
    /// assert_eq!(matches.subcommand_name().unwrap(), "sync");
    /// let sync_matches = matches.subcommand_matches("sync").unwrap();
    /// assert!(sync_matches.is_present("search"));
    /// ```
    /// [`Arg::short`]: Arg::short()
    #[must_use]
    pub fn short_flag(mut self, short: char) -> Self {
        self.short_flag = Some(short);
        self
    }

    /// Sets the long version of the subcommand flag without the preceding `--`.
    ///
    /// Allows the subcommand to be used as if it were an [`Arg::long`].
    ///
    /// **NOTE:** Any leading `-` characters will be stripped.
    ///
    /// # Examples
    ///
    /// To set `long_flag` use a word containing valid UTF-8 codepoints. If you supply a double leading
    /// `--` such as `--sync` they will be stripped. Hyphens in the middle of the word; however,
    /// will *not* be stripped (i.e. `sync-file` is allowed).
    ///
    /// ```
    /// # use clap::{App, Arg};
    /// let matches = App::new("pacman")
    ///     .subcommand(
    ///         App::new("sync").long_flag("sync").arg(
    ///             Arg::new("search")
    ///                 .short('s')
    ///                 .long("search")
    ///                 .help("search remote repositories for matching strings"),
    ///         ),
    ///     )
    ///     .get_matches_from(vec!["pacman", "--sync", "--search"]);
    ///
    /// assert_eq!(matches.subcommand_name().unwrap(), "sync");
    /// let sync_matches = matches.subcommand_matches("sync").unwrap();
    /// assert!(sync_matches.is_present("search"));
    /// ```
    ///
    /// [`Arg::long`]: Arg::long()
    #[must_use]
    pub fn long_flag(mut self, long: &'help str) -> Self {
        self.long_flag = Some(long.trim_start_matches(|c| c == '-'));
        self
    }

    /// Sets a hidden alias to this subcommand.
    ///
    /// This allows the subcommand to be accessed via *either* the original name, or this given
    /// alias. This is more efficient and easier than creating multiple hidden subcommands as one
    /// only needs to check for the existence of this command, and not all aliased variants.
    ///
    /// **NOTE:** Aliases defined with this method are *hidden* from the help
    /// message. If you're looking for aliases that will be displayed in the help
    /// message, see [`App::visible_alias`].
    ///
    /// **NOTE:** When using aliases and checking for the existence of a
    /// particular subcommand within an [`ArgMatches`] struct, one only needs to
    /// search for the original name and not all aliases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///     .subcommand(App::new("test")
    ///         .alias("do-stuff"))
    ///     .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::visible_alias`]: App::visible_alias()
    #[must_use]
    pub fn alias<S: Into<&'help str>>(mut self, name: S) -> Self {
        self.aliases.push((name.into(), false));
        self
    }

    /// Add an alias, which functions as  "hidden" short flag subcommand
    ///
    /// This will automatically dispatch as if this subcommand was used. This is more efficient,
    /// and easier than creating multiple hidden subcommands as one only needs to check for the
    /// existence of this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").short_flag('t')
    ///                 .short_flag_alias('d'))
    ///             .get_matches_from(vec!["myprog", "-d"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    #[must_use]
    pub fn short_flag_alias(mut self, name: char) -> Self {
        assert!(name != '-', "short alias name cannot be `-`");
        self.short_flag_aliases.push((name, false));
        self
    }

    /// Add an alias, which functions as a "hidden" long flag subcommand.
    ///
    /// This will automatically dispatch as if this subcommand was used. This is more efficient,
    /// and easier than creating multiple hidden subcommands as one only needs to check for the
    /// existence of this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").long_flag("test")
    ///                 .long_flag_alias("testing"))
    ///             .get_matches_from(vec!["myprog", "--testing"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    #[must_use]
    pub fn long_flag_alias(mut self, name: &'help str) -> Self {
        self.long_flag_aliases.push((name, false));
        self
    }

    /// Sets multiple hidden aliases to this subcommand.
    ///
    /// This allows the subcommand to be accessed via *either* the original name or any of the
    /// given aliases. This is more efficient, and easier than creating multiple hidden subcommands
    /// as one only needs to check for the existence of this command and not all aliased variants.
    ///
    /// **NOTE:** Aliases defined with this method are *hidden* from the help
    /// message. If looking for aliases that will be displayed in the help
    /// message, see [`App::visible_aliases`].
    ///
    /// **NOTE:** When using aliases and checking for the existence of a
    /// particular subcommand within an [`ArgMatches`] struct, one only needs to
    /// search for the original name and not all aliases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .subcommand(App::new("test")
    ///         .aliases(&["do-stuff", "do-tests", "tests"]))
    ///         .arg(Arg::new("input")
    ///             .help("the file to add")
    ///             .index(1)
    ///             .required(false))
    ///     .get_matches_from(vec!["myprog", "do-tests"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::visible_aliases`]: App::visible_aliases()
    #[must_use]
    pub fn aliases(mut self, names: &[&'help str]) -> Self {
        self.aliases.extend(names.iter().map(|n| (*n, false)));
        self
    }

    /// Add aliases, which function as "hidden" short flag subcommands.
    ///
    /// These will automatically dispatch as if this subcommand was used. This is more efficient,
    /// and easier than creating multiple hidden subcommands as one only needs to check for the
    /// existence of this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///     .subcommand(App::new("test").short_flag('t')
    ///         .short_flag_aliases(&['a', 'b', 'c']))
    ///         .arg(Arg::new("input")
    ///             .help("the file to add")
    ///             .index(1)
    ///             .required(false))
    ///     .get_matches_from(vec!["myprog", "-a"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    #[must_use]
    pub fn short_flag_aliases(mut self, names: &[char]) -> Self {
        for s in names {
            assert!(s != &'-', "short alias name cannot be `-`");
            self.short_flag_aliases.push((*s, false));
        }
        self
    }

    /// Add aliases, which function as "hidden" long flag subcommands.
    ///
    /// These will automatically dispatch as if this subcommand was used. This is more efficient,
    /// and easier than creating multiple hidden subcommands as one only needs to check for the
    /// existence of this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").long_flag("test")
    ///                 .long_flag_aliases(&["testing", "testall", "test_all"]))
    ///                 .arg(Arg::new("input")
    ///                             .help("the file to add")
    ///                             .index(1)
    ///                             .required(false))
    ///             .get_matches_from(vec!["myprog", "--testing"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    #[must_use]
    pub fn long_flag_aliases(mut self, names: &[&'help str]) -> Self {
        for s in names {
            self.long_flag_aliases.push((s, false));
        }
        self
    }

    /// Sets a visible alias to this subcommand.
    ///
    /// This allows the subcommand to be accessed via *either* the
    /// original name or the given alias. This is more efficient and easier
    /// than creating hidden subcommands as one only needs to check for
    /// the existence of this command and not all aliased variants.
    ///
    /// **NOTE:** The alias defined with this method is *visible* from the help
    /// message and displayed as if it were just another regular subcommand. If
    /// looking for an alias that will not be displayed in the help message, see
    /// [`App::alias`].
    ///
    /// **NOTE:** When using aliases and checking for the existence of a
    /// particular subcommand within an [`ArgMatches`] struct, one only needs to
    /// search for the original name and not all aliases.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .subcommand(App::new("test")
    ///         .visible_alias("do-stuff"))
    ///     .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::alias`]: App::alias()
    #[must_use]
    pub fn visible_alias<S: Into<&'help str>>(mut self, name: S) -> Self {
        self.aliases.push((name.into(), true));
        self
    }

    /// Add an alias, which functions as  "visible" short flag subcommand
    ///
    /// This will automatically dispatch as if this subcommand was used. This is more efficient,
    /// and easier than creating multiple hidden subcommands as one only needs to check for the
    /// existence of this command, and not all variants.
    ///
    /// See also [`App::short_flag_alias`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").short_flag('t')
    ///                 .visible_short_flag_alias('d'))
    ///             .get_matches_from(vec!["myprog", "-d"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::short_flag_alias`]: App::short_flag_alias()
    #[must_use]
    pub fn visible_short_flag_alias(mut self, name: char) -> Self {
        assert!(name != '-', "short alias name cannot be `-`");
        self.short_flag_aliases.push((name, true));
        self
    }

    /// Add an alias, which functions as a "visible" long flag subcommand.
    ///
    /// This will automatically dispatch as if this subcommand was used. This is more efficient,
    /// and easier than creating multiple hidden subcommands as one only needs to check for the
    /// existence of this command, and not all variants.
    ///
    /// See also [`App::long_flag_alias`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").long_flag("test")
    ///                 .visible_long_flag_alias("testing"))
    ///             .get_matches_from(vec!["myprog", "--testing"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::long_flag_alias`]: App::long_flag_alias()
    #[must_use]
    pub fn visible_long_flag_alias(mut self, name: &'help str) -> Self {
        self.long_flag_aliases.push((name, true));
        self
    }

    /// Sets multiple visible aliases to this subcommand.
    ///
    /// This allows the subcommand to be accessed via *either* the
    /// original name or any of the given aliases. This is more efficient and easier
    /// than creating multiple hidden subcommands as one only needs to check for
    /// the existence of this command and not all aliased variants.
    ///
    /// **NOTE:** The alias defined with this method is *visible* from the help
    /// message and displayed as if it were just another regular subcommand. If
    /// looking for an alias that will not be displayed in the help message, see
    /// [`App::alias`].
    ///
    /// **NOTE:** When using aliases, and checking for the existence of a
    /// particular subcommand within an [`ArgMatches`] struct, one only needs to
    /// search for the original name and not all aliases.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///     .subcommand(App::new("test")
    ///         .visible_aliases(&["do-stuff", "tests"]))
    ///     .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::alias`]: App::alias()
    #[must_use]
    pub fn visible_aliases(mut self, names: &[&'help str]) -> Self {
        self.aliases.extend(names.iter().map(|n| (*n, true)));
        self
    }

    /// Add aliases, which function as *visible* short flag subcommands.
    ///
    /// See [`App::short_flag_aliases`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").short_flag('b')
    ///                 .visible_short_flag_aliases(&['t']))
    ///             .get_matches_from(vec!["myprog", "-t"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::short_flag_aliases`]: App::short_flag_aliases()
    #[must_use]
    pub fn visible_short_flag_aliases(mut self, names: &[char]) -> Self {
        for s in names {
            assert!(s != &'-', "short alias name cannot be `-`");
            self.short_flag_aliases.push((*s, true));
        }
        self
    }

    /// Add aliases, which function as *visible* long flag subcommands.
    ///
    /// See [`App::long_flag_aliases`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").long_flag("test")
    ///                 .visible_long_flag_aliases(&["testing", "testall", "test_all"]))
    ///             .get_matches_from(vec!["myprog", "--testing"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`App::long_flag_aliases`]: App::long_flag_aliases()
    #[must_use]
    pub fn visible_long_flag_aliases(mut self, names: &[&'help str]) -> Self {
        for s in names {
            self.long_flag_aliases.push((s, true));
        }
        self
    }

    /// Set the placement of this subcommand within the help.
    ///
    /// Subcommands with a lower value will be displayed first in the help message.  Subcommands
    /// with duplicate display orders will be displayed in alphabetical order.
    ///
    /// This is helpful when one would like to emphasize frequently used subcommands, or prioritize
    /// those towards the top of the list.
    ///
    /// **NOTE:** The default is 999 for all subcommands.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, };
    /// let m = App::new("cust-ord")
    ///     .subcommand(App::new("alpha") // typically subcommands are grouped
    ///                                                // alphabetically by name. Subcommands
    ///                                                // without a display_order have a value of
    ///                                                // 999 and are displayed alphabetically with
    ///                                                // all other 999 subcommands
    ///         .about("Some help and text"))
    ///     .subcommand(App::new("beta")
    ///         .display_order(1)   // In order to force this subcommand to appear *first*
    ///                             // all we have to do is give it a value lower than 999.
    ///                             // Any other subcommands with a value of 1 will be displayed
    ///                             // alphabetically with this one...then 2 values, then 3, etc.
    ///         .about("I should be first!"))
    ///     .get_matches_from(vec![
    ///         "cust-ord", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays the following help message
    ///
    /// ```text
    /// cust-ord
    ///
    /// USAGE:
    ///     cust-ord [OPTIONS]
    ///
    /// OPTIONS:
    ///     -h, --help       Print help information
    ///     -V, --version    Print version information
    ///
    /// SUBCOMMANDS:
    ///     beta    I should be first!
    ///     alpha   Some help and text
    /// ```
    #[inline]
    #[must_use]
    pub fn display_order(mut self, ord: usize) -> Self {
        self.disp_ord = Some(ord);
        self
    }

    /// Sets the value name used for subcommands when printing usage and help.
    ///
    /// By default, this is "SUBCOMMAND".
    ///
    /// See also [`App::subcommand_help_heading`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .subcommand(App::new("sub1"))
    ///     .print_help()
    /// # ;
    /// ```
    ///
    /// will produce
    ///
    /// ```text
    /// myprog
    ///
    /// USAGE:
    ///     myprog [SUBCOMMAND]
    ///
    /// OPTIONS:
    ///     -h, --help       Print help information
    ///     -V, --version    Print version information
    ///
    /// SUBCOMMANDS:
    ///     help    Print this message or the help of the given subcommand(s)
    ///     sub1
    /// ```
    ///
    /// but usage of `subcommand_value_name`
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .subcommand(App::new("sub1"))
    ///     .subcommand_value_name("THING")
    ///     .print_help()
    /// # ;
    /// ```
    ///
    /// will produce
    ///
    /// ```text
    /// myprog
    ///
    /// USAGE:
    ///     myprog [THING]
    ///
    /// OPTIONS:
    ///     -h, --help       Print help information
    ///     -V, --version    Print version information
    ///
    /// SUBCOMMANDS:
    ///     help    Print this message or the help of the given subcommand(s)
    ///     sub1
    /// ```
    #[must_use]
    pub fn subcommand_value_name<S>(mut self, value_name: S) -> Self
    where
        S: Into<&'help str>,
    {
        self.subcommand_value_name = Some(value_name.into());
        self
    }

    /// Sets the help heading used for subcommands when printing usage and help.
    ///
    /// By default, this is "SUBCOMMANDS".
    ///
    /// See also [`App::subcommand_value_name`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .subcommand(App::new("sub1"))
    ///     .print_help()
    /// # ;
    /// ```
    ///
    /// will produce
    ///
    /// ```text
    /// myprog
    ///
    /// USAGE:
    ///     myprog [SUBCOMMAND]
    ///
    /// OPTIONS:
    ///     -h, --help       Print help information
    ///     -V, --version    Print version information
    ///
    /// SUBCOMMANDS:
    ///     help    Print this message or the help of the given subcommand(s)
    ///     sub1
    /// ```
    ///
    /// but usage of `subcommand_help_heading`
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .subcommand(App::new("sub1"))
    ///     .subcommand_help_heading("THINGS")
    ///     .print_help()
    /// # ;
    /// ```
    ///
    /// will produce
    ///
    /// ```text
    /// myprog
    ///
    /// USAGE:
    ///     myprog [SUBCOMMAND]
    ///
    /// OPTIONS:
    ///     -h, --help       Print help information
    ///     -V, --version    Print version information
    ///
    /// THINGS:
    ///     help    Print this message or the help of the given subcommand(s)
    ///     sub1
    /// ```
    #[must_use]
    pub fn subcommand_help_heading<T>(mut self, heading: T) -> Self
    where
        T: Into<&'help str>,
    {
        self.subcommand_heading = Some(heading.into());
        self
    }
}

/// Reflection
impl<'help> App<'help> {
    /// Get the name of the binary.
    #[inline]
    pub fn get_bin_name(&self) -> Option<&str> {
        self.bin_name.as_deref()
    }

    /// Set binary name. Uses `&mut self` instead of `self`.
    pub fn set_bin_name<S: Into<String>>(&mut self, name: S) {
        self.bin_name = Some(name.into());
    }

    /// Get the name of the app.
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the short flag of the subcommand.
    #[inline]
    pub fn get_short_flag(&self) -> Option<char> {
        self.short_flag
    }

    /// Get the long flag of the subcommand.
    #[inline]
    pub fn get_long_flag(&self) -> Option<&'help str> {
        self.long_flag
    }

    /// Get the help message specified via [`App::about`].
    ///
    /// [`App::about`]: App::about()
    #[inline]
    pub fn get_about(&self) -> Option<&'help str> {
        self.about
    }

    /// Get the help message specified via [`App::long_about`].
    ///
    /// [`App::long_about`]: App::long_about()
    #[inline]
    pub fn get_long_about(&self) -> Option<&'help str> {
        self.long_about
    }

    /// Get the custom section heading specified via [`App::help_heading`].
    ///
    /// [`App::help_heading`]: App::help_heading()
    #[inline]
    pub fn get_help_heading(&self) -> Option<&'help str> {
        self.current_help_heading
    }

    /// Iterate through the *visible* aliases for this subcommand.
    #[inline]
    pub fn get_visible_aliases(&self) -> impl Iterator<Item = &'help str> + '_ {
        self.aliases.iter().filter(|(_, vis)| *vis).map(|a| a.0)
    }

    /// Iterate through the *visible* short aliases for this subcommand.
    #[inline]
    pub fn get_visible_short_flag_aliases(&self) -> impl Iterator<Item = char> + '_ {
        self.short_flag_aliases
            .iter()
            .filter(|(_, vis)| *vis)
            .map(|a| a.0)
    }

    /// Iterate through the *visible* long aliases for this subcommand.
    #[inline]
    pub fn get_visible_long_flag_aliases(&self) -> impl Iterator<Item = &'help str> + '_ {
        self.long_flag_aliases
            .iter()
            .filter(|(_, vis)| *vis)
            .map(|a| a.0)
    }

    /// Iterate through the set of *all* the aliases for this subcommand, both visible and hidden.
    #[inline]
    pub fn get_all_aliases(&self) -> impl Iterator<Item = &str> + '_ {
        self.aliases.iter().map(|a| a.0)
    }

    /// Iterate through the set of *all* the short aliases for this subcommand, both visible and hidden.
    #[inline]
    pub fn get_all_short_flag_aliases(&self) -> impl Iterator<Item = char> + '_ {
        self.short_flag_aliases.iter().map(|a| a.0)
    }

    /// Iterate through the set of *all* the long aliases for this subcommand, both visible and hidden.
    #[inline]
    pub fn get_all_long_flag_aliases(&self) -> impl Iterator<Item = &'help str> + '_ {
        self.long_flag_aliases.iter().map(|a| a.0)
    }

    /// Check if the given [`AppSettings`] variant is currently set on the `App`.
    ///
    /// This checks both [local] and [global settings].
    ///
    /// [local]: App::setting()
    /// [global settings]: App::global_setting()
    #[inline]
    pub fn is_set(&self, s: AppSettings) -> bool {
        self.settings.is_set(s) || self.g_settings.is_set(s)
    }

    /// Should we color the output?
    #[inline]
    pub fn get_color(&self) -> ColorChoice {
        debug!("App::color: Color setting...");

        if cfg!(feature = "color") {
            #[allow(deprecated)]
            if self.is_set(AppSettings::ColorNever) {
                debug!("Never");
                ColorChoice::Never
            } else if self.is_set(AppSettings::ColorAlways) {
                debug!("Always");
                ColorChoice::Always
            } else {
                debug!("Auto");
                ColorChoice::Auto
            }
        } else {
            ColorChoice::Never
        }
    }

    /// Iterate through the set of subcommands, getting a reference to each.
    #[inline]
    pub fn get_subcommands(&self) -> impl Iterator<Item = &App<'help>> {
        self.subcommands.iter()
    }

    /// Iterate through the set of subcommands, getting a mutable reference to each.
    #[inline]
    pub fn get_subcommands_mut(&mut self) -> impl Iterator<Item = &mut App<'help>> {
        self.subcommands.iter_mut()
    }

    /// Returns `true` if this `App` has subcommands.
    #[inline]
    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_empty()
    }

    /// Find subcommand such that its name or one of aliases equals `name`.
    ///
    /// This does not recurse through subcommands of subcommands.
    #[inline]
    pub fn find_subcommand<T>(&self, name: &T) -> Option<&App<'help>>
    where
        T: PartialEq<str> + ?Sized,
    {
        self.get_subcommands().find(|s| s.aliases_to(name))
    }

    /// Find subcommand such that its name or one of aliases equals `name`, returning
    /// a mutable reference to the subcommand.
    ///
    /// This does not recurse through subcommands of subcommands.
    #[inline]
    pub fn find_subcommand_mut<T>(&mut self, name: &T) -> Option<&mut App<'help>>
    where
        T: PartialEq<str> + ?Sized,
    {
        self.get_subcommands_mut().find(|s| s.aliases_to(name))
    }

    /// Iterate through the set of arguments.
    #[inline]
    pub fn get_arguments(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.args.args()
    }

    /// Iterate through the *positionals* arguments.
    #[inline]
    pub fn get_positionals(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_arguments().filter(|a| a.is_positional())
    }

    /// Iterate through the *options*.
    pub fn get_opts(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_arguments()
            .filter(|a| a.is_set(ArgSettings::TakesValue) && !a.is_positional())
    }

    /// Get a list of all arguments the given argument conflicts with.
    ///
    /// If the provided argument is declared as global, the conflicts will be determined
    /// based on the propagation rules of global arguments.
    ///
    /// ### Panics
    ///
    /// If the given arg contains a conflict with an argument that is unknown to
    /// this `App`.
    pub fn get_arg_conflicts_with(&self, arg: &Arg) -> Vec<&Arg<'help>> // FIXME: This could probably have been an iterator
    {
        if arg.get_global() {
            self.get_global_arg_conflicts_with(arg)
        } else {
            arg.blacklist
                .iter()
                .map(|id| {
                    self.args.args().find(|arg| arg.id == *id).expect(
                        "App::get_arg_conflicts_with: \
                    The passed arg conflicts with an arg unknown to the app",
                    )
                })
                .collect()
        }
    }

    // Get a unique list of all arguments of all commands and continuous subcommands the given argument conflicts with.
    //
    // This behavior follows the propagation rules of global arguments.
    // It is useful for finding conflicts for arguments declared as global.
    //
    // ### Panics
    //
    // If the given arg contains a conflict with an argument that is unknown to
    // this `App`.
    fn get_global_arg_conflicts_with(&self, arg: &Arg) -> Vec<&Arg<'help>> // FIXME: This could probably have been an iterator
    {
        arg.blacklist
            .iter()
            .map(|id| {
                self.args
                    .args()
                    .chain(
                        self.get_subcommands_containing(arg)
                            .iter()
                            .flat_map(|x| x.args.args()),
                    )
                    .find(|arg| arg.id == *id)
                    .expect(
                        "App::get_arg_conflicts_with: \
                    The passed arg conflicts with an arg unknown to the app",
                    )
            })
            .collect()
    }

    // Get a list of subcommands which contain the provided Argument
    //
    // This command will only include subcommands in its list for which the subcommands
    // parent also contains the Argument.
    //
    // This search follows the propagation rules of global arguments.
    // It is useful to finding subcommands, that have inherited a global argument.
    //
    // **NOTE:** In this case only Sucommand_1 will be included
    //   Subcommand_1 (contains Arg)
    //     Subcommand_1.1 (doesn't contain Arg)
    //       Subcommand_1.1.1 (contains Arg)
    //
    fn get_subcommands_containing(&self, arg: &Arg) -> Vec<&App<'help>> {
        let mut vec = std::vec::Vec::new();
        for idx in 0..self.subcommands.len() {
            if self.subcommands[idx].args.args().any(|ar| ar.id == arg.id) {
                vec.push(&self.subcommands[idx]);
                vec.append(&mut self.subcommands[idx].get_subcommands_containing(arg));
            }
        }
        vec
    }
}

/// Deprecated
impl<'help> App<'help> {
    /// Deprecated in [Issue #3087](https://github.com/clap-rs/clap/issues/3087), maybe [`clap::Parser`][crate::Parser] would fit your use case?
    #[cfg(feature = "yaml")]
    #[deprecated(
        since = "3.0.0",
        note = "Deprecated in Issue #3087, maybe clap::Parser would fit your use case?"
    )]
    pub fn from_yaml(y: &'help Yaml) -> Self {
        #![allow(deprecated)]
        let yaml_file_hash = y.as_hash().expect("YAML file must be a hash");
        // We WANT this to panic on error...so expect() is good.
        let (mut a, yaml, err) = if let Some(name) = y["name"].as_str() {
            (App::new(name), yaml_file_hash, "app".into())
        } else {
            let (name_yaml, value_yaml) = yaml_file_hash
                .iter()
                .next()
                .expect("There must be one subcommand in the YAML file");
            let name_str = name_yaml
                .as_str()
                .expect("Subcommand name must be a string");

            (
                App::new(name_str),
                value_yaml.as_hash().expect("Subcommand must be a hash"),
                format!("subcommand '{}'", name_str),
            )
        };

        for (k, v) in yaml {
            a = match k.as_str().expect("App fields must be strings") {
                "version" => yaml_to_str!(a, v, version),
                "long_version" => yaml_to_str!(a, v, long_version),
                "author" => yaml_to_str!(a, v, author),
                "bin_name" => yaml_to_str!(a, v, bin_name),
                "about" => yaml_to_str!(a, v, about),
                "long_about" => yaml_to_str!(a, v, long_about),
                "before_help" => yaml_to_str!(a, v, before_help),
                "after_help" => yaml_to_str!(a, v, after_help),
                "template" => yaml_to_str!(a, v, help_template),
                "usage" => yaml_to_str!(a, v, override_usage),
                "help" => yaml_to_str!(a, v, override_help),
                "help_message" => yaml_to_str!(a, v, help_message),
                "version_message" => yaml_to_str!(a, v, version_message),
                "alias" => yaml_to_str!(a, v, alias),
                "aliases" => yaml_vec_or_str!(a, v, alias),
                "visible_alias" => yaml_to_str!(a, v, visible_alias),
                "visible_aliases" => yaml_vec_or_str!(a, v, visible_alias),
                "display_order" => yaml_to_usize!(a, v, display_order),
                "args" => {
                    if let Some(vec) = v.as_vec() {
                        for arg_yaml in vec {
                            a = a.arg(Arg::from_yaml(arg_yaml));
                        }
                    } else {
                        panic!("Failed to convert YAML value {:?} to a vec", v);
                    }
                    a
                }
                "subcommands" => {
                    if let Some(vec) = v.as_vec() {
                        for sc_yaml in vec {
                            a = a.subcommand(App::from_yaml(sc_yaml));
                        }
                    } else {
                        panic!("Failed to convert YAML value {:?} to a vec", v);
                    }
                    a
                }
                "groups" => {
                    if let Some(vec) = v.as_vec() {
                        for ag_yaml in vec {
                            a = a.group(ArgGroup::from(ag_yaml));
                        }
                    } else {
                        panic!("Failed to convert YAML value {:?} to a vec", v);
                    }
                    a
                }
                "setting" | "settings" => {
                    yaml_to_setting!(a, v, setting, AppSettings, "AppSetting", err)
                }
                "global_setting" | "global_settings" => {
                    yaml_to_setting!(a, v, global_setting, AppSettings, "AppSetting", err)
                }
                _ => a,
            }
        }

        a
    }

    /// Deprecated, replaced with [`App::override_usage`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::override_usage`")]
    #[must_use]
    pub fn usage<S: Into<&'help str>>(self, usage: S) -> Self {
        self.override_usage(usage)
    }

    /// Deprecated, replaced with [`App::override_help`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::override_help`")]
    #[must_use]
    pub fn help<S: Into<&'help str>>(self, help: S) -> Self {
        self.override_help(help)
    }

    /// Deprecated, replaced with [`App::mut_arg`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::mut_arg`")]
    #[must_use]
    pub fn help_short(self, c: char) -> Self {
        self.mut_arg("help", |a| a.short(c))
    }

    /// Deprecated, replaced with [`App::mut_arg`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::mut_arg`")]
    #[must_use]
    pub fn version_short(self, c: char) -> Self {
        self.mut_arg("version", |a| a.short(c))
    }

    /// Deprecated, replaced with [`App::mut_arg`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::mut_arg`")]
    #[must_use]
    pub fn help_message(self, s: impl Into<&'help str>) -> Self {
        self.mut_arg("help", |a| a.help(s.into()))
    }

    /// Deprecated, replaced with [`App::mut_arg`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::mut_arg`")]
    #[must_use]
    pub fn version_message(self, s: impl Into<&'help str>) -> Self {
        self.mut_arg("version", |a| a.help(s.into()))
    }

    /// Deprecated, replaced with [`App::help_template`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::help_template`")]
    #[must_use]
    pub fn template<S: Into<&'help str>>(self, s: S) -> Self {
        self.help_template(s)
    }

    /// Deprecated, replaced with [`App::setting(a| b)`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::setting(a | b)`")]
    #[must_use]
    pub fn settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.settings.insert((*s).into());
        }
        self
    }

    /// Deprecated, replaced with [`App::unset_setting(a| b)`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::unset_setting(a | b)`")]
    #[must_use]
    pub fn unset_settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.settings.remove((*s).into());
        }
        self
    }

    /// Deprecated, replaced with [`App::global_setting(a| b)`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::global_setting(a | b)`")]
    #[must_use]
    pub fn global_settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.settings.insert((*s).into());
            self.g_settings.insert((*s).into());
        }
        self
    }

    /// Deprecated, replaced with [`App::term_width`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::term_width`")]
    #[must_use]
    pub fn set_term_width(self, width: usize) -> Self {
        self.term_width(width)
    }

    /// Deprecated in [Issue #3086](https://github.com/clap-rs/clap/issues/3086), see [`arg!`][crate::arg!].
    #[deprecated(since = "3.0.0", note = "Deprecated in Issue #3086, see `clap::arg!")]
    #[must_use]
    pub fn arg_from_usage(self, usage: &'help str) -> Self {
        #![allow(deprecated)]
        self.arg(Arg::from_usage(usage))
    }

    /// Deprecated in [Issue #3086](https://github.com/clap-rs/clap/issues/3086), see [`arg!`][crate::arg!].
    #[deprecated(since = "3.0.0", note = "Deprecated in Issue #3086, see `clap::arg!")]
    #[must_use]
    pub fn args_from_usage(mut self, usage: &'help str) -> Self {
        #![allow(deprecated)]
        for line in usage.lines() {
            let l = line.trim();
            if l.is_empty() {
                continue;
            }
            self = self.arg(Arg::from_usage(l));
        }
        self
    }

    /// Deprecated, replaced with [`App::render_version`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::render_version`")]
    pub fn write_version<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        write!(w, "{}", self.render_version()).map_err(From::from)
    }

    /// Deprecated, replaced with [`App::render_long_version`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::render_long_version`")]
    pub fn write_long_version<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        write!(w, "{}", self.render_long_version()).map_err(From::from)
    }

    /// Deprecated, replaced with [`App::try_get_matches`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::try_get_matches`")]
    pub fn get_matches_safe(self) -> ClapResult<ArgMatches> {
        self.try_get_matches()
    }

    /// Deprecated, replaced with [`App::try_get_matches_from`]
    #[deprecated(since = "3.0.0", note = "Replaced with `App::try_get_matches_from`")]
    pub fn get_matches_from_safe<I, T>(self, itr: I) -> ClapResult<ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from(itr)
    }

    /// Deprecated, replaced with [`App::try_get_matches_from_mut`]
    #[deprecated(
        since = "3.0.0",
        note = "Replaced with `App::try_get_matches_from_mut`"
    )]
    pub fn get_matches_from_safe_borrow<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from_mut(itr)
    }
}

// Internally used only
impl<'help> App<'help> {
    fn get_used_global_args(&self, matcher: &ArgMatcher) -> Vec<Id> {
        let global_args: Vec<_> = self
            .args
            .args()
            .filter(|a| a.get_global())
            .map(|ga| ga.id.clone())
            .collect();
        if let Some(used_subcommand) = matcher.0.subcommand.as_ref() {
            if let Some(used_subcommand) = self
                .subcommands
                .iter()
                .find(|subcommand| subcommand.id == used_subcommand.id)
            {
                return [global_args, used_subcommand.get_used_global_args(matcher)].concat();
            }
        }
        global_args
    }

    fn _do_parse(&mut self, it: &mut Input) -> ClapResult<ArgMatches> {
        debug!("App::_do_parse");

        // If there are global arguments, or settings we need to propagate them down to subcommands
        // before parsing in case we run into a subcommand
        self._build();

        let mut matcher = ArgMatcher::new(self);

        // do the real parsing
        let mut parser = Parser::new(self);
        if let Err(error) = parser.get_matches_with(&mut matcher, it) {
            if self.is_set(AppSettings::IgnoreErrors) {
                debug!("App::_do_parse: ignoring error: {}", error);
            } else {
                return Err(error);
            }
        }

        let global_arg_vec: Vec<Id> = self.get_used_global_args(&matcher);

        matcher.propagate_globals(&global_arg_vec);

        Ok(matcher.into_inner())
    }

    // used in clap_complete (https://github.com/clap-rs/clap_complete)
    #[doc(hidden)]
    pub fn _build_all(&mut self) {
        self._build();
        for subcmd in self.get_subcommands_mut() {
            subcmd._build();
        }
        self._build_bin_names();
    }

    // used in clap_complete (https://github.com/clap-rs/clap_complete)
    #[doc(hidden)]
    pub fn _build(&mut self) {
        debug!("App::_build");
        if !self.settings.is_set(AppSettings::Built) {
            // Make sure all the globally set flags apply to us as well
            self.settings = self.settings | self.g_settings;

            self._propagate();
            self._check_help_and_version();
            self._propagate_global_args();
            self._derive_display_order();

            let mut pos_counter = 1;
            let self_override = self.is_set(AppSettings::AllArgsOverrideSelf);
            for a in self.args.args_mut() {
                // Fill in the groups
                for g in &a.groups {
                    if let Some(ag) = self.groups.iter_mut().find(|grp| grp.id == *g) {
                        ag.args.push(a.id.clone());
                    } else {
                        let mut ag = ArgGroup::with_id(g.clone());
                        ag.args.push(a.id.clone());
                        self.groups.push(ag);
                    }
                }

                // Figure out implied settings
                if a.is_set(ArgSettings::Last) {
                    // if an arg has `Last` set, we need to imply DontCollapseArgsInUsage so that args
                    // in the usage string don't get confused or left out.
                    self.settings.set(AppSettings::DontCollapseArgsInUsage);
                }
                if self_override {
                    let self_id = a.id.clone();
                    a.overrides.push(self_id);
                }
                a._build();
                if a.is_positional() && a.index.is_none() {
                    a.index = Some(pos_counter);
                    pos_counter += 1;
                }
            }

            self.args._build();

            #[cfg(debug_assertions)]
            self::debug_asserts::assert_app(self);
            self.settings.set(AppSettings::Built);
        } else {
            debug!("App::_build: already built");
        }
    }

    fn _panic_on_missing_help(&self, help_required_globally: bool) {
        if self.is_set(AppSettings::HelpExpected) || help_required_globally {
            let args_missing_help: Vec<String> = self
                .args
                .args()
                .filter(|arg| arg.help.is_none() && arg.long_help.is_none())
                .map(|arg| String::from(arg.name))
                .collect();

            assert!(args_missing_help.is_empty(),
                    "AppSettings::HelpExpected is enabled for the App {}, but at least one of its arguments does not have either `help` or `long_help` set. List of such arguments: {}",
                    self.name,
                    args_missing_help.join(", ")
                );
        }

        for sub_app in &self.subcommands {
            sub_app._panic_on_missing_help(help_required_globally);
        }
    }

    #[cfg(debug_assertions)]
    fn two_args_of<F>(&self, condition: F) -> Option<(&Arg<'help>, &Arg<'help>)>
    where
        F: Fn(&Arg) -> bool,
    {
        two_elements_of(self.args.args().filter(|a: &&Arg| condition(a)))
    }

    // just in case
    #[allow(unused)]
    fn two_groups_of<F>(&self, condition: F) -> Option<(&ArgGroup, &ArgGroup)>
    where
        F: Fn(&ArgGroup) -> bool,
    {
        two_elements_of(self.groups.iter().filter(|a| condition(a)))
    }

    /// Propagate global args
    pub(crate) fn _propagate_global_args(&mut self) {
        debug!("App::_propagate_global_args:{}", self.name);

        for sc in &mut self.subcommands {
            for a in self.args.args().filter(|a| a.get_global()) {
                let mut propagate = false;
                let is_generated = matches!(
                    a.provider,
                    ArgProvider::Generated | ArgProvider::GeneratedMutated
                );

                // Remove generated help and version args in the subcommand
                //
                // Don't remove if those args are further mutated
                if is_generated {
                    let generated_pos = sc
                        .args
                        .args()
                        .position(|x| x.id == a.id && x.provider == ArgProvider::Generated);

                    if let Some(index) = generated_pos {
                        sc.args.remove(index);
                        propagate = true;
                    }
                }

                if propagate || sc.find(&a.id).is_none() {
                    sc.args.push(a.clone());
                }
            }
        }
    }

    /// Propagate settings
    pub(crate) fn _propagate(&mut self) {
        debug!("App::_propagate:{}", self.name);
        let mut subcommands = std::mem::take(&mut self.subcommands);
        for sc in &mut subcommands {
            self._propagate_subcommand(sc);
        }
        self.subcommands = subcommands;
    }

    fn _propagate_subcommand(&self, sc: &mut Self) {
        // We have to create a new scope in order to tell rustc the borrow of `sc` is
        // done and to recursively call this method
        {
            if self.settings.is_set(AppSettings::PropagateVersion) {
                if sc.version.is_none() && self.version.is_some() {
                    sc.version = Some(self.version.unwrap());
                }
                if sc.long_version.is_none() && self.long_version.is_some() {
                    sc.long_version = Some(self.long_version.unwrap());
                }
            }

            sc.settings = sc.settings | self.g_settings;
            sc.g_settings = sc.g_settings | self.g_settings;
            sc.term_w = self.term_w;
            sc.max_w = self.max_w;
        }
    }

    #[allow(clippy::blocks_in_if_conditions)]
    pub(crate) fn _check_help_and_version(&mut self) {
        debug!("App::_check_help_and_version");

        if self.is_set(AppSettings::DisableHelpFlag)
            || self.args.args().any(|x| {
                x.provider == ArgProvider::User
                    && (x.long == Some("help") || x.id == Id::help_hash())
            })
            || self
                .subcommands
                .iter()
                .any(|sc| sc.long_flag == Some("help"))
        {
            debug!("App::_check_help_and_version: Removing generated help");

            let generated_help_pos = self
                .args
                .args()
                .position(|x| x.id == Id::help_hash() && x.provider == ArgProvider::Generated);

            if let Some(index) = generated_help_pos {
                self.args.remove(index);
            }
        } else {
            let other_arg_has_short = self.args.args().any(|x| x.short == Some('h'));
            let help = self
                .args
                .args_mut()
                .find(|x| x.id == Id::help_hash())
                .expect(INTERNAL_ERROR_MSG);

            if !(help.short.is_some()
                || other_arg_has_short
                || self.subcommands.iter().any(|sc| sc.short_flag == Some('h')))
            {
                help.short = Some('h');
            }
        }

        // Determine if we should remove the generated --version flag
        //
        // Note that if only mut_arg() was used, the first expression will evaluate to `true`
        // however inside the condition block, we only check for Generated args, not
        // GeneratedMutated args, so the `mut_arg("version", ..) will be skipped and fall through
        // to the following condition below (Adding the short `-V`)
        if self.settings.is_set(AppSettings::DisableVersionFlag)
            || (self.version.is_none() && self.long_version.is_none())
            || self.args.args().any(|x| {
                x.provider == ArgProvider::User
                    && (x.long == Some("version") || x.id == Id::version_hash())
            })
            || self
                .subcommands
                .iter()
                .any(|sc| sc.long_flag == Some("version"))
        {
            debug!("App::_check_help_and_version: Removing generated version");

            // This is the check mentioned above that only checks for Generated, not
            // GeneratedMuated args by design.
            let generated_version_pos = self
                .args
                .args()
                .position(|x| x.id == Id::version_hash() && x.provider == ArgProvider::Generated);

            if let Some(index) = generated_version_pos {
                self.args.remove(index);
            }
        }

        // If we still have a generated --version flag, determine if we can apply the short `-V`
        if self.args.args().any(|x| {
            x.id == Id::version_hash()
                && matches!(
                    x.provider,
                    ArgProvider::Generated | ArgProvider::GeneratedMutated
                )
        }) {
            let other_arg_has_short = self.args.args().any(|x| x.short == Some('V'));
            let version = self
                .args
                .args_mut()
                .find(|x| x.id == Id::version_hash())
                .expect(INTERNAL_ERROR_MSG);

            if !(version.short.is_some()
                || other_arg_has_short
                || self.subcommands.iter().any(|sc| sc.short_flag == Some('V')))
            {
                version.short = Some('V');
            }
        }

        if !self.is_set(AppSettings::DisableHelpSubcommand)
            && self.has_subcommands()
            && !self.subcommands.iter().any(|s| s.id == Id::help_hash())
        {
            debug!("App::_check_help_and_version: Building help subcommand");
            let mut help_subcmd = App::new("help")
                .about("Print this message or the help of the given subcommand(s)")
                .arg(
                    Arg::new("subcommand")
                        .index(1)
                        .takes_value(true)
                        .multiple_occurrences(true)
                        .value_name("SUBCOMMAND")
                        .help("The subcommand whose help message to display"),
                );
            self._propagate_subcommand(&mut help_subcmd);

            // The parser acts like this is set, so let's set it so we don't falsely
            // advertise it to the user
            help_subcmd.version = None;
            help_subcmd.long_version = None;
            help_subcmd = help_subcmd
                .setting(AppSettings::DisableHelpFlag)
                .unset_global_setting(AppSettings::PropagateVersion);

            self.subcommands.push(help_subcmd);
        }
    }

    pub(crate) fn _derive_display_order(&mut self) {
        debug!("App::_derive_display_order:{}", self.name);

        if self.settings.is_set(AppSettings::DeriveDisplayOrder) {
            for (i, a) in self
                .args
                .args_mut()
                .filter(|a| !a.is_positional())
                .filter(|a| a.provider != ArgProvider::Generated)
                .enumerate()
            {
                a.disp_ord.get_or_insert(i);
            }
            for (i, sc) in &mut self.subcommands.iter_mut().enumerate() {
                sc.disp_ord.get_or_insert(i);
            }
        }
        for sc in &mut self.subcommands {
            sc._derive_display_order();
        }
    }

    // used in clap_complete (https://github.com/clap-rs/clap_complete)
    #[doc(hidden)]
    pub fn _build_bin_names(&mut self) {
        debug!("App::_build_bin_names");

        if !self.is_set(AppSettings::BinNameBuilt) {
            for mut sc in &mut self.subcommands {
                debug!("App::_build_bin_names:iter: bin_name set...");

                if sc.bin_name.is_none() {
                    debug!("No");
                    let bin_name = format!(
                        "{}{}{}",
                        self.bin_name.as_ref().unwrap_or(&self.name.clone()),
                        if self.bin_name.is_some() { " " } else { "" },
                        &*sc.name
                    );
                    debug!(
                        "App::_build_bin_names:iter: Setting bin_name of {} to {}",
                        self.name, bin_name
                    );
                    sc.bin_name = Some(bin_name);
                } else {
                    debug!("yes ({:?})", sc.bin_name);
                }
                debug!(
                    "App::_build_bin_names:iter: Calling build_bin_names from...{}",
                    sc.name
                );
                sc._build_bin_names();
            }
            self.set(AppSettings::BinNameBuilt);
        } else {
            debug!("App::_build_bin_names: already built");
        }
    }

    pub(crate) fn _render_version(&self, use_long: bool) -> String {
        debug!("App::_render_version");

        let ver = if use_long {
            self.long_version
                .unwrap_or_else(|| self.version.unwrap_or(""))
        } else {
            self.version
                .unwrap_or_else(|| self.long_version.unwrap_or(""))
        };
        if let Some(bn) = self.bin_name.as_ref() {
            if bn.contains(' ') {
                // In case we're dealing with subcommands i.e. git mv is translated to git-mv
                format!("{} {}\n", bn.replace(' ', "-"), ver)
            } else {
                format!("{} {}\n", &self.name[..], ver)
            }
        } else {
            format!("{} {}\n", &self.name[..], ver)
        }
    }

    pub(crate) fn format_group(&self, g: &Id) -> String {
        let g_string = self
            .unroll_args_in_group(g)
            .iter()
            .filter_map(|x| self.find(x))
            .map(|x| {
                if x.is_positional() {
                    // Print val_name for positional arguments. e.g. <file_name>
                    x.name_no_brackets().to_string()
                } else {
                    // Print usage string for flags arguments, e.g. <--help>
                    x.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("<{}>", &*g_string)
    }
}

/// A workaround:
/// <https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999>
pub(crate) trait Captures<'a> {}
impl<'a, T> Captures<'a> for T {}

// Internal Query Methods
impl<'help> App<'help> {
    /// Iterate through the *flags* & *options* arguments.
    pub(crate) fn get_non_positionals(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_arguments().filter(|a| !a.is_positional())
    }

    /// Iterate through the *positionals* that don't have custom heading.
    pub(crate) fn get_positionals_with_no_heading(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_positionals()
            .filter(|a| a.get_help_heading().is_none())
    }

    /// Iterate through the *flags* & *options* that don't have custom heading.
    pub(crate) fn get_non_positionals_with_no_heading(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_non_positionals()
            .filter(|a| a.get_help_heading().is_none())
    }

    pub(crate) fn find(&self, arg_id: &Id) -> Option<&Arg<'help>> {
        self.args.args().find(|a| a.id == *arg_id)
    }

    #[inline]
    pub(crate) fn contains_short(&self, s: char) -> bool {
        assert!(
            self.is_set(AppSettings::Built),
            "If App::_build hasn't been called, manually search through Arg shorts"
        );

        self.args.contains(s)
    }

    #[inline]
    pub(crate) fn set(&mut self, s: AppSettings) {
        self.settings.set(s)
    }

    #[inline]
    pub(crate) fn has_args(&self) -> bool {
        !self.args.is_empty()
    }

    pub(crate) fn has_positionals(&self) -> bool {
        self.args.keys().any(|x| x.is_position())
    }

    pub(crate) fn has_visible_subcommands(&self) -> bool {
        self.subcommands
            .iter()
            .any(|sc| sc.name != "help" && !sc.is_set(AppSettings::Hidden))
    }

    /// Check if this subcommand can be referred to as `name`. In other words,
    /// check if `name` is the name of this subcommand or is one of its aliases.
    #[inline]
    pub(crate) fn aliases_to<T>(&self, name: &T) -> bool
    where
        T: PartialEq<str> + ?Sized,
    {
        *name == *self.get_name() || self.get_all_aliases().any(|alias| *name == *alias)
    }

    /// Check if this subcommand can be referred to as `name`. In other words,
    /// check if `name` is the name of this short flag subcommand or is one of its short flag aliases.
    #[inline]
    pub(crate) fn short_flag_aliases_to(&self, flag: char) -> bool {
        Some(flag) == self.short_flag
            || self.get_all_short_flag_aliases().any(|alias| flag == alias)
    }

    /// Check if this subcommand can be referred to as `name`. In other words,
    /// check if `name` is the name of this long flag subcommand or is one of its long flag aliases.
    #[inline]
    pub(crate) fn long_flag_aliases_to<T>(&self, flag: &T) -> bool
    where
        T: PartialEq<str> + ?Sized,
    {
        match self.long_flag {
            Some(long_flag) => {
                flag == long_flag || self.get_all_long_flag_aliases().any(|alias| flag == alias)
            }
            None => self.get_all_long_flag_aliases().any(|alias| flag == alias),
        }
    }

    #[cfg(debug_assertions)]
    pub(crate) fn id_exists(&self, id: &Id) -> bool {
        self.args.args().any(|x| x.id == *id) || self.groups.iter().any(|x| x.id == *id)
    }

    /// Iterate through the groups this arg is member of.
    pub(crate) fn groups_for_arg<'a>(&'a self, arg: &Id) -> impl Iterator<Item = Id> + 'a {
        debug!("App::groups_for_arg: id={:?}", arg);
        let arg = arg.clone();
        self.groups
            .iter()
            .filter(move |grp| grp.args.iter().any(|a| a == &arg))
            .map(|grp| grp.id.clone())
    }

    pub(crate) fn find_group(&self, group_id: &Id) -> Option<&ArgGroup<'help>> {
        self.groups.iter().find(|g| g.id == *group_id)
    }

    /// Iterate through all the names of all subcommands (not recursively), including aliases.
    /// Used for suggestions.
    pub(crate) fn all_subcommand_names(&self) -> impl Iterator<Item = &str> + Captures<'help> {
        self.get_subcommands().flat_map(|sc| {
            let name = sc.get_name();
            let aliases = sc.get_all_aliases();
            std::iter::once(name).chain(aliases)
        })
    }

    pub(crate) fn unroll_args_in_group(&self, group: &Id) -> Vec<Id> {
        debug!("App::unroll_args_in_group: group={:?}", group);
        let mut g_vec = vec![group];
        let mut args = vec![];

        while let Some(g) = g_vec.pop() {
            for n in self
                .groups
                .iter()
                .find(|grp| grp.id == *g)
                .expect(INTERNAL_ERROR_MSG)
                .args
                .iter()
            {
                debug!("App::unroll_args_in_group:iter: entity={:?}", n);
                if !args.contains(n) {
                    if self.find(n).is_some() {
                        debug!("App::unroll_args_in_group:iter: this is an arg");
                        args.push(n.clone())
                    } else {
                        debug!("App::unroll_args_in_group:iter: this is a group");
                        g_vec.push(n);
                    }
                }
            }
        }

        args
    }

    pub(crate) fn unroll_requirements_for_arg(&self, arg: &Id, matcher: &ArgMatcher) -> Vec<Id> {
        let requires_if_or_not = |(val, req_arg): &(Option<&str>, Id)| -> Option<Id> {
            if let Some(v) = val {
                if matcher
                    .get(arg)
                    .map(|ma| ma.contains_val(v))
                    .unwrap_or(false)
                {
                    Some(req_arg.clone())
                } else {
                    None
                }
            } else {
                Some(req_arg.clone())
            }
        };

        let mut processed = vec![];
        let mut r_vec = vec![arg];
        let mut args = vec![];

        while let Some(a) = r_vec.pop() {
            if processed.contains(&a) {
                continue;
            }

            processed.push(a);

            if let Some(arg) = self.find(a) {
                for r in arg.requires.iter().filter_map(requires_if_or_not) {
                    if let Some(req) = self.find(&r) {
                        if !req.requires.is_empty() {
                            r_vec.push(&req.id)
                        }
                    }
                    args.push(r);
                }
            }
        }

        args
    }

    /// Find a flag subcommand name by short flag or an alias
    pub(crate) fn find_short_subcmd(&self, c: char) -> Option<&str> {
        self.get_subcommands()
            .find(|sc| sc.short_flag_aliases_to(c))
            .map(|sc| sc.get_name())
    }

    /// Find a flag subcommand name by long flag or an alias
    pub(crate) fn find_long_subcmd(&self, long: &RawOsStr) -> Option<&str> {
        self.get_subcommands()
            .find(|sc| sc.long_flag_aliases_to(long))
            .map(|sc| sc.get_name())
    }

    pub(crate) fn get_display_order(&self) -> usize {
        self.disp_ord.unwrap_or(999)
    }
}

impl<'help> Index<&'_ Id> for App<'help> {
    type Output = Arg<'help>;

    fn index(&self, key: &Id) -> &Self::Output {
        self.find(key).expect(INTERNAL_ERROR_MSG)
    }
}

impl fmt::Display for App<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn two_elements_of<I, T>(mut iter: I) -> Option<(T, T)>
where
    I: Iterator<Item = T>,
{
    let first = iter.next();
    let second = iter.next();

    match (first, second) {
        (Some(first), Some(second)) => Some((first, second)),
        _ => None,
    }
}
