mod settings;
pub mod parser;
mod help;
mod validator;
mod usage;

// Std
use std::env;
use std::ffi::OsString;
use std::fmt;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::fs::File;
use std::iter::Peekable;

// Third Party
#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

// Internal
use app::parser::Parser;
use app::help::Help;
use args::{Arg, ArgGroup, ArgMatcher, ArgMatches};
use args::settings::ArgSettings;
use errors::Result as ClapResult;
pub use self::settings::{AppFlags, AppSettings};
use completions::{ComplGen, Shell};
use fmt::ColorWhen;

#[doc(hidden)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Propagation<'a> {
    To(&'a str),
    Full,
    NextLevel,
    None
}

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
///         Arg::with_name("in_file").index(1)
///     )
///     .after_help("Longer explanation to appear after the options when \
///                  displaying the help information from --help or -h")
///     .get_matches();
///
/// // Your program logic starts here...
/// ```
/// [`App::get_matches`]: ./struct.App.html#method.get_matches
#[derive(Default, Debug, Clone)]
pub struct App<'a, 'b>
where
    'a: 'b,
{
    #[doc(hidden)]
    pub name: String,
    #[doc(hidden)]
    pub bin_name: Option<String>,
    #[doc(hidden)]
    pub author: Option<&'b str>,
    #[doc(hidden)]
    pub version: Option<&'b str>,
    #[doc(hidden)]
    pub long_version: Option<&'b str>,
    #[doc(hidden)]
    pub about: Option<&'b str>,
    #[doc(hidden)]
    pub long_about: Option<&'b str>,
    #[doc(hidden)]
    pub more_help: Option<&'b str>,
    #[doc(hidden)]
    pub pre_help: Option<&'b str>,
    #[doc(hidden)]
    pub aliases: Option<Vec<(&'b str, bool)>>, // (name, visible)
    #[doc(hidden)]
    pub usage_str: Option<&'b str>,
    #[doc(hidden)]
    pub usage: Option<String>,
    #[doc(hidden)]
    pub help_str: Option<&'b str>,
    #[doc(hidden)]
    pub disp_ord: usize,
    #[doc(hidden)]
    pub term_w: Option<usize>,
    #[doc(hidden)]
    pub max_w: Option<usize>,
    #[doc(hidden)]
    pub template: Option<&'b str>,
    #[doc(hidden)]
    pub settings: AppFlags,
    #[doc(hidden)]
    pub g_settings: AppFlags,
    #[doc(hidden)]
    pub args: Vec<Arg<'a, 'b>>,
    #[doc(hidden)]
    pub subcommands: Vec<App<'a, 'b>>,
    #[doc(hidden)]
    pub groups: Vec<ArgGroup<'a>>,
    #[doc(hidden)]
    help_short: Option<char>,
    #[doc(hidden)]
    version_short: Option<char>,
    #[doc(hidden)]
    pub help_message: Option<&'a str>,
    #[doc(hidden)]
    pub version_message: Option<&'a str>,
}

impl<'a, 'b> App<'a, 'b> {
    /// Creates a new instance of an application requiring a name. The name may be, but doesn't
    /// have to be same as the binary. The name will be displayed to the user when they request to
    /// print version or help and usage information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let prog = App::new("My Program")
    /// # ;
    /// ```
    pub fn new<S: Into<String>>(n: S) -> Self {
        App {
            name: n.into(),
            ..Default::default()
        }
    }

    /// Get the name of the app
    pub fn get_name(&self) -> &str { &self.name }

    /// Get the name of the binary
    pub fn get_bin_name(&self) -> Option<&str> { self.bin_name.as_ref().map(|s| s.as_str()) }

    /// Sets a string of author(s) that will be displayed to the user when they
    /// request the help information with `--help` or `-h`.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_authors!`] to automatically set your
    /// application's author(s) to the same thing as your crate at compile time. See the [`examples/`]
    /// directory for more information
    ///
    /// See the [`examples/`]
    /// directory for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///      .author("Me, me@mymain.com")
    /// # ;
    /// ```
    /// [`crate_authors!`]: ./macro.crate_authors!.html
    /// [`examples/`]: https://github.com/kbknapp/clap-rs/tree/master/examples
    pub fn author<S: Into<&'b str>>(mut self, author: S) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Overrides the system-determined binary name. This should only be used when absolutely
    /// necessary, such as when the binary name for your application is misleading, or perhaps
    /// *not* how the user should invoke your program.
    ///
    /// **Pro-tip:** When building things such as third party `cargo` subcommands, this setting
    /// **should** be used!
    ///
    /// **NOTE:** This command **should not** be used for [`SubCommand`]s.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("My Program")
    ///      .bin_name("my_binary")
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    pub fn bin_name<S: Into<String>>(mut self, name: S) -> Self {
        self.bin_name = Some(name.into());
        self
    }

    /// Sets a string describing what the program does. This will be displayed when displaying help
    /// information with `-h`.
    ///
    /// **NOTE:** If only `about` is provided, and not [`App::long_about`] but the user requests
    /// `--help` clap will still display the contents of `about` appropriately
    ///
    /// **NOTE:** Only [`App::about`] is used in completion script generation in order to be
    /// concise
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .about("Does really amazing things to great people")
    /// # ;
    /// ```
    /// [`App::long_about`]: ./struct.App.html#method.long_about
    pub fn about<S: Into<&'b str>>(mut self, about: S) -> Self {
        self.about = Some(about.into());
        self
    }

    /// Sets a string describing what the program does. This will be displayed when displaying help
    /// information.
    ///
    /// **NOTE:** If only `long_about` is provided, and not [`App::about`] but the user requests
    /// `-h` clap will still display the contents of `long_about` appropriately
    ///
    /// **NOTE:** Only [`App::about`] is used in completion script generation in order to be
    /// concise
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .long_about(
    /// "Does really amazing things to great people. Now let's talk a little
    ///  more in depth about how this subcommand really works. It may take about
    ///  a few lines of text, but that's ok!")
    /// # ;
    /// ```
    /// [`App::about`]: ./struct.App.html#method.about
    pub fn long_about<S: Into<&'b str>>(mut self, about: S) -> Self {
        self.long_about = Some(about.into());
        self
    }

    /// Sets the program's name. This will be displayed when displaying help information.
    ///
    /// **Pro-top:** This function is particularly useful when configuring a program via
    /// [`App::from_yaml`] in conjunction with the [`crate_name!`] macro to derive the program's
    /// name from its `Cargo.toml`.
    ///
    /// # Examples
    /// ```ignore
    /// # #[macro_use]
    /// # extern crate clap;
    /// # use clap::App;
    /// # fn main() {
    /// let yml = load_yaml!("app.yml");
    /// let app = App::from_yaml(yml)
    ///     .name(crate_name!());
    ///
    /// // continued logic goes here, such as `app.get_matches()` etc.
    /// # }
    /// ```
    ///
    /// [`App::from_yaml`]: ./struct.App.html#method.from_yaml
    /// [`crate_name!`]: ./macro.crate_name.html
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Adds additional help information to be displayed in addition to auto-generated help. This
    /// information is displayed **after** the auto-generated help information. This is often used
    /// to describe how to use the arguments, or caveats to be noted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .after_help("Does really amazing things to great people...but be careful with -R")
    /// # ;
    /// ```
    pub fn after_help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.more_help = Some(help.into());
        self
    }

    /// Adds additional help information to be displayed in addition to auto-generated help. This
    /// information is displayed **before** the auto-generated help information. This is often used
    /// for header information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .before_help("Some info I'd like to appear before the help info")
    /// # ;
    /// ```
    pub fn before_help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.pre_help = Some(help.into());
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information with `-V`.
    ///
    /// **NOTE:** If only `version` is provided, and not [`App::long_version`] but the user
    /// requests `--version` clap will still display the contents of `version` appropriately
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to automatically set your
    /// application's version to the same thing as your crate at compile time. See the [`examples/`]
    /// directory for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .version("v0.1.24")
    /// # ;
    /// ```
    /// [`crate_version!`]: ./macro.crate_version!.html
    /// [`examples/`]: https://github.com/kbknapp/clap-rs/tree/master/examples
    /// [`App::long_version`]: ./struct.App.html#method.long_version
    pub fn version<S: Into<&'b str>>(mut self, ver: S) -> Self {
        self.version = Some(ver.into());
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information with `--version`.
    ///
    /// **NOTE:** If only `long_version` is provided, and not [`App::version`] but the user
    /// requests `-V` clap will still display the contents of `long_version` appropriately
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to automatically set your
    /// application's version to the same thing as your crate at compile time. See the [`examples/`]
    /// directory for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
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
    /// [`examples/`]: https://github.com/kbknapp/clap-rs/tree/master/examples
    /// [`App::version`]: ./struct.App.html#method.version
    pub fn long_version<S: Into<&'b str>>(mut self, ver: S) -> Self {
        self.long_version = Some(ver.into());
        self
    }

    /// Overrides the `clap` generated usage string.
    ///
    /// This will be displayed to the user when errors are found in argument parsing.
    ///
    /// **CAUTION:** Using this setting disables `clap`s "context-aware" usage strings. After this
    /// setting is set, this will be the only usage string displayed to the user!
    ///
    /// **NOTE:** This will not replace the entire help message, *only* the portion
    /// showing the usage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .override_usage("myapp [-clDas] <some_file>")
    /// # ;
    /// ```
    /// [`ArgMatches::usage`]: ./struct.ArgMatches.html#method.usage
    pub fn override_usage<S: Into<&'b str>>(mut self, usage: S) -> Self {
        self.usage_str = Some(usage.into());
        self
    }

    /// Overrides the `clap` generated help message. This should only be used
    /// when the auto-generated message does not suffice.
    ///
    /// This will be displayed to the user when they use `--help` or `-h`
    ///
    /// **NOTE:** This replaces the **entire** help message, so nothing will be auto-generated.
    ///
    /// **NOTE:** This **only** replaces the help message for the current command, meaning if you
    /// are using subcommands, those help messages will still be auto-generated unless you
    /// specify a [`Arg::override_help`] for them as well.
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
    ///            USAGE: myapp <opts> <comamnd>\n\n\
    ///
    ///            Options:\n\
    ///            -h, --helpe      Dispay this message\n\
    ///            -V, --version    Display version info\n\
    ///            -s <stuff>       Do something with stuff\n\
    ///            -v               Be verbose\n\n\
    ///
    ///            Commmands:\n\
    ///            help             Prints this message\n\
    ///            work             Do some work")
    /// # ;
    /// ```
    /// [`Arg::override_help`]: ./struct.Arg.html#method.override_help
    pub fn override_help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.help_str = Some(help.into());
        self
    }

    /// Sets the help template to be used, overriding the default format.
    ///
    /// Tags arg given inside curly brackets.
    ///
    /// Valid tags are:
    ///
    ///   * `{bin}`         - Binary name.
    ///   * `{version}`     - Version number.
    ///   * `{author}`      - Author information.
    ///   * `{about}`       - General description (from [`App::about`])
    ///   * `{usage}`       - Automatically generated or given usage string.
    ///   * `{all-args}`    - Help for all arguments (options, flags, positionals arguments,
    ///                       and subcommands) including titles.
    ///   * `{unified}`     - Unified help for options and flags. Note, you must *also* set
    ///                       [`AppSettings::UnifiedHelpMessage`] to fully merge both options and
    ///                       flags, otherwise the ordering is "best effort"
    ///   * `{flags}`       - Help for flags.
    ///   * `{options}`     - Help for options.
    ///   * `{positionals}` - Help for positionals arguments.
    ///   * `{subcommands}` - Help for subcommands.
    ///   * `{after-help}`  - Help from [`App::after_help`]
    ///   * `{before-help}`  - Help from [`App::before_help`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .version("1.0")
    ///     .help_template("{bin} ({version}) - {usage}")
    /// # ;
    /// ```
    /// **NOTE:**The template system is, on purpose, very simple. Therefore the tags have to writen
    /// in the lowercase and without spacing.
    /// [`App::about`]: ./struct.App.html#method.about
    /// [`App::after_help`]: ./struct.App.html#method.after_help
    /// [`App::before_help`]: ./struct.App.html#method.before_help
    /// [`AppSettings::UnifiedHelpMessage`]: ./enum.AppSettings.html#variant.UnifiedHelpMessage
    pub fn help_template<S: Into<&'b str>>(mut self, s: S) -> Self {
        self.template = Some(s.into());
        self
    }

    /// Enables a single command, or [`SubCommand`], level settings.
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
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`AppSettings`]: ./enum.AppSettings.html
    pub fn setting(mut self, setting: AppSettings) -> Self {
        self.settings.set(setting);
        self
    }

    /// Disables a single command, or [`SubCommand`], level setting.
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
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`AppSettings`]: ./enum.AppSettings.html
    /// [global]: ./struct.App.html#method.global_setting
    pub fn unset_setting(mut self, setting: AppSettings) -> Self {
        self.settings.unset(setting);
        self
    }

    /// Enables a single setting that is propagated down through all child subcommands.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// **NOTE**: The setting is *only* propagated *down* and not up through parent commands.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .global_setting(AppSettings::SubcommandRequired)
    /// # ;
    /// ```
    /// [`AppSettings`]: ./enum.AppSettings.html
    pub fn global_setting(mut self, setting: AppSettings) -> Self {
        self.settings.set(setting);
        self.g_settings.set(setting);
        self
    }

    /// Disables a global setting, and stops propagating down to child subcommands.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// **NOTE:** The setting being unset will be unset from both local and [global] settings
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .unset_global_setting(AppSettings::ColorAuto)
    /// # ;
    /// ```
    /// [`AppSettings`]: ./enum.AppSettings.html
    /// [global]: ./struct.App.html#method.global_setting
    pub fn unset_global_setting(mut self, setting: AppSettings) -> Self {
        self.settings.unset(setting);
        self.g_settings.unset(setting);
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
        self.term_w = Some(width);
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
        self.max_w = Some(w);
        self
    }

    /// Adds an [argument] to the list of valid possibilities.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     // Adding a single "flag" argument with a short and help text, using Arg::with_name()
    ///     .arg(
    ///         Arg::with_name("debug")
    ///            .short("d")
    ///            .help("turns on debugging mode")
    ///     )
    ///     // Adding a single "option" argument with a short, a long, and help text using the less
    ///     // verbose Arg::from_usage()
    ///     .arg(
    ///         Arg::from_usage("-c --config=[CONFIG] 'Optionally sets a config file to use'")
    ///     )
    /// # ;
    /// ```
    /// [argument]: ./struct.Arg.html
    pub fn arg<A: Into<Arg<'a, 'b>>>(mut self, a: A) -> Self {
        self.args.push(a.into());
        self
    }

    /// Adds multiple [arguments] to the list of valid possibilities
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args(&[
    ///         Arg::from_usage("[debug] -d 'turns on debugging info'"),
    ///         Arg::with_name("input").index(1).help("the input file to use")
    ///     ])
    /// # ;
    /// ```
    /// [arguments]: ./struct.Arg.html
    pub fn args<I, T>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Arg<'a, 'b>>,
    {
        // @TODO @perf @p4 @v3-beta: maybe extend_from_slice would be possible and perform better?
        // But that may also not let us do `&["-a 'some'", "-b 'other']` because of not Into<Arg>
        for arg in args.into_iter() {
            self.args.push(arg.into());
        }
        self
    }

    /// Allows adding a [`SubCommand`] alias, which function as "hidden" subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// let m = App::new("myprog")
    ///             .subcommand(SubCommand::with_name("test")
    ///                 .alias("do-stuff"))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    pub fn alias<S: Into<&'b str>>(mut self, name: S) -> Self {
        if let Some(ref mut als) = self.aliases {
            als.push((name.into(), false));
        } else {
            self.aliases = Some(vec![(name.into(), false)]);
        }
        self
    }

    /// Allows adding [`SubCommand`] aliases, which function as "hidden" subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand};
    /// let m = App::new("myprog")
    ///             .subcommand(SubCommand::with_name("test")
    ///                 .aliases(&["do-stuff", "do-tests", "tests"]))
    ///                 .arg(Arg::with_name("input")
    ///                             .help("the file to add")
    ///                             .index(1)
    ///                             .required(false))
    ///             .get_matches_from(vec!["myprog", "do-tests"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    pub fn aliases(mut self, names: &[&'b str]) -> Self {
        if let Some(ref mut als) = self.aliases {
            for n in names {
                als.push((n, false));
            }
        } else {
            self.aliases = Some(names.iter().map(|n| (*n, false)).collect::<Vec<_>>());
        }
        self
    }

    /// Allows adding a [`SubCommand`] alias that functions exactly like those defined with
    /// [`App::alias`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// let m = App::new("myprog")
    ///             .subcommand(SubCommand::with_name("test")
    ///                 .visible_alias("do-stuff"))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`App::alias`]: ./struct.App.html#method.alias
    pub fn visible_alias<S: Into<&'b str>>(mut self, name: S) -> Self {
        if let Some(ref mut als) = self.aliases {
            als.push((name.into(), true));
        } else {
            self.aliases = Some(vec![(name.into(), true)]);
        }
        self
    }

    /// Allows adding multiple [`SubCommand`] aliases that functions exactly like those defined
    /// with [`App::aliases`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// let m = App::new("myprog")
    ///             .subcommand(SubCommand::with_name("test")
    ///                 .visible_aliases(&["do-stuff", "tests"]))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`App::aliases`]: ./struct.App.html#method.aliases
    pub fn visible_aliases(mut self, names: &[&'b str]) -> Self {
        if let Some(ref mut als) = self.aliases {
            for n in names {
                als.push((n, true));
            }
        } else {
            self.aliases = Some(names.iter().map(|n| (*n, true)).collect::<Vec<_>>());
        }
        self
    }

    /// Adds an [`ArgGroup`] to the application. [`ArgGroup`]s are a family of related arguments.
    /// By placing them in a logical group, you can build easier requirement and exclusion rules.
    /// For instance, you can make an entire [`ArgGroup`] required, meaning that one (and *only*
    /// one) argument from that group must be present at runtime.
    ///
    /// You can also do things such as name an [`ArgGroup`] as a conflict to another argument.
    /// Meaning any of the arguments that belong to that group will cause a failure if present with
    /// the conflicting argument.
    ///
    /// Another added benefit of [`ArgGroup`]s is that you can extract a value from a group instead
    /// of determining exactly which argument was used.
    ///
    /// Finally, using [`ArgGroup`]s to ensure exclusion between arguments is another very common
    /// use
    ///
    /// # Examples
    ///
    /// The following example demonstrates using an [`ArgGroup`] to ensure that one, and only one,
    /// of the arguments from the specified group is present at runtime.
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// App::new("app")
    ///     .args_from_usage(
    ///         "--set-ver [ver] 'set the version manually'
    ///          --major         'auto increase major'
    ///          --minor         'auto increase minor'
    ///          --patch         'auto increase patch'")
    ///     .group(ArgGroup::with_name("vers")
    ///          .args(&["set-ver", "major", "minor","patch"])
    ///          .required(true))
    /// # ;
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    pub fn group(mut self, group: ArgGroup<'a>) -> Self {
        self.groups.push(group);
        self
    }

    /// Adds multiple [`ArgGroup`]s to the [`App`] at once.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// App::new("app")
    ///     .args_from_usage(
    ///         "--set-ver [ver] 'set the version manually'
    ///          --major         'auto increase major'
    ///          --minor         'auto increase minor'
    ///          --patch         'auto increase patch'
    ///          -c [FILE]       'a config file'
    ///          -i [IFACE]      'an interface'")
    ///     .groups(&[
    ///         ArgGroup::with_name("vers")
    ///             .args(&["set-ver", "major", "minor","patch"])
    ///             .required(true),
    ///         ArgGroup::with_name("input")
    ///             .args(&["c", "i"])
    ///     ])
    /// # ;
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    /// [`App`]: ./struct.App.html
    pub fn groups(mut self, groups: &[ArgGroup<'a>]) -> Self {
        for g in groups {
            self = self.group(g.into());
        }
        self
    }

    /// Adds a [`SubCommand`] to the list of valid possibilities. Subcommands are effectively
    /// sub-[`App`]s, because they can contain their own arguments, subcommands, version, usage,
    /// etc. They also function just like [`App`]s, in that they get their own auto generated help,
    /// version, and usage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .subcommand(SubCommand::with_name("config")
    ///         .about("Controls configuration features")
    ///         .arg_from_usage("<config> 'Required configuration file to use'"))
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`App`]: ./struct.App.html
    pub fn subcommand(mut self, subcmd: App<'a, 'b>) -> Self {
        self.subcommands.push(subcmd);
        self
    }

    /// Adds multiple subcommands to the list of valid possibilities by iterating over an
    /// [`IntoIterator`] of [`SubCommand`]s
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand};
    /// # App::new("myprog")
    /// .subcommands( vec![
    ///        SubCommand::with_name("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::with_name("config_file").index(1)),
    ///        SubCommand::with_name("debug").about("Controls debug functionality")])
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
    pub fn subcommands<I>(mut self, subcmds: I) -> Self
    where
        I: IntoIterator<Item = App<'a, 'b>>,
    {
        for subcmd in subcmds {
            self.subcommands.push(subcmd);
        }
        self
    }

    /// Allows custom ordering of [`SubCommand`]s within the help message. Subcommands with a lower
    /// value will be displayed first in the help message. This is helpful when one would like to
    /// emphasise frequently used subcommands, or prioritize those towards the top of the list.
    /// Duplicate values **are** allowed. Subcommands with duplicate display orders will be
    /// displayed in alphabetical order.
    ///
    /// **NOTE:** The default is 999 for all subcommands.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, SubCommand};
    /// let m = App::new("cust-ord")
    ///     .subcommand(SubCommand::with_name("alpha") // typically subcommands are grouped
    ///                                                // alphabetically by name. Subcommands
    ///                                                // without a display_order have a value of
    ///                                                // 999 and are displayed alphabetically with
    ///                                                // all other 999 subcommands
    ///         .about("Some help and text"))
    ///     .subcommand(SubCommand::with_name("beta")
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
    ///     cust-ord [FLAGS] [OPTIONS]
    ///
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// SUBCOMMANDS:
    ///     beta    I should be first!
    ///     alpha   Some help and text
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    pub fn display_order(mut self, ord: usize) -> Self {
        self.disp_ord = ord;
        self
    }

    /// Allows one to mutate an [`Arg`] after it's been added to an `App`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    ///
    /// let mut app = App::new("foo")
    ///     .arg(Arg::with_name("bar")
    ///         .short("b"))
    ///     .mut_arg("bar", |a| a.short("B"));
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
    /// [`Arg`]: ./struct.Arg.html
    pub fn mut_arg<F>(mut self, arg: &'a str, f: F) -> Self
    where F: FnOnce(Arg<'a, 'b>) -> Arg<'a, 'b> {
        let i = self.args.iter().enumerate().filter(|&(i, a)| a.name == arg).map(|(i, _)| i).next();
        let a = if let Some(idx) = i {
            let mut a = self.args.swap_remove(idx);
            f(a)
        } else {
            let mut a = Arg::with_name(arg);
            f(a)
        };
        self.args.push(a);
        self
    }

    /// Prints the full help message to [`io::stdout()`] using a [`BufWriter`] using the same
    /// method as if someone ran `-h` to request the help message
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// let mut app = App::new("myprog");
    /// app.print_help();
    /// ```
    /// [`io::stdout()`]: https://doc.rust-lang.org/std/io/fn.stdout.html
    /// [`BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html
    /// [`-h` (short)]: ./struct.Arg.html#method.help
    /// [`--help` (long)]: ./struct.Arg.html#method.long_help
    pub fn print_help(&mut self) -> ClapResult<()> {
        // If there are global arguments, or settings we need to propagate them down to subcommands
        // before parsing incase we run into a subcommand
        self._build(Propagation::NextLevel);

        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        self.write_help(&mut buf_w)
    }

    /// Prints the full help message to [`io::stdout()`] using a [`BufWriter`] using the same
    /// method as if someone ran `--help` to request the help message
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// let mut app = App::new("myprog");
    /// app.print_long_help();
    /// ```
    /// [`io::stdout()`]: https://doc.rust-lang.org/std/io/fn.stdout.html
    /// [`BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html
    /// [`-h` (short)]: ./struct.Arg.html#method.help
    /// [`--help` (long)]: ./struct.Arg.html#method.long_help
    pub fn print_long_help(&mut self) -> ClapResult<()> {
        // If there are global arguments, or settings we need to propagate them down to subcommands
        // before parsing incase we run into a subcommand
        self._build(Propagation::NextLevel);

        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        self.write_long_help(&mut buf_w)
    }

    /// Writes the full help message to the user to a [`io::Write`] object in the same method as if
    /// the user ran `-h`
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)]
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
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`-h` (short)]: ./struct.Arg.html#method.help
    /// [`--help` (long)]: ./struct.Arg.html#method.long_help
    pub fn write_help<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        self._build(Propagation::NextLevel);

        let p = Parser::new(self);
        Help::write_parser_help(w, &p, false)
    }

    /// Writes the full help message to the user to a [`io::Write`] object in the same method as if
    /// the user ran `--help`
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)]
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
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`-h` (short)]: ./struct.Arg.html#method.help
    /// [`--help` (long)]: ./struct.Arg.html#method.long_help
    pub fn write_long_help<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        self._build(Propagation::NextLevel);

        let p = Parser::new(self);
        Help::write_parser_help(w, &p, true)
    }

    /// Writes the version message to the user to a [`io::Write`] object as if the user ran `-V`.
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" version messages
    /// depending on if the user ran [`-V` (short)] or [`--version` (long)]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_version(&mut out).expect("failed to write to stdout");
    /// ```
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`-V` (short)]: ./struct.App.html#method.version
    /// [`--version` (long)]: ./struct.App.html#method.long_version
    pub fn write_version<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        self._write_version(w, false).map_err(From::from)
    }

    /// Writes the version message to the user to a [`io::Write`] object
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" version messages
    /// depending on if the user ran [`-V` (short)] or [`--version` (long)]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::App;
    /// use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_long_version(&mut out).expect("failed to write to stdout");
    /// ```
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`-V` (short)]: ./struct.App.html#method.version
    /// [`--version` (long)]: ./struct.App.html#method.long_version
    pub fn write_long_version<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        self._write_version(w, true).map_err(From::from)
    }

    /// Starts the parsing process, upon a failed parse an error will be displayed to the user and
    /// the process will exit with the appropriate error code. By default this method gets all user
    /// provided arguments from [`env::args_os`] in order to allow for invalid UTF-8 code points,
    /// which are legal on many platforms.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches();
    /// ```
    /// [`env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
    pub fn get_matches(self) -> ArgMatches<'a> { self.get_matches_from(&mut env::args_os()) }

    /// Starts the parsing process, just like [`App::get_matches`] but doesn't consume the `App`
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
    /// [`env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
    /// [`App::get_matches`]: ./struct.App.html#method.get_matches
    pub fn get_matches_mut(&mut self) -> ArgMatches<'a> {
        self.try_get_matches_from_mut(&mut env::args_os()).unwrap_or_else(|e| {
            // Otherwise, write to stderr and exit
            if e.use_stderr() {
                wlnerr!("{}", e.message);
                if self.settings.is_set(AppSettings::WaitOnError) {
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
        })
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
    pub fn try_get_matches(self) -> ClapResult<ArgMatches<'a>> {
        // Start the parsing
        self.try_get_matches_from(&mut env::args_os())
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
    pub fn get_matches_from<I, T>(mut self, itr: I) -> ArgMatches<'a>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from_mut(itr).unwrap_or_else(|e| {
            // Otherwise, write to stderr and exit
            if e.use_stderr() {
                wlnerr!("{}", e.message);
                if self.settings.is_set(AppSettings::WaitOnError) {
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
        })
    }

    /// Starts the parsing process. A combination of [`App::get_matches_from`], and
    /// [`App::try_get_matches`]
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
    /// [`App::try_get_matches`]: ./struct.App.html#method.get_matches_safe
    /// [`ErrorKind::HelpDisplayed`]: ./enum.ErrorKind.html#variant.HelpDisplayed
    /// [`ErrorKind::VersionDisplayed`]: ./enum.ErrorKind.html#variant.VersionDisplayed
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`std::process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
    /// [`clap::Error`]: ./struct.Error.html
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`kind`]: ./struct.Error.html
    /// [`AppSettings::NoBinaryName`]: ./enum.AppSettings.html#variant.NoBinaryName
    pub fn try_get_matches_from<I, T>(mut self, itr: I) -> ClapResult<ArgMatches<'a>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from_mut(itr)
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
    pub fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches<'a>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut it = itr.into_iter();
        // Get the name of the program (argument 1 of env::args()) and determine the
        // actual file
        // that was used to execute the program. This is because a program called
        // ./target/release/my_prog -a
        // will have two arguments, './target/release/my_prog', '-a' but we don't want
        // to display
        // the full path when displaying help messages and such
        if !self.settings.is_set(AppSettings::NoBinaryName) {
            if let Some(name) = it.next() {
                let bn_os = name.into();
                let p = Path::new(&*bn_os);
                if let Some(f) = p.file_name() {
                    if let Some(s) = f.to_os_string().to_str() {
                        if self.bin_name.is_none() {
                            self.bin_name = Some(s.to_owned());
                        }
                    }
                }
            }
        }

        self._do_parse(&mut it.peekable())
    }
}

// Internally used only
#[doc(hidden)]
impl<'a, 'b> App<'a, 'b> {
    #[doc(hidden)]
    fn _do_parse<I, T>(&mut self, it: &mut Peekable<I>) -> ClapResult<ArgMatches<'a>>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        debugln!("App::_do_parse;");
        let mut matcher = ArgMatcher::new();

        // If there are global arguments, or settings we need to propgate them down to subcommands
        // before parsing incase we run into a subcommand
        if !self.settings.is_set(AppSettings::Propagated) {
            self._build(Propagation::NextLevel);
        }

        {
            let mut parser = Parser::new(self);

            // do the real parsing
            if let Err(e) = parser.get_matches_with(&mut matcher, it) {
                return Err(e);
            }
        }

        let global_arg_vec: Vec<&str> = (&self)
            .args
            .iter()
            .filter(|a| a.is_set(ArgSettings::Global))
            .map(|ga| ga.name)
            .collect();
        matcher.propagate_globals(&global_arg_vec);

        Ok(matcher.into())
    }

    fn _build(&mut self, prop: Propagation) {
        debugln!("App::_build;");
        // Make sure all the globally set flags apply to us as well
        self.settings = self.settings | self.g_settings;

        // Depending on if DeriveDisplayOrder is set or not, we need to determine when we build
        // the help and version flags, otherwise help message orders get screwed up
        if self.settings.is_set(AppSettings::DeriveDisplayOrder) {
            self._derive_display_order();
            self._create_help_and_version();
            self._propagate(prop);
        } else {
            self._create_help_and_version();
            self._propagate(prop);
            self._derive_display_order();
        }
        // Perform expensive debug assertions
        debug_assert!({
            for a in &self.args {
                self._arg_debug_asserts(a);
            }
            true
        });
        for a in &mut self.args {
            // Fill in the groups
            if let Some(ref grps) = a.groups {
                for g in grps {
                    let mut found = false;
                    if let Some(ref mut ag) = groups_mut!(self).find(|grp| &grp.name == g) {
                        ag.args.push(a.name);
                        found = true;
                    }
                    if !found {
                        let mut ag = ArgGroup::with_name(g);
                        ag.args.push(a.name);
                        self.groups.push(ag);
                    }
                }
            }

            // Figure out implied settings
            if a.is_set(ArgSettings::Last) {
                // if an arg has `Last` set, we need to imply DontCollapseArgsInUsage so that args
                // in the usage string don't get confused or left out.
                self.settings.set(AppSettings::DontCollapseArgsInUsage);
                self.settings.set(AppSettings::ContainsLast);
            }
            a._build();
        }

        debug_assert!(self._app_debug_asserts());
        self.settings.set(AppSettings::Propagated);
    }

    // Perform some expensive assertions on the Parser itself
    fn _app_debug_asserts(&mut self) -> bool {
        debugln!("App::app_debug_asserts;");
        // * Args listed inside groups should exist
        // * Groups should not have naming conflicts with Args
        let g = groups!(self).find(|g| {
            g.args
                .iter()
                .any(|arg| !(find!(self, arg).is_some() || groups!(self).any(|g| &g.name == arg)))
        });
        assert!(
            g.is_none(),
            "The group '{}' contains an arg that doesn't exist or has a naming conflict with a group.",
            g.unwrap().name
        );
        true
    }

    // @TODO @v3-alpha @perf: should only propagate globals to subcmd we find, or for help
    pub fn _propagate(&mut self, prop: Propagation) {
        debugln!("App::_propagate:{}", self.name);
        for sc in &mut self.subcommands {
            // We have to create a new scope in order to tell rustc the borrow of `sc` is
            // done and to recursively call this method
            {
                let vsc = self.settings.is_set(AppSettings::VersionlessSubcommands);
                let gv = self.settings.is_set(AppSettings::GlobalVersion);

                if vsc {
                    sc.set(AppSettings::DisableVersion);
                }
                if gv && sc.version.is_none() && self.version.is_some() {
                    sc.set(AppSettings::GlobalVersion);
                    sc.version = Some(self.version.unwrap());
                }
                sc.settings = sc.settings | self.g_settings;
                sc.g_settings = sc.g_settings | self.g_settings;
                sc.term_w = self.term_w;
                sc.max_w = self.max_w;
            }
            {
                for a in self.args.iter().filter(|a| a.is_set(ArgSettings::Global)) {
                    sc.args.push(a.clone());
                }
            }
            // @TODO @deadcode @perf @v3-alpha: Currently we're not propagating
            if prop == Propagation::Full {
                sc._build(Propagation::Full);
            }
        }
    }

    pub(crate) fn _create_help_and_version(&mut self) {
        debugln!("App::_create_help_and_version;");
        // name is "hclap_help" because flags are sorted by name
        if !self.contains_long("help") {
            debugln!("App::_create_help_and_version: Building --help");
            if self.help_short.is_none() && !self.contains_short('h') {
                self.help_short = Some('h');
            }
            let mut arg = Arg::with_name("hclap_help")
                .long("help")
                .help(self.help_message.unwrap_or("Prints help information"));

            // we have to set short manually because we're dealing with char's
            arg.short = self.help_short;
            self.args.push(arg);
        } else {
            self.settings.unset(AppSettings::NeedsLongHelp);
        }
        if !self.is_set(AppSettings::DisableVersion) && !self.contains_long("version") {
            debugln!("App::_create_help_and_version: Building --version");
            if self.version_short.is_none() && !self.contains_short('V') {
                self.version_short = Some('V');
            }
            // name is "vclap_version" because flags are sorted by name
            let mut arg = Arg::with_name("vclap_version")
                .long("version")
                .help(self.version_message.unwrap_or("Prints version information"));
            // we have to set short manually because we're dealing with char's
            arg.short = self.version_short;
            self.args.push(arg);
        } else {
            self.settings.unset(AppSettings::NeedsLongVersion);
        }
        if self.has_subcommands() && !self.is_set(AppSettings::DisableHelpSubcommand)
            && !subcommands!(self).any(|s| s.name == "help")
        {
            debugln!("App::_create_help_and_version: Building help");
            self.subcommands.push(
                App::new("help")
                    .about("Prints this message or the help of the given subcommand(s)"),
            );
        } else {
            self.settings.unset(AppSettings::NeedsSubcommandHelp);
        }
    }

    pub(crate) fn _derive_display_order(&mut self) {
        debugln!("App::_derive_display_order:{}", self.name);
        if self.settings.is_set(AppSettings::DeriveDisplayOrder) {
            for (i, a) in args_mut!(self).filter(|a| a.has_switch())
                .filter(|a| a.disp_ord == 999)
                .enumerate()
            {
                a.disp_ord = i;
            }
            for (i, sc) in &mut subcommands_mut!(self)
                .enumerate()
                .filter(|&(_, ref sc)| sc.disp_ord == 999)
            {
                sc.disp_ord = i;
            }
        }
        for sc in subcommands_mut!(self) {
            sc._derive_display_order();
        }
    }

    // Perform expensive assertions on the Arg instance
    fn _arg_debug_asserts(&self, a: &Arg) -> bool {
        debugln!("App::_arg_debug_asserts:{}", a.name);
        // No naming conflicts
        assert!(
            arg_names!(self).fold(0, |acc, n| if n == a.name { acc + 1 } else { acc }) < 2,
            format!("Non-unique argument name: {} is already in use", a.name)
        );

        // Long conflicts
        if let Some(l) = a.long {
            assert!(
                args!(self).fold(0, |acc, arg| if arg.long == Some(l) { acc + 1 } else { acc }) < 2,
                "Argument long must be unique\n\n\t--{} is already in use",
                l
            );
        }

        // Short conflicts
        if let Some(s) = a.short {
            assert!(
                args!(self).fold(0, |acc, arg| if arg.short == Some(s) { acc + 1 } else { acc }) < 2,
                "Argument short must be unique\n\n\t-{} is already in use",
                s
            );
        }

        if let Some(idx) = a.index {
            // No index conflicts
            assert!(
                positionals!(self).fold(0, |acc, p| if p.index == Some(idx as u64){acc+1}else{acc}) < 2,
                "Argument '{}' has the same index as another positional \
                 argument\n\n\tUse Arg::setting(ArgSettings::MultipleValues) to allow one \
                 positional argument to take multiple values",
                a.name
            );
        }
        if a.is_set(ArgSettings::Last) {
            assert!(a.long.is_none(),
                    "Flags or Options may not have last(true) set. {} has both a long and \
                    last(true) set.",
                    a.name);
            assert!(a.short.is_none(),
                    "Flags or Options may not have last(true) set. {} has both a short and \
                    last(true) set.",
                    a.name);
        }
        assert!(
            !(a.is_set(ArgSettings::Required) && a.is_set(ArgSettings::Global)),
            "Global arguments cannot be required.\n\n\t'{}' is marked as \
             global and required",
            a.name
        );

        true
    }

    fn _build_bin_names(&mut self) {
        debugln!("App::_build_bin_names;");
        for sc in subcommands_mut!(self) {
            debug!("Parser::build_bin_names:iter: bin_name set...");
            if sc.bin_name.is_none() {
                sdebugln!("No");
                let bin_name = format!(
                    "{}{}{}",
                    self.bin_name.as_ref().unwrap_or(&self.name.clone()),
                    if self.bin_name.is_some() { " " } else { "" },
                    &*sc.name
                );
                debugln!(
                    "Parser::build_bin_names:iter: Setting bin_name of {} to {}",
                    self.name,
                    bin_name
                );
                sc.bin_name = Some(bin_name);
            } else {
                sdebugln!("yes ({:?})", sc.bin_name);
            }
            debugln!(
                "Parser::build_bin_names:iter: Calling build_bin_names from...{}",
                sc.name
            );
            sc._build_bin_names();
        }
    }

    pub(crate) fn _write_version<W: Write>(&self, w: &mut W, use_long: bool) -> io::Result<()> {
        debugln!("App::_write_version;");
        let ver = if use_long {
            self.long_version
                .unwrap_or_else(|| self.version.unwrap_or(""))
        } else {
            self.version
                .unwrap_or_else(|| self.long_version.unwrap_or(""))
        };
        if let Some(bn) = self.bin_name.as_ref() {
            if bn.contains(' ') {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                write!(w, "{} {}", bn.replace(" ", "-"), ver)
            } else {
                write!(w, "{} {}", &self.name[..], ver)
            }
        } else {
            write!(w, "{} {}", &self.name[..], ver)
        }
    }
}

// Internal Query Methods
#[doc(hidden)]
impl<'a, 'b> App<'a, 'b> {
    // Should we color the output? None=determined by output location, true=yes, false=no
    #[doc(hidden)]
    pub fn color(&self) -> ColorWhen {
        debugln!("App::color;");
        debug!("App::color: Color setting...");
        if self.is_set(AppSettings::ColorNever) {
            sdebugln!("Never");
            ColorWhen::Never
        } else if self.is_set(AppSettings::ColorAlways) {
            sdebugln!("Always");
            ColorWhen::Always
        } else {
            sdebugln!("Auto");
            ColorWhen::Auto
        }
    }
    fn contains_long(&self, l: &str) -> bool { longs!(self).any(|al| al == l) }

    fn contains_short(&self, s: char) -> bool { shorts!(self).any(|arg_s| arg_s == s) }

    pub fn is_set(&self, s: AppSettings) -> bool {
        self.settings.is_set(s) || self.g_settings.is_set(s)
    }

    pub fn set(&mut self, s: AppSettings) { self.settings.set(s) }

    pub fn set_global(&mut self, s: AppSettings) { self.g_settings.set(s) }

    pub fn unset_global(&mut self, s: AppSettings) { self.g_settings.unset(s) }

    pub fn unset(&mut self, s: AppSettings) { self.settings.unset(s) }

    pub fn has_subcommands(&self) -> bool { !self.subcommands.is_empty() }

    pub fn has_args(&self) -> bool { !self.args.is_empty() }

    pub fn has_opts(&self) -> bool { opts!(self).count() > 0 }

    pub fn has_flags(&self) -> bool { flags!(self).count() > 0 }

    pub fn has_positionals(&self) -> bool { positionals!(self).count() > 0 }

    pub fn has_visible_opts(&self) -> bool { opts!(self).any(|o| !o.is_set(ArgSettings::Hidden)) }

    pub fn has_visible_flags(&self) -> bool { flags!(self).any(|o| !o.is_set(ArgSettings::Hidden)) }

    pub fn has_visible_positionals(&self) -> bool {
        positionals!(self).any(|o| !o.is_set(ArgSettings::Hidden))
    }

    pub fn has_visible_subcommands(&self) -> bool {
        subcommands!(self)
            .filter(|sc| sc.name != "help")
            .any(|sc| !sc.is_set(AppSettings::Hidden))
    }

    fn use_long_help(&self) -> bool {
        self.long_about.is_some() || self.args.iter().any(|f| f.long_help.is_some())
            || subcommands!(self).any(|s| s.long_about.is_some())
    }
}

// @TODO @v3-beta: remove
// Deprecations
impl<'a, 'b> App<'a, 'b> {

    /// **Deprecated:** Use `App::global_setting( SettingOne | SettingTwo )` instead
    #[deprecated(since="2.33.0", note="Use `App::global_setting( SettingOne | SettingTwo )` instead")]
    pub fn global_settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.settings.set(*s);
            self.g_settings.set(*s)
        }
        self
    }

    /// **Deprecated:** Use `App::setting( SettingOne | SettingTwo )` instead
    #[deprecated(since="2.33.0", note="Use `App::setting( SettingOne | SettingTwo )` instead")]
    pub fn settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.settings.set(*s);
        }
        self
    }


    /// **Deprecated:** Use `App::unset_setting( SettingOne | SettingTwo )` instead
    #[deprecated(since="2.33.0", note="Use `App::unset_setting( SettingOne | SettingTwo )` instead")]
    pub fn unset_settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.settings.unset(*s);
            self.g_settings.unset(*s);
        }
        self
    }

    /// **Deprecated:** Use explicit `App::author()` and `App::version()` calls instead.
    #[deprecated(since="2.14.1", note="Can never work; use explicit App::author() and \
        App::version() calls instead. Will be removed in v3.0-beta")]
    pub fn with_defaults<S: Into<String>>(n: S) -> Self {
        App {
            name: n.into(),
            author: Some("Kevin K. <kbknapp@gmail.com>"),
            version: Some("2.19.2"),
            ..Default::default()
        }
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use serde instead. Will be removed in v3.0-beta")]
    #[cfg(feature = "yaml")]
    pub fn from_yaml(yaml: &'a Yaml) -> App<'a, 'a> { App::from(yaml) }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use `App::mut_arg(\"help\", |a| a.short(\"H\"))` instead. Will be removed in v3.0-beta")]
    pub fn help_short<S: AsRef<str> + 'b>(mut self, s: S) -> Self {
        let c = s.as_ref()
            .trim_left_matches(|c| c == '-')
            .chars()
            .nth(0)
            .unwrap_or('h');
        self.help_short = Some(c);
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use `App::mut_arg(\"version\", |a| a.short(\"v\"))` instead. Will be removed in v3.0-beta")]
    pub fn version_short<S: AsRef<str>>(mut self, s: S) -> Self {
        let c = s.as_ref()
            .trim_left_matches(|c| c == '-')
            .chars()
            .nth(0)
            .unwrap_or('V');
        self.version_short = Some(c);
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use `App::mut_arg(\"help\", |a| a.help(\"Some message\"))` instead. Will be removed in v3.0-beta")]
    pub fn help_message<S: Into<&'a str>>(mut self, s: S) -> Self {
        self.help_message = Some(s.into());
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use `App::mut_arg(\"version\", |a| a.short(\"Some message\"))` instead. Will be removed in v3.0-beta")]
    pub fn version_message<S: Into<&'a str>>(mut self, s: S) -> Self {
        self.version_message = Some(s.into());
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Renamed to `App::override_usage`. Will be removed in v3.0-beta")]
    pub fn usage<S: Into<&'b str>>(mut self, usage: S) -> Self {
        self.usage_str = Some(usage.into());
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Renamed to `App::override_help`. Will be removed in v3.0-beta")]
    pub fn help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.help_str = Some(help.into());
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Renamed to `App::help_template`. Will be removed in v3.0-beta")]
    pub fn template<S: Into<&'b str>>(mut self, s: S) -> Self {
        self.template = Some(s.into());
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use `App::arg(Arg::from(&str)` instead. Will be removed in v3.0-beta")]
    pub fn arg_from_usage(mut self, usage: &'a str) -> Self {
        self.args.push(Arg::from_usage(usage));
        self
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use `App::args(&str)` instead. Will be removed in v3.0-beta")]
    pub fn args_from_usage(mut self, usage: &'a str) -> Self {
        for line in usage.lines() {
            let l = line.trim();
            if l.is_empty() {
                continue;
            }
            self.args.push(Arg::from_usage(l));
        }
        self
    }

    /// **Deprecated:** Use
    #[allow(deprecated)]
    #[deprecated(since="2.30.0", note="Use `clap_completions crate and clap_completions::generate` instead. Will be removed in v3.0-beta")]
    pub fn gen_completions<T: Into<OsString>, S: Into<String>>(
        &mut self,
        bin_name: S,
        for_shell: Shell,
        out_dir: T,
    ) {
        use std::error::Error;

        let out_dir = PathBuf::from(out_dir.into());
        let name = &*self.bin_name.as_ref().unwrap().clone();
        let file_name = match for_shell {
            Shell::Bash => format!("{}.bash", name),
            Shell::Fish => format!("{}.fish", name),
            Shell::Zsh => format!("_{}", name),
            Shell::PowerShell => format!("_{}.ps1", name),
        };

        let mut file = match File::create(out_dir.join(file_name)) {
            Err(why) => panic!("couldn't create completion file: {}", why.description()),
            Ok(file) => file,
        };
        self.gen_completions_to(bin_name.into(), for_shell, &mut file)
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Use `clap_completions crate and clap_completions::generate_to` instead. Will be removed in v3.0-beta")]
    pub fn gen_completions_to<W: Write, S: Into<String>>(
        &mut self,
        bin_name: S,
        for_shell: Shell,
        buf: &mut W,
    ) {
        self.bin_name = Some(bin_name.into());
        if !self.is_set(AppSettings::Propagated) {
            self._build(Propagation::Full);
            self._build_bin_names();
        }

        ComplGen::new(self).generate(for_shell, buf)
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Renamed `App::try_get_matches` to be consistent with Rust naming conventions. Will be removed in v3.0-beta")]
    pub fn get_matches_safe(self) -> ClapResult<ArgMatches<'a>> {
        // Start the parsing
        self.try_get_matches_from(&mut env::args_os())
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Renamed `App::try_get_matches_from` to be consistent with Rust naming conventions. Will be removed in v3.0-beta")]
    pub fn get_matches_from_safe<I, T>(mut self, itr: I) -> ClapResult<ArgMatches<'a>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from_mut(itr)
    }

    /// **Deprecated:** Use
    #[deprecated(since="2.30.0", note="Renamed `App::try_get_matches_from_mut` to be consistent with Rust naming conventions. Will be removed in v3.0-beta")]
    pub fn get_matches_from_safe_borrow<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches<'a>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.try_get_matches_from_mut(itr)
    }

}

#[cfg(feature = "yaml")]
impl<'a> From<&'a Yaml> for App<'a, 'a> {
    fn from(mut yaml: &'a Yaml) -> Self {
        use args::SubCommand;
        // We WANT this to panic on error...so expect() is good.
        let mut is_sc = None;
        let mut a = if let Some(name) = yaml["name"].as_str() {
            App::new(name)
        } else {
            let yaml_hash = yaml.as_hash().unwrap();
            let sc_key = yaml_hash.keys().nth(0).unwrap();
            is_sc = Some(yaml_hash.get(sc_key).unwrap());
            App::new(sc_key.as_str().unwrap())
        };
        yaml = if let Some(sc) = is_sc { sc } else { yaml };

        macro_rules! yaml_str {
            ($a:ident, $y:ident, $i:ident) => {
                if let Some(v) = $y[stringify!($i)].as_str() {
                    $a = $a.$i(v);
                } else if $y[stringify!($i)] != Yaml::BadValue {
                    panic!("Failed to convert YAML value {:?} to a string", $y[stringify!($i)]);
                }
            };
        }

        yaml_str!(a, yaml, version);
        yaml_str!(a, yaml, author);
        yaml_str!(a, yaml, bin_name);
        yaml_str!(a, yaml, about);
        yaml_str!(a, yaml, before_help);
        yaml_str!(a, yaml, after_help);
        yaml_str!(a, yaml, template);
        yaml_str!(a, yaml, usage);
        yaml_str!(a, yaml, help);
        yaml_str!(a, yaml, help_short);
        yaml_str!(a, yaml, version_short);
        yaml_str!(a, yaml, help_message);
        yaml_str!(a, yaml, version_message);
        yaml_str!(a, yaml, alias);
        yaml_str!(a, yaml, visible_alias);

        if let Some(v) = yaml["display_order"].as_i64() {
            a = a.display_order(v as usize);
        } else if yaml["display_order"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to a u64",
                yaml["display_order"]
            );
        }
        if let Some(v) = yaml["setting"].as_str() {
            a = a.setting(v.parse().expect("unknown AppSetting found in YAML file"));
        } else if yaml["setting"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to an AppSetting",
                yaml["setting"]
            );
        }
        if let Some(v) = yaml["settings"].as_vec() {
            for ys in v {
                if let Some(s) = ys.as_str() {
                    a = a.setting(s.parse().expect("unknown AppSetting found in YAML file"));
                }
            }
        } else if let Some(v) = yaml["settings"].as_str() {
            a = a.setting(v.parse().expect("unknown AppSetting found in YAML file"));
        } else if yaml["settings"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to a string",
                yaml["settings"]
            );
        }
        if let Some(v) = yaml["global_setting"].as_str() {
            a = a.setting(v.parse().expect("unknown AppSetting found in YAML file"));
        } else if yaml["global_setting"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to an AppSetting",
                yaml["setting"]
            );
        }
        if let Some(v) = yaml["global_settings"].as_vec() {
            for ys in v {
                if let Some(s) = ys.as_str() {
                    a = a.global_setting(s.parse().expect("unknown AppSetting found in YAML file"));
                }
            }
        } else if let Some(v) = yaml["global_settings"].as_str() {
            a = a.global_setting(v.parse().expect("unknown AppSetting found in YAML file"));
        } else if yaml["global_settings"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to a string",
                yaml["global_settings"]
            );
        }

        macro_rules! vec_or_str {
            ($a:ident, $y:ident, $as_vec:ident, $as_single:ident) => {{
                    let maybe_vec = $y[stringify!($as_vec)].as_vec();
                    if let Some(vec) = maybe_vec {
                        for ys in vec {
                            if let Some(s) = ys.as_str() {
                                $a = $a.$as_single(s);
                            } else {
                                panic!("Failed to convert YAML value {:?} to a string", ys);
                            }
                        }
                    } else {
                        if let Some(s) = $y[stringify!($as_vec)].as_str() {
                            $a = $a.$as_single(s);
                        } else if $y[stringify!($as_vec)] != Yaml::BadValue {
                            panic!("Failed to convert YAML value {:?} to either a vec or string", $y[stringify!($as_vec)]);
                        }
                    }
                    $a
                }
            };
        }

        a = vec_or_str!(a, yaml, aliases, alias);
        a = vec_or_str!(a, yaml, visible_aliases, visible_alias);

        if let Some(v) = yaml["args"].as_vec() {
            for arg_yaml in v {
                a = a.arg(Arg::from_yaml(arg_yaml.as_hash().unwrap()));
            }
        }
        if let Some(v) = yaml["subcommands"].as_vec() {
            for sc_yaml in v {
                a = a.subcommand(SubCommand::from_yaml(sc_yaml));
            }
        }
        if let Some(v) = yaml["groups"].as_vec() {
            for ag_yaml in v {
                a = a.group(ArgGroup::from(ag_yaml.as_hash().unwrap()));
            }
        }

        a
    }
}

impl<'n, 'e> fmt::Display for App<'n, 'e> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.name) }
}