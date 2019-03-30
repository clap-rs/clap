// Std
use std::ops::BitOr;
use std::str::FromStr;

bitflags! {
    struct CmdFlags: u64 {
        const SC_NEGATE_REQS       = 1;
        const SC_REQUIRED          = 1 << 1;
        const A_REQUIRED_ELSE_HELP = 1 << 2;
        const GLOBAL_VERSION       = 1 << 3;
        const VERSIONLESS_SC       = 1 << 4;
        const UNIFIED_HELP         = 1 << 5;
        const SC_REQUIRED_ELSE_HELP= 1 << 6;
        const NO_AUTO_HELP         = 1 << 7;
        const NO_AUTO_VERSION      = 1 << 8;
        const DISABLE_VERSION      = 1 << 9;
        const HIDDEN               = 1 << 10;
        const TRAILING_VARARG      = 1 << 11;
        const ALLOW_UNK_SC         = 1 << 12;
        const UTF8_STRICT          = 1 << 13;
        const UTF8_NONE            = 1 << 14;
        const LEADING_HYPHEN       = 1 << 15;
        const NO_POS_VALUES        = 1 << 16;
        const NEXT_LINE_HELP       = 1 << 17;
        const DERIVE_DISP_ORDER    = 1 << 28;
        const COLORED_HELP         = 1 << 29;
        const COLOR_ALWAYS         = 1 << 30;
        const COLOR_AUTO           = 1 << 31;
        const COLOR_NEVER          = 1 << 32;
        const DONT_DELIM_TRAIL     = 1 << 33;
        const ALLOW_NEG_NUMS       = 1 << 34;
        const LOW_INDEX_MUL_POS    = 1 << 35;
        const DISABLE_HELP_SC      = 1 << 36;
        const DONT_COLLAPSE_ARGS   = 1 << 37;
        const ARGS_NEGATE_SCS      = 1 << 38;
        const PROPAGATE_VALS_DOWN  = 1 << 39;
        const ALLOW_MISSING_POS    = 1 << 40;
        const TRAILING_VALUES      = 1 << 41;
        const VALID_NEG_NUM_FOUND  = 1 << 42;
        const PROPAGATED           = 1 << 43;
        const VALID_ARG_FOUND      = 1 << 44;
        const INFER_SUBCOMMANDS    = 1 << 45;
        const CONTAINS_LAST        = 1 << 46;
        const ARGS_OVERRIDE_SELF   = 1 << 47;
    }
}

impl_settings! { CmdFlags, CmdSettings,
    ArgRequiredElseHelp => Flags::A_REQUIRED_ELSE_HELP,
    ArgsNegateSubcommands => Flags::ARGS_NEGATE_SCS,
    AllowExternalSubcommands => Flags::ALLOW_UNK_SC,
    AllowInvalidUtf8 => Flags::UTF8_NONE,
    AllowLeadingHyphen => Flags::LEADING_HYPHEN,
    AllowNegativeNumbers => Flags::ALLOW_NEG_NUMS,
    AllowMissingPositional => Flags::ALLOW_MISSING_POS,
    ColoredHelp => Flags::COLORED_HELP,
    ColorAlways => Flags::COLOR_ALWAYS,
    ColorAuto => Flags::COLOR_AUTO,
    ColorNever => Flags::COLOR_NEVER,
    DontDelimitTrailingValues => Flags::DONT_DELIM_TRAIL,
    DontCollapseArgsInUsage => Flags::DONT_COLLAPSE_ARGS,
    DeriveDisplayOrder => Flags::DERIVE_DISP_ORDER,
    DisableHelpSubcommand => Flags::DISABLE_HELP_SC,
    DisableVersion => Flags::DISABLE_VERSION,
    GlobalVersion => Flags::GLOBAL_VERSION,
    HidePossibleValuesInHelp => Flags::NO_POS_VALUES,
    Hidden => Flags::HIDDEN,
    LowIndexMultiplePositional => Flags::LOW_INDEX_MUL_POS,
    NoAutoHelp => Flags::NO_AUTO_HELP,
    NoAutoVersion => Flags::NO_AUTO_VERSION,
    PropagateGlobalValuesDown=> Flags::PROPAGATE_VALS_DOWN,
    StrictUtf8 => Flags::UTF8_STRICT,
    SubcommandsNegateReqs => Flags::SC_NEGATE_REQS,
    SubcommandRequired => Flags::SC_REQUIRED,
    SubcommandRequiredElseHelp => Flags::SC_REQUIRED_ELSE_HELP,
    TrailingVarArg => Flags::TRAILING_VARARG,
    UnifiedHelpMessage => Flags::UNIFIED_HELP,
    NextLineHelp => Flags::NEXT_LINE_HELP,
    VersionlessSubcommands => Flags::VERSIONLESS_SC,
    TrailingValues => Flags::TRAILING_VALUES,
    ValidNegNumFound => Flags::VALID_NEG_NUM_FOUND,
    Propagated => Flags::PROPAGATED,
    ValidArgFound => Flags::VALID_ARG_FOUND,
    InferSubcommands => Flags::INFER_SUBCOMMANDS,
    AllArgsOverrideSelf => Flags::ARGS_OVERRIDE_SELF,
    ContainsLast => Flags::CONTAINS_LAST
}

