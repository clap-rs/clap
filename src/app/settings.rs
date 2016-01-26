use std::str::FromStr;
use std::ascii::AsciiExt;

bitflags! {
    flags Flags: u32 {
        const SC_NEGATE_REQS       = 0b0000000000000000001,
        const SC_REQUIRED          = 0b0000000000000000010,
        const A_REQUIRED_ELSE_HELP = 0b0000000000000000100,
        const GLOBAL_VERSION       = 0b0000000000000001000,
        const VERSIONLESS_SC       = 0b0000000000000010000,
        const UNIFIED_HELP         = 0b0000000000000100000,
        const WAIT_ON_ERROR        = 0b0000000000001000000,
        const SC_REQUIRED_ELSE_HELP= 0b0000000000010000000,
        const NEEDS_LONG_HELP      = 0b0000000000100000000,
        const NEEDS_LONG_VERSION   = 0b0000000001000000000,
        const NEEDS_SC_HELP        = 0b0000000010000000000,
        const DISABLE_VERSION      = 0b0000000100000000000,
        const HIDDEN               = 0b0000001000000000000,
        const TRAILING_VARARG      = 0b0000010000000000000,
        const NO_BIN_NAME          = 0b0000100000000000000,
        const ALLOW_UNK_SC         = 0b0001000000000000000,
        const UTF8_STRICT          = 0b0010000000000000000,
        const UTF8_NONE            = 0b0100000000000000000,
        const LEADING_HYPHEN       = 0b1000000000000000000,
    }
}

#[derive(Debug)]
pub struct AppFlags(Flags);

impl AppFlags {
    pub fn new() -> Self {
        AppFlags(NEEDS_LONG_VERSION | NEEDS_LONG_HELP | NEEDS_SC_HELP | UTF8_NONE)
    }

