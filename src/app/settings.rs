// Std
use std::ascii::AsciiExt;
use std::str::FromStr;
use std::ops::BitOr;

bitflags! {
    flags Flags: u64 {
        const SC_NEGATE_REQS       = 0b0000000000000000000000000000000001,
        const SC_REQUIRED          = 0b0000000000000000000000000000000010,
        const A_REQUIRED_ELSE_HELP = 0b0000000000000000000000000000000100,
        const GLOBAL_VERSION       = 0b0000000000000000000000000000001000,
        const VERSIONLESS_SC       = 0b0000000000000000000000000000010000,
        const UNIFIED_HELP         = 0b0000000000000000000000000000100000,
        const WAIT_ON_ERROR        = 0b0000000000000000000000000001000000,
        const SC_REQUIRED_ELSE_HELP= 0b0000000000000000000000000010000000,
        const NEEDS_LONG_HELP      = 0b0000000000000000000000000100000000,
        const NEEDS_LONG_VERSION   = 0b0000000000000000000000001000000000,
        const NEEDS_SC_HELP        = 0b0000000000000000000000010000000000,
        const DISABLE_VERSION      = 0b0000000000000000000000100000000000,
        const HIDDEN               = 0b0000000000000000000001000000000000,
        const TRAILING_VARARG      = 0b0000000000000000000010000000000000,
        const NO_BIN_NAME          = 0b0000000000000000000100000000000000,
        const ALLOW_UNK_SC         = 0b0000000000000000001000000000000000,
        const UTF8_STRICT          = 0b0000000000000000010000000000000000,
        const UTF8_NONE            = 0b0000000000000000100000000000000000,
        const LEADING_HYPHEN       = 0b0000000000000001000000000000000000,
        const NO_POS_VALUES        = 0b0000000000000010000000000000000000,
        const NEXT_LINE_HELP       = 0b0000000000000100000000000000000000,
        const DERIVE_DISP_ORDER    = 0b0000000000001000000000000000000000,
        const COLORED_HELP         = 0b0000000000010000000000000000000000,
        const COLOR_ALWAYS         = 0b0000000000100000000000000000000000,
        const COLOR_AUTO           = 0b0000000001000000000000000000000000,
        const COLOR_NEVER          = 0b0000000010000000000000000000000000,
        const DONT_DELIM_TRAIL     = 0b0000000100000000000000000000000000,
        const ALLOW_NEG_NUMS       = 0b0000001000000000000000000000000000,
        const LOW_INDEX_MUL_POS    = 0b0000010000000000000000000000000000,
        const DISABLE_HELP_SC      = 0b0000100000000000000000000000000000,
        const DONT_COLLAPSE_ARGS   = 0b0001000000000000000000000000000000,
        const ARGS_NEGATE_SCS      = 0b0010000000000000000000000000000000,
        const PROPAGATE_VALS_DOWN  = 0b0100000000000000000000000000000000,
        const ALLOW_MISSING_POS    = 0b1000000000000000000000000000000000,
    }
}

#[doc(hidden)]
#[derive(Debug, Copy)]
pub struct AppFlags(Flags);

impl Clone for AppFlags {
    fn clone(&self) -> Self { AppFlags(self.0) }
}

impl BitOr for AppFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        AppFlags(self.0 | rhs.0)
    }
}

impl Default for AppFlags {
    fn default() -> Self {
        AppFlags(NEEDS_LONG_VERSION | NEEDS_LONG_HELP | NEEDS_SC_HELP | UTF8_NONE | COLOR_AUTO)
    }
}

impl AppFlags {
    pub fn new() -> Self { AppFlags::default() }

