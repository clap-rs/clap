use std::str::FromStr;
use std::ascii::AsciiExt;

bitflags! {
    flags Flags: u32 {
        const SC_NEGATE_REQS       = 0b000000000000001,
        const SC_REQUIRED          = 0b000000000000010,
        const A_REQUIRED_ELSE_HELP = 0b000000000000100,
        const GLOBAL_VERSION       = 0b000000000001000,
        const VERSIONLESS_SC       = 0b000000000010000,
        const UNIFIED_HELP         = 0b000000000100000,
        const WAIT_ON_ERROR        = 0b000000001000000,
        const SC_REQUIRED_ELSE_HELP= 0b000000010000000,
        const NEEDS_LONG_HELP      = 0b000000100000000,
        const NEEDS_LONG_VERSION   = 0b000001000000000,
        const NEEDS_SC_HELP        = 0b000010000000000,
        const DISABLE_VERSION      = 0b000100000000000,
        const HIDDEN               = 0b001000000000000,
        const TRAILING_VARARG      = 0b010000000000000,
        const NO_BIN_NAME          = 0b100000000000000,
    }
}

#[derive(Debug)]
pub struct AppFlags(Flags);

impl AppFlags {
    pub fn new() -> Self {
        AppFlags(NEEDS_LONG_VERSION | NEEDS_LONG_HELP | NEEDS_SC_HELP)
    }

    pub fn set(&mut self, s: &AppSettings) {
        match *s {
            AppSettings::SubcommandsNegateReqs => self.0.insert(SC_NEGATE_REQS),
            AppSettings::VersionlessSubcommands => self.0.insert(VERSIONLESS_SC),
            AppSettings::SubcommandRequired => self.0.insert(SC_REQUIRED),
            AppSettings::ArgRequiredElseHelp => self.0.insert(A_REQUIRED_ELSE_HELP),
            AppSettings::GlobalVersion => self.0.insert(GLOBAL_VERSION),
            AppSettings::UnifiedHelpMessage => self.0.insert(UNIFIED_HELP),
            AppSettings::WaitOnError => self.0.insert(WAIT_ON_ERROR),
            AppSettings::SubcommandRequiredElseHelp => self.0.insert(SC_REQUIRED_ELSE_HELP),
            AppSettings::NeedsLongHelp => self.0.insert(NEEDS_LONG_HELP),
            AppSettings::NeedsLongVersion => self.0.insert(NEEDS_LONG_VERSION),
            AppSettings::NeedsSubcommandHelp => self.0.insert(NEEDS_SC_HELP),
            AppSettings::DisableVersion => self.0.insert(DISABLE_VERSION),
            AppSettings::Hidden => self.0.insert(HIDDEN),
            AppSettings::TrailingVarArg => self.0.insert(TRAILING_VARARG),
            AppSettings::NoBinaryName => self.0.insert(NO_BIN_NAME),
        }
    }

    pub fn unset(&mut self, s: &AppSettings) {
        match *s {
            AppSettings::SubcommandsNegateReqs => self.0.remove(SC_NEGATE_REQS),
            AppSettings::VersionlessSubcommands => self.0.remove(VERSIONLESS_SC),
            AppSettings::SubcommandRequired => self.0.remove(SC_REQUIRED),
            AppSettings::ArgRequiredElseHelp => self.0.remove(A_REQUIRED_ELSE_HELP),
            AppSettings::GlobalVersion => self.0.remove(GLOBAL_VERSION),
            AppSettings::UnifiedHelpMessage => self.0.remove(UNIFIED_HELP),
            AppSettings::WaitOnError => self.0.remove(WAIT_ON_ERROR),
            AppSettings::SubcommandRequiredElseHelp => self.0.remove(SC_REQUIRED_ELSE_HELP),
            AppSettings::NeedsLongHelp => self.0.remove(NEEDS_LONG_HELP),
            AppSettings::NeedsLongVersion => self.0.remove(NEEDS_LONG_VERSION),
            AppSettings::NeedsSubcommandHelp => self.0.remove(NEEDS_SC_HELP),
            AppSettings::DisableVersion => self.0.remove(DISABLE_VERSION),
            AppSettings::Hidden => self.0.remove(HIDDEN),
            AppSettings::TrailingVarArg => self.0.remove(TRAILING_VARARG),
            AppSettings::NoBinaryName => self.0.remove(NO_BIN_NAME),
        }
    }

