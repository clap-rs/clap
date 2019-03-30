#[allow(dead_code)]
#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Propagation<'a> {
    To(id),
    Full,
    NextLevel,
    None,
}

pub struct Cmd<'help> {
    // Used to identify the Cmd in an efficient manner
    #[doc(hidden)]
    pub id: u64,
    // Used in the Help message title (typically the same as the binary file name used to call
    // the program). This can also be just a title, "My Awesome App" where the binary name is "maa".
    #[doc(hidden)]
    pub name: &'help str,
    // The binary file name used to call this program as overridden by the consumer.
    // Displayed in usage strings and help message.
    #[doc(hidden)]
    pub bin_name: Option<String>,
    // The actual binary file name used to call this program as determined at runtime, OR as
    // overridden by the consumer. Displayed in usage strings and help message.
    #[doc(hidden)]
    pub actual_bin_name: Option<String>,
    // A list of aliases this command could be called by
    #[doc(hidden)]
    pub aliases: Aliases<'help>,
    // Sets a way to manually override the order this App appears in, in the Help message
    #[doc(hidden)]
    pub disp_ord: usize,
    // Settings that change how the args are parsed, or App behaves
    #[doc(hidden)]
    pub settings: CmdFlags,
    // Global settings (i.e. all subcommands)
    #[doc(hidden)]
    pub g_settings: CmdFlags,
    // The list of valid arguments
    #[doc(hidden)]
    pub args: ArgsVec<'help>,
    // A list of valid subcommands
    #[doc(hidden)]
    pub subcommands: Vec<Cmd<'help>>,
    // A list of Arg Groups
    #[doc(hidden)]
    pub groups: Vec<ArgGroup>,
    #[doc(hidden)]
    pub help_msg: HelpMsg<'help>,
    #[doc(hidden)]
    pub version_msg: VersionMsg<'help>,
}

impl<'help> Cmd<'help> {
    /// Creates a new instance of an application requiring a name. The name may be, but doesn't
    /// have to be same as the binary. The name will be displayed to the user when they request to
    /// print version or help and usage information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Cmd, Arg};
    /// let prog = Cmd::new("my_prog")
    /// # ;
    /// ```
    pub fn new<S: AsRef<str>>(n: S) -> Self
    where
        S: 'help,
    {
        let name = n.as_ref();
        Cmd {
            id: hash(name),
            name,
            ..Default::default()
        }
    }

    /// Preallocate `args` number of Arguments
    pub fn with_args<S: AsRef<str>>(n: S, args: usize) -> Self
    where
        S: 'help,
    {
        let name = n.as_ref();
        SubCommand {
            id: hash(name),
            name,
            args: ArgsVec::with_capacity(args),
            ..Default::default()
        }
    }

    /// Preallocate `scs` number of SubCommands
    pub fn with_subcommands<S: AsRef<str>>(n: S, scs: usize) -> Self
    where
        S: 'help,
    {
        let name = n.as_ref();
        SubCommand {
            id: hash(name),
            name,
            subcommands: Vec::with_capacity(scs),
            ..Default::default()
        }
    }

    /// Preallocate `args` number of Arguments and `scs` number of SubCommands (not recursive)
    pub fn with_args_and_subcommands<S: AsRef<str>>(n: S, args: usize, scs: usize) -> Self
    where
        S: 'help,
    {
        let name = n.as_ref();
        SubCommand {
            id: hash(name),
            name,
            args: ArgsVec::with_capacity(args),
            subcommands: Vec::with_capacity(scs),
            ..Default::default()
        }
    }

    /// Get the name of the app
    pub fn get_name(&self) -> &str { &self.name }