/// Command level settings, which affect how [`Cmd`] operates
///
/// **NOTE:** When these settings are used, they apply only to current command, and are *not*
/// propagated down or up through child or parent subcommands
///
/// [`Cmd`]: ./struct.Cmd.html
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CmdSettings {
    /// Specifies that any invalid UTF-8 code points should *not* be treated as an error.
    /// This is the default behavior of `clap`.
    ///
    /// **NOTE:** Using argument values with invalid UTF-8 code points requires using
    /// [`ArgMatches::os_value_of`], [`ArgMatches::os_values_of`], [`ArgMatches::lossy_value_of`],
    /// or [`ArgMatches::lossy_values_of`] for those particular arguments which may contain invalid
    /// UTF-8 values
    ///
    /// **NOTE:** This rule only applies to  argument values, as flags, options, and
    /// [``]s themselves only allow valid UTF-8 code points.
    ///
    /// # Platform Specific
    ///
    /// Non Windows systems only
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{Cmd, CmdSettings};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let r = Cmd::new("myprog")
    ///   //.setting(CmdSettings::AllowInvalidUtf8)
    ///     .arg("<arg> 'some positional arg'")
    ///     .try_get_matches_from(
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
    /// [``]: ./struct..html
    AllowInvalidUtf8,

    /// Specifies that leading hyphens are allowed in argument *values*, such as negative numbers
    /// like `-10`. (which would otherwise be parsed as another flag or option)
    ///
    /// **NOTE:** Use this setting with caution as it silences certain circumstances which would
    /// otherwise be an error (such as accidentally forgetting to specify a value for leading
    /// option). It is preferred to set this on a per argument basis, via [`Arg::allow_hyphen_values`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Arg, Cmd, CmdSettings};
    /// // Imagine you needed to represent negative numbers as well, such as -10
    /// let m = Cmd::new("nums")
    ///     .setting(CmdSettings::AllowLeadingHyphen)
    ///     .arg(Arg::new("neg").index(1))
    ///     .get_matches_from(vec![
    ///         "nums", "-20"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("neg"), Some("-20"));
    /// # ;
    /// ```
    /// [`Arg::allow_hyphen_values`]: ./struct.Arg.html#method.allow_hyphen_values
    AllowLeadingHyphen,

    /// Specifies that all arguments override themselves. This is the equivolent to saying the `foo`
    /// arg using [`Arg::overrides_with("foo")`] for all defined arguments.
    /// [`Arg::overrides_with("foo")`]: ./struct.Arg.html#method.overrides_with
    AllArgsOverrideSelf,

    /// Allows negative numbers to pass as values. This is similar to
    /// `AllowLeadingHyphen` except that it only allows numbers, all
    /// other undefined leading hyphens will fail to parse.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// let res = Cmd::new("myprog")
    ///     .version("v1.1")
    ///     .setting(CmdSettings::AllowNegativeNumbers)
    ///     .arg(Arg::new("num"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "-20"
    ///     ]);
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert_eq!(m.value_of("num").unwrap(), "-20");
    /// ```
    /// [`AllowLeadingHyphen`]: ./enum.CmdSettings.html#variant.AllowLeadingHyphen
    AllowNegativeNumbers,

    /// Allows one to implement two styles of CLIs where positionals can be used out of order.
    ///
    /// The first example is a CLI where the second to last positional argument is optional, but
    /// the final positional argument is required. Such as `$ prog [optional] <required>` where one
    /// of the two following usages is allowed:
    ///
    /// * `$ prog [optional] <required>`
    /// * `$ prog <required>`
    ///
    /// This would otherwise not be allowed. This is useful when `[optional]` has a default value.
    ///
    /// **Note:** when using this style of "missing positionals" the final positional *must* be
    /// [required] if `--` will not be used to skip to the final positional argument.
    ///
    /// **Note:** This style also only allows a single positional argument to be "skipped" without
    /// the use of `--`. To skip more than one, see the second example.
    ///
    /// The second example is when one wants to skip multiple optional positional arguments, and use
    /// of the `--` operator is OK (but not required if all arguments will be specified anyways).
    ///
    /// For example, imagine a CLI which has three positional arguments `[foo] [bar] [baz]...` where
    /// `baz` accepts multiple values (similar to man `ARGS...` style training arguments).
    ///
    /// With this setting the following invocations are posisble:
    ///
    /// * `$ prog foo bar baz1 baz2 baz3`
    /// * `$ prog foo -- baz1 baz2 baz3`
    /// * `$ prog -- baz1 baz2 baz3`
    ///
    /// # Examples
    ///
    /// Style number one from above:
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = Cmd::new("myprog")
    ///     .setting(CmdSettings::AllowMissingPositional)
    ///     .arg(Arg::new("arg1"))
    ///     .arg(Arg::new("arg2")
    ///         .required(true))
    ///     .get_matches_from(vec![
    ///         "prog", "other"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("arg1"), None);
    /// assert_eq!(m.value_of("arg2"), Some("other"));
    /// ```
    ///
    /// Now the same example, but using a default value for the first optional positional argument
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = Cmd::new("myprog")
    ///     .setting(CmdSettings::AllowMissingPositional)
    ///     .arg(Arg::new("arg1")
    ///         .default_value("something"))
    ///     .arg(Arg::new("arg2")
    ///         .required(true))
    ///     .get_matches_from(vec![
    ///         "prog", "other"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("arg1"), Some("something"));
    /// assert_eq!(m.value_of("arg2"), Some("other"));
    /// ```
    /// Style number two from above:
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = Cmd::new("myprog")
    ///     .setting(CmdSettings::AllowMissingPositional)
    ///     .arg(Arg::new("foo"))
    ///     .arg(Arg::new("bar"))
    ///     .arg(Arg::new("baz").multiple(true))
    ///     .get_matches_from(vec![
    ///         "prog", "foo", "bar", "baz1", "baz2", "baz3"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("foo"), Some("foo"));
    /// assert_eq!(m.value_of("bar"), Some("bar"));
    /// assert_eq!(m.values_of("baz").unwrap().collect::<Vec<_>>(), &["baz1", "baz2", "baz3"]);
    /// ```
    ///
    /// Now nofice if we don't specifiy `foo` or `baz` but use the `--` operator.
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = Cmd::new("myprog")
    ///     .setting(CmdSettings::AllowMissingPositional)
    ///     .arg(Arg::new("foo"))
    ///     .arg(Arg::new("bar"))
    ///     .arg(Arg::new("baz").multiple(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--", "baz1", "baz2", "baz3"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("foo"), None);
    /// assert_eq!(m.value_of("bar"), None);
    /// assert_eq!(m.values_of("baz").unwrap().collect::<Vec<_>>(), &["baz1", "baz2", "baz3"]);
    /// ```
    /// [required]: ./struct.Arg.html#method.required
    AllowMissingPositional,

    /// Specifies that an unexpected positional argument,
    /// which would otherwise cause a [`ErrorKind::UnknownArgument`] error,
    /// should instead be treated as a [``] within the [`ArgMatches`] struct.
    ///
    /// **NOTE:** Use this setting with caution,
    /// as a truly unexpected argument (i.e. one that is *NOT* an external subcommand)
    /// will **not** cause an error and instead be treated as a potential subcommand.
    /// One should check for such cases manually and inform the user appropriately.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, CmdSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = Cmd::new("myprog")
    ///     .setting(CmdSettings::AllowExternalSubcommands)
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
    /// [``]: ./struct..html
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
    /// # use clap::{Cmd, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::ArgsNegateSubcommands)
    /// # ;
    /// ```
    /// [subcommands]: ./struct..html
    /// [argument]: ./struct.Arg.html
    ArgsNegateSubcommands,

    /// Specifies that the help text should be displayed (and then exit gracefully),
    /// if no arguments are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **NOTE:** [``]s count as arguments
    ///
    /// **NOTE:** Setting [`Arg::default_value`] effectively disables this option as it will
    /// ensure that some argument is always present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::ArgRequiredElseHelp)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`Arg::default_value`]: ./struct.Arg.html#method.default_value
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
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::ColoredHelp)
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
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::ColorAuto)
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
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::ColorAlways)
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
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::ColorNever)
    ///     .get_matches();
    /// ```
    ColorNever,

    /// Disables the automatic collapsing of positional args into `[ARGS]` inside the usage string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::DontCollapseArgsInUsage)
    ///     .get_matches();
    /// ```
    DontCollapseArgsInUsage,

    /// Disables the automatic delimiting of values when `--` or [`CmdSettings::TrailingVarArg`]
    /// was used.
    ///
    /// **NOTE:** The same thing can be done manually by setting the final positional argument to
    /// [`Arg::use_delimiter(false)`]. Using this setting is safer, because it's easier to locate
    /// when making changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::DontDelimitTrailingValues)
    ///     .get_matches();
    /// ```
    /// [`CmdSettings::TrailingVarArg`]: ./enum.CmdSettings.html#variant.TrailingVarArg
    /// [`Arg::use_delimiter(false)`]: ./struct.Arg.html#method.use_delimiter
    DontDelimitTrailingValues,

    /// Disables the `help` subcommand
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, CmdSettings, ErrorKind, };
    /// let res = Cmd::new("myprog")
    ///     .version("v1.1")
    ///     .setting(CmdSettings::DisableHelpSubcommand)
    ///     // Normally, creating a subcommand causes a `help` subcommand to automaticaly
    ///     // be generated as well
    ///     .subcommand(Cmd::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "help"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [``]: ./struct..html
    DisableHelpSubcommand,

    /// Disables `-V` and `--version` [`Cmd`] without affecting any of the [``]s
    /// (Defaults to `false`; application *does* have a version flag)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, CmdSettings, ErrorKind};
    /// let res = Cmd::new("myprog")
    ///     .version("v1.1")
    ///     .setting(CmdSettings::DisableVersion)
    ///     .try_get_matches_from(vec![
    ///         "myprog", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    ///
    /// ```rust
    /// # use clap::{Cmd, CmdSettings, ErrorKind};
    /// let res = Cmd::new("myprog")
    ///     .version("v1.1")
    ///     .setting(CmdSettings::DisableVersion)
    ///     .subcommand(Cmd::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "test", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::VersionDisplayed);
    /// ```
    /// [``]: ./struct..html
    /// [`Cmd`]: ./struct.Cmd.html
    DisableVersion,

    /// Displays the arguments and [``]s in the help message in the order that they were
    /// declared in, and not alphabetically which is the default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::DeriveDisplayOrder)
    ///     .get_matches();
    /// ```
    /// [``]: ./struct..html
    DeriveDisplayOrder,

    /// Specifies to use the version of the current command for all child [``]s.
    /// (Defaults to `false`; subcommands have independent version strings from their parents.)
    ///
    /// **NOTE:** The version for the current command **and** this setting must be set **prior** to
    /// adding any child subcommands
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .version("v1.1")
    ///     .setting(CmdSettings::GlobalVersion)
    ///     .subcommand(Cmd::new("test"))
    ///     .get_matches();
    /// // running `$ myprog test --version` will display
    /// // "myprog-test v1.1"
    /// ```
    /// [``]: ./struct..html
    GlobalVersion,

    /// Specifies that this [``] should be hidden from help messages
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings, };
    /// Cmd::new("myprog")
    ///     .subcommand(Cmd::new("test")
    ///     .setting(CmdSettings::Hidden))
    /// # ;
    /// ```
    /// [``]: ./struct..html
    Hidden,

    /// Tells `clap` *not* to print possible values when displaying help information.
    /// This can be useful if there are many values, or they are explained elsewhere.
    HidePossibleValuesInHelp,

    /// Tries to match unknown args to partial [`subcommands`] or their [aliases]. For example to
    /// match a subcommand named `test`, one could use `t`, `te`, `tes`, and `test`.
    ///
    /// **NOTE:** The match *must not* be ambiguous at all in order to succeed. i.e. to match `te`
    /// to `test` there could not also be a subcommand or alias `temp` because both start with `te`
    ///
    /// **CAUTION:** This setting can interfere with [positional/free arguments], take care when
    /// designing CLIs which allow inferred subcommands and have potential positional/free
    /// arguments whose values could start with the same characters as subcommands. If this is the
    /// case, it's recommended to use settings such as [`CmdSeettings::ArgsNegateSubcommands`] in
    /// conjunction with this setting.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// let m = Cmd::new("prog")
    ///     .setting(CmdSettings::InferSubcommands)
    ///     .subcommand(Cmd::new("test"))
    ///     .get_matches_from(vec![
    ///         "prog", "te"
    ///     ]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`subcommands`]: ./struct..html
    /// [positional/free arguments]: ./struct.Arg.html#method.index
    /// [aliases]: ./struct.Cmd.html#method.alias
    /// [`CmdSeettings::ArgsNegateSubcommands`]: ./enum.CmdSettings.html#variant.ArgsNegateSubcommands
    InferSubcommands,

    /// Places the help string for all arguments on the line after the argument.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::NextLineHelp)
    ///     .get_matches();
    /// ```
    NextLineHelp,

    /// **DEPRECATED**: This setting is no longer required in order to propagate values up or down
    ///
    /// Specifies that the parser should propagate global arg's values down or up through any *used*
    /// child subcommands. Meaning, if a subcommand wasn't used, the values won't be propagated to
    /// said subcommand.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings, };
    /// let m = Cmd::new("myprog")
    ///     .arg(Arg::from("[cmd] 'command to run'")
    ///         .global(true))
    ///     .subcommand(Cmd::new("foo"))
    ///     .get_matches_from(vec!["myprog", "set", "foo"]);
    ///
    /// assert_eq!(m.value_of("cmd"), Some("set"));
    ///
    /// let sub_m = m.subcommand_matches("foo").unwrap();
    /// assert_eq!(sub_m.value_of("cmd"), Some("set"));
    /// ```
    /// Now doing the same thing, but *not* using any subcommands will result in the value not being
    /// propagated down.
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings, };
    /// let m = Cmd::new("myprog")
    ///     .arg(Arg::from("[cmd] 'command to run'")
    ///         .global(true))
    ///     .subcommand(Cmd::new("foo"))
    ///     .get_matches_from(vec!["myprog", "set"]);
    ///
    /// assert_eq!(m.value_of("cmd"), Some("set"));
    ///
    /// assert!(m.subcommand_matches("foo").is_none());
    /// ```
    #[deprecated(since = "2.27.0", note = "No longer required to propagate values")]
    PropagateGlobalValuesDown,

    /// Allows [``]s to override all requirements of the parent command.
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
    /// # use clap::{Cmd, Arg, CmdSettings, ErrorKind};
    /// let err = Cmd::new("myprog")
    ///     .setting(CmdSettings::SubcommandsNegateReqs)
    ///     .arg(Arg::new("opt").required(true))
    ///     .subcommand(Cmd::new("test"))
    ///     .try_get_matches_from(vec![
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
    /// # use clap::{Cmd, Arg, CmdSettings, ErrorKind};
    /// let noerr = Cmd::new("myprog")
    ///     .setting(CmdSettings::SubcommandsNegateReqs)
    ///     .arg(Arg::new("opt").required(true))
    ///     .subcommand(Cmd::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "test"
    ///     ]);
    /// assert!(noerr.is_ok());
    /// # ;
    /// ```
    /// [`Arg::required(true)`]: ./struct.Arg.html#method.required
    /// [``]: ./struct..html
    SubcommandsNegateReqs,

    /// Specifies that the help text should be displayed (before exiting gracefully) if no
    /// [``]s are present at runtime (i.e. an empty run such as `$ myprog`).
    ///
    /// **NOTE:** This should *not* be used with [`CmdSettings::SubcommandRequired`] as they do
    /// nearly same thing; this prints the help text, and the other prints an error.
    ///
    /// **NOTE:** If the user specifies arguments at runtime, but no subcommand the help text will
    /// still be displayed and exit. If this is *not* the desired result, consider using
    /// [`CmdSettings::ArgRequiredElseHelp`] instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::SubcommandRequiredElseHelp)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`CmdSettings::SubcommandRequired`]: ./enum.CmdSettings.html#variant.SubcommandRequired
    /// [`CmdSettings::ArgRequiredElseHelp`]: ./enum.CmdSettings.html#variant.ArgRequiredElseHelp
    SubcommandRequiredElseHelp,

    /// Specifies that any invalid UTF-8 code points should be treated as an error and fail
    /// with a [`ErrorKind::InvalidUtf8`] error.
    ///
    /// **NOTE:** This rule only applies to argument values; Things such as flags, options, and
    /// [``]s themselves only allow valid UTF-8 code points.
    ///
    /// # Platform Specific
    ///
    /// Non Windows systems only
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{Cmd, CmdSettings, ErrorKind};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStringExt;
    ///
    /// let m = Cmd::new("myprog")
    ///     .setting(CmdSettings::StrictUtf8)
    ///     .arg("<arg> 'some positional arg'")
    ///     .try_get_matches_from(
    ///         vec![
    ///             OsString::from("myprog"),
    ///             OsString::from_vec(vec![0xe9])]);
    ///
    /// assert!(m.is_err());
    /// assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
    /// ```
    /// [``]: ./struct..html
    /// [`ErrorKind::InvalidUtf8`]: ./enum.ErrorKind.html#variant.InvalidUtf8
    StrictUtf8,

    /// Allows specifying that if no [``] is present at runtime,
    /// error and exit gracefully.
    ///
    /// **NOTE:** This defaults to `false` (subcommands do *not* need to be present)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, CmdSettings, ErrorKind};
    /// let err = Cmd::new("myprog")
    ///     .setting(CmdSettings::SubcommandRequired)
    ///     .subcommand(Cmd::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog",
    ///     ]);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err().kind, ErrorKind::MissingSubcommand);
    /// # ;
    /// ```
    /// [``]: ./struct..html
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
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// let m = Cmd::new("myprog")
    ///     .setting(CmdSettings::TrailingVarArg)
    ///     .arg(Arg::from("<cmd>... 'commands to run'"))
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
    /// # use clap::{Cmd, Arg, CmdSettings};
    /// Cmd::new("myprog")
    ///     .setting(CmdSettings::UnifiedHelpMessage)
    ///     .get_matches();
    /// // running `myprog --help` will display a unified "docopt" or "getopts" style help message
    /// ```
    UnifiedHelpMessage,

    /// Disables `-V` and `--version` for all [``]s
    /// (Defaults to `false`; subcommands *do* have version flags.)
    ///
    /// **NOTE:** This setting must be set **prior** adding any subcommands
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Cmd, CmdSettings, ErrorKind};
    /// let res = Cmd::new("myprog")
    ///     .version("v1.1")
    ///     .setting(CmdSettings::VersionlessSubcommands)
    ///     .subcommand(Cmd::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "test", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [``]: ./struct..html
    VersionlessSubcommands,

    /// @TODO-v3: @docs write them...maybe rename
    NoAutoHelp,

    /// @TODO-v3: @docs write them...maybe rename
    NoAutoVersion,

    #[doc(hidden)]
    LowIndexMultiplePositional,

    #[doc(hidden)]
    TrailingValues,

    #[doc(hidden)]
    ValidNegNumFound,

    #[doc(hidden)]
    Propagated,

    #[doc(hidden)]
    ValidArgFound,

    #[doc(hidden)]
    ContainsLast,
}