    impl_settings! { AppSettings,
        SubcommandsNegateReqs => SC_NEGATE_REQS,
        VersionlessSubcommands => VERSIONLESS_SC,
        SubcommandRequired => SC_REQUIRED,
        ArgRequiredElseHelp => A_REQUIRED_ELSE_HELP,
        GlobalVersion => GLOBAL_VERSION,
        UnifiedHelpMessage => UNIFIED_HELP,
        WaitOnError => WAIT_ON_ERROR,
        SubcommandRequiredElseHelp => SC_REQUIRED_ELSE_HELP,
        NeedsLongHelp => NEEDS_LONG_HELP,
        NeedsLongVersion => NEEDS_LONG_VERSION,
        NeedsSubcommandHelp => NEEDS_SC_HELP,
        DisableVersion => DISABLE_VERSION,
        Hidden => HIDDEN,
        TrailingVarArg => TRAILING_VARARG,
        NoBinaryName => NO_BIN_NAME,
        AllowExternalSubcommands => ALLOW_UNK_SC,
        StrictUtf8 => UTF8_STRICT,
        AllowInvalidUtf8 => UTF8_NONE,
        AllowLeadingHyphen => LEADING_HYPHEN
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
    /// Specifies that the help text should be displayed (and then exit gracefully), if no
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
    /// Specifies that the help text should be displayed (and then exit gracefully), if no
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
    /// equivalent.
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
    /// assert_eq!(m.values_of("cmd").unwrap().collect::<Vec<_>>(), &["some_command", "-r", "set"]);
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
    /// assert_eq!(m.values_of("cmd").unwrap().collect::<Vec<_>>(), &["some_command", "-r", "set"]);
    /// ```
    NoBinaryName,
    /// Specifies that an unexpected argument positional arguments which would otherwise cause a
    /// `ErrorKind::UnknownArgument` error, should instead be treated as a subcommand in the
    /// `ArgMatches` struct.
    ///
    /// **NOTE:** Use this setting with caution, as a truly unexpected argument (i.e. one that is
    /// *NOT* an external subcommand) will not cause an error and instead be treatd as a potential
    /// subcommand. You shoud inform the user appropriatly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// use std::process::{self, Command};
    ///
    /// // Assume there is a third party subcommand named myprog-subcmd
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowExternalSubcommands)
    ///     .get_matches_from(vec!["myprog", "subcmd", "--option", "value"]);
    ///
    /// // All trailing arguments will be stored under the subcommands sub-matches under a value
    /// // of their runtime name (in this case "subcmd")
    /// match m.subcommand() {
    ///     (external, Some(ext_m)) => {
    ///         let args: Vec<&str> = ext_m.values_of(external).unwrap().collect();
    ///         let exit_status = Command::new(format!("myprog-{}", external))
    ///             .args(&*args)
    ///             .status()
    ///             .unwrap_or_else(|e| {
    ///             // Invalid subcommand. Here you would probably inform the user and list valid
    ///             // subcommands for them to try...but in this example we just panic!
    ///             process::exit(1);
    ///         });
    ///     },
    ///     _ => unreachable!()
    /// }
    /// ```
    AllowExternalSubcommands,
    /// Specifies that any invalid UTF-8 code points should be treated as an error and fail
    /// with a `ErrorKind::InvalidUtf8` error.
    ///
    /// **NOTE:** This rule only applies to  argument values, as flags, options, and subcommands
    /// only allow valid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg, AppSettings, ErrorKind};
    /// use std::ffi::OsString;
    ///
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::StrictUtf8)
    ///     .arg_from_usage("<arg> 'some positional arg'")
    ///     .get_matches_from_safe(
    ///         vec![
    ///             OsString::from("myprog"),
    ///             OsString::from_vec(vec![0xe9])]);
    ///
    /// assert!(m.is_err());
    /// assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
    /// }
    /// ```
    StrictUtf8,
    /// Specifies that any invalid UTF-8 code points should *not* be treated as an error. This is
    /// the default behavior of `clap`
    ///
    /// **NOTE:** Using argument values with invalid UTF-8 code points requires using Either
    /// `ArgMatches::os_value(s)_of` or `ArgMatches::lossy_value(s)_of` for those particular
    /// arguments which may have have invalid UTF-8 values
    ///
    /// **NOTE:** This rule only applies to  argument values, as flags, options, and subcommands
    /// only allow valid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg, AppSettings};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let r = App::new("myprog")
    ///     .setting(AppSettings::StrictUtf8)
    ///     .arg_from_usage("<arg> 'some positional arg'")
    ///     .get_matches_from_safe(
    ///         vec![
    ///             OsString::from("myprog"),
    ///             OsString::from_vec(vec![0xe9])]);
    ///
    /// assert!(r.is_ok());
    /// let m = r.unwrap();
    /// assert_eq!(m.os_value_of("arg").unwrap().as_bytes(), &[0xe9]);
    /// ```
    AllowInvalidUtf8,
    /// Specifies whether or not leading hyphens are allowed in argument values, such as `-10`
    ///
    /// **NOTE:** This can only be set application wide
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Arg, App, AppSettings};
    /// // Imagine you needed to represent negative numbers as well, such as -10
    /// let m = App::new("nums")
    ///     .setting(AppSettings::AllowLeadingHyphen)
    ///     .arg(Arg::with_name("neg"))
    ///     .get_matches_from(vec![
    ///         "nums", "-20"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("neg"), Some("-20"));
    /// # ;
    AllowLeadingHyphen,
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
            "AllowExternalSubcommands" => Ok(AppSettings::AllowExternalSubcommands),
            "trailingvararg" => Ok(AppSettings::TrailingVarArg),
            "nobinaryname" => Ok(AppSettings::NoBinaryName),
            "allowexternalsubcommands" => Ok(AppSettings::AllowExternalSubcommands),
            "strictutf8" => Ok(AppSettings::StrictUtf8),
            "allowinvalidutf8" => Ok(AppSettings::AllowInvalidUtf8),
            "allowleadinghyphen" => Ok(AppSettings::AllowLeadingHyphen),
            _ => Err("unknown AppSetting, cannot convert from str".to_owned()),
        }
    }
}