    // @TODO @docs make a note of subcommands and how they differ
    /// Get the name of the binary file used to execute this program
    pub fn get_bin_name(&self) -> Option<&str> { self.bin_name.map(|x| &*x) }

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
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///      .author("Me, me@mymain.com")
    /// # ;
    /// ```
    /// [`crate_authors!`]: ./macro.crate_authors!.html
    /// [`examples/`]: https://github.com/kbknapp/clap-rs/tree/master/examples
    pub fn author<S: Into<&'help str>>(mut self, author: S) -> Self {
        self.help_msg.author = Some(author.into());
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
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("My Program")
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
    /// **NOTE:** If only `about` is provided, and not [`SubCommand::long_about`] but the user requests
    /// `--help` clap will still display the contents of `about` appropriately
    ///
    /// **NOTE:** Only [`SubCommand::about`] is used in completion script generation in order to be
    /// concise
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///     .about("Does really amazing things to great people")
    /// # ;
    /// ```
    /// [`SubCommand::long_about`]: ./struct.SubCommand.html#method.long_about
    pub fn about<S: Into<&'help str>>(mut self, about: S) -> Self {
        self.help_msg.about = Some(about.into());
        self
    }

    /// Sets a string describing what the program does. This will be displayed when displaying help
    /// information.
    ///
    /// **NOTE:** If only `long_about` is provided, and not [`SubCommand::about`] but the user requests
    /// `-h` clap will still display the contents of `long_about` appropriately
    ///
    /// **NOTE:** Only [`SubCommand::about`] is used in completion script generation in order to be
    /// concise
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///     .long_about(
    /// "Does really amazing things to great people. Now let's talk a little
    ///  more in depth about how this subcommand really works. It may take about
    ///  a few lines of text, but that's ok!")
    /// # ;
    /// ```
    /// [`SubCommand::about`]: ./struct.SubCommand.html#method.about
    pub fn long_about<S: Into<&'help str>>(mut self, about: S) -> Self {
        self.help_msg.long_about = Some(about.into());
        self
    }

    /// Sets the program's name. This will be displayed when displaying help information.
    ///
    /// **Pro-top:** This function is particularly useful when configuring a program via
    /// [`SubCommand::from_yaml`] in conjunction with the [`crate_name!`] macro to derive the program's
    /// name from its `Cargo.toml`.
    ///
    /// # Examples
    /// ```ignore
    /// # #[macro_use]
    /// # extern crate clap;
    /// # use clap::SubCommand;
    /// # fn main() {
    /// let yml = load_yaml!("app.yml");
    /// let app = SubCommand::from_yaml(yml)
    ///     .name(crate_name!());
    ///
    /// // continued logic goes here, such as `app.get_matches()` etc.
    /// # }
    /// ```
    ///
    /// [`SubCommand::from_yaml`]: ./struct.SubCommand.html#method.from_yaml
    /// [`crate_name!`]: ./macro.crate_name.html
    pub fn name<S: AsRef<str>>(mut self, name: S) -> Self
    where
        S: 'help,
    {
        self.name = name.as_ref();
        self
    }

    /// Adds additional help information to be displayed in addition to auto-generated help. This
    /// information is displayed **after** the auto-generated help information. This is often used
    /// to describe how to use the arguments, or caveats to be noted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::SubCommand;
    /// SubCommand::new("myprog")
    ///     .after_help("Does really amazing things to great people...but be careful with -R")
    /// # ;
    /// ```
    pub fn after_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.help_msg.more_help = Some(help.into());
        self
    }

    /// Adds additional help information to be displayed in addition to auto-generated help. This
    /// information is displayed **before** the auto-generated help information. This is often used
    /// for header information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::SubCommand;
    /// SubCommand::new("myprog")
    ///     .before_help("Some info I'd like to appear before the help info")
    /// # ;
    /// ```
    pub fn before_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.help_msg.pre_help = Some(help.into());
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information with `-V`.
    ///
    /// **NOTE:** If only `version` is provided, and not [`SubCommand::long_version`] but the user
    /// requests `--version` clap will still display the contents of `version` appropriately
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to automatically set your
    /// application's version to the same thing as your crate at compile time. See the [`examples/`]
    /// directory for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///     .version("v0.1.24")
    /// # ;
    /// ```
    /// [`crate_version!`]: ./macro.crate_version!.html
    /// [`examples/`]: https://github.com/kbknapp/clap-rs/tree/master/examples
    /// [`SubCommand::long_version`]: ./struct.SubCommand.html#method.long_version
    pub fn version<S: Into<&'help str>>(mut self, ver: S) -> Self {
        self.help_msg.version = Some(ver.into());
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information with `--version`.
    ///
    /// **NOTE:** If only `long_version` is provided, and not [`SubCommand::version`] but the user
    /// requests `-V` clap will still display the contents of `long_version` appropriately
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to automatically set your
    /// application's version to the same thing as your crate at compile time. See the [`examples/`]
    /// directory for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
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
    /// [`SubCommand::version`]: ./struct.SubCommand.html#method.version
    pub fn long_version<S: Into<&'help str>>(mut self, ver: S) -> Self {
        self.help_msg.long_version = Some(ver.into());
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
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///     .override_usage("myapp [-clDas] <some_file>")
    /// # ;
    /// ```
    /// [`ArgMatches::usage`]: ./struct.ArgMatches.html#method.usage
    pub fn override_usage<S: Into<&'help str>>(mut self, usage: S) -> Self {
        self.help_msg.usage_str = Some(usage.into());
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
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myapp")
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
    pub fn override_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.help_msg.help_str = Some(help.into());
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
    ///   * `{about}`       - General description (from [`SubCommand::about`])
    ///   * `{usage}`       - Automatically generated or given usage string.
    ///   * `{all-args}`    - Help for all arguments (options, flags, positionals arguments,
    ///                       and subcommands) including titles.
    ///   * `{unified}`     - Unified help for options and flags. Note, you must *also* set
    ///                       [`CmdSettings::UnifiedHelpMessage`] to fully merge both options and
    ///                       flags, otherwise the ordering is "best effort"
    ///   * `{flags}`       - Help for flags.
    ///   * `{options}`     - Help for options.
    ///   * `{positionals}` - Help for positionals arguments.
    ///   * `{subcommands}` - Help for subcommands.
    ///   * `{after-help}`  - Help from [`SubCommand::after_help`]
    ///   * `{before-help}`  - Help from [`SubCommand::before_help`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///     .version("1.0")
    ///     .help_template("{bin} ({version}) - {usage}")
    /// # ;
    /// ```
    /// **NOTE:**The template system is, on purpose, very simple. Therefore the tags have to writen
    /// in the lowercase and without spacing.
    /// [`SubCommand::about`]: ./struct.SubCommand.html#method.about
    /// [`SubCommand::after_help`]: ./struct.SubCommand.html#method.after_help
    /// [`SubCommand::before_help`]: ./struct.SubCommand.html#method.before_help
    /// [`CmdSettings::UnifiedHelpMessage`]: ./enum.CmdSettings.html#variant.UnifiedHelpMessage
    pub fn help_template<S: Into<&'help str>>(mut self, s: S) -> Self {
        self.help_msg.template = Some(s.into());
        self
    }

    /// Enables a single command, or [``], level settings.
    ///
    /// See [`CmdSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg, CmdSettings};
    /// SubCommand::new("myprog")
    ///     .setting(CmdSettings::SubcommandRequired)
    ///     .setting(CmdSettings::WaitOnError)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`CmdSettings`]: ./enum.CmdSettings.html
    pub fn setting(mut self, setting: CmdSettings) -> Self {
        self.settings.set(setting);
        self
    }

    /// Disables a single command, or [``], level setting.
    ///
    /// See [`CmdSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, CmdSettings};
    /// SubCommand::new("myprog")
    ///     .unset_setting(CmdSettings::ColorAuto)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`CmdSettings`]: ./enum.CmdSettings.html
    /// [global]: ./struct.SubCommand.html#method.global_setting
    pub fn unset_setting(mut self, setting: CmdSettings) -> Self {
        self.settings.unset(setting);
        self.g_settings.unset(setting);
        self
    }

    /// Enables a single setting that is propagated down through all child subcommands.
    ///
    /// See [`CmdSettings`] for a full list of possibilities and examples.
    ///
    /// **NOTE**: The setting is *only* propagated *down* and not up through parent commands.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg, CmdSettings};
    /// SubCommand::new("myprog")
    ///     .global_setting(CmdSettings::SubcommandRequired)
    /// # ;
    /// ```
    /// [`CmdSettings`]: ./enum.CmdSettings.html
    pub fn global_setting(mut self, setting: CmdSettings) -> Self {
        self.settings.set(setting);
        self.g_settings.set(setting);
        self
    }

    /// Disables a global setting, and stops propagating down to child subcommands.
    ///
    /// See [`CmdSettings`] for a full list of possibilities and examples.
    ///
    /// **NOTE:** The setting being unset will be unset from both local and [global] settings
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, CmdSettings};
    /// SubCommand::new("myprog")
    ///     .unset_global_setting(CmdSettings::ColorAuto)
    /// # ;
    /// ```
    /// [`CmdSettings`]: ./enum.CmdSettings.html
    /// [global]: ./struct.Cmd.html#method.global_setting
    pub fn unset_global_setting(mut self, setting: CmdSettings) -> Self {
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
    /// # use clap::SubCommand;
    /// SubCommand::new("myprog")
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
    /// # use clap::SubCommand;
    /// SubCommand::new("myprog")
    ///     .max_term_width(100)
    /// # ;
    /// ```
    pub fn max_term_width(mut self, w: usize) -> Self {
        self.term.max_width = Some(w);
        self
    }

    /// Adds an [argument] to the list of valid possibilities.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///     // Adding a single "flag" argument with a short and help text, using Arg::new()
    ///     .arg(
    ///         Arg::new("debug")
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
    pub fn arg<A: Into<Arg<'help>>>(mut self, a: A) -> Self {
        // @TODO @perf @p1 perhaps use a heading index instead of the actual heading itself
        let help_heading: Option<&'help str> =
            if let Some(option_str) = self.help_msg.help_headings.last() {
                *option_str
            } else {
                None
            };
        let arg = a.into().help_heading(help_heading);
        self.args.push(arg);
        self
    }

    // @TODO @docs link between methods
    /// Set a custom section heading for future args. Every call to arg will
    /// have this header (instead of its default header) until a subsequent
    /// call to `SubCommand::help_heading` or `SubCommand::stop_help_heading`
    pub fn help_heading(mut self, heading: &'help str) -> Self {
        self.help_msg.help_headings.push(Some(heading));
        self
    }

    /// Stop using custom section headings for future args and move back to their default headings.
    pub fn stop_custom_headings(mut self) -> Self {
        self.help_headings.push(None);
        self
    }

    /// Adds multiple [arguments] to the list of valid possibilties
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg};
    /// SubCommand::new("myprog")
    ///     .args(&[
    ///         Arg::from("[debug] -d 'turns on debugging info'"),
    ///         Arg::new("input").index(1).help("the input file to use")
    ///     ])
    /// # ;
    /// ```
    /// [arguments]: ./struct.Arg.html
    pub fn args<I, T>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item=T>,
        T: Into<Arg<'help>>,
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
    /// # use clap::{SubCommand, Arg, };
    /// let m = SubCommand::new("myprog")
    ///             .subcommand(SubCommand::new("test")
    ///                 .alias("do-stuff"))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
    pub fn alias<S: AsRef<&'help str>>(mut self, name: S) -> Self {
        self.aliases.add_hidden(name);
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
    /// # use clap::{SubCommand, Arg, };
    /// let m = SubCommand::new("myprog")
    ///             .subcommand(SubCommand::new("test")
    ///                 .aliases(&["do-stuff", "do-tests", "tests"]))
    ///                 .arg(Arg::new("input")
    ///                             .help("the file to add")
    ///                             .index(1)
    ///                             .required(false))
    ///             .get_matches_from(vec!["myprog", "do-tests"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
    pub fn aliases(mut self, names: &[&'help str]) -> Self {
        for &n in names {
            self.aliases.add_hidden(n);
        }
        self
    }

    /// Allows adding a [``] alias that functions exactly like those defined with
    /// [`SubCommand::alias`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg, };
    /// let m = SubCommand::new("myprog")
    ///             .subcommand(SubCommand::new("test")
    ///                 .visible_alias("do-stuff"))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
    /// [`SubCommand::alias`]: ./struct.SubCommand.html#method.alias
    pub fn visible_alias<S: Into<&'help str>>(mut self, name: S) -> Self {
        self.aliases.add_visible(name);
        self
    }

    /// Allows adding multiple [``] aliases that functions exactly like those defined
    /// with [`SubCommand::aliases`], except that they are visible inside the help message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg, };
    /// let m = SubCommand::new("myprog")
    ///             .subcommand(SubCommand::new("test")
    ///                 .visible_aliases(&["do-stuff", "tests"]))
    ///             .get_matches_from(vec!["myprog", "do-stuff"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [``]: ./struct..html
    /// [`SubCommand::aliases`]: ./struct.SubCommand.html#method.aliases
    pub fn visible_aliases(mut self, names: &[&'help str]) -> Self {
        for &n in names {
            self.aliases.add_visible(n);
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
    /// # use clap::{SubCommand, ArgGroup};
    /// SubCommand::new("app")
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
    pub fn group(mut self, group: ArgGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Adds multiple [`ArgGroup`]s to the [`SubCommand`] at once.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, ArgGroup};
    /// SubCommand::new("app")
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
    /// [`SubCommand`]: ./struct.SubCommand.html
    pub fn groups(mut self, groups: &[ArgGroup]) -> Self {
        for g in groups {
            self = self.group(g.into());
        }
        self
    }

    /// Adds a [``] to the list of valid possibilities. Subcommands are effectively
    /// sub-[`SubCommand`]s, because they can contain their own arguments, subcommands, version, usage,
    /// etc. They also function just like [`SubCommand`]s, in that they get their own auto generated help,
    /// version, and usage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{SubCommand, Arg, };
    /// SubCommand::new("myprog")
    ///     .subcommand(SubCommand::new("config")
    ///         .about("Controls configuration features")
    ///         .arg("<config> 'Required configuration file to use'"))
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`SubCommand`]: ./struct.SubCommand.html
    pub fn subcommand(mut self, subcmd: SubCommand<'help>) -> Self {
        self.subcommands.push(subcmd);
        self
    }

    /// Adds multiple subcommands to the list of valid possibilities by iterating over an
    /// [`IntoIterator`] of [``]s
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{SubCommand, Arg, };
    /// # SubCommand::new("myprog")
    /// .subcommands( vec![
    ///        SubCommand::new("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::new("config_file").index(1)),
    ///        SubCommand::new("debug").about("Controls debug functionality")])
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
    pub fn subcommands<I>(mut self, subcmds: I) -> Self
    where
        I: IntoIterator<Item=SubCommand<'help>>,
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
    /// # use clap::{SubCommand, };
    /// let m = SubCommand::new("cust-ord")
    ///     .subcommand(SubCommand::new("alpha") // typically subcommands are grouped
    ///                                                // alphabetically by name. Subcommands
    ///                                                // without a display_order have a value of
    ///                                                // 999 and are displayed alphabetically with
    ///                                                // all other 999 subcommands
    ///         .about("Some help and text"))
    ///     .subcommand(SubCommand::new("beta")
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
        self.help_msg.disp_ord = ord;
        self
    }

    /// Allows one to mutate an [`Arg`] after it's been added to an `SubCommand`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{SubCommand, Arg};
    ///
    /// let mut app = SubCommand::new("foo")
    ///     .arg(Arg::new("bar")
    ///         .short('h'))
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
    pub fn mut_arg<T, F>(mut self, arg: T, f: F) -> Self
    where
        F: FnOnce(Arg<'help>) -> Arg<'help>,
        T: Hash,
    {
        let a = self.args.remove(hash(arg)).unwrap_or_else(|| Arg::new(arg));
        self.args.push(f(a));

        self
    }

    // Sets the actual binary name as determined by the first cmd line arg
    pub(crate) fn _actual_bin_name<S: AsRef<OsStr>>(&mut self, name: S) {
        let bn_os = name.as_ref();
        let p = Path::new(&*bn_os);
        if let Some(f) = p.file_name() {
            if let Some(s) = f.to_os_string().to_str() {
                if self.actual_bin_name.is_none() {
                    self.actual_bin_name = Some(s.to_owned());
                }
            }
        }
    }
}

// Preparing for Parsing
#[doc(hidden)]
impl<'help> Cmd<'help> {
    // @TODO re-evaluate use in clap_generate (https://github.com/clap-rs/clap_generate)
    // used as Cmd::_build
    pub fn _build(&mut self, prop: Propagation) {
        debugln!("Cmd::build;");
        // Make sure all the globally set flags apply to us as well
        self.settings = self.settings | self.g_settings;

        // Depending on if DeriveDisplayOrder is set or not, we need to determine when we build
        // the help and version flags, otherwise help message orders get screwed up
        if self.is_set(CmdSettings::DeriveDisplayOrder) {
            self.derive_display_order();
        }
        self.create_help_and_version();
        self.propagate(prop);

        // Perform expensive debug assertions
        debug_assert!({
            self.args.args().for_each(|a| self.arg_debug_asserts(a));
            true
        });

        for a in self.args_mut() {
            // Figure out implied settings
            if a.is_set(ArgSettings::Last) {
                // if an arg has `Last` set, we need to imply DontCollapseArgsInUsage so that args
                // in the usage string don't get confused or left out.
                self.a.set(CmdSettings::DontCollapseArgsInUsage);
                self.a.set(CmdSettings::ContainsLast);
            }
            a.build();
        }

        self.app_debug_asserts();

        self.args.build();

        self.set(CmdSettings::Propagated);

        self.positional_asserts();
    }

    fn positional_asserts(&mut self) {
        let positionals: Vec<(u64, &Arg)> = self
            .args
            .positionals()
            .map(|x| (x.index.unwrap(), x))
            .collect();

        if positionals.is_empty() {
            return;
        }

        positional_asserts::assert_highest_index_matches_len(&*positionals);

        if positionals
            .iter()
            .filter(|(_, p)| p.is_set(ArgSettings::MultipleValues))
            .count()
            > 1
        {
            assert_low_index_multiples(&*positionals);
            self.set(AS::LowIndexMultiplePositional);
        }

        if !self.is_set(AS::AllowMissingPositional) {
            assert_missing_positionals(&*positionals);
        }

        positional_asserts::assert_only_one_last(&*positionals);

        positional_asserts::assert_required_last_and_subcommands(
            &*positionals,
            self.has_subcommands(),
            self.is_set(AS::SubcommandsNegateReqs),
        );
    }

    // Perform some expensive assertions on the Parser itself
    fn app_debug_asserts(&mut self) {
        debugln!("Cmd::_app_debug_asserts;");
        debug_assert!(
            self.args
                .args()
                .map(|x| x.id)
                .map(|x| self.a.args().map(|x| x.id).filter(|y| x == y).count())
                .any(|x| x > 1),
            "Arg names must be unique"
        );
    }

    pub fn propagate(&mut self, prop: Propagation) {
        debugln!("Cmd::propagate:{}", self.name);
        match prop {
            Propagation::None => {
                return;
            }
            Propagation::Full | Propagation::NextLevel => {
                for mut sc in &mut self.subcommands {
                    // We have to create a new scope in order to tell rustc the borrow of `sc` is
                    // done and to recursively call this method
                    {
                        let vsc = self.a.is_set(CmdSettings::VersionlessSubcommands);
                        let gv = self.a.is_set(CmdSettings::GlobalVersion);

                        if vsc {
                            sc.set(CmdSettings::DisableVersion);
                        }
                        if gv && sc.help_msg.version.is_none() && self.help_msg.version.is_some() {
                            sc.set(CmdSettings::GlobalVersion);
                            sc.help_msg.version = Some(self.help_msg.version.unwrap());
                        }
                        sc.settings = sc.settings | self.g_settings;
                        sc.g_settings = sc.g_settings | self.g_settings;
                        sc.term.width = self.term.width;
                        sc.term.max_width = self.term.max_width;
                    }
                    {
                        for a in self.global_args().cloned() {
                            sc.args.push(a);
                        }
                        if prop == Propagation::Full {
                            sc._build(Propagation::Full);
                        }
                    }
                }
            }
            Propagation::To(id) => unimplemented!(),
        }
    }

    pub(crate) fn create_help_and_version(&mut self) {
        debugln!("Cmd::_create_help_and_version;");
        // @TODO @perf hardcode common hashes?
        if !(self.args.find("help").is_some() || self.args.get_by_id(HELP_HASH).is_some()) {
            debugln!("Cmd::_create_help_and_version: Building --help");
            let mut help = Arg::new("help")
                .long("help")
                .help("Prints help information");
            if !self.args.args().any(|x| x.short == Some('h')) {
                help = help.short('h');
            }

            self.args.push(help);
        }
        if !((self.args.get_by_long("version").is_some()
            || self.args.get_by_id(VERSION_HASH).is_some())
            || self.is_set(CmdSettings::DisableVersion))
        {
            debugln!("Cmd::_create_help_and_version: Building --version");
            let mut version = Arg::new("version")
                .long("version")
                .help("Prints version information");
            if !self.args.args().any(|x| x.short == Some('V')) {
                version = version.short('V');
            }

            self.args.push(version);
        }
        if self.has_subcommands()
            && !self.is_set(CmdSettings::DisableHelpSubcommand)
            && !self.subcommands.iter().any(|s| s.id == HELP_HASH)
        // hardcode common hashes?
        {
            debugln!("Cmd::_create_help_and_version: Building help");
            self.subcommands.push(
                Cmd::new("help")
                    .about("Prints this message or the help of the given subcommand(s)"),
            );
        }
    }

    pub(crate) fn derive_display_order(&mut self) {
        debugln!("Cmd::derive_display_order:{}", self.name);
        self.args
            .args_mut()
            .filter(|a| a.has_switch())
            .filter(|a| a.disp_ord == 999)
            .enumerate()
            .for_each(|(i, mut x)| x.disp_ord = i);

        self.subcommands
            .iter_mut()
            .enumerate()
            .filter(|&(_, sc)| sc.disp_ord == 999)
            .for_each(|(i, mut sc)| sc.disp_ord = i);

        for sc in self.subcommands.iter_mut() {
            sc._derive_display_order();
        }
    }

    // Perform expensive assertions on the Arg instance
    fn arg_debug_asserts(&self, a: &Arg) -> bool {
        debugln!("Cmd::arg_debug_asserts:{}", a.name);

        // Long conflicts
        for l in a.longs() {
            assert!(
                self.args.args().filter(|x| x.uses_long(l)).count() < 2,
                "Argument long must be unique\n\n\t--{} is already in use",
                l
            );
        }

        // Short conflicts
        if let Some(s) = a.get_short() {
            assert!(
                self.args.args().filter(|x| x.uses_short(s)).count() < 2,
                "Argument short must be unique\n\n\t-{} is already in use",
                s
            );
        }

        if let Some(idx) = a.get_position() {
            // No index conflicts
            assert!(
                self.args
                    .positionals()
                    .filter(|x| x.uses_position(idx))
                    .count()
                    < 2,
                "Argument '{}' has the same index as another positional \
                 argument\n\n\tUse Arg::setting(ArgSettings::MultipleValues) to allow one \
                 positional argument to take multiple values",
                a.id
            );
        }
        if a.is_set(ArgSettings::Last) {
            assert!(
                a.has_switch(),
                "Flags or Options may not have last(true) set. {} has either a long or short and \
                 last(true) set.",
                a.id
            );
        }
        assert!(
            !(a.is_set(ArgSettings::Required) && a.is_set(ArgSettings::Global)),
            "Global arguments cannot be required.\n\n\t'{}' is marked as \
             global and required",
            a.id
        );

        true
    }

    // used in clap_generate (https://github.com/clap-rs/clap_generate)
    #[doc(hidden)]
    pub fn build_bin_names(&mut self) {
        debugln!("Cmd::_build_bin_names;");
        for sc in &mut self.subcommands {
            debug!("Parser::build_bin_names:iter: bin_name set...");
            if sc.bin_name.is_none() {
                sdebugln!("No");
                let bin_name = format!(
                    "{}{}{}",
                    self.bin_name.as_ref().unwrap_or(&self.name.into()),
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

    pub(crate) fn write_version<W: Write>(&self, w: &mut W, use_long: bool) -> io::Result<()> {
        debugln!("Cmd::_write_version;");
        let ver = if use_long {
            self.help_msg
                .long_version
                .unwrap_or_else(|| self.help_msg.version.unwrap_or(""))
        } else {
            self.help_msg
                .version
                .unwrap_or_else(|| self.help_msg.long_version.unwrap_or(""))
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

    pub(crate) fn format_group(&self, g: u64) -> String {
        let g_string = self
            .unroll_args_in_group(g)
            .iter()
            .filter_map(|x| self.find(*x))
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("|");
        format!("<{}>", &*g_string)
    }

    fn handle_help_subcommand<I, T>(&mut self, it: &mut I) -> ClapResult<ParseCtx>
    where
        I: Iterator<Item=T>,
        T: Into<OsString>,
    {
        debugln!("Parser::parse_help_subcommand;");
        let cmds: Vec<OsString> = it.map(|c| c.into()).collect();
        let mut help_help = false;
        let mut bin_name = self.bin_name.as_ref().unwrap_or(&self.name.into()).clone();
        let mut sc = {
            // @TODO @perf: cloning all these Cmds ins't great, but since it's just displaying the
            // help message there are bigger fish to fry
            let mut sc = self.clone();
            for (i, cmd) in cmds.iter().enumerate() {
                if &*cmd.to_string_lossy() == "help" {
                    // cmd help help
                    help_help = true;
                    break; // Maybe?
                }
                if let Some(mut c) = sc.subcommands.iter().cloned().find(|x| x.name == cmd) {
                    c._build(Propagation::NextLevel);
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else if let Some(mut c) = sc.subcommands.iter().cloned().find(|x| x.name == &*cmd)
                {
                    c._build(Propagation::NextLevel);
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        cmd.to_string_lossy().into_owned(),
                        self.bin_name.as_ref().unwrap_or(&app.name.into()),
                        self.color(),
                    ));
                }
                bin_name = format!("{} {}", bin_name, &*sc.name);
            }
            sc
        };
        if help_help {
            let mut pb = Arg::new("subcommand")
                .index(1)
                .setting(ArgSettings::MultipleValues)
                .help("The subcommand whose help message to display");
            pb._build();
        }
        if self.bin_name != self.bin_name {
            self.bin_name = Some(format!("{} {}", bin_name, self.name));
        }
        Err(self.help_err(false))
    }
}

// Internal use only
#[doc(hidden)]
impl<'help> Cmd<'help> {
    pub(crate) fn help_err(&self, mut use_long: bool) -> ClapError {
        debugln!(
            "Cmd::help_err: use_long={:?}",
            use_long && self.use_long_help()
        );
        use_long = use_long && self.use_long_help();
        let mut buf = vec![];
        match Help::new(&mut buf, self, use_long, false).write_help() {
            Err(e) => e,
            _ => ClapError {
                message: String::from_utf8(buf).unwrap_or_default(),
                kind: ErrorKind::HelpDisplayed,
                info: None,
            },
        }
    }

    // Prints the version to the user and exits if quit=true
    pub(crate) fn print_version<W: Write>(&self, w: &mut W, use_long: bool) -> ClapResult<()> {
        self.._write_version(w, use_long)?;
        w.flush().map_err(ClapError::from)
    }

    pub(crate) fn write_help_err<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        Help::new(w, self, false, true).write_help()
    }

    pub(crate) fn version_err(&self, use_long: bool) -> ClapError {
        debugln!("Parser::version_err: ");
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        match self.print_version(&mut buf_w, use_long) {
            Err(e) => e,
            _ => ClapError {
                message: String::new(),
                kind: ErrorKind::VersionDisplayed,
                info: None,
            },
        }
    }

    pub(crate) fn find(&self, id: u64) -> Option<&Arg<'help>> { self.args.get_by_id(id) }

    // Should we color the output? None=determined by output location, true=yes, false=no
    #[doc(hidden)]
    pub fn color(&self) -> ColorWhen {
        debugln!("Cmd::color;");
        debug!("Cmd::color: Color setting...");
        if self.is_set(CmdSettings::ColorNever) {
            sdebugln!("Never");
            ColorWhen::Never
        } else if self.is_set(CmdSettings::ColorAlways) {
            sdebugln!("Always");
            ColorWhen::Always
        } else {
            sdebugln!("Auto");
            ColorWhen::Auto
        }
    }

    pub fn is_set(&self, s: CmdSettings) -> bool {
        self.settings.is_set(s) || self.g_settings.is_set(s)
    }

    pub fn set(&mut self, s: CmdSettings) { self.settings.set(s) }

    pub fn set_global(&mut self, s: CmdSettings) { self.g_settings.set(s) }

    pub fn unset_global(&mut self, s: CmdSettings) { self.g_settings.unset(s) }

    pub fn unset(&mut self, s: CmdSettings) { self.settings.unset(s) }

    pub fn has_subcommands(&self) -> bool { !self.subcommands.is_empty() }

    pub(crate) fn unroll_args_in_group(&self, group: u64) -> Vec<u64> {
        let mut g_vec = vec![group];
        let mut args = vec![];

        while let Some(ref g) = g_vec.pop() {
            for n in self
                .groups
                .iter()
                .find(|grp| &grp.id == g)
                .expect(INTERNAL_ERROR_MSG)
                .args
                .iter()
            {
                if !args.contains(n) {
                    if self.args.find(*n).is_some() {
                        args.push(*n)
                    } else {
                        g_vec.push(*n);
                    }
                }
            }
        }

        args
    }

    pub(crate) fn unroll_requirements_for_arg(&self, arg: u64, matcher: &ArgMatcher) -> Vec<u64> {
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

        while let Some(ref a) = r_vec.pop() {
            if let Some(arg) = self.find(*a) {
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

    pub(crate) fn use_long_help(&self) -> bool {
        debugln!("Cmd::use_long_help;");
        // In this case, both must be checked. This allows the retention of
        // original formatting, but also ensures that the actual -h or --help
        // specified by the user is sent through. If HiddenShortHelp is not included,
        // then items specified with hidden_short_help will also be hidden.
        let should_long = |v: &Arg| {
            v.long_help.is_some()
                || v.is_set(ArgSettings::HiddenLongHelp)
                || v.is_set(ArgSettings::HiddenShortHelp)
        };

        self.long_about.is_some()
            || self.args.args.iter().any(|f| should_long(&f))
            || self.subcommands.iter().any(|s| s.long_about.is_some())
    }
}

// Facade for ArgsVec
#[doc(hidden)]
impl<'help> Cmd<'help> {
    pub fn args(&self) -> Args<'help> { self.args.args() }
    pub fn args_mut(&mut self) -> ArgsMut<'help> { self.args.args_mut() }
    pub fn flags(&self) -> Flags<'help> { self.args.flags() }
    pub fn flags_mut(&mut self) -> FlagsMut<'help> { self.args.flags_mut() }
    pub fn options(&self) -> Options<'help> { self.args.args() }
    pub fn options_mut(&mut self) -> OptionsMut<'help> { self.args.args() }
    pub fn positionals(&self) -> Positionals<'help> { self.args.positionals() }
    pub fn positionals_mut(&mut self) -> PositionalsMut<'help> { self.args.positionals_mut() }
    pub fn global_args(&self) -> impl Iterator<Item=&Arg<'help>> {
        self.args().filter(|x| x.is_set(ArgSettings::Global))
    }
    pub fn global_args_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> {
        self.args_mut().filter(|x| x.is_set(ArgSettings::Global))
    }
}

#[cfg(feature = "yaml")]
impl<'a> From<&'a Yaml> for Cmd<'a> {
    fn from(mut yaml: &'a Yaml) -> Self {
        // We WANT this to panic on error...so expect() is good.
        let mut is_sc = None;
        let mut a = if let Some(name) = yaml["name"].as_str() {
            Cmd::new(name)
        } else {
            let yaml_hash = yaml.as_hash().unwrap();
            let sc_key = yaml_hash.keys().nth(0).unwrap();
            is_sc = Some(yaml_hash.get(sc_key).unwrap());
            Cmd::new(sc_key.as_str().unwrap())
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
        yaml_str!(a, yaml, template);
        yaml_str!(a, yaml, usage);
        yaml_str!(a, yaml, help);
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
            a = a.setting(v.parse().expect("unknown CmdSetting found in YAML file"));
        } else if yaml["setting"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to an CmdSetting",
                yaml["setting"]
            );
        }
        if let Some(v) = yaml["settings"].as_vec() {
            for ys in v {
                if let Some(s) = ys.as_str() {
                    a = a.setting(s.parse().expect("unknown CmdSetting found in YAML file"));
                }
            }
        } else if let Some(v) = yaml["settings"].as_str() {
            a = a.setting(v.parse().expect("unknown CmdSetting found in YAML file"));
        } else if yaml["settings"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to a string",
                yaml["settings"]
            );
        }
        if let Some(v) = yaml["global_setting"].as_str() {
            a = a.setting(v.parse().expect("unknown CmdSetting found in YAML file"));
        } else if yaml["global_setting"] != Yaml::BadValue {
            panic!(
                "Failed to convert YAML value {:?} to an CmdSetting",
                yaml["setting"]
            );
        }
        if let Some(v) = yaml["global_settings"].as_vec() {
            for ys in v {
                if let Some(s) = ys.as_str() {
                    a = a.global_setting(s.parse().expect("unknown CmdSetting found in YAML file"));
                }
            }
        } else if let Some(v) = yaml["global_settings"].as_str() {
            a = a.global_setting(v.parse().expect("unknown CmdSetting found in YAML file"));
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
                a = a.subcommand(::from_yaml(sc_yaml));
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

impl<'help> fmt::Display for Cmd<'help> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.name) }
}
