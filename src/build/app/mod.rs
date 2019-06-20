mod settings;
pub use self::settings::{AppFlags, AppSettings};

// Std
use std::env;
use std::ffi::OsString;
use std::fmt;
use std::io::{self, BufRead, BufWriter, Write};
use std::iter::Peekable;
use std::path::Path;
use std::process;

// Third Party
#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

// Internal
use crate::build::{Arg, ArgGroup, ArgSettings};
use crate::mkeymap::MKeyMap;
use crate::output::fmt::ColorWhen;
use crate::output::{Help, Usage};
use crate::parse::errors::Result as ClapResult;
use crate::parse::{ArgMatcher, ArgMatches, Parser};
use crate::util::{Key, HELP_HASH, VERSION_HASH};
use crate::INTERNAL_ERROR_MSG;

type Id = u64;

#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Propagation {
    To(Id),
    Full,
    NextLevel,
    None,
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
pub struct App<'b> {
    #[doc(hidden)]
    pub id: Id,
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
    pub args: MKeyMap<'b>,
    #[doc(hidden)]
    pub subcommands: Vec<App<'b>>,
    #[doc(hidden)]
    pub groups: Vec<ArgGroup<'b>>,
    #[doc(hidden)]
    pub help_headings: Vec<Option<&'b str>>,
}

