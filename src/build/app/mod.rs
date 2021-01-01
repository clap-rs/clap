#[cfg(debug_assertions)]
mod debug_asserts;
mod settings;
#[cfg(test)]
mod tests;

pub use self::settings::AppSettings;

// Std
use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fmt,
    io::{self, BufRead, Write},
    ops::Index,
    path::Path,
};

// Third Party
#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

// Internal
use crate::{
    build::{app::settings::AppFlags, Arg, ArgGroup, ArgSettings},
    mkeymap::MKeyMap,
    output::{fmt::Colorizer, Help, HelpWriter, Usage},
    parse::{ArgMatcher, ArgMatches, Input, Parser},
    util::{safe_exit, termcolor::ColorChoice, ArgStr, Id, Key},
    Result as ClapResult, INTERNAL_ERROR_MSG,
};

/// Represents a command line interface which is made up of all possible
/// command line arguments and subcommands. Interface arguments and settings are
/// configured using the "builder pattern." Once all configuration is complete,
/// the [`App::get_matches`] family of methods starts the runtime-parsing
/// process. These methods then return information about the user supplied
/// arguments (or lack thereof).
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
    pub(crate) license: Option<&'help str>,
    pub(crate) about: Option<&'help str>,
    pub(crate) long_about: Option<&'help str>,
    pub(crate) help_about: Option<&'help str>,
    pub(crate) version_about: Option<&'help str>,
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
    pub(crate) disp_ord: usize,
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
    pub(crate) subcommand_placeholder: Option<&'help str>,
    pub(crate) subcommand_header: Option<&'help str>,
}

impl<'help> App<'help> {
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
    pub fn get_long_flag(&self) -> Option<&str> {
        self.long_flag
    }

    /// Get the name of the binary.
    #[inline]
    pub fn get_bin_name(&self) -> Option<&str> {
        self.bin_name.as_deref()
    }

    /// Set binary name. Uses `&mut self` instead of `self`.
    pub fn set_bin_name<S: Into<String>>(&mut self, name: S) {
        self.bin_name = Some(name.into());
    }

    /// Get the help message specified via [`App::about`].
    ///
    /// [`App::about`]: ./struct.App.html#method.about
    #[inline]
    pub fn get_about(&self) -> Option<&str> {
        self.about.as_deref()
    }

    /// Iterate through the *visible* aliases for this subcommand.
    #[inline]
    pub fn get_visible_aliases(&self) -> impl Iterator<Item = &str> {
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
    pub fn get_all_aliases(&self) -> impl Iterator<Item = &str> {
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

    /// Iterate through the set of arguments.
    #[inline]
    pub fn get_arguments(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.args.args()
    }

    /// Get the list of *positional* arguments.
    #[inline]
    pub fn get_positionals(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_arguments().filter(|a| a.is_positional())
    }

    /// Iterate through the *flags*.
    pub fn get_flags(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_arguments()
            .filter(|a| !a.is_set(ArgSettings::TakesValue) && a.get_index().is_none())
    }

    /// Iterate through the *options*.
    pub fn get_opts(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_arguments()
            .filter(|a| a.is_set(ArgSettings::TakesValue) && a.get_index().is_none())
    }

    /// Iterate through the *flags* that don't have custom heading.
    pub fn get_flags_with_no_heading(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_flags().filter(|a| a.get_help_heading().is_none())
    }

    /// Iterate through the *options* that don't have custom heading.
    pub fn get_opts_with_no_heading(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.get_opts().filter(|a| a.get_help_heading().is_none())
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
        if arg.global {
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

    /// Returns `true` if the given [`AppSettings`] variant is currently set in
    /// this `App` (checks both [local] and [global settings]).
    ///
    /// [`AppSettings`]: ./enum.AppSettings.html
    /// [local]: ./struct.App.html#method.setting
    /// [global settings]: ./struct.App.html#method.global_setting
    #[inline]
    pub fn is_set(&self, s: AppSettings) -> bool {
        self.settings.is_set(s) || self.g_settings.is_set(s)
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
}

impl<'help> App<'help> {
    /// Creates a new instance of an `App` requiring a `name`.
    ///
    /// It is common, but not required, to use binary name as the `name`. This
    /// name will only be displayed to the user when they request to print
    /// version or help and usage information.
    ///
    /// An `App` represents a command line interface (CLI) which is made up of
    /// all possible command line arguments and subcommands. "Subcommands" are
    /// sub-CLIs with their own arguments, settings, and even subcommands
    /// forming a sort of hierarchy.
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
            disp_ord: 999,
            ..Default::default()
        }
    }

    /// Sets a string of author(s) that will be displayed to the user when they
    /// request the help message.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_authors!`] to
    /// automatically set your application's author(s) to the same thing as your
    /// crate at compile time.
    ///
    /// See the [`examples/`] directory for more information.
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
    /// [`examples/`]: https://github.com/clap-rs/clap/tree/master/examples
    pub fn author<S: Into<&'help str>>(mut self, author: S) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Overrides the runtime-determined name of the binary. This should only be
    /// used when absolutely necessary, such as when the binary name for your
    /// application is misleading, or perhaps *not* how the user should invoke
    /// your program.
    ///
    /// Normally, the binary name is used in help and error messages. `clap`
    /// automatically determines the binary name at runtime, however by manually
    /// setting the binary name, one can effectively override what will be
    /// displayed in the help or error messages.
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
    pub fn bin_name<S: Into<String>>(mut self, name: S) -> Self {
        self.bin_name = Some(name.into());
        self
    }

    /// Sets a string describing what the program does. This will be displayed
    /// when the user requests the short format help message (`-h`).
    ///
    /// `clap` can display two different help messages, a [long format] and a
    /// [short format] depending on whether the user used `-h` (short) or
    /// `--help` (long). This method sets the message during the short format
    /// (`-h`) message. However, if no long format message is configured, this
    /// message will be displayed for *both* the long format, or short format
    /// help message.
    ///
    /// **NOTE:** Only [`App::about`] (short format) is used in completion
    /// script generation in order to be concise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .about("Does really amazing things for great people")
    /// # ;
    /// ```
    /// [long format]: ./struct.App.html#method.long_about
    /// [short format]: ./struct.App.html#method.about
    /// [`App::about`]: ./struct.App.html#method.about
    pub fn about<S: Into<&'help str>>(mut self, about: S) -> Self {
        self.about = Some(about.into());
        self
    }

    /// Sets a long format string describing what the program does. This will be
    /// displayed when the user requests the long format help message (`--help`).
    ///
    /// ## Advanced
    ///
    /// `clap` can display two different help messages, a [long format] and a
    /// [short format] depending on whether the user used `-h` (short) or
    /// `--help` (long). This method sets the message during the long format
    /// (`--help`) message. However, if no short format message is configured,
    /// this message will be displayed for *both* the long format, or short
    /// format help message.
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
    /// [long format]: ./struct.App.html#method.long_about
    /// [short format]: ./struct.App.html#method.about
    /// [`App::about`]: ./struct.App.html#method.about
    pub fn long_about<S: Into<&'help str>>(mut self, about: S) -> Self {
        self.long_about = Some(about.into());
        self
    }

    /// Sets the help text for the auto-generated help argument and subcommand.
    ///
    /// By default clap sets this to "Prints help information" for the help
    /// argument and "Prints this message or the help of the given subcommand(s)"
    /// for the help subcommand but if you're using a different convention
    /// for your help messages and would prefer a different phrasing you can
    /// override it.
    ///
    /// **NOTE:** This setting propagates to subcommands.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .help_about("Print help information") // Imperative tone
    /// # ;
    /// ```
    pub fn help_about<S: Into<&'help str>>(mut self, help_about: S) -> Self {
        self.help_about = Some(help_about.into());
        self
    }