    pub fn is_set(&self, s: &AppSettings) -> bool {
        match *s {
            AppSettings::SubcommandsNegateReqs => self.0.contains(SC_NEGATE_REQS),
            AppSettings::VersionlessSubcommands => self.0.contains(VERSIONLESS_SC),
            AppSettings::SubcommandRequired => self.0.contains(SC_REQUIRED),
            AppSettings::ArgRequiredElseHelp => self.0.contains(A_REQUIRED_ELSE_HELP),
            AppSettings::GlobalVersion => self.0.contains(GLOBAL_VERSION),
            AppSettings::UnifiedHelpMessage => self.0.contains(UNIFIED_HELP),
            AppSettings::WaitOnError => self.0.contains(WAIT_ON_ERROR),
            AppSettings::SubcommandRequiredElseHelp => self.0.contains(SC_REQUIRED_ELSE_HELP),
            AppSettings::NeedsLongHelp => self.0.contains(NEEDS_LONG_HELP),
            AppSettings::NeedsLongVersion => self.0.contains(NEEDS_LONG_VERSION),
            AppSettings::NeedsSubcommandHelp => self.0.contains(NEEDS_SC_HELP),
            AppSettings::DisableVersion => self.0.contains(DISABLE_VERSION),
            AppSettings::Hidden => self.0.contains(HIDDEN),
            AppSettings::TrailingVarArg => self.0.contains(TRAILING_VARARG),
            AppSettings::NoBinaryName => self.0.contains(NO_BIN_NAME),
        }
    }
}

