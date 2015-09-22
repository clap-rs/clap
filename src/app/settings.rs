use std::str::FromStr;
use std::ascii::AsciiExt;

/// Application level settings, which affect how `App` operates
#[derive(PartialEq, Debug)]
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
}

impl FromStr for AppSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "subcommandsnegatereqs"  => Ok(AppSettings::SubcommandsNegateReqs),
            "subcommandsrequired"    => Ok(AppSettings::SubcommandRequired),
            "argrequiredelsehelp"    => Ok(AppSettings::ArgRequiredElseHelp),
            "globalversion"          => Ok(AppSettings::GlobalVersion),
            "versionlesssubcommands" => Ok(AppSettings::VersionlessSubcommands),
            "unifiedhelpmessage"     => Ok(AppSettings::UnifiedHelpMessage),
            "waitonerror"            => Ok(AppSettings::WaitOnError),
            "subcommandrequiredelsehelp" => Ok(AppSettings::SubcommandRequiredElseHelp),
            _                        => Err("unknown AppSetting, cannot convert from str".to_owned())
        }
    }
}