impl FromStr for CmdSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "argrequiredelsehelp" => Ok(CmdSettings::ArgRequiredElseHelp),
            "argsnegatesubcommands" => Ok(CmdSettings::ArgsNegateSubcommands),
            "allowinvalidutf8" => Ok(CmdSettings::AllowInvalidUtf8),
            "allowleadinghyphen" => Ok(CmdSettings::AllowLeadingHyphen),
            "allowexternalsubcommands" => Ok(CmdSettings::AllowExternalSubcommands),
            "allownegativenumbers" => Ok(CmdSettings::AllowNegativeNumbers),
            "colorauto" => Ok(CmdSettings::ColorAuto),
            "coloralways" => Ok(CmdSettings::ColorAlways),
            "colornever" => Ok(CmdSettings::ColorNever),
            "coloredhelp" => Ok(CmdSettings::ColoredHelp),
            "derivedisplayorder" => Ok(CmdSettings::DeriveDisplayOrder),
            "dontcollapseargsinusage" => Ok(CmdSettings::DontCollapseArgsInUsage),
            "dontdelimittrailingvalues" => Ok(CmdSettings::DontDelimitTrailingValues),
            "disablehelpsubcommand" => Ok(CmdSettings::DisableHelpSubcommand),
            "disableversion" => Ok(CmdSettings::DisableVersion),
            "globalversion" => Ok(CmdSettings::GlobalVersion),
            "hidden" => Ok(CmdSettings::Hidden),
            "hidepossiblevaluesinhelp" => Ok(CmdSettings::HidePossibleValuesInHelp),
            "infersubcommands" => Ok(CmdSettings::InferSubcommands),
            "lowindexmultiplepositional" => Ok(CmdSettings::LowIndexMultiplePositional),
            "nextlinehelp" => Ok(CmdSettings::NextLineHelp),
            "strictutf8" => Ok(CmdSettings::StrictUtf8),
            "subcommandsnegatereqs" => Ok(CmdSettings::SubcommandsNegateReqs),
            "subcommandrequired" => Ok(CmdSettings::SubcommandRequired),
            "subcommandrequiredelsehelp" => Ok(CmdSettings::SubcommandRequiredElseHelp),
            "trailingvararg" => Ok(CmdSettings::TrailingVarArg),
            "unifiedhelpmessage" => Ok(CmdSettings::UnifiedHelpMessage),
            "versionlesssubcommands" => Ok(CmdSettings::VersionlessSubcommands),
            "validnegnumfound" => Ok(CmdSettings::ValidNegNumFound),
            "validargfound" => Ok(CmdSettings::ValidArgFound),
            "propagated" => Ok(CmdSettings::Propagated),
            "trailingvalues" => Ok(CmdSettings::TrailingValues),
            _ => Err(String::from("unknown CmdSetting, cannot convert from str")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CmdSettings;

    #[test]
    fn app_settings_fromstr() {
        assert_eq!(
            "argsnegatesubcommands".parse::<CmdSettings>().unwrap(),
            CmdSettings::ArgsNegateSubcommands
        );
        assert_eq!(
            "argrequiredelsehelp".parse::<CmdSettings>().unwrap(),
            CmdSettings::ArgRequiredElseHelp
        );
        assert_eq!(
            "allowexternalsubcommands".parse::<CmdSettings>().unwrap(),
            CmdSettings::AllowExternalSubcommands
        );
        assert_eq!(
            "allowinvalidutf8".parse::<CmdSettings>().unwrap(),
            CmdSettings::AllowInvalidUtf8
        );
        assert_eq!(
            "allowleadinghyphen".parse::<CmdSettings>().unwrap(),
            CmdSettings::AllowLeadingHyphen
        );
        assert_eq!(
            "allownegativenumbers".parse::<CmdSettings>().unwrap(),
            CmdSettings::AllowNegativeNumbers
        );
        assert_eq!(
            "coloredhelp".parse::<CmdSettings>().unwrap(),
            CmdSettings::ColoredHelp
        );
        assert_eq!(
            "colorauto".parse::<CmdSettings>().unwrap(),
            CmdSettings::ColorAuto
        );
        assert_eq!(
            "coloralways".parse::<CmdSettings>().unwrap(),
            CmdSettings::ColorAlways
        );
        assert_eq!(
            "colornever".parse::<CmdSettings>().unwrap(),
            CmdSettings::ColorNever
        );
        assert_eq!(
            "disablehelpsubcommand".parse::<CmdSettings>().unwrap(),
            CmdSettings::DisableHelpSubcommand
        );
        assert_eq!(
            "disableversion".parse::<CmdSettings>().unwrap(),
            CmdSettings::DisableVersion
        );
        assert_eq!(
            "dontcollapseargsinusage".parse::<CmdSettings>().unwrap(),
            CmdSettings::DontCollapseArgsInUsage
        );
        assert_eq!(
            "dontdelimittrailingvalues".parse::<CmdSettings>().unwrap(),
            CmdSettings::DontDelimitTrailingValues
        );
        assert_eq!(
            "derivedisplayorder".parse::<CmdSettings>().unwrap(),
            CmdSettings::DeriveDisplayOrder
        );
        assert_eq!(
            "globalversion".parse::<CmdSettings>().unwrap(),
            CmdSettings::GlobalVersion
        );
        assert_eq!(
            "hidden".parse::<CmdSettings>().unwrap(),
            CmdSettings::Hidden
        );
        assert_eq!(
            "hidepossiblevaluesinhelp".parse::<CmdSettings>().unwrap(),
            CmdSettings::HidePossibleValuesInHelp
        );
        assert_eq!(
            "lowindexmultiplePositional".parse::<CmdSettings>().unwrap(),
            CmdSettings::LowIndexMultiplePositional
        );
        assert_eq!(
            "nextlinehelp".parse::<CmdSettings>().unwrap(),
            CmdSettings::NextLineHelp
        );
        assert_eq!(
            "subcommandsnegatereqs".parse::<CmdSettings>().unwrap(),
            CmdSettings::SubcommandsNegateReqs
        );
        assert_eq!(
            "subcommandrequired".parse::<CmdSettings>().unwrap(),
            CmdSettings::SubcommandRequired
        );
        assert_eq!(
            "subcommandrequiredelsehelp".parse::<CmdSettings>().unwrap(),
            CmdSettings::SubcommandRequiredElseHelp
        );
        assert_eq!(
            "strictutf8".parse::<CmdSettings>().unwrap(),
            CmdSettings::StrictUtf8
        );
        assert_eq!(
            "trailingvararg".parse::<CmdSettings>().unwrap(),
            CmdSettings::TrailingVarArg
        );
        assert_eq!(
            "unifiedhelpmessage".parse::<CmdSettings>().unwrap(),
            CmdSettings::UnifiedHelpMessage
        );
        assert_eq!(
            "versionlesssubcommands".parse::<CmdSettings>().unwrap(),
            CmdSettings::VersionlessSubcommands
        );
        assert_eq!(
            "validnegnumfound".parse::<CmdSettings>().unwrap(),
            CmdSettings::ValidNegNumFound
        );
        assert_eq!(
            "validargfound".parse::<CmdSettings>().unwrap(),
            CmdSettings::ValidArgFound
        );
        assert_eq!(
            "propagated".parse::<CmdSettings>().unwrap(),
            CmdSettings::Propagated
        );
        assert_eq!(
            "trailingvalues".parse::<CmdSettings>().unwrap(),
            CmdSettings::TrailingValues
        );
        assert_eq!(
            "infersubcommands".parse::<CmdSettings>().unwrap(),
            CmdSettings::InferSubcommands
        );
        assert!("hahahaha".parse::<CmdSettings>().is_err());
    }
}