impl<'b> App<'b> {
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
        let name = n.into();
        let id = name.key();
        App {
            id,
            name,
            ..Default::default()
        }
    }

    /// Get the name of the app
    pub fn get_name(&self) -> &str { &self.name }

    /// Get the name of the binary
    pub fn get_bin_name(&self) -> Option<&str> { self.bin_name.as_ref().map(String::as_str) }

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
    /// **NOTE:** This command **should not** be used for [``]s.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("My Program")
    ///      .bin_name("my_binary")
    /// # ;
    /// ```
    /// [``]: ./struct..html
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
        self.settings.set(setting);
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
        self.settings.unset(setting);
        self.g_settings.unset(setting);
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
    ///            .short('d')
    ///            .help("turns on debugging mode")
    ///     )
    ///     // Adding a single "option" argument with a short, a long, and help text using the less
    ///     // verbose Arg::from()
    ///     .arg(
    ///         Arg::from("-c --config=[CONFIG] 'Optionally sets a config file to use'")
    ///     )
    /// # ;
    /// ```
    /// [argument]: ./struct.Arg.html
    pub fn arg<A: Into<Arg<'b>>>(mut self, a: A) -> Self {
        let help_heading: Option<&'b str> = if let Some(option_str) = self.help_headings.last() {
            *option_str
        } else {
            None
        };
        let arg = a.into().help_heading(help_heading);
        self.args.push(arg);
        self
    }

    /// Set a custom section heading for future args. Every call to arg will
    /// have this header (instead of its default header) until a subsequent
    /// call to help_heading
    pub fn help_heading(mut self, heading: &'b str) -> Self {
        self.help_headings.push(Some(heading));
        self
    }

    /// Stop using custom section headings.
    pub fn stop_custom_headings(mut self) -> Self {
        self.help_headings.push(None);
        self
    }

    /// Adds multiple [arguments] to the list of valid possibilties
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args(&[
    ///         Arg::from("[debug] -d 'turns on debugging info'"),
    ///         Arg::with_name("input").index(1).help("the input file to use")
    ///     ])
    /// # ;
    /// ```
    /// [arguments]: ./struct.Arg.html
    pub fn args<I, T>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Arg<'b>>,
    {
        // @TODO @perf @p4 @v3-beta: maybe extend_from_slice would be possible and perform better?
        // But that may also not let us do `&["-a 'some'", "-b 'other']` because of not Into<Arg>
        for arg in args.into_iter() {
            self.args.push(arg.into());
        }
        self
    }

    /// Allows adding a [``] alias, which function as "hidden" subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test")
    ///                 .alias("do-stuff"))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
    pub fn alias<S: Into<&'b str>>(mut self, name: S) -> Self {
        if let Some(ref mut als) = self.aliases {
            als.push((name.into(), false));
        } else {
            self.aliases = Some(vec![(name.into(), false)]);
        }
        self
    }

    /// Allows adding [``] aliases, which function as "hidden" subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test")
    ///                 .aliases(&["do-stuff", "do-tests", "tests"]))
    ///                 .arg(Arg::with_name("input")
    ///                             .help("the file to add")
    ///                             .index(1)
    ///                             .required(false))
    ///             .get_matches_from(vec!["myprog", "do-tests"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
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

    /// Allows adding a [``] alias that functions exactly like those defined with
    /// [`App::alias`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test")
    ///                 .visible_alias("do-stuff"))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
    /// [`App::alias`]: ./struct.App.html#method.alias
    pub fn visible_alias<S: Into<&'b str>>(mut self, name: S) -> Self {
        if let Some(ref mut als) = self.aliases {
            als.push((name.into(), true));
        } else {
            self.aliases = Some(vec![(name.into(), true)]);
        }
        self
    }

    /// Allows adding multiple [``] aliases that functions exactly like those defined
    /// with [`App::aliases`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test")
    ///                 .visible_aliases(&["do-stuff", "tests"]))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
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
    ///     .arg("--set-ver [ver] 'set the version manually'")
    ///     .arg("--major 'auto increase major'")
    ///     .arg("--minor 'auto increase minor'")
    ///     .arg("--patch 'auto increase patch'")
    ///     .group(ArgGroup::with_name("vers")
    ///          .args(&["set-ver", "major", "minor","patch"])
    ///          .required(true))
    /// # ;
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    pub fn group(mut self, group: ArgGroup<'b>) -> Self {
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
    ///     .arg("--set-ver [ver] 'set the version manually'")
    ///     .arg("--major         'auto increase major'")
    ///     .arg("--minor         'auto increase minor'")
    ///     .arg("--patch         'auto increase patch'")
    ///     .arg("-c [FILE]       'a config file'")
    ///     .arg("-i [IFACE]      'an interface'")
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
    pub fn groups(mut self, groups: &[ArgGroup<'b>]) -> Self {
        for g in groups {
            self = self.group(g.into());
        }
        self
    }

    /// Adds a [``] to the list of valid possibilities. Subcommands are effectively
    /// sub-[`App`]s, because they can contain their own arguments, subcommands, version, usage,
    /// etc. They also function just like [`App`]s, in that they get their own auto generated help,
    /// version, and usage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    /// App::new("myprog")
    ///     .subcommand(App::new("config")
    ///         .about("Controls configuration features")
    ///         .arg("<config> 'Required configuration file to use'"))
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`App`]: ./struct.App.html
    pub fn subcommand(mut self, subcmd: App<'b>) -> Self {
        self.subcommands.push(subcmd);
        self
    }

    /// Adds multiple subcommands to the list of valid possibilities by iterating over an
    /// [`IntoIterator`] of [``]s
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// # App::new("myprog")
    /// .subcommands( vec![
    ///        App::new("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::with_name("config_file").index(1)),
    ///        App::new("debug").about("Controls debug functionality")])
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
    pub fn subcommands<I>(mut self, subcmds: I) -> Self
    where
        I: IntoIterator<Item = App<'b>>,
    {
        for subcmd in subcmds {
            self.subcommands.push(subcmd);
        }
        self
    }

    /// Allows custom ordering of [``]s within the help message. Subcommands with a lower
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
    /// [``]: ./struct..html
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
    /// [`Arg`]: ./struct.Arg.html
    pub fn mut_arg<T, F>(mut self, arg_id: T, f: F) -> Self
    where
        F: FnOnce(Arg<'b>) -> Arg<'b>,
        T: Key + Into<&'b str>,
    {
        let id = arg_id.key();
        let a = self.args.remove_by_name(id).unwrap_or_else(|| Arg {
            id,
            name: arg_id.into(),
            ..Arg::default()
        });
        self.args.push(f(a));

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
        self._build();

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
        self._build();

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
        self._build();

        let p = Parser::new(self);
        Help::new(w, &p, false, false).write_help()
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
        self._build();

        let p = Parser::new(self);
        Help::new(w, &p, true, false).write_help()
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

    /// @TODO-v3-alpha @docs @p2: write docs
    pub fn generate_usage(&mut self) -> String {
        // If there are global arguments, or settings we need to propgate them down to subcommands
        // before parsing incase we run into a subcommand
        if !self.settings.is_set(AppSettings::Built) {
            self._build();
        }

        let mut parser = Parser::new(self);
        parser._build();
        Usage::new(&parser).create_usage_with_title(&[])
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
    pub fn get_matches(self) -> ArgMatches { self.get_matches_from(&mut env::args_os()) }

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
    pub fn get_matches_mut(&mut self) -> ArgMatches {
        self.try_get_matches_from_mut(&mut env::args_os())
            .unwrap_or_else(|e| {
                // Otherwise, write to stderr and exit
                if e.use_stderr() {
                    wlnerr!("{}", e.message);
                    if self.settings.is_set(AppSettings::WaitOnError) {
                        wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                        let mut s = String::new();
                        let i = io::stdin();
                        i.lock().read_line(&mut s).unwrap();
                    }
                    drop(e);
                    process::exit(1);
                }

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
    pub fn try_get_matches(self) -> ClapResult<ArgMatches> {
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
    pub fn get_matches_from<I, T>(mut self, itr: I) -> ArgMatches
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
    /// [`App::try_get_matches`]: ./struct.App.html#method.try_get_matches
    /// [`ErrorKind::HelpDisplayed`]: ./enum.ErrorKind.html#variant.HelpDisplayed
    /// [`ErrorKind::VersionDisplayed`]: ./enum.ErrorKind.html#variant.VersionDisplayed
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`std::process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
    /// [`clap::Error`]: ./struct.Error.html
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`kind`]: ./struct.Error.html
    /// [`AppSettings::NoBinaryName`]: ./enum.AppSettings.html#variant.NoBinaryName
    pub fn try_get_matches_from<I, T>(mut self, itr: I) -> ClapResult<ArgMatches>
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
    pub fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>
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
impl<'b> App<'b> {
    #[doc(hidden)]
    fn _do_parse<I, T>(&mut self, it: &mut Peekable<I>) -> ClapResult<ArgMatches>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        debugln!("App::_do_parse;");
        let mut matcher = ArgMatcher::new();

        // If there are global arguments, or settings we need to propgate them down to subcommands
        // before parsing incase we run into a subcommand
        if !self.settings.is_set(AppSettings::Built) {
            self._build();
        }

        {
            let mut parser = Parser::new(self);

            // do the real parsing
            parser.get_matches_with(&mut matcher, it)?;
        }

        let global_arg_vec: Vec<Id> = self
            .args
            .args
            .iter()
            .filter(|a| a.global)
            .map(|ga| ga.id)
            .collect();

        matcher.propagate_globals(&global_arg_vec);

        Ok(matcher.into_inner())
    }

    // used in clap_generate (https://github.com/clap-rs/clap_generate)
    #[doc(hidden)]
    pub fn _build(&mut self) {
        debugln!("App::_build;");
        // Make sure all the globally set flags apply to us as well
        self.settings = self.settings | self.g_settings;

        self._derive_display_order();
        self._create_help_and_version();
        // Perform expensive debug assertions
        debug_assert!({
            for a in self.args.args.iter() {
                self._arg_debug_asserts(a);
            }
            true
        });

        let mut pos_counter = 1;
        for a in self.args.args.iter_mut() {
            // Fill in the groups
            if let Some(ref grps) = a.groups {
                for &g in grps {
                    let mut found = false;
                    if let Some(ag) = self.groups.iter_mut().find(|grp| grp.id == g) {
                        ag.args.push(a.id);
                        found = true;
                    }
                    if !found {
                        let mut ag = ArgGroup::_with_id(g);
                        ag.args.push(a.id);
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
            if a.short.is_none() && a.long.is_none() && a.index.is_none() {
                a.index = Some(pos_counter);
                pos_counter += 1;
            }
        }

        debug_assert!(self._app_debug_asserts());
        self.args._build();
        self.settings.set(AppSettings::Built);
    }

    // Perform some expensive assertions on the Parser itself
    fn _app_debug_asserts(&mut self) -> bool {
        debugln!("App::_app_debug_asserts;");
        for name in self.args.args.iter().map(|x| x.id) {
            if self.args.args.iter().filter(|x| x.id == name).count() > 1 {
                panic!(format!(
                    "Arg names must be unique, found {} more than once",
                    name
                ));
            }
        }
        // * Args listed inside groups should exist
        // * Groups should not have naming conflicts with Args

        // * Will be removed as a part of removing String types
        // let g = groups!(self).find(|g| {
        //     g.args
        //         .iter()
        //         .any(|arg| !(find!(self, arg).is_some() || groups!(self).any(|g| &g.name == arg)))
        // });
        // assert!(
        //     g.is_none(),
        //     "The group '{}' contains an arg that doesn't exist or has a naming conflict with a group.",
        //     g.unwrap().name
        // );
        true
    }

    pub fn _propagate(&mut self, prop: Propagation) {
        macro_rules! propagate_subcmd {
            ($_self:ident, $sc:expr) => {{
                // We have to create a new scope in order to tell rustc the borrow of `sc` is
                // done and to recursively call this method
                {
                    let vsc = $_self.settings.is_set(AppSettings::VersionlessSubcommands);
                    let gv = $_self.settings.is_set(AppSettings::GlobalVersion);

                    if vsc {
                        $sc.set(AppSettings::DisableVersion);
                    }
                    if gv && $sc.version.is_none() && $_self.version.is_some() {
                        $sc.set(AppSettings::GlobalVersion);
                        $sc.version = Some($_self.version.unwrap());
                    }
                    $sc.settings = $sc.settings | $_self.g_settings;
                    $sc.g_settings = $sc.g_settings | $_self.g_settings;
                    $sc.term_w = $_self.term_w;
                    $sc.max_w = $_self.max_w;
                }
                {
                    for a in $_self
                        .args
                        .args
                        .iter()
                        .filter(|a| a.global)
                        {
                            $sc.args.push(a.clone());
                        }
                }
            }}
        }

        debugln!("App::_propagate:{}", self.name);
        match prop {
            Propagation::NextLevel | Propagation::Full => {
                for sc in &mut self.subcommands {
                    propagate_subcmd!(self, sc);
                    if prop == Propagation::Full {
                        sc._propagate(prop);
                    }
                }
            },
            Propagation::To(id) => {
                let mut sc = self.subcommands.iter_mut().find(|sc| sc.id == id).expect(INTERNAL_ERROR_MSG);
                propagate_subcmd!(self, sc);
            },
            Propagation::None => {
                return;
            },
        }
    }

    pub(crate) fn _create_help_and_version(&mut self) {
        debugln!("App::_create_help_and_version;");
        if !(self
            .args
            .args
            .iter()
            .any(|x| x.long == Some("help") || x.id == HELP_HASH))
        {
            debugln!("App::_create_help_and_version: Building --help");
            let mut help = Arg::with_name("help")
                .long("help")
                .help("Prints help information");
            if !self.args.args.iter().any(|x| x.short == Some('h')) {
                help = help.short('h');
            }

            self.args.push(help);
        }
        if !(self
            .args
            .args
            .iter()
            .any(|x| x.long == Some("version") || x.id == VERSION_HASH)
            || self.is_set(AppSettings::DisableVersion))
        {
            debugln!("App::_create_help_and_version: Building --version");
            let mut version = Arg::with_name("version")
                .long("version")
                .help("Prints version information");
            if !self.args.args.iter().any(|x| x.short == Some('V')) {
                version = version.short('V');
            }

            self.args.push(version);
        }
        if self.has_subcommands()
            && !self.is_set(AppSettings::DisableHelpSubcommand)
            && !subcommands!(self).any(|s| s.id == HELP_HASH)
        {
            debugln!("App::_create_help_and_version: Building help");
            self.subcommands.push(
                App::new("help")
                    .about("Prints this message or the help of the given subcommand(s)"),
            );
        }
    }

    pub(crate) fn _derive_display_order(&mut self) {
        debugln!("App::_derive_display_order:{}", self.name);
        if self.settings.is_set(AppSettings::DeriveDisplayOrder) {
            for (i, a) in self
                .args
                .args
                .iter_mut()
                .filter(|a| a.has_switch())
                .filter(|a| a.disp_ord == 999)
                .enumerate()
            {
                a.disp_ord = i;
            }
            for (i, mut sc) in &mut subcommands_mut!(self)
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

        // Long conflicts
        if let Some(l) = a.long {
            assert!(
                self.args.args.iter().filter(|x| x.long == Some(l)).count() < 2,
                "Argument long must be unique\n\n\t--{} is already in use",
                l
            );
        }

        // Short conflicts
        if let Some(s) = a.short {
            assert!(
                self.args.args.iter().filter(|x| x.short == Some(s)).count() < 2,
                "Argument short must be unique\n\n\t-{} is already in use",
                s
            );
        }

        if let Some(idx) = a.index {
            // No index conflicts
            assert!(
                positionals!(self).fold(0, |acc, p| if p.index == Some(idx) {
                    acc + 1
                } else {
                    acc
                }) < 2,
                "Argument '{}' has the same index as another positional \
                 argument\n\n\tUse Arg::setting(ArgSettings::MultipleValues) to allow one \
                 positional argument to take multiple values",
                a.name
            );
        }
        if a.is_set(ArgSettings::Last) {
            assert!(
                a.long.is_none(),
                "Flags or Options may not have last(true) set. {} has both a long and \
                 last(true) set.",
                a.name
            );
            assert!(
                a.short.is_none(),
                "Flags or Options may not have last(true) set. {} has both a short and \
                 last(true) set.",
                a.name
            );
        }
        assert!(
            !(a.is_set(ArgSettings::Required) && a.global),
            "Global arguments cannot be required.\n\n\t'{}' is marked as \
             global and required",
            a.name
        );

        true
    }

    // used in clap_generate (https://github.com/clap-rs/clap_generate)
    #[doc(hidden)]
    pub fn _build_bin_names(&mut self) {
        debugln!("App::_build_bin_names;");
        for mut sc in subcommands_mut!(self) {
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

    pub(crate) fn format_group(&self, g: Id) -> String {
        let g_string = self
            .unroll_args_in_group(g)
            .iter()
            .filter_map(|&x| self.find(x))
            .map(|x| {
                if x.index.is_some() {
                    x.name.to_owned()
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("<{}>", &*g_string)
    }
}

// Internal Query Methods
#[doc(hidden)]
impl<'b> App<'b> {
    pub(crate) fn find(&self, arg_id: Id) -> Option<&Arg<'b>> {
        self.args.args.iter().find(|a| a.id == arg_id)
    }

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

    pub(crate) fn contains_short(&self, s: char) -> bool {
        if !self.is_set(AppSettings::Built) {
            panic!("If App::_build hasn't been called, manually search through Arg shorts");
        }
        self.args.contains_short(s)
    }

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

    pub(crate) fn unroll_args_in_group(&self, group: Id) -> Vec<Id> {
        let mut g_vec = vec![group];
        let mut args = vec![];

        while let Some(ref g) = g_vec.pop() {
            for &n in self
                .groups
                .iter()
                .find(|grp| &grp.id == g)
                .expect(INTERNAL_ERROR_MSG)
                .args
                .iter()
            {
                if !args.contains(&n) {
                    if self.find(n).is_some() {
                        args.push(n)
                    } else {
                        g_vec.push(n);
                    }
                }
            }
        }

        args
    }

    pub(crate) fn unroll_requirements_for_arg(&self, arg: Id, matcher: &ArgMatcher) -> Vec<Id> {
        let requires_if_or_not = |&(val, req_arg)| {
            if let Some(v) = val {
                if matcher
                    .get(arg)
                    .and_then(|ma| Some(ma.contains_val(v)))
                    .unwrap_or(false)
                {
                    Some(req_arg)
                } else {
                    None
                }
            } else {
                Some(req_arg)
            }
        };

        let mut r_vec = vec![arg];
        let mut args = vec![];

        while let Some(a) = r_vec.pop() {
            if let Some(arg) = self.find(a) {
                if let Some(ref reqs) = arg.requires {
                    for r in reqs.iter().filter_map(requires_if_or_not) {
                        if let Some(req) = self.find(r) {
                            if req.requires.is_some() {
                                r_vec.push(req.id)
                            }
                        }
                        args.push(r);
                    }
                }
            }
        }

        args
    }
}

#[cfg(feature = "yaml")]
impl<'a> From<&'a Yaml> for App<'a> {
    fn from(mut yaml: &'a Yaml) -> Self {
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
                    panic!(
                        "Failed to convert YAML value {:?} to a string",
                        $y[stringify!($i)]
                    );
                }
            };
        }

        yaml_str!(a, yaml, version);
        yaml_str!(a, yaml, author);
        yaml_str!(a, yaml, bin_name);
        yaml_str!(a, yaml, about);
        yaml_str!(a, yaml, before_help);
        yaml_str!(a, yaml, after_help);
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
                        panic!(
                            "Failed to convert YAML value {:?} to either a vec or string",
                            $y[stringify!($as_vec)]
                        );
                    }
                }
                $a
            }};
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
                a = a.subcommand(App::from(sc_yaml));
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

impl<'e> fmt::Display for App<'e> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.name) }
}