    /// Sets the help text for the auto-generated version argument.
    ///
    /// By default clap sets this to "Prints version information" but if
    /// you're using a different convention for your help messages and
    /// would prefer a different phrasing you can override it.
    ///
    /// **NOTE:** This setting propagates to subcommands.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .version_about("Print version information") // Imperative tone
    /// # ;
    /// ```
    pub fn version_about<S: Into<&'help str>>(mut self, version_about: S) -> Self {
        self.version_about = Some(version_about.into());
        self
    }

    /// (Re)Sets the program's name. This will be displayed when displaying help
    /// or version messages.
    ///
    /// **Pro-tip:** This function is particularly useful when configuring a
    /// program via `App::from(yaml)` in conjunction with the [`crate_name!`]
    /// macro to derive the program's name from its `Cargo.toml`.
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
    ///
    /// [`crate_name!`]: ./macro.crate_name.html
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Adds additional help information to be displayed at the end of the
    /// auto-generated help. This is often used to describe how to use the
    /// arguments, caveats to be noted, or license and contact information.
    ///
    /// **NOTE:** If only `after_long_help` is provided, and not [`App::after_help`] but the user requests
    /// `-h` clap will still display the contents of `after_help` appropriately.
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
    /// [`App::after_help`]: ./struct.App.html#method.after_help
    pub fn after_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.after_help = Some(help.into());
        self
    }

    /// Adds additional help information to be displayed in addition to auto-generated help. This
    /// information is displayed **after** the auto-generated help information and is meant to be
    /// more verbose than `after_help`. This is often used to describe how to use the arguments, or
    /// caveats to be noted in man pages.
    ///
    /// **NOTE:** If only `after_help` is provided, and not [`App::after_long_help`] but the user
    /// requests `--help`, clap will still display the contents of `after_help` appropriately.
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
    /// [`App::after_long_help`]: ./struct.App.html#method.after_long_help
    pub fn after_long_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.after_long_help = Some(help.into());
        self
    }

    /// Adds additional help information to be displayed prior to the
    /// auto-generated help. This is often used for header, copyright, or
    /// license information.
    ///
    /// **NOTE:** If only `before_long_help` is provided, and not [`App::before_help`] but the user
    /// requests `-h` clap will still display the contents of `before_long_help` appropriately.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .before_help("Some info I'd like to appear before the help info")
    /// # ;
    /// ```
    /// [`App::before_help`]: ./struct.App.html#method.before_help
    pub fn before_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.before_help = Some(help.into());
        self
    }

    /// Adds additional help information to be displayed prior to the
    /// auto-generated help. This is often used for header, copyright, or
    /// license information.
    ///
    /// **NOTE:** If only `before_help` is provided, and not [`App::before_long_help`] but the user
    /// requests `--help`, clap will still display the contents of `before_help` appropriately.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .before_long_help("Some verbose and long info I'd like to appear before the help info")
    /// # ;
    /// ```
    /// [`App::before_long_help`]: ./struct.App.html#method.before_long_help
    pub fn before_long_help<S: Into<&'help str>>(mut self, help: S) -> Self {
        self.before_long_help = Some(help.into());
        self
    }

    /// Allows the subcommand to be used as if it were an [`Arg::short`].
    ///
    /// Sets the short version of the subcommand flag without the preceding `-`.
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
    ///                 .about("search remote repositories for matching strings"),
    ///         ),
    ///     )
    ///     .get_matches_from(vec!["pacman", "-Ss"]);
    ///
    /// assert_eq!(matches.subcommand_name().unwrap(), "sync");
    /// let sync_matches = matches.subcommand_matches("sync").unwrap();
    /// assert!(sync_matches.is_present("search"));
    /// ```
    /// [`Arg::short`]: ./struct.Arg.html#method.short
    pub fn short_flag(mut self, short: char) -> Self {
        self.short_flag = Some(short);
        self
    }

    /// Allows the subcommand to be used as if it were an [`Arg::long`].
    ///
    /// Sets the long version of the subcommand flag without the preceding `--`.
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
    ///                 .about("search remote repositories for matching strings"),
    ///         ),
    ///     )
    ///     .get_matches_from(vec!["pacman", "--sync", "--search"]);
    ///
    /// assert_eq!(matches.subcommand_name().unwrap(), "sync");
    /// let sync_matches = matches.subcommand_matches("sync").unwrap();
    /// assert!(sync_matches.is_present("search"));
    /// ```
    ///
    /// [`Arg::long`]: ./struct.Arg.html#method.long
    pub fn long_flag(mut self, long: &'help str) -> Self {
        self.long_flag = Some(long.trim_start_matches(|c| c == '-'));
        self
    }

    /// Sets a string of the version number to be displayed when displaying the
    /// short format version message (`-V`) or the help message.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to
    /// automatically set your application's version to the same thing as your
    /// crate at compile time. See the [`examples/`] directory for more
    /// information.
    ///
    /// `clap` can display two different version messages, a [long format] and a
    /// [short format] depending on whether the user used `-V` (short) or
    /// `--version` (long). This method sets the message during the short format
    /// (`-V`). However, if no long format message is configured, this
    /// message will be displayed for *both* the long format, or short format
    /// version message.
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
    /// [`examples/`]: https://github.com/clap-rs/clap/tree/master/examples
    /// [`App::long_version`]: ./struct.App.html#method.long_version
    pub fn version<S: Into<&'help str>>(mut self, ver: S) -> Self {
        self.version = Some(ver.into());
        self
    }

    /// Sets a string of the version number to be displayed when the user
    /// requests the long format version message (`--version`) or the help
    /// message.
    ///
    /// This is often used to display things such as commit ID, or compile time
    /// configured options.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_version!`] to
    /// automatically set your application's version to the same thing as your
    /// crate at compile time. See the [`examples/`] directory for more
    /// information.
    ///
    /// `clap` can display two different version messages, a [long format] and a
    /// [short format] depending on whether the user used `-V` (short) or
    /// `--version` (long). This method sets the message during the long format
    /// (`--version`). However, if no short format message is configured, this
    /// message will be displayed for *both* the long format, or short format
    /// version message.
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
    /// [`examples/`]: https://github.com/kbknapp/clap-rs/tree/master/examples
    /// [`App::version`]: ./struct.App.html#method.version
    pub fn long_version<S: Into<&'help str>>(mut self, ver: S) -> Self {
        self.long_version = Some(ver.into());
        self
    }

    /// Sets a string of the license to be displayed when displaying help information.
    ///
    /// **Pro-tip:** Use `clap`s convenience macro [`crate_license!`] to automatically set your
    /// application's license to the same thing as your crate at compile time. See the
    /// [`examples/`] directory for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .license("MIT OR Apache-2.0")
    /// # ;
    /// ```
    /// [`crate_license!`]: ./macro.crate_license!.html
    /// [`examples/`]: https://github.com/clap-rs/clap/tree/master/examples
    pub fn license<S: Into<&'help str>>(mut self, license: S) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Overrides the `clap` generated usage string.
    ///
    /// This will be displayed to the user when errors are found in argument parsing.
    ///
    /// **CAUTION:** Using this setting disables `clap`s "context-aware" usage
    /// strings. After this setting is set, this will be *the only* usage string
    /// displayed to the user!
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
    pub fn override_usage<S: Into<&'help str>>(mut self, usage: S) -> Self {
        self.usage_str = Some(usage.into());
        self
    }

    /// Overrides the `clap` generated help message. This should only be used
    /// when the auto-generated message does not suffice.
    ///
    /// This will be displayed to the user when they use `--help` or `-h`.
    ///
    /// **NOTE:** This replaces the **entire** help message, so nothing will be
    /// auto-generated.
    ///
    /// **NOTE:** This **only** replaces the help message for the current
    /// command, meaning if you are using subcommands, those help messages will
    /// still be auto-generated unless you specify a [`Arg::override_help`] for
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
    ///            USAGE: myapp <opts> <comamnd>\n\n\
    ///
    ///            Options:\n\
    ///            -h, --help       Display this message\n\
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
    ///   * `{about}`               - General description (from [`App::about`] or
    ///                               [`App::long_about`]).
    ///   * `{about-with-newline}`  - About followed by `\n`.
    ///   * `{usage-heading}`       - Automatically generated usage heading.
    ///   * `{usage}`               - Automatically generated or given usage string.
    ///   * `{all-args}`            - Help for all arguments (options, flags, positional
    ///                               arguments, and subcommands) including titles.
    ///   * `{unified}`             - Unified help for options and flags. Note, you must *also*
    ///                               set [`AppSettings::UnifiedHelpMessage`] to fully merge both
    ///                               options and flags, otherwise the ordering is "best effort".
    ///   * `{flags}`               - Help for flags.
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
    /// [`App::about`]: ./struct.App.html#method.about
    /// [`App::long_about`]: ./struct.App.html#method.long_about
    /// [`App::after_help`]: ./struct.App.html#method.after_help
    /// [`App::after_long_help`]: ./struct.App.html#method.after_long_help
    /// [`App::before_help`]: ./struct.App.html#method.before_help
    /// [`App::before_long_help`]: ./struct.App.html#method.before_long_help
    /// [`AppSettings::UnifiedHelpMessage`]: ./enum.AppSettings.html#variant.UnifiedHelpMessage
    pub fn help_template<S: Into<&'help str>>(mut self, s: S) -> Self {
        self.template = Some(s.into());
        self
    }

    /// Enables a single settings for the current (this `App` instance) command or subcommand.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .setting(AppSettings::WaitOnError)
    /// # ;
    /// ```
    /// [`AppSettings`]: ./enum.AppSettings.html
    #[inline]
    pub fn setting(mut self, setting: AppSettings) -> Self {
        self.settings.set(setting);
        self
    }

    /// Disables a single setting for the current (this `App` instance) command or subcommand.
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
    /// [`AppSettings`]: ./enum.AppSettings.html
    /// [global]: ./struct.App.html#method.global_setting
    #[inline]
    pub fn unset_setting(mut self, setting: AppSettings) -> Self {
        self.settings.unset(setting);
        self
    }

    /// Enables a single setting that is propagated **down** through all child
    /// subcommands.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// **NOTE**: The setting is *only* propagated *down* and not up through parent commands.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .global_setting(AppSettings::SubcommandRequired)
    /// # ;
    /// ```
    /// [`AppSettings`]: ./enum.AppSettings.html
    #[inline]
    pub fn global_setting(mut self, setting: AppSettings) -> Self {
        self.settings.set(setting);
        self.g_settings.set(setting);
        self
    }

    /// Disables a global setting, and stops propagating down to child
    /// subcommands.
    ///
    /// See [`AppSettings`] for a full list of possibilities and examples.
    ///
    /// **NOTE:** The setting being unset will be unset from both local and
    /// [global] settings.
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
    #[inline]
    pub fn unset_global_setting(mut self, setting: AppSettings) -> Self {
        self.settings.unset(setting);
        self.g_settings.unset(setting);
        self
    }

    /// Sets the terminal width at which to wrap help messages. Defaults to
    /// `100`. Using `0` will ignore terminal widths and use source formatting.
    ///
    /// `clap` automatically tries to determine the terminal width on Unix,
    /// Linux, OSX and Windows if the `wrap_help` cargo "feature" has been enabled
    /// at compile time. If the terminal width cannot be determined, `clap`
    /// fall back to `100`.
    ///
    /// **NOTE:** This setting applies globally and *not* on a per-command basis.
    ///
    /// **NOTE:** This setting must be set **before** any subcommands are added!
    ///
    /// # Platform Specific
    ///
    /// Only Unix, Linux, OSX and Windows support automatic determination of
    /// terminal width. Even on those platforms, this setting is useful if for
    /// any reason the terminal width cannot be determined.
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
    pub fn term_width(mut self, width: usize) -> Self {
        self.term_w = Some(width);
        self
    }

    /// Sets the maximum terminal width at which to wrap help messages. Using `0`
    /// will ignore terminal widths and use source formatting.
    ///
    /// `clap` automatically tries to determine the terminal width on Unix,
    /// Linux, OSX and Windows if the `wrap_help` cargo "feature" has been
    /// enabled at compile time, but one might want to limit the size to some
    /// maximum (e.g. when the terminal is running fullscreen).
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
    #[inline]
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
    ///     // Adding a single "flag" argument with a short and help text, using Arg::new()
    ///     .arg(
    ///         Arg::new("debug")
    ///            .short('d')
    ///            .about("turns on debugging mode")
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
        let mut arg = a.into();
        if let Some(help_heading) = self.current_help_heading {
            arg = arg.help_heading(Some(help_heading));
        }
        self.args.push(arg);
        self
    }

    /// Set a custom section heading for future args. Every call to [`App::arg`]
    /// (and its related methods) will use this header (instead of the default
    /// header for the specified argument type) until a subsequent call to
    /// [`App::help_heading`] or [`App::stop_custom_headings`].
    ///
    /// This is useful if the default `FLAGS`, `OPTIONS`, or `ARGS` headings are
    /// not specific enough for one's use case.
    ///
    /// [`App::arg`]: ./struct.App.html#method.arg
    /// [`App::help_heading`]: ./struct.App.html#method.help_heading
    /// [`App::stop_custom_headings`]: ./struct.App.html#method.stop_custom_headings
    #[inline]
    pub fn help_heading(mut self, heading: &'help str) -> Self {
        self.current_help_heading = Some(heading);
        self
    }

    /// Stop using [custom argument headings] and return to default headings.
    ///
    /// [custom argument headings]: ./struct.App.html#method.help_heading
    #[inline]
    pub fn stop_custom_headings(mut self) -> Self {
        self.current_help_heading = None;
        self
    }

    /// Adds multiple [arguments] to the list of valid possibilities.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args(&[
    ///         Arg::from("[debug] -d 'turns on debugging info'"),
    ///         Arg::new("input").index(1).about("the input file to use")
    ///     ])
    /// # ;
    /// ```
    /// [arguments]: ./struct.Arg.html
    pub fn args<I, T>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Arg<'help>>,
    {
        // @TODO @perf @p4 @v3-beta: maybe extend_from_slice would be possible and perform better?
        // But that may also not let us do `&["-a 'some'", "-b 'other']` because of not Into<Arg>
        for arg in args.into_iter() {
            self.args.push(arg.into());
        }
        self
    }

    /// If this `App` instance is a subcommand, this method adds an alias, which
    /// allows this subcommand to be accessed via *either* the original name, or
    /// this given alias. This is more efficient and easier than creating
    /// multiple hidden subcommands as one only needs to check for the existence
    /// of this command, and not all aliased variants.
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
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    /// [`App::visible_alias`]: ./struct.App.html#method.visible_alias
    pub fn alias<S: Into<&'help str>>(mut self, name: S) -> Self {
        self.aliases.push((name.into(), false));
        self
    }

    /// Allows adding an alias, which function as "hidden" short flag subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
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
    pub fn short_flag_alias(mut self, name: char) -> Self {
        if name == '-' {
            panic!("short alias name cannot be `-`");
        }
        self.short_flag_aliases.push((name, false));
        self
    }

    /// Allows adding an alias, which function as "hidden" long flag subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
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
    pub fn long_flag_alias(mut self, name: &'help str) -> Self {
        self.long_flag_aliases.push((name, false));
        self
    }

    /// If this `App` instance is a subcommand, this method adds a multiple
    /// aliases, which allows this subcommand to be accessed via *either* the
    /// original name or any of the given aliases. This is more efficient, and
    /// easier than creating multiple hidden subcommands as one only needs to
    /// check for the existence of this command and not all aliased variants.
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
    ///             .about("the file to add")
    ///             .index(1)
    ///             .required(false))
    ///     .get_matches_from(vec!["myprog", "do-tests"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    /// [`App::visible_aliases`]: ./struct.App.html#method.visible_aliases
    pub fn aliases(mut self, names: &[&'help str]) -> Self {
        self.aliases.extend(names.iter().map(|n| (*n, false)));
        self
    }

    /// Allows adding aliases, which function as "hidden" short flag subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///     .subcommand(App::new("test").short_flag('t')
    ///         .short_flag_aliases(&['a', 'b', 'c']))
    ///         .arg(Arg::new("input")
    ///             .about("the file to add")
    ///             .index(1)
    ///             .required(false))
    ///     .get_matches_from(vec!["myprog", "-a"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    pub fn short_flag_aliases(mut self, names: &[char]) -> Self {
        for s in names {
            if s == &'-' {
                panic!("short alias name cannot be `-`");
            }
            self.short_flag_aliases.push((*s, false));
        }
        self
    }

    /// Allows adding aliases, which function as "hidden" long flag subcommands that
    /// automatically dispatch as if this subcommand was used. This is more efficient, and easier
    /// than creating multiple hidden subcommands as one only needs to check for the existence of
    /// this command, and not all variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// let m = App::new("myprog")
    ///             .subcommand(App::new("test").long_flag("test")
    ///                 .long_flag_aliases(&["testing", "testall", "test_all"]))
    ///                 .arg(Arg::new("input")
    ///                             .about("the file to add")
    ///                             .index(1)
    ///                             .required(false))
    ///             .get_matches_from(vec!["myprog", "--testing"]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    pub fn long_flag_aliases(mut self, names: &[&'help str]) -> Self {
        for s in names {
            self.long_flag_aliases.push((s, false));
        }
        self
    }

    /// If this `App` instance is a subcommand, this method adds a visible
    /// alias, which allows this subcommand to be accessed via *either* the
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
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    /// [`App::alias`]: ./struct.App.html#method.alias
    pub fn visible_alias<S: Into<&'help str>>(mut self, name: S) -> Self {
        self.aliases.push((name.into(), true));
        self
    }

    /// Allows adding an alias that functions exactly like those defined with
    /// [`App::short_flag_alias`], except that they are visible inside the help message.
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
    /// [`App::short_flag_alias`]: ./struct.App.html#method.short_flag_alias
    pub fn visible_short_flag_alias(mut self, name: char) -> Self {
        if name == '-' {
            panic!("short alias name cannot be `-`");
        }
        self.short_flag_aliases.push((name, true));
        self
    }

    /// Allows adding an alias that functions exactly like those defined with
    /// [`App::long_flag_alias`], except that they are visible inside the help message.
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
    /// [`App::long_flag_alias`]: ./struct.App.html#method.long_flag_alias
    pub fn visible_long_flag_alias(mut self, name: &'help str) -> Self {
        self.long_flag_aliases.push((name, true));
        self
    }

    /// If this `App` instance is a subcommand, this method adds multiple visible
    /// aliases, which allows this subcommand to be accessed via *either* the
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
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    /// [`App::alias`]: ./struct.App.html#method.alias
    pub fn visible_aliases(mut self, names: &[&'help str]) -> Self {
        self.aliases.extend(names.iter().map(|n| (*n, true)));
        self
    }

    /// Allows adding multiple short flag aliases that functions exactly like those defined
    /// with [`App::short_flag_aliases`], except that they are visible inside the help message.
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
    /// [`App::short_flag_aliases`]: ./struct.App.html#method.short_flag_aliases
    pub fn visible_short_flag_aliases(mut self, names: &[char]) -> Self {
        for s in names {
            if s == &'-' {
                panic!("short alias name cannot be `-`");
            }
            self.short_flag_aliases.push((*s, true));
        }
        self
    }

    /// Allows adding multiple long flag aliases that functions exactly like those defined
    /// with [`App::long_flag_aliases`], except that they are visible inside the help message.
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
    /// [`App::long_flag_aliases`]: ./struct.App.html#method.long_flag_aliases
    pub fn visible_long_flag_aliases(mut self, names: &[&'help str]) -> Self {
        for s in names {
            self.long_flag_aliases.push((s, true));
        }
        self
    }

    /// Replaces an argument or subcommand used on the CLI at runtime with other arguments or subcommands.
    ///
    /// When this method is used, `name` is removed from the CLI, and `target`
    /// is inserted in its place. Parsing continues as if the user typed
    /// `target` instead of `name`.
    ///
    /// This can be used to create "shortcuts" for subcommands, or if a
    /// particular argument has the semantic meaning of several other specific
    /// arguments and values.
    ///
    /// Some examples may help to clear this up.
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
    ///         .possible_values(&["txt", "json"]))
    ///     .replace("--save-all", &["--save-context", "--save-runtime", "--format=json"])
    ///     .get_matches_from(vec!["app", "--save-all"]);
    ///
    /// assert!(m.is_present("save-context"));
    /// assert!(m.is_present("save-runtime"));
    /// assert_eq!(m.value_of("format"), Some("json"));
    /// ```
    ///
    /// [`App::replace`]: ./struct.App.html#method.replace
    #[inline]
    pub fn replace(mut self, name: &'help str, target: &'help [&'help str]) -> Self {
        self.replacers.insert(name, target);
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
    /// use.
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
    ///     .group(ArgGroup::new("vers")
    ///          .args(&["set-ver", "major", "minor","patch"])
    ///          .required(true))
    /// # ;
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    #[inline]
    pub fn group<G: Into<ArgGroup<'help>>>(mut self, group: G) -> Self {
        self.groups.push(group.into());
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
    ///         ArgGroup::new("vers")
    ///             .args(&["set-ver", "major", "minor","patch"])
    ///             .required(true),
    ///         ArgGroup::new("input")
    ///             .args(&["c", "i"])
    ///     ])
    /// # ;
    /// ```
    /// [`ArgGroup`]: ./struct.ArgGroup.html
    /// [`App`]: ./struct.App.html
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

    /// Adds a subcommand to the list of valid possibilities. Subcommands are effectively
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
    /// [`App`]: ./struct.App.html
    #[inline]
    pub fn subcommand<S: Into<App<'help>>>(mut self, subcmd: S) -> Self {
        self.subcommands.push(subcmd.into());
        self
    }

    /// Adds multiple subcommands to the list of valid possibilities by iterating over an
    /// [`IntoIterator`] of [`App`]s.
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
    /// [`App`]: ./struct.App.html
    /// [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
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

    /// Allows custom ordering of subcommands within the help message. Subcommands with a lower
    /// value will be displayed first in the help message. This is helpful when one would like to
    /// emphasize frequently used subcommands, or prioritize those towards the top of the list.
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
    #[inline]
    pub fn display_order(mut self, ord: usize) -> Self {
        self.disp_ord = ord;
        self
    }

    /// Allows one to mutate an [`Arg`] after it's been added to an [`App`].
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
    /// [`Arg`]: ./struct.Arg.html
    /// [`App`]: ./struct.App.html
    pub fn mut_arg<T, F>(mut self, arg_id: T, f: F) -> Self
    where
        F: FnOnce(Arg<'help>) -> Arg<'help>,
        T: Key + Into<&'help str>,
    {
        let arg_id: &str = arg_id.into();
        let id = Id::from(arg_id);
        let a = self.args.remove_by_name(&id).unwrap_or_else(|| Arg {
            id,
            name: arg_id,
            ..Arg::default()
        });
        self.args.push(f(a));

        self
    }

    /// Prints the full help message to [`io::stdout()`] using a [`BufWriter`] using the same
    /// method as if someone ran `-h` to request the help message.
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)].
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
    /// [`-h` (short)]: ./struct.Arg.html#method.about
    /// [`--help` (long)]: ./struct.Arg.html#method.long_about
    pub fn print_help(&mut self) -> io::Result<()> {
        self._build();

        let p = Parser::new(self);
        let mut c = Colorizer::new(false, p.color_help());
        Help::new(HelpWriter::Buffer(&mut c), &p, false).write_help()?;
        c.print()
    }

    /// Prints the full help message to [`io::stdout()`] using a [`BufWriter`] using the same
    /// method as if someone ran `--help` to request the help message.
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)].
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
    /// [`-h` (short)]: ./struct.Arg.html#method.about
    /// [`--help` (long)]: ./struct.Arg.html#method.long_about
    pub fn print_long_help(&mut self) -> io::Result<()> {
        self._build();

        let p = Parser::new(self);
        let mut c = Colorizer::new(false, p.color_help());
        Help::new(HelpWriter::Buffer(&mut c), &p, true).write_help()?;
        c.print()
    }

    /// Writes the full help message to the user to a [`io::Write`] object in the same method as if
    /// the user ran `-h`.
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)].
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
    /// [`-h` (short)]: ./struct.Arg.html#method.about
    /// [`--help` (long)]: ./struct.Arg.html#method.long_about
    pub fn write_help<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        self._build();

        let p = Parser::new(self);
        Help::new(HelpWriter::Normal(w), &p, false).write_help()?;
        w.flush()
    }

    /// Writes the full help message to the user to a [`io::Write`] object in the same method as if
    /// the user ran `--help`.
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" help messages
    /// depending on if the user ran [`-h` (short)] or [`--help` (long)].
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
    /// [`-h` (short)]: ./struct.Arg.html#method.about
    /// [`--help` (long)]: ./struct.Arg.html#method.long_about
    pub fn write_long_help<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        self._build();

        let p = Parser::new(self);
        Help::new(HelpWriter::Normal(w), &p, true).write_help()?;
        w.flush()
    }

    /// Returns the version message rendered as if the user ran `-V`.
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" version messages
    /// depending on if the user ran [`-V` (short)] or [`--version` (long)].
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
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`-V` (short)]: ./struct.App.html#method.version
    /// [`--version` (long)]: ./struct.App.html#method.long_version
    /// [ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
    pub fn render_version(&self) -> String {
        self._render_version(false)
    }

    /// Returns the version message rendered as if the user ran `--version`.
    ///
    /// **NOTE:** clap has the ability to distinguish between "short" and "long" version messages
    /// depending on if the user ran [`-V` (short)] or [`--version` (long)].
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
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`-V` (short)]: ./struct.App.html#method.version
    /// [`--version` (long)]: ./struct.App.html#method.long_version
    /// [ANSI escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
    pub fn render_long_version(&self) -> String {
        self._render_version(true)
    }

    /// @TODO-v3-alpha @docs @p2: write docs
    pub fn generate_usage(&mut self) -> String {
        // If there are global arguments, or settings we need to propagate them down to subcommands
        // before parsing incase we run into a subcommand
        self._build();

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
    #[inline]
    pub fn get_matches(self) -> ArgMatches {
        self.get_matches_from(&mut env::args_os())
    }

    /// Starts the parsing process, just like [`App::get_matches`] but doesn't consume the `App`.
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
                    e.message.print().expect("Error writing Error to stderr");

                    if self.settings.is_set(AppSettings::WaitOnError) {
                        wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                        let mut s = String::new();
                        let i = io::stdin();
                        i.lock().read_line(&mut s).unwrap();
                    }

                    drop(e);
                    safe_exit(2);
                }

                e.exit()
            })
    }

    /// Starts the parsing process. This method will return a [`clap::Result`] type instead of exiting
    /// the process on failed parse. By default this method gets matches from [`env::args_os`].
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return a [`clap::Error`], where the [`kind`] is a
    /// [`ErrorKind::DisplayHelp`] or [`ErrorKind::DisplayVersion`] respectively. You must call
    /// [`Error::exit`] or perform a [`std::process::exit`].
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
    /// [`env::args_os`]: https://doc.rust-lang.org/std/env/fn.args_os.html
    /// [`ErrorKind::DisplayHelp`]: ./enum.ErrorKind.html#variant.DisplayHelp
    /// [`ErrorKind::DisplayVersion`]: ./enum.ErrorKind.html#variant.DisplayVersion
    /// [`Error::exit`]: ./struct.Error.html#method.exit
    /// [`std::process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
    /// [`clap::Result`]: ./type.Result.html
    /// [`clap::Error`]: ./struct.Error.html
    /// [`kind`]: ./struct.Error.html
    #[inline]
    pub fn try_get_matches(self) -> ClapResult<ArgMatches> {
        // Start the parsing
        self.try_get_matches_from(&mut env::args_os())
    }

    /// Starts the parsing process. Like [`App::get_matches`] this method does not return a [`clap::Result`]
    /// and will automatically exit with an error message. This method, however, lets you specify
    /// what iterator to use when performing matches, such as a [`Vec`] of your making.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used.
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
                e.message.print().expect("Error writing Error to stderr");

                if self.settings.is_set(AppSettings::WaitOnError) {
                    wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                    let mut s = String::new();
                    let i = io::stdin();
                    i.lock().read_line(&mut s).unwrap();
                }

                drop(self);
                drop(e);
                safe_exit(2);
            }

            drop(self);
            e.exit()
        })
    }

    /// Starts the parsing process. A combination of [`App::get_matches_from`], and
    /// [`App::try_get_matches`].
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return a [`clap::Error`], where the [`kind`] is a [`ErrorKind::DisplayHelp`]
    /// or [`ErrorKind::DisplayVersion`] respectively. You must call [`Error::exit`] or
    /// perform a [`std::process::exit`] yourself.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// [`AppSettings::NoBinaryName`] is used.
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
    /// [`App::get_matches_from`]: ./struct.App.html#method.get_matches_from
    /// [`App::try_get_matches`]: ./struct.App.html#method.try_get_matches
    /// [`ErrorKind::DisplayHelp`]: ./enum.ErrorKind.html#variant.DisplayHelp
    /// [`ErrorKind::DisplayVersion`]: ./enum.ErrorKind.html#variant.DisplayVersion
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
    /// [`AppSettings::NoBinaryName`] is used.
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
    /// [`App`]: ./struct.App.html
    /// [`App::try_get_matches_from`]: ./struct.App.html#method.try_get_matches_from
    /// [`AppSettings::NoBinaryName`]: ./enum.AppSettings.html#variant.NoBinaryName
    pub fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut it = Input::from(itr.into_iter());
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

    /// Sets the placeholder text used for subcommands when printing usage and help.
    /// By default, this is "SUBCOMMAND" with a header of "SUBCOMMANDS".
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
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// SUBCOMMANDS:
    ///     help    Prints this message or the help of the given subcommand(s)
    ///     sub1
    /// ```
    ///
    /// but usage of `subcommand_placeholder`
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .subcommand(App::new("sub1"))
    ///     .subcommand_placeholder("THING", "THINGS")
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
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// THINGS:
    ///     help    Prints this message or the help of the given subcommand(s)
    ///     sub1
    /// ```
    pub fn subcommand_placeholder<S, T>(mut self, placeholder: S, header: T) -> Self
    where
        S: Into<&'help str>,
        T: Into<&'help str>,
    {
        self.subcommand_placeholder = Some(placeholder.into());
        self.subcommand_header = Some(header.into());
        self
    }
}