/// Application level settings, which affect how `App` operates
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AppSettings {
    /// Allows subcommands to override all requirements of the parent (this command). For example
    /// if you had a subcommand or even top level application which had a required arguments that
    /// are only required as long as there is no subcommand present.
    ///
    /// **NOTE:** This defaults to false (using subcommand does *not* negate requirements)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandsNegateReqs)
    /// # ;
    /// ```
    SubcommandsNegateReqs,
    /// Allows specifying that if no subcommand is present at runtime, error and exit gracefully
    ///
    /// **NOTE:** This defaults to false (subcommands do *not* need to be present)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    /// # ;
    /// ```
    SubcommandRequired,
    /// Specifies that the help text sould be displayed (and then exit gracefully), if no
    /// arguments are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **NOTE:** Subcommands count as arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    /// # ;
    /// ```
    ArgRequiredElseHelp,
    /// Uses version of the current command for all subcommands. (Defaults to false; subcommands
    /// have independant version strings)
    ///
    /// **NOTE:** The version for the current command and this setting must be set **prior** to
    /// adding any subcommands
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::GlobalVersion)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches();
    /// // running `myprog test --version` will display
    /// // "myprog-test v1.1"
    /// ```
    GlobalVersion,
    /// Disables `-V` and `--version` for all subcommands (Defaults to false; subcommands have
    /// version flags)
    ///
    /// **NOTE:** This setting must be set **prior** adding any subcommands
    ///
    /// **NOTE:** Do not set this value to false, it will have undesired results!
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::VersionlessSubcommands)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches();
    /// // running `myprog test --version` will display unknown argument error
    /// ```
    VersionlessSubcommands,
    /// By default the auto-generated help message groups flags, options, and positional arguments
    /// separately. This setting disable that and groups flags and options together presenting a
    /// more unified help message (a la getopts or docopt style).
    ///
    /// **NOTE:** This setting is cosmetic only and does not affect any functionality.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::UnifiedHelpMessage)
    ///     .get_matches();
    /// // running `myprog --help` will display a unified "docopt" or "getopts" style help message
    /// ```
    UnifiedHelpMessage,
    /// Will display a message "Press [ENTER]/[RETURN] to continue..." and wait user before
    /// exiting
    ///
    /// This is most useful when writing an application which is run from a GUI shortcut, or on
    /// Windows where a user tries to open the binary by double-clicking instead of using the
    /// command line (i.e. set `.arg_required_else_help(true)` and `.wait_on_error(true)` to
    /// display the help in such a case).
    ///
    /// **NOTE:** This setting is **not** recursive with subcommands, meaning if you wish this
    /// behavior for all subcommands, you must set this on each command (needing this is extremely
    /// rare)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::WaitOnError)
    /// # ;
    /// ```
    WaitOnError,
    /// Specifies that the help text sould be displayed (and then exit gracefully), if no
    /// subcommands are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **NOTE:** This should *not* be used with `.subcommand_required()` as they do the same
    /// thing, except one prints the help text, and one prints an error.
    ///
    /// **NOTE:** If the user specifies arguments at runtime, but no subcommand the help text will
    /// still be displayed and exit. If this is *not* the desired result, consider using
    /// `.arg_required_else_help()`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequiredElseHelp)
    /// # ;
    /// ```
    SubcommandRequiredElseHelp,
    /// Specifies that this subcommand should be hidden from help messages
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings, SubCommand};
    /// App::new("myprog")
    ///     .subcommand(SubCommand::with_name("test")
    ///     .setting(AppSettings::Hidden))
    /// # ;
    /// ```
    Hidden,
    /// Specifies that the final positional argument is a vararg and that `clap` should not attempt
    /// to parse any further args.
    ///
    /// The values of the trailing positional argument will contain all args from itself on.
    ///
    /// **NOTE:** The final positional argument **must** have `.multiple(true)` or usage token
    /// equivilant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::TrailingVarArg)
    ///     .arg(Arg::from_usage("<cmd>... 'commands to run'"))
    ///     .get_matches_from(vec!["myprog", "some_command", "-r", "set"]);
    ///
    /// assert_eq!(m.values_of("cmd").unwrap(), &["some_command", "-r", "set"]);
    /// ```
    TrailingVarArg,
    /// Specifies that the parser should not assume the first argument passed is the binary name.
    /// This is normally the case when using a "daemon" style mode, or an interactive CLI where one
    /// one would not normally type the binary or program name for each command.
    ///
    /// **NOTE:** This should only be used when you absolutely know it's what you need. 99% of the
    /// cases out there don't need this setting.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::NoBinaryName)
    ///     .arg(Arg::from_usage("<cmd>... 'commands to run'"))
    ///     .get_matches_from(vec!["some_command", "-r", "set"]);
    ///
    /// assert_eq!(m.values_of("cmd").unwrap(), &["some_command", "-r", "set"]);
    /// ```
    NoBinaryName,
    #[doc(hidden)]
    NeedsLongVersion,
    #[doc(hidden)]
    NeedsLongHelp,
    #[doc(hidden)]
    NeedsSubcommandHelp,
    #[doc(hidden)]
    DisableVersion,
}

impl FromStr for AppSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "subcommandsnegatereqs" => Ok(AppSettings::SubcommandsNegateReqs),
            "subcommandsrequired" => Ok(AppSettings::SubcommandRequired),
            "argrequiredelsehelp" => Ok(AppSettings::ArgRequiredElseHelp),
            "globalversion" => Ok(AppSettings::GlobalVersion),
            "versionlesssubcommands" => Ok(AppSettings::VersionlessSubcommands),
            "unifiedhelpmessage" => Ok(AppSettings::UnifiedHelpMessage),
            "waitonerror" => Ok(AppSettings::WaitOnError),
            "subcommandrequiredelsehelp" => Ok(AppSettings::SubcommandRequiredElseHelp),
            "hidden" => Ok(AppSettings::Hidden),
            _ => Err("unknown AppSetting, cannot convert from str".to_owned()),
        }
    }
}