    impl_settings! { AppSettings,
        ArgRequiredElseHelp => A_REQUIRED_ELSE_HELP,
        ArgsNegateSubcommands => ARGS_NEGATE_SCS,
        AllowExternalSubcommands => ALLOW_UNK_SC,
        AllowInvalidUtf8 => UTF8_NONE,
        AllowLeadingHyphen => LEADING_HYPHEN,
        AllowNegativeNumbers => ALLOW_NEG_NUMS,
        AllowMissingPositional => ALLOW_MISSING_POS,
        ColoredHelp => COLORED_HELP,
        ColorAlways => COLOR_ALWAYS,
        ColorAuto => COLOR_AUTO,
        ColorNever => COLOR_NEVER,
        DontDelimitTrailingValues => DONT_DELIM_TRAIL,
        DontCollapseArgsInUsage => DONT_COLLAPSE_ARGS,
        DeriveDisplayOrder => DERIVE_DISP_ORDER,
        DisableHelpSubcommand => DISABLE_HELP_SC,
        DisableVersion => DISABLE_VERSION,
        GlobalVersion => GLOBAL_VERSION,
        HidePossibleValuesInHelp => NO_POS_VALUES,
        Hidden => HIDDEN,
        LowIndexMultiplePositional => LOW_INDEX_MUL_POS,
        NeedsLongHelp => NEEDS_LONG_HELP,
        NeedsLongVersion => NEEDS_LONG_VERSION,
        NeedsSubcommandHelp => NEEDS_SC_HELP,
        NoBinaryName => NO_BIN_NAME,
        PropagateGlobalValuesDown=> PROPAGATE_VALS_DOWN,
        StrictUtf8 => UTF8_STRICT,
        SubcommandsNegateReqs => SC_NEGATE_REQS,
        SubcommandRequired => SC_REQUIRED,
        SubcommandRequiredElseHelp => SC_REQUIRED_ELSE_HELP,
        TrailingVarArg => TRAILING_VARARG,
        UnifiedHelpMessage => UNIFIED_HELP,
        NextLineHelp => NEXT_LINE_HELP,
        VersionlessSubcommands => VERSIONLESS_SC,
        WaitOnError => WAIT_ON_ERROR
    }
}