// Internally used only
impl<'help> App<'help> {
    fn get_used_global_args(&self, matcher: &ArgMatcher) -> Vec<Id> {
        let global_args: Vec<_> = self
            .args
            .args()
            .filter(|a| a.global)
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
        let mut matcher = ArgMatcher::default();

        // If there are global arguments, or settings we need to propagate them down to subcommands
        // before parsing in case we run into a subcommand
        self._build();

        // do the real parsing
        let mut parser = Parser::new(self);
        parser.get_matches_with(&mut matcher, it)?;

        let global_arg_vec: Vec<Id> = self.get_used_global_args(&matcher);

        matcher.propagate_globals(&global_arg_vec);

        Ok(matcher.into_inner())
    }

    // used in clap_generate (https://github.com/clap-rs/clap_generate)
    #[doc(hidden)]
    pub fn _build(&mut self) {
        debug!("App::_build");
        if !self.settings.is_set(AppSettings::Built) {
            // Make sure all the globally set flags apply to us as well
            self.settings = self.settings | self.g_settings;

            self._propagate();

            self._derive_display_order();
            self._create_help_and_version();

            let mut pos_counter = 1;
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
                a._build();
                if a.short.is_none() && a.long.is_none() && a.index.is_none() {
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
        if self.is_set(AppSettings::HelpRequired) || help_required_globally {
            let args_missing_help: Vec<String> = self
                .args
                .args()
                .filter(|arg| arg.about.is_none() && arg.long_about.is_none())
                .map(|arg| String::from(arg.name))
                .collect();

            if !args_missing_help.is_empty() {
                panic!(format!(
                    "AppSettings::HelpRequired is enabled for the App {}, but at least one of its arguments does not have either `help` or `long_help` set. List of such arguments: {}",
                    self.name,
                    args_missing_help.join(", ")
                ));
            }
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

    /// Propagate one and only one level.
    pub(crate) fn _propagate(&mut self) {
        macro_rules! propagate_subcmd {
            ($_self:expr, $sc:expr) => {{
                // We have to create a new scope in order to tell rustc the borrow of `sc` is
                // done and to recursively call this method
                {
                    let vsc = $_self
                        .settings
                        .is_set(AppSettings::DisableVersionForSubcommands);
                    let gv = $_self.settings.is_set(AppSettings::GlobalVersion);

                    if vsc {
                        $sc.set(AppSettings::DisableVersionFlag);
                    }
                    if gv && $sc.version.is_none() && $_self.version.is_some() {
                        $sc.set(AppSettings::GlobalVersion);
                        $sc.version = Some($_self.version.unwrap());
                    }
                    $sc.settings = $sc.settings | $_self.g_settings;
                    $sc.g_settings = $sc.g_settings | $_self.g_settings;
                    $sc.term_w = $_self.term_w;
                    $sc.max_w = $_self.max_w;
                    if $sc.help_about.is_none() && $_self.help_about.is_some() {
                        $sc.help_about = $_self.help_about.clone();
                    }
                    if $sc.version_about.is_none() && $_self.version_about.is_some() {
                        $sc.version_about = $_self.version_about.clone();
                    }
                }
                {
                    // FIXME: This doesn't belong here at all.
                    for a in $_self.args.args().filter(|a| a.global) {
                        if $sc.find(&a.id).is_none() {
                            $sc.args.push(a.clone());
                        }
                    }
                }
            }};
        }

        debug!("App::_propagate:{}", self.name);

        for sc in &mut self.subcommands {
            propagate_subcmd!(self, sc);
        }
    }

    pub(crate) fn _create_help_and_version(&mut self) {
        debug!("App::_create_help_and_version");

        if !(self.is_set(AppSettings::DisableHelpFlag)
            || self
                .args
                .args()
                .any(|x| x.long == Some("help") || x.id == Id::help_hash())
            || self
                .subcommands
                .iter()
                .any(|sc| sc.long_flag == Some("help")))
        {
            debug!("App::_create_help_and_version: Building --help");
            let mut help = Arg::new("help")
                .long("help")
                .about(self.help_about.unwrap_or("Prints help information"));

            if !(self.args.args().any(|x| x.short == Some('h'))
                || self.subcommands.iter().any(|sc| sc.short_flag == Some('h')))
            {
                help = help.short('h');
            }

            self.args.push(help);
        }

        if !(self.is_set(AppSettings::DisableVersionFlag)
            || self
                .args
                .args()
                .any(|x| x.long == Some("version") || x.id == Id::version_hash())
            || self
                .subcommands
                .iter()
                .any(|sc| sc.long_flag == Some("version")))
        {
            debug!("App::_create_help_and_version: Building --version");
            let mut version = Arg::new("version")
                .long("version")
                .about(self.version_about.unwrap_or("Prints version information"));

            if !(self.args.args().any(|x| x.short == Some('V'))
                || self.subcommands.iter().any(|sc| sc.short_flag == Some('V')))
            {
                version = version.short('V');
            }

            self.args.push(version);
        }

        if !self.is_set(AppSettings::DisableHelpSubcommand)
            && self.has_subcommands()
            && !self.subcommands.iter().any(|s| s.id == Id::help_hash())
        {
            debug!("App::_create_help_and_version: Building help");
            self.subcommands.push(
                App::new("help").about(
                    self.help_about
                        .unwrap_or("Prints this message or the help of the given subcommand(s)"),
                ),
            );
        }
    }

    pub(crate) fn _derive_display_order(&mut self) {
        debug!("App::_derive_display_order:{}", self.name);

        if self.settings.is_set(AppSettings::DeriveDisplayOrder) {
            for (i, a) in self
                .args
                .args_mut()
                .filter(|a| a.has_switch())
                .filter(|a| a.disp_ord == 999)
                .enumerate()
            {
                a.disp_ord = i;
            }
            for (i, mut sc) in &mut self
                .subcommands
                .iter_mut()
                .enumerate()
                .filter(|&(_, ref sc)| sc.disp_ord == 999)
            {
                sc.disp_ord = i;
            }
        }
        for sc in &mut self.subcommands {
            sc._derive_display_order();
        }
    }

    // used in clap_generate (https://github.com/clap-rs/clap_generate)
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
                format!("{} {}\n", bn.replace(" ", "-"), ver)
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
                if x.index.is_some() {
                    // Print val_name for positional arguments. e.g. <file_name>
                    x.name_no_brackets().to_string()
                } else {
                    // Print useage string for flags arguments, e.g. <--help>
                    x.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("<{}>", &*g_string)
    }
}

// Internal Query Methods
impl<'help> App<'help> {
    pub(crate) fn find(&self, arg_id: &Id) -> Option<&Arg<'help>> {
        self.args.args().find(|a| a.id == *arg_id)
    }

    #[inline]
    // Should we color the output?
    pub(crate) fn color(&self) -> ColorChoice {
        debug!("App::color: Color setting...");

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
    }

    #[inline]
    pub(crate) fn contains_short(&self, s: char) -> bool {
        if !self.is_set(AppSettings::Built) {
            panic!("If App::_build hasn't been called, manually search through Arg shorts");
        }

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

    #[inline]
    pub(crate) fn has_opts(&self) -> bool {
        self.get_opts_with_no_heading().count() > 0
    }

    #[inline]
    pub(crate) fn has_flags(&self) -> bool {
        self.get_flags_with_no_heading().count() > 0
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

    /// Iterate through all the names of all subcommands (not recursively), including aliases.
    /// Used for suggestions.
    pub(crate) fn all_subcommand_names<'a>(&'a self) -> impl Iterator<Item = &'a str>
    where
        'help: 'a,
    {
        let a: Vec<_> = self
            .get_subcommands()
            .flat_map(|sc| {
                let name = sc.get_name();
                let aliases = sc.get_all_aliases();
                std::iter::once(name).chain(aliases)
            })
            .collect();

        // Strictly speaking, we don't need this trip through the Vec.
        // We should have been able to return FlatMap iter above directly.
        //
        // Unfortunately, that would trigger
        // https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999
        //
        // I think this "collect to vec" solution is better than the linked one
        // because it's simpler and it doesn't really matter performance-wise.
        a.into_iter()
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
    pub(crate) fn find_long_subcmd(&self, long: &ArgStr) -> Option<&str> {
        self.get_subcommands()
            .find(|sc| sc.long_flag_aliases_to(long))
            .map(|sc| sc.get_name())
    }
}

impl<'help> Index<&'_ Id> for App<'help> {
    type Output = Arg<'help>;

    fn index(&self, key: &Id) -> &Self::Output {
        self.find(key).expect(INTERNAL_ERROR_MSG)
    }
}

#[cfg(feature = "yaml")]
impl<'help> From<&'help Yaml> for App<'help> {
    #[allow(clippy::cognitive_complexity)]
    fn from(mut yaml: &'help Yaml) -> Self {
        // We WANT this to panic on error...so expect() is good.
        let mut is_sc = None;
        let mut a = if let Some(name) = yaml["name"].as_str() {
            App::new(name)
        } else {
            let yaml_hash = yaml.as_hash().unwrap();
            let sc_key = yaml_hash.keys().next().unwrap();
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
        yaml_str!(a, yaml, long_version);
        yaml_str!(a, yaml, author);
        yaml_str!(a, yaml, bin_name);
        yaml_str!(a, yaml, about);
        yaml_str!(a, yaml, before_help);
        yaml_str!(a, yaml, before_long_help);
        yaml_str!(a, yaml, after_help);
        yaml_str!(a, yaml, after_long_help);
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
                a = a.arg(Arg::from(arg_yaml));
            }
        }
        if let Some(v) = yaml["subcommands"].as_vec() {
            for sc_yaml in v {
                a = a.subcommand(App::from(sc_yaml));
            }
        }
        if let Some(v) = yaml["groups"].as_vec() {
            for ag_yaml in v {
                a = a.group(ArgGroup::from(ag_yaml));
            }
        }

        a
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