/// Application level settings, which affect how [`App`] operates
///
/// **NOTE:** When these settings are used, they apply only to current command, and are *not*
/// propagated down or up through child or parent subcommands
///
/// [`App`]: ./struct.App.html
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AppSettings {
    /// Specifies that any invalid UTF-8 code points should *not* be treated as an error.
    /// This is the default behavior of `clap`.
    ///
    /// **NOTE:** Using argument values with invalid UTF-8 code points requires using
    /// [`ArgMatches::os_value_of`], [`ArgMatches::os_values_of`], [`ArgMatches::lossy_value_of`],
    /// or [`ArgMatches::lossy_values_of`] for those particular arguments which may contain invalid
    /// UTF-8 values
    ///
    /// **NOTE:** This rule only applies to  argument values, as flags, options, and
    /// [`SubCommand`]s themselves only allow valid UTF-8 code points.
    ///
    /// # Platform Specific
    ///
    /// Non Windows systems only
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc=" ```ignore")]
    #[cfg_attr(    unix , doc=" ```")]
    /// # use clap::{App, AppSettings};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let r = App::new("myprog")
    ///   //.setting(AppSettings::AllowInvalidUtf8)
    ///     .arg_from_usage("<arg> 'some positional arg'")
    ///     .get_matches_from_safe(
    ///         vec![
    ///             OsString::from("myprog"),
    ///             OsString::from_vec(vec![0xe9])]);
    ///
    /// assert!(r.is_ok());
    /// let m = r.unwrap();
    /// assert_eq!(m.value_of_os("arg").unwrap().as_bytes(), &[0xe9]);
    /// ```
    /// [`ArgMatches::os_value_of`]: ./struct.ArgMatches.html#method.os_value_of
    /// [`ArgMatches::os_values_of`]: ./struct.ArgMatches.html#method.os_values_of
    /// [`ArgMatches::lossy_value_of`]: ./struct.ArgMatches.html#method.lossy_value_of
    /// [`ArgMatches::lossy_values_of`]: ./struct.ArgMatches.html#method.lossy_values_of
    AllowInvalidUtf8,

    /// Specifies that leading hyphens are allowed in argument *values*, such as negative numbers
    /// like `-10`. (which would otherwise be parsed as another flag or option)
    ///
    /// **NOTE:** This can only be set application wide and not on a per argument basis.
    ///
    /// **NOTE:** Use this setting with caution as it silences certain circumstances which would
    /// otherwise be an error (such as accidentally forgetting to specify a value for leading
    /// option)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Arg, App, AppSettings};
    /// // Imagine you needed to represent negative numbers as well, such as -10
    /// let m = App::new("nums")
    ///     .setting(AppSettings::AllowLeadingHyphen)
    ///     .arg(Arg::with_name("neg").index(1))
    ///     .get_matches_from(vec![
    ///         "nums", "-20"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("neg"), Some("-20"));
    /// # ;
    /// ```
    AllowLeadingHyphen,

    /// Allows negative numbers to pass as values. This is similar to
    /// `AllowLeadingHyphen` except that it only allows numbers, all
    /// other undefined leading hyphens will fail to parse.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::AllowNegativeNumbers)
    ///     .arg(Arg::with_name("num"))
    ///     .get_matches_from_safe(vec![
    ///         "myprog", "-20"
    ///     ]);
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert_eq!(m.value_of("num").unwrap(), "-20");
    /// ```
    /// [`AllowLeadingHyphen`]: ./enum.AppSettings.html#variant.AllowLeadingHyphen
    AllowNegativeNumbers,

    /// Allows one to implement a CLI where the second to last positional argument is optional, but
    /// the final positional argument is required. Such as `$ prog [optional] <required>` where one
    /// of the two following usages is allowed:
    ///
    /// * `$ prog [optional] <required>`
    /// * `$ prog <required>`
    ///
    /// This would otherwise not be allowed. This is useful when `[optional]` has a default value.
    ///
    /// **Note:** In addition to using this setting, the second positional argument *must* be 
    /// [required]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowMissingPositional)
    ///     .arg(Arg::with_name("arg1")
    ///         .default_value("something"))
    ///     .arg(Arg::with_name("arg2")
    ///         .required(true))
    ///     .get_matches_from(vec![
    ///         "myprog", "other"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("arg1"), Some("something"));
    /// assert_eq!(m.value_of("arg2"), Some("other"));
    /// ```
    /// [required]: ./struct.Arg.html#method.required
    AllowMissingPositional,

    /// Specifies that an unexpected positional argument,
    /// which would otherwise cause a [`ErrorKind::UnknownArgument`] error,
    /// should instead be treated as a [`SubCommand`] within the [`ArgMatches`] struct.
    ///
    /// **NOTE:** Use this setting with caution,
    /// as a truly unexpected argument (i.e. one that is *NOT* an external subcommand)
    /// will **not** cause an error and instead be treated as a potential subcommand.
    /// One should check for such cases manually and inform the user appropriately.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowExternalSubcommands)
    ///     .get_matches_from(vec![
    ///         "myprog", "subcmd", "--option", "value", "-fff", "--flag"
    ///     ]);
    ///
    /// // All trailing arguments will be stored under the subcommand's sub-matches using an empty
    /// // string argument name
    /// match m.subcommand() {
    ///     (external, Some(ext_m)) => {
    ///          let ext_args: Vec<&str> = ext_m.values_of("").unwrap().collect();
    ///          assert_eq!(external, "subcmd");
    ///          assert_eq!(ext_args, ["--option", "value", "-fff", "--flag"]);
    ///     },
    ///     _ => {},
    /// }
    /// ```
    /// [`ErrorKind::UnknownArgument`]: ./enum.ErrorKind.html#variant.UnknownArgument
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    AllowExternalSubcommands,

    /// Specifies that use of a valid [argument] negates [subcomands] being used after. By default
    /// `clap` allows arguments between subcommands such as 
    /// `<cmd> [cmd_args] <cmd2> [cmd2_args] <cmd3> [cmd3_args]`. This setting disables that 
    /// functionality and says that arguments can only follow the *final* subcommand. For instance 
    /// using this setting makes only the following invocations possible:
    ///
    /// * `<cmd> <cmd2> <cmd3> [cmd3_args]`
    /// * `<cmd> <cmd2> [cmd2_args]`
    /// * `<cmd> [cmd_args]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ArgsNegateSubcommands)
    /// # ;
    /// ```
    /// [subcommands]: ./struct.SubCommand.html
    /// [argument]: ./struct.Arg.html
    ArgsNegateSubcommands,

    /// Specifies that the help text should be displayed (and then exit gracefully),
    /// if no arguments are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **NOTE:** [`SubCommand`]s count as arguments
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    ArgRequiredElseHelp,

    /// Uses colorized help messages.
    ///
    /// **NOTE:** Must be compiled with the `color` cargo feature
    ///
    /// # Platform Specific
    ///
    /// This setting only applies to Unix, Linux, and OSX (i.e. non-Windows platforms)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ColoredHelp)
    ///     .get_matches();
    /// ```
    ColoredHelp,

    /// Enables colored output only when the output is going to a terminal or TTY.
    ///
    /// **NOTE:** This is the default behavior of `clap`.
    ///
    /// **NOTE:** Must be compiled with the `color` cargo feature.
    ///
    /// # Platform Specific
    ///
    /// This setting only applies to Unix, Linux, and OSX (i.e. non-Windows platforms).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ColorAuto)
    ///     .get_matches();
    /// ```
    ColorAuto,

    /// Enables colored output regardless of whether or not the output is going to a terminal/TTY.
    ///
    /// **NOTE:** Must be compiled with the `color` cargo feature.
    ///
    /// # Platform Specific
    ///
    /// This setting only applies to Unix, Linux, and OSX (i.e. non-Windows platforms).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ColorAlways)
    ///     .get_matches();
    /// ```
    ColorAlways,

    /// Disables colored output no matter if the output is going to a terminal/TTY, or not.
    ///
    /// **NOTE:** Must be compiled with the `color` cargo feature
    ///
    /// # Platform Specific
    ///
    /// This setting only applies to Unix, Linux, and OSX (i.e. non-Windows platforms)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ColorNever)
    ///     .get_matches();
    /// ```
    ColorNever,

    /// Disables the automatic collapsing of positional args into `[ARGS]` inside the usage string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::DontCollapseArgsInUsage)
    ///     .get_matches();
    /// ```
    DontCollapseArgsInUsage,

    /// Disables the automatic delimiting of values when `--` or [`AppSettings::TrailingVarArg`]
    /// was used.
    ///
    /// **NOTE:** The same thing can be done manually by setting the final positional argument to
    /// [`Arg::use_delimiter(false)`]. Using this setting is safer, because it's easier to locate
    /// when making changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::DontDelimitTrailingValues)
    ///     .get_matches();
    /// ```
    /// [`AppSettings::TrailingVarArg`]: ./enum.AppSettings.html#variant.TrailingVarArg
    /// [`Arg::use_delimiter(false)`]: ./struct.Arg.html#method.use_delimiter
    DontDelimitTrailingValues,

    /// Disables the `help` subcommand 
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind, SubCommand};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::DisableHelpSubcommand)
    ///     // Normally, creating a subcommand causes a `help` subcommand to automaticaly
    ///     // be generated as well
    ///     .subcommand(SubCommand::with_name("test")) 
    ///     .get_matches_from_safe(vec![
    ///         "myprog", "help"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    DisableHelpSubcommand,

    /// Disables `-V` and `--version` [`App`] without affecting any of the [`SubCommand`]s
    /// (Defaults to `false`; application *does* have a version flag)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::DisableVersion)
    ///     .get_matches_from_safe(vec![
    ///         "myprog", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, SubCommand, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::DisableVersion)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches_from_safe(vec![
    ///         "myprog", "test", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::VersionDisplayed);
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    DisableVersion,

    /// Displays the arguments and [`SubCommand`]s in the help message in the order that they were
    /// declared in, and not alphabetically which is the default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::DeriveDisplayOrder)
    ///     .get_matches();
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    DeriveDisplayOrder,

    /// Specifies to use the version of the current command for all child [`SubCommand`]s.
    /// (Defaults to `false`; subcommands have independant version strings from their parents.)
    ///
    /// **NOTE:** The version for the current command **and** this setting must be set **prior** to
    /// adding any child subcommands
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
    /// // running `$ myprog test --version` will display
    /// // "myprog-test v1.1"
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    GlobalVersion,

    /// Specifies that this [`SubCommand`] should be hidden from help messages
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, SubCommand};
    /// App::new("myprog")
    ///     .subcommand(SubCommand::with_name("test")
    ///     .setting(AppSettings::Hidden))
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    Hidden,

    /// Tells `clap` *not* to print possible values when displaying help information.
    /// This can be useful if there are many values, or they are explained elsewhere.
    HidePossibleValuesInHelp,

    /// Specifies that the parser should not assume the first argument passed is the binary name.
    /// This is normally the case when using a "daemon" style mode, or an interactive CLI where one
    /// one would not normally type the binary or program name for each command.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::NoBinaryName)
    ///     .arg(Arg::from_usage("<cmd>... 'commands to run'"))
    ///     .get_matches_from(vec!["command", "set"]);
    ///
    /// let cmds: Vec<&str> = m.values_of("cmd").unwrap().collect();
    /// assert_eq!(cmds, ["command", "set"]);
    /// ```
    NoBinaryName,

    /// Places the help string for all arguments on the line after the argument.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::NextLineHelp)
    ///     .get_matches();
    /// ```
    NextLineHelp,

    /// Specifies that the parser should propagate global arg's values down through any *used* child
    /// subcommands. Meaning, if a subcommand wasn't used, the values won't be propagated down to
    /// said subcommand.
    ///
    /// **NOTE:** Values are only propagated *down* through futher child commands, not up
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, SubCommand};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::PropagateGlobalValuesDown)
    ///     .arg(Arg::from_usage("[cmd] 'command to run'")
    ///         .global(true))
    ///     .subcommand(SubCommand::with_name("foo"))
    ///     .get_matches_from(vec!["myprog", "set", "foo"]);
    ///
    /// assert_eq!(m.value_of("cmd"), Some("set"));
    ///
    /// let sub_m = m.subcommand_matches("foo").unwrap();
    /// assert_eq!(sub_m.value_of("cmd"), Some("set"));
    /// ```
    /// Now doing the same thing, but *not* using any subcommands will result in the value not being
    /// propagated down.
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::PropagateGlobalValuesDown)
    ///     .global_arg(Arg::from_usage("<cmd> 'command to run'"))
    ///     .subcommand(SubCommand::with_name("foo"))
    ///     .get_matches_from(vec!["myprog", "set"]);
    ///
    /// assert_eq!(m.value_of("cmd"), Some("set"));
    ///
    /// assert!(m.subcommand_matches("foo").is_none());
    /// ```
    PropagateGlobalValuesDown,

    /// Allows [`SubCommand`]s to override all requirements of the parent command.
    /// For example if you had a subcommand or top level application with a required argument
    /// that is only required as long as there is no subcommand present,
    /// using this setting would allow you to set those arguments to [`Arg::required(true)`]
    /// and yet receive no error so long as the user uses a valid subcommand instead.
    ///
    /// **NOTE:** This defaults to false (using subcommand does *not* negate requirements)
    ///
    /// # Examples
    ///
    /// This first example shows that it is an error to not use a required argument
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, SubCommand, ErrorKind};
    /// let err = App::new("myprog")
    ///     .setting(AppSettings::SubcommandsNegateReqs)
    ///     .arg(Arg::with_name("opt").required(true))
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches_from_safe(vec![
    ///         "myprog"
    ///     ]);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// # ;
    /// ```
    ///
    /// This next example shows that it is no longer error to not use a required argument if a
    /// valid subcommand is used.
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, SubCommand, ErrorKind};
    /// let noerr = App::new("myprog")
    ///     .setting(AppSettings::SubcommandsNegateReqs)
    ///     .arg(Arg::with_name("opt").required(true))
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches_from_safe(vec![
    ///         "myprog", "test"
    ///     ]);
    /// assert!(noerr.is_ok());
    /// # ;
    /// ```
    /// [`Arg::required(true)`]: ./struct.Arg.html#method.required
    /// [`SubCommand`]: ./struct.SubCommand.html
    SubcommandsNegateReqs,

    /// Specifies that the help text should be displayed (before exiting gracefully) if no
    /// [`SubCommand`]s are present at runtime (i.e. an empty run such as `$ myprog`).
    ///
    /// **NOTE:** This should *not* be used with [`AppSettings::SubcommandRequired`] as they do
    /// nearly same thing; this prints the help text, and the other prints an error.
    ///
    /// **NOTE:** If the user specifies arguments at runtime, but no subcommand the help text will
    /// still be displayed and exit. If this is *not* the desired result, consider using
    /// [`AppSettings::ArgRequiredElseHelp`] instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequiredElseHelp)
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`AppSettings::SubcommandRequired`]: ./enum.AppSettings.html#variant.SubcommandRequired
    /// [`AppSettings::ArgRequiredElseHelp`]: ./enum.AppSettings.html#variant.ArgRequiredElseHelp
    SubcommandRequiredElseHelp,

    /// Specifies that any invalid UTF-8 code points should be treated as an error and fail
    /// with a [`ErrorKind::InvalidUtf8`] error.
    ///
    /// **NOTE:** This rule only applies to argument values; Things such as flags, options, and
    /// [`SubCommand`]s themselves only allow valid UTF-8 code points.
    ///
    /// # Platform Specific
    ///
    /// Non Windows systems only
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc=" ```ignore")]
    #[cfg_attr(    unix , doc=" ```")]
    /// # use clap::{App, AppSettings, ErrorKind};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStringExt;
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
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    /// [`ErrorKind::InvalidUtf8`]: ./enum.ErrorKind.html#variant.InvalidUtf8
    StrictUtf8,

    /// Allows specifying that if no [`SubCommand`] is present at runtime,
    /// error and exit gracefully.
    ///
    /// **NOTE:** This defaults to `false` (subcommands do *not* need to be present)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, SubCommand, ErrorKind};
    /// let err = App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches_from_safe(vec![
    ///         "myprog",
    ///     ]);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err().kind, ErrorKind::MissingSubcommand);
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    SubcommandRequired,

    /// Specifies that the final positional argument is a "VarArg" and that `clap` should not
    /// attempt to parse any further args.
    ///
    /// The values of the trailing positional argument will contain all args from itself on.
    ///
    /// **NOTE:** The final positional argument **must** have [`Arg::multiple(true)`] or the usage
    /// string equivalent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::TrailingVarArg)
    ///     .arg(Arg::from_usage("<cmd>... 'commands to run'"))
    ///     .get_matches_from(vec!["myprog", "arg1", "-r", "val1"]);
    ///
    /// let trail: Vec<&str> = m.values_of("cmd").unwrap().collect();
    /// assert_eq!(trail, ["arg1", "-r", "val1"]);
    /// ```
    /// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
    TrailingVarArg,

    /// Groups flags and options together, presenting a more unified help message
    /// (a la `getopts` or `docopt` style).
    ///
    /// The default is that the auto-generated help message will group flags, and options
    /// separately.
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

    /// Disables `-V` and `--version` for all [`SubCommand`]s
    /// (Defaults to `false`; subcommands *do* have version flags.)
    ///
    /// **NOTE:** This setting must be set **prior** adding any subcommands
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, SubCommand, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::VersionlessSubcommands)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches_from_safe(vec![
    ///         "myprog", "test", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    VersionlessSubcommands,

    /// Will display a message "Press [ENTER]/[RETURN] to continue..." and wait for user before
    /// exiting
    ///
    /// This is most useful when writing an application which is run from a GUI shortcut, or on
    /// Windows where a user tries to open the binary by double-clicking instead of using the
    /// command line.
    ///
    /// **NOTE:** This setting is **not** recursive with [`SubCommand`]s, meaning if you wish this
    /// behavior for all subcommands, you must set this on each command (needing this is extremely
    /// rare)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::WaitOnError)
    /// # ;
    /// ```
    /// [`SubCommand`]: ./struct.SubCommand.html
    WaitOnError,

    #[doc(hidden)]
    NeedsLongVersion,

    #[doc(hidden)]
    NeedsLongHelp,

    #[doc(hidden)]
    NeedsSubcommandHelp,

    #[doc(hidden)]
    LowIndexMultiplePositional,
}

impl FromStr for AppSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "argrequiredelsehelp" => Ok(AppSettings::ArgRequiredElseHelp),
            "argsnegatesubcommands" => Ok(AppSettings::ArgsNegateSubcommands),
            "allowinvalidutf8" => Ok(AppSettings::AllowInvalidUtf8),
            "allowleadinghyphen" => Ok(AppSettings::AllowLeadingHyphen),
            "allowexternalsubcommands" => Ok(AppSettings::AllowExternalSubcommands),
            "allownegativenumbers" => Ok(AppSettings::AllowNegativeNumbers),
            "colorauto" => Ok(AppSettings::ColorAuto),
            "coloralways" => Ok(AppSettings::ColorAlways),
            "colornever" => Ok(AppSettings::ColorNever),
            "coloredhelp" => Ok(AppSettings::ColoredHelp),
            "derivedisplayorder" => Ok(AppSettings::DeriveDisplayOrder),
            "dontcollapseargsinusage" => Ok(AppSettings::DontCollapseArgsInUsage),
            "dontdelimittrailingvalues" => Ok(AppSettings::DontDelimitTrailingValues),
            "disablehelpsubcommand" => Ok(AppSettings::DisableHelpSubcommand),
            "disableversion" => Ok(AppSettings::DisableVersion),
            "globalversion" => Ok(AppSettings::GlobalVersion),
            "hidden" => Ok(AppSettings::Hidden),
            "hidepossiblevaluesinhelp" => Ok(AppSettings::HidePossibleValuesInHelp),
            "lowindexmultiplepositional" => Ok(AppSettings::LowIndexMultiplePositional),
            "nobinaryname" => Ok(AppSettings::NoBinaryName),
            "nextlinehelp" => Ok(AppSettings::NextLineHelp),
            "strictutf8" => Ok(AppSettings::StrictUtf8),
            "subcommandsnegatereqs" => Ok(AppSettings::SubcommandsNegateReqs),
            "subcommandrequired" => Ok(AppSettings::SubcommandRequired),
            "subcommandrequiredelsehelp" => Ok(AppSettings::SubcommandRequiredElseHelp),
            "trailingvararg" => Ok(AppSettings::TrailingVarArg),
            "unifiedhelpmessage" => Ok(AppSettings::UnifiedHelpMessage),
            "versionlesssubcommands" => Ok(AppSettings::VersionlessSubcommands),
            "waitonerror" => Ok(AppSettings::WaitOnError),
            _ => Err("unknown AppSetting, cannot convert from str".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::AppSettings;

    #[test]
    fn app_settings_fromstr() {
        assert_eq!("argsnegatesubcommands".parse::<AppSettings>().unwrap(),
                   AppSettings::ArgsNegateSubcommands);
        assert_eq!("argrequiredelsehelp".parse::<AppSettings>().unwrap(),
                   AppSettings::ArgRequiredElseHelp);
        assert_eq!("allowexternalsubcommands".parse::<AppSettings>().unwrap(),
                   AppSettings::AllowExternalSubcommands);
        assert_eq!("allowinvalidutf8".parse::<AppSettings>().unwrap(),
                   AppSettings::AllowInvalidUtf8);
        assert_eq!("allowleadinghyphen".parse::<AppSettings>().unwrap(),
                   AppSettings::AllowLeadingHyphen);
        assert_eq!("allownegativenumbers".parse::<AppSettings>().unwrap(),
                   AppSettings::AllowNegativeNumbers);
        assert_eq!("coloredhelp".parse::<AppSettings>().unwrap(),
                   AppSettings::ColoredHelp);
        assert_eq!("colorauto".parse::<AppSettings>().unwrap(),
                   AppSettings::ColorAuto);
        assert_eq!("coloralways".parse::<AppSettings>().unwrap(),
                   AppSettings::ColorAlways);
        assert_eq!("colornever".parse::<AppSettings>().unwrap(),
                   AppSettings::ColorNever);
        assert_eq!("disablehelpsubcommand".parse::<AppSettings>().unwrap(),
                   AppSettings::DisableHelpSubcommand);
        assert_eq!("disableversion".parse::<AppSettings>().unwrap(),
                   AppSettings::DisableVersion);
        assert_eq!("dontcollapseargsinusage".parse::<AppSettings>().unwrap(),
                   AppSettings::DontCollapseArgsInUsage);
        assert_eq!("dontdelimittrailingvalues".parse::<AppSettings>().unwrap(),
                   AppSettings::DontDelimitTrailingValues);
        assert_eq!("derivedisplayorder".parse::<AppSettings>().unwrap(),
                   AppSettings::DeriveDisplayOrder);
        assert_eq!("globalversion".parse::<AppSettings>().unwrap(),
                   AppSettings::GlobalVersion);
        assert_eq!("hidden".parse::<AppSettings>().unwrap(),
                   AppSettings::Hidden);
        assert_eq!("hidepossiblevaluesinhelp".parse::<AppSettings>().unwrap(),
                   AppSettings::HidePossibleValuesInHelp);
        assert_eq!("lowindexmultiplePositional".parse::<AppSettings>().unwrap(),
                   AppSettings::LowIndexMultiplePositional);
        assert_eq!("nobinaryname".parse::<AppSettings>().unwrap(),
                   AppSettings::NoBinaryName);
        assert_eq!("nextlinehelp".parse::<AppSettings>().unwrap(),
                   AppSettings::NextLineHelp);
        assert_eq!("subcommandsnegatereqs".parse::<AppSettings>().unwrap(),
                   AppSettings::SubcommandsNegateReqs);
        assert_eq!("subcommandrequired".parse::<AppSettings>().unwrap(),
                   AppSettings::SubcommandRequired);
        assert_eq!("subcommandrequiredelsehelp".parse::<AppSettings>().unwrap(),
                   AppSettings::SubcommandRequiredElseHelp);
        assert_eq!("strictutf8".parse::<AppSettings>().unwrap(),
                   AppSettings::StrictUtf8);
        assert_eq!("trailingvararg".parse::<AppSettings>().unwrap(),
                   AppSettings::TrailingVarArg);
        assert_eq!("unifiedhelpmessage".parse::<AppSettings>().unwrap(),
                   AppSettings::UnifiedHelpMessage);
        assert_eq!("versionlesssubcommands".parse::<AppSettings>().unwrap(),
                   AppSettings::VersionlessSubcommands);
        assert_eq!("waitonerror".parse::<AppSettings>().unwrap(),
                   AppSettings::WaitOnError);
        assert!("hahahaha".parse::<AppSettings>().is_err());
    }
}
