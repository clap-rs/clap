// Std
use std::{ops::BitOr, str::FromStr};

// Third party
use bitflags::bitflags;

bitflags! {
    struct Flags: u64 {
        const SC_NEGATE_REQS                 = 1;
        const SC_REQUIRED                    = 1 << 1;
        const A_REQUIRED_ELSE_HELP           = 1 << 2;
        const GLOBAL_VERSION                 = 1 << 3;
        const DISABLE_VERSION_FOR_SC         = 1 << 4;
        const UNIFIED_HELP                   = 1 << 5;
        const WAIT_ON_ERROR                  = 1 << 6;
        const SC_REQUIRED_ELSE_HELP          = 1 << 7;
        const NO_AUTO_HELP                   = 1 << 8;
        const NO_AUTO_VERSION                = 1 << 9;
        const DISABLE_VERSION_FLAG           = 1 << 10;
        const HIDDEN                         = 1 << 11;
        const TRAILING_VARARG                = 1 << 12;
        const NO_BIN_NAME                    = 1 << 13;
        const ALLOW_UNK_SC                   = 1 << 14;
        const UTF8_STRICT                    = 1 << 15;
        const UTF8_NONE                      = 1 << 16;
        const LEADING_HYPHEN                 = 1 << 17;
        const NO_POS_VALUES                  = 1 << 18;
        const NEXT_LINE_HELP                 = 1 << 19;
        const DERIVE_DISP_ORDER              = 1 << 20;
        const COLORED_HELP                   = 1 << 21;
        const COLOR_ALWAYS                   = 1 << 22;
        const COLOR_AUTO                     = 1 << 23;
        const COLOR_NEVER                    = 1 << 24;
        const DONT_DELIM_TRAIL               = 1 << 25;
        const ALLOW_NEG_NUMS                 = 1 << 26;
        const DISABLE_HELP_SC                = 1 << 28;
        const DONT_COLLAPSE_ARGS             = 1 << 29;
        const ARGS_NEGATE_SCS                = 1 << 30;
        const PROPAGATE_VALS_DOWN            = 1 << 31;
        const ALLOW_MISSING_POS              = 1 << 32;
        const TRAILING_VALUES                = 1 << 33;
        const BUILT                          = 1 << 34;
        const BIN_NAME_BUILT                 = 1 << 35;
        const VALID_ARG_FOUND                = 1 << 36;
        const INFER_SUBCOMMANDS              = 1 << 37;
        const CONTAINS_LAST                  = 1 << 38;
        const ARGS_OVERRIDE_SELF             = 1 << 39;
        const HELP_REQUIRED                  = 1 << 40;
        const SUBCOMMAND_PRECEDENCE_OVER_ARG = 1 << 41;
        const DISABLE_HELP_FLAG              = 1 << 42;
        const USE_LONG_FORMAT_FOR_HELP_SC    = 1 << 43;
        const DISABLE_ENV                    = 1 << 44;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct AppFlags(Flags);

impl BitOr for AppFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        AppFlags(self.0 | rhs.0)
    }
}

impl Default for AppFlags {
    fn default() -> Self {
        AppFlags(Flags::UTF8_NONE | Flags::COLOR_AUTO)
    }
}

impl_settings! { AppSettings, AppFlags,
    ArgRequiredElseHelp("argrequiredelsehelp")
        => Flags::A_REQUIRED_ELSE_HELP,
    SubcommandPrecedenceOverArg("subcommandprecedenceoverarg")
        => Flags::SUBCOMMAND_PRECEDENCE_OVER_ARG,
    ArgsNegateSubcommands("argsnegatesubcommands")
        => Flags::ARGS_NEGATE_SCS,
    AllowExternalSubcommands("allowexternalsubcommands")
        => Flags::ALLOW_UNK_SC,
    AllowInvalidUtf8("allowinvalidutf8")
        => Flags::UTF8_NONE,
    AllowLeadingHyphen("allowleadinghyphen")
        => Flags::LEADING_HYPHEN,
    AllowNegativeNumbers("allownegativenumbers")
        => Flags::ALLOW_NEG_NUMS,
    AllowMissingPositional("allowmissingpositional")
        => Flags::ALLOW_MISSING_POS,
    ColoredHelp("coloredhelp")
        => Flags::COLORED_HELP,
    ColorAlways("coloralways")
        => Flags::COLOR_ALWAYS,
    ColorAuto("colorauto")
        => Flags::COLOR_AUTO,
    ColorNever("colornever")
        => Flags::COLOR_NEVER,
    DisableEnv("disableenv")
        => Flags::DISABLE_ENV,
    DontDelimitTrailingValues("dontdelimittrailingvalues")
        => Flags::DONT_DELIM_TRAIL,
    DontCollapseArgsInUsage("dontcollapseargsinusage")
        => Flags::DONT_COLLAPSE_ARGS,
    DeriveDisplayOrder("derivedisplayorder")
        => Flags::DERIVE_DISP_ORDER,
    DisableHelpSubcommand("disablehelpsubcommand")
        => Flags::DISABLE_HELP_SC,
    DisableHelpFlag("disablehelpflag")
        => Flags::DISABLE_HELP_FLAG,
    DisableVersionFlag("disableversionflag")
        => Flags::DISABLE_VERSION_FLAG,
    GlobalVersion("globalversion")
        => Flags::GLOBAL_VERSION,
    HidePossibleValuesInHelp("hidepossiblevaluesinhelp")
        => Flags::NO_POS_VALUES,
    HelpRequired("helprequired")
        => Flags::HELP_REQUIRED,
    Hidden("hidden")
        => Flags::HIDDEN,
    NoAutoHelp("noautohelp")
        => Flags::NO_AUTO_HELP,
    NoAutoVersion("noautoversion")
        => Flags::NO_AUTO_VERSION,
    NoBinaryName("nobinaryname")
        => Flags::NO_BIN_NAME,
    StrictUtf8("strictutf8")
        => Flags::UTF8_STRICT,
    SubcommandsNegateReqs("subcommandsnegatereqs")
        => Flags::SC_NEGATE_REQS,
    SubcommandRequired("subcommandrequired")
        => Flags::SC_REQUIRED,
    SubcommandRequiredElseHelp("subcommandrequiredelsehelp")
        => Flags::SC_REQUIRED_ELSE_HELP,
    UseLongFormatForHelpSubcommand("uselongformatforhelpsubcommand")
        => Flags::USE_LONG_FORMAT_FOR_HELP_SC,
    TrailingVarArg("trailingvararg")
        => Flags::TRAILING_VARARG,
    UnifiedHelpMessage("unifiedhelpmessage")
        => Flags::UNIFIED_HELP,
    NextLineHelp("nextlinehelp")
        => Flags::NEXT_LINE_HELP,
    DisableVersionForSubcommands("disableversionforsubcommands")
        => Flags::DISABLE_VERSION_FOR_SC,
    WaitOnError("waitonerror")
        => Flags::WAIT_ON_ERROR,
    TrailingValues("trailingvalues")
        => Flags::TRAILING_VALUES,
    Built("built")
        => Flags::BUILT,
    BinNameBuilt("binnamebuilt")
        => Flags::BIN_NAME_BUILT,
    ValidArgFound("validargfound")
        => Flags::VALID_ARG_FOUND,
    InferSubcommands("infersubcommands")
        => Flags::INFER_SUBCOMMANDS,
    AllArgsOverrideSelf("allargsoverrideself")
        => Flags::ARGS_OVERRIDE_SELF
}

/// Application level settings, which affect how [`App`] operates
///
/// **NOTE:** When these settings are used, they apply only to current command, and are *not*
/// propagated down or up through child or parent subcommands
///
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
    /// # use clap::{App, AppSettings};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let r = App::new("myprog")
    ///   //.setting(AppSettings::AllowInvalidUtf8)
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
    /// [`ArgMatches::os_value_of`]: ArgMatches::os_value_of()
    /// [`ArgMatches::os_values_of`]: ArgMatches::os_values_of()
    /// [`ArgMatches::lossy_value_of`]: ArgMatches::lossy_value_of()
    /// [`ArgMatches::lossy_values_of`]: ArgMatches::lossy_values_of()
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
    /// # use clap::{Arg, App, AppSettings};
    /// // Imagine you needed to represent negative numbers as well, such as -10
    /// let m = App::new("nums")
    ///     .setting(AppSettings::AllowLeadingHyphen)
    ///     .arg(Arg::new("neg").index(1))
    ///     .get_matches_from(vec![
    ///         "nums", "-20"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("neg"), Some("-20"));
    /// # ;
    /// ```
    /// [`Arg::allow_hyphen_values`]: Arg::allow_hyphen_values()
    AllowLeadingHyphen,

    /// Specifies that all arguments override themselves. This is the equivalent to saying the `foo`
    /// arg using [`Arg::overrides_with("foo")`] for all defined arguments.
    ///
    /// [`Arg::overrides_with("foo")`]: Arg::overrides_with()
    AllArgsOverrideSelf,

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
    ///     .arg(Arg::new("num"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "-20"
    ///     ]);
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert_eq!(m.value_of("num").unwrap(), "-20");
    /// ```
    /// [`AllowLeadingHyphen`]: AppSettings::AllowLeadingHyphen
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
    /// # use clap::{App, Arg, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowMissingPositional)
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
    /// # use clap::{App, Arg, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowMissingPositional)
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
    /// # use clap::{App, Arg, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowMissingPositional)
    ///     .arg(Arg::new("foo"))
    ///     .arg(Arg::new("bar"))
    ///     .arg(Arg::new("baz").takes_value(true).multiple(true))
    ///     .get_matches_from(vec![
    ///         "prog", "foo", "bar", "baz1", "baz2", "baz3"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("foo"), Some("foo"));
    /// assert_eq!(m.value_of("bar"), Some("bar"));
    /// assert_eq!(m.values_of("baz").unwrap().collect::<Vec<_>>(), &["baz1", "baz2", "baz3"]);
    /// ```
    ///
    /// Now nofice if we don't specify `foo` or `baz` but use the `--` operator.
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowMissingPositional)
    ///     .arg(Arg::new("foo"))
    ///     .arg(Arg::new("bar"))
    ///     .arg(Arg::new("baz").takes_value(true).multiple(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--", "baz1", "baz2", "baz3"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("foo"), None);
    /// assert_eq!(m.value_of("bar"), None);
    /// assert_eq!(m.values_of("baz").unwrap().collect::<Vec<_>>(), &["baz1", "baz2", "baz3"]);
    /// ```
    /// [required]: Arg::required()
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
    ///     Some((external, ext_m)) => {
    ///          let ext_args: Vec<&str> = ext_m.values_of("").unwrap().collect();
    ///          assert_eq!(external, "subcmd");
    ///          assert_eq!(ext_args, ["--option", "value", "-fff", "--flag"]);
    ///     },
    ///     _ => {},
    /// }
    /// ```
    /// [``]: ./struct..html
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
    /// [subcommands]: ./struct..html
    /// [argument]: Arg
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
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    /// # ;
    /// ```
    /// [``]: ./struct..html
    /// [`Arg::default_value`]: Arg::default_value()
    ArgRequiredElseHelp,

    /// Instructs the parser to stop when encountering a subcommand instead of greedily consuming
    /// args.
    ///
    /// By default, if an option taking multiple values is followed by a subcommand, the
    /// subcommand will be parsed as another value.
    ///
    /// ```text
    /// app --foo val1 val2 subcommand
    ///           --------- ----------
    ///             values   another value
    /// ```
    ///
    /// This setting instructs the parser to stop when encountering a subcommand instead of
    /// greedily consuming arguments.
    ///
    /// ```text
    /// app --foo val1 val2 subcommand
    ///           --------- ----------
    ///             values   subcommand
    /// ```
    ///
    /// **Note:** Make sure you apply it as `global_setting` if you want it to be propagated to
    /// sub-sub commands!
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, Arg};
    /// let app = App::new("app").subcommand(App::new("sub")).arg(
    ///     Arg::new("arg")
    ///         .long("arg")
    ///         .multiple(true)
    ///         .takes_value(true),
    /// );
    ///
    /// let matches = app
    ///     .clone()
    ///     .try_get_matches_from(&["app", "--arg", "1", "2", "3", "sub"])
    ///     .unwrap();
    /// assert_eq!(
    ///     matches.values_of("arg").unwrap().collect::<Vec<_>>(),
    ///     &["1", "2", "3", "sub"]
    /// );
    /// assert!(matches.subcommand_matches("sub").is_none());
    ///
    /// let app = app.setting(AppSettings::SubcommandPrecedenceOverArg);
    /// let matches = app
    ///     .try_get_matches_from(&["app", "--arg", "1", "2", "3", "sub"])
    ///     .unwrap();
    /// assert_eq!(
    ///     matches.values_of("arg").unwrap().collect::<Vec<_>>(),
    ///     &["1", "2", "3"]
    /// );
    /// assert!(matches.subcommand_matches("sub").is_some());
    /// ```
    SubcommandPrecedenceOverArg,

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
    /// # use clap::{App, Arg, AppSettings};
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
    /// # use clap::{App, Arg, AppSettings};
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
    /// # use clap::{App, Arg, AppSettings};
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
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ColorNever)
    ///     .get_matches();
    /// ```
    ColorNever,

    /// Disables the use of environment variables in the app
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .global_setting(AppSettings::DisableEnv)
    ///     .get_matches();
    /// ```
    DisableEnv,

    /// Disables the automatic collapsing of positional args into `[ARGS]` inside the usage string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
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
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::DontDelimitTrailingValues)
    ///     .get_matches();
    /// ```
    /// [`Arg::use_delimiter(false)`]: Arg::use_delimiter()
    DontDelimitTrailingValues,

    /// Disables `-h` and `--help` [`App`] without affecting any of the [`SubCommand`]s
    /// (Defaults to `false`; application *does* have help flags)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .setting(AppSettings::DisableHelpFlag)
    ///     .try_get_matches_from(vec![
    ///         "myprog", "-h"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .setting(AppSettings::DisableHelpFlag)
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "test", "-h"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::DisplayHelp);
    /// ```
    DisableHelpFlag,

    /// Disables the `help` subcommand
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind, };
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::DisableHelpSubcommand)
    ///     // Normally, creating a subcommand causes a `help` subcommand to automatically
    ///     // be generated as well
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "help"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [``]: ./struct..html
    DisableHelpSubcommand,

    /// Disables `-V` and `--version` for this [`App`] without affecting any of the [``]s
    /// (Defaults to `false`; application *does* have a version flag)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::DisableVersionFlag)
    ///     .try_get_matches_from(vec![
    ///         "myprog", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::DisableVersionFlag)
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "test", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::DisplayVersion);
    /// ```
    /// [``]: ./struct..html
    DisableVersionFlag,

    /// Disables `-V` and `--version` for all [`subcommand`]s of this [`App`]
    /// (Defaults to `false`; subcommands *do* have version flags.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::DisableVersionForSubcommands)
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "test", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [``]: ./struct..html
    DisableVersionForSubcommands,

    /// Displays the arguments and [``]s in the help message in the order that they were
    /// declared in, and not alphabetically which is the default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::DeriveDisplayOrder)
    ///     .get_matches();
    /// ```
    /// [``]: ./struct..html
    DeriveDisplayOrder,

    /// Specifies to use the version of the current command for all child [``]s.
    /// (Defaults to `false`; subcommands have independent version strings from their parents.)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::GlobalVersion)
    ///     .subcommand(App::new("test"))
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
    /// # use clap::{App, Arg, AppSettings, };
    /// App::new("myprog")
    ///     .subcommand(App::new("test")
    ///     .setting(AppSettings::Hidden))
    /// # ;
    /// ```
    /// [``]: ./struct..html
    Hidden,

    /// Tells `clap` *not* to print possible values when displaying help information.
    /// This can be useful if there are many values, or they are explained elsewhere.
    HidePossibleValuesInHelp,

    /// Tells `clap` to panic if help strings are omitted
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::HelpRequired)
    ///     .arg(
    ///         Arg::new("foo").about("It does foo stuff")
    ///         // As required via AppSettings::HelpRequired, a help message was supplied
    ///      )
    /// #    .get_matches();
    /// ```
    ///
    /// # Panics
    ///
    /// ```rust,no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myapp")
    ///     .setting(AppSettings::HelpRequired)
    ///     .arg(
    ///         Arg::new("foo")
    ///         // Someone forgot to put .about("...") here
    ///         // Since the setting AppSettings::HelpRequired is activated, this will lead to
    ///         // a panic (if you are in debug mode)
    ///     )
    /// #   .get_matches();
    ///```
    HelpRequired,

    /// Tries to match unknown args to partial [`subcommands`] or their [aliases]. For example, to
    /// match a subcommand named `test`, one could use `t`, `te`, `tes`, and `test`.
    ///
    /// **NOTE:** The match *must not* be ambiguous at all in order to succeed. i.e. to match `te`
    /// to `test` there could not also be a subcommand or alias `temp` because both start with `te`
    ///
    /// **CAUTION:** This setting can interfere with [positional/free arguments], take care when
    /// designing CLIs which allow inferred subcommands and have potential positional/free
    /// arguments whose values could start with the same characters as subcommands. If this is the
    /// case, it's recommended to use settings such as [`AppSettings::ArgsNegateSubcommands`] in
    /// conjunction with this setting.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("prog")
    ///     .setting(AppSettings::InferSubcommands)
    ///     .subcommand(App::new("test"))
    ///     .get_matches_from(vec![
    ///         "prog", "te"
    ///     ]);
    /// assert_eq!(m.subcommand_name(), Some("test"));
    /// ```
    /// [`subcommands`]: ./struct..html
    /// [positional/free arguments]: Arg::index()
    /// [aliases]: App::alias()
    InferSubcommands,

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
    ///     .arg(Arg::from("<cmd>... 'commands to run'"))
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
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::NextLineHelp)
    ///     .get_matches();
    /// ```
    NextLineHelp,

    /// Allows [``]s to override all requirements of the parent command.
    /// For example, if you had a subcommand or top level application with a required argument
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
    /// # use clap::{App, Arg, AppSettings, ErrorKind};
    /// let err = App::new("myprog")
    ///     .setting(AppSettings::SubcommandsNegateReqs)
    ///     .arg(Arg::new("opt").required(true))
    ///     .subcommand(App::new("test"))
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
    /// # use clap::{App, Arg, AppSettings, ErrorKind};
    /// let noerr = App::new("myprog")
    ///     .setting(AppSettings::SubcommandsNegateReqs)
    ///     .arg(Arg::new("opt").required(true))
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "test"
    ///     ]);
    /// assert!(noerr.is_ok());
    /// # ;
    /// ```
    /// [`Arg::required(true)`]: Arg::required()
    /// [``]: ./struct..html
    SubcommandsNegateReqs,

    /// Specifies that the help text should be displayed (before exiting gracefully) if no
    /// [``]s are present at runtime (i.e. an empty run such as `$ myprog`).
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
    /// [``]: ./struct..html
    SubcommandRequiredElseHelp,

    /// Specifies that the help subcommand should print the [long format] help message.
    ///
    /// **NOTE:** This setting is useless if [`AppSettings::DisableHelpSubcommand`] or [`AppSettings::NoAutoHelp`] is set,
    /// or if the app contains no subcommands at all.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::UseLongFormatForHelpSubcommand)
    ///     .subcommand(App::new("test")
    ///         .arg(Arg::new("foo")
    ///             .about("short form about message")
    ///             .long_about("long form about message")
    ///         )
    ///     )
    ///     .get_matches();
    /// ```
    /// [long format]: App::long_about
    UseLongFormatForHelpSubcommand,

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
    /// # use clap::{App, AppSettings, ErrorKind};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStringExt;
    ///
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::StrictUtf8)
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
    StrictUtf8,

    /// Allows specifying that if no [``] is present at runtime,
    /// error and exit gracefully.
    ///
    /// **NOTE:** This defaults to `false` (subcommands do *not* need to be present)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let err = App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .subcommand(App::new("test"))
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
    /// # use clap::{App, Arg, AppSettings};
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::TrailingVarArg)
    ///     .arg(Arg::from("<cmd>... 'commands to run'"))
    ///     .get_matches_from(vec!["myprog", "arg1", "-r", "val1"]);
    ///
    /// let trail: Vec<&str> = m.values_of("cmd").unwrap().collect();
    /// assert_eq!(trail, ["arg1", "-r", "val1"]);
    /// ```
    /// [`Arg::multiple(true)`]: Arg::multiple()
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
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::UnifiedHelpMessage)
    ///     .get_matches();
    /// // running `myprog --help` will display a unified "docopt" or "getopts" style help message
    /// ```
    UnifiedHelpMessage,

    /// Will display a message "Press \[ENTER\]/\[RETURN\] to continue..." and wait for user before
    /// exiting
    ///
    /// This is most useful when writing an application which is run from a GUI shortcut, or on
    /// Windows where a user tries to open the binary by double-clicking instead of using the
    /// command line.
    ///
    /// **NOTE:** This setting is **not** recursive with [``]s, meaning if you wish this
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
    /// [``]: ./struct..html
    WaitOnError,

    /// @TODO-v3: @docs write them...maybe rename
    NoAutoHelp,

    /// @TODO-v3: @docs write them...maybe rename
    NoAutoVersion,

    #[doc(hidden)]
    /// If the user already passed '--'. Meaning only positional args follow.
    TrailingValues,

    #[doc(hidden)]
    /// If the app is already built, used for caching.
    Built,

    #[doc(hidden)]
    /// If the app's bin name is already built, used for caching.
    BinNameBuilt,

    #[doc(hidden)]
    /// This is paired with `ArgsNegateSubcommands`. Used to determine if we
    /// already met any valid arg(then we shouldn't expect subcommands after it).
    ValidArgFound,
}

#[cfg(test)]
mod test {
    use super::AppSettings;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn app_settings_fromstr() {
        assert_eq!(
            "disablehelpflag".parse::<AppSettings>().unwrap(),
            AppSettings::DisableHelpFlag
        );
        assert_eq!(
            "argsnegatesubcommands".parse::<AppSettings>().unwrap(),
            AppSettings::ArgsNegateSubcommands
        );
        assert_eq!(
            "argrequiredelsehelp".parse::<AppSettings>().unwrap(),
            AppSettings::ArgRequiredElseHelp
        );
        assert_eq!(
            "subcommandprecedenceoverarg"
                .parse::<AppSettings>()
                .unwrap(),
            AppSettings::SubcommandPrecedenceOverArg
        );
        assert_eq!(
            "allowexternalsubcommands".parse::<AppSettings>().unwrap(),
            AppSettings::AllowExternalSubcommands
        );
        assert_eq!(
            "allowinvalidutf8".parse::<AppSettings>().unwrap(),
            AppSettings::AllowInvalidUtf8
        );
        assert_eq!(
            "allowleadinghyphen".parse::<AppSettings>().unwrap(),
            AppSettings::AllowLeadingHyphen
        );
        assert_eq!(
            "allownegativenumbers".parse::<AppSettings>().unwrap(),
            AppSettings::AllowNegativeNumbers
        );
        assert_eq!(
            "coloredhelp".parse::<AppSettings>().unwrap(),
            AppSettings::ColoredHelp
        );
        assert_eq!(
            "colorauto".parse::<AppSettings>().unwrap(),
            AppSettings::ColorAuto
        );
        assert_eq!(
            "coloralways".parse::<AppSettings>().unwrap(),
            AppSettings::ColorAlways
        );
        assert_eq!(
            "colornever".parse::<AppSettings>().unwrap(),
            AppSettings::ColorNever
        );
        assert_eq!(
            "disablehelpsubcommand".parse::<AppSettings>().unwrap(),
            AppSettings::DisableHelpSubcommand
        );
        assert_eq!(
            "disableversionflag".parse::<AppSettings>().unwrap(),
            AppSettings::DisableVersionFlag
        );
        assert_eq!(
            "dontcollapseargsinusage".parse::<AppSettings>().unwrap(),
            AppSettings::DontCollapseArgsInUsage
        );
        assert_eq!(
            "dontdelimittrailingvalues".parse::<AppSettings>().unwrap(),
            AppSettings::DontDelimitTrailingValues
        );
        assert_eq!(
            "derivedisplayorder".parse::<AppSettings>().unwrap(),
            AppSettings::DeriveDisplayOrder
        );
        assert_eq!(
            "globalversion".parse::<AppSettings>().unwrap(),
            AppSettings::GlobalVersion
        );
        assert_eq!(
            "hidden".parse::<AppSettings>().unwrap(),
            AppSettings::Hidden
        );
        assert_eq!(
            "hidepossiblevaluesinhelp".parse::<AppSettings>().unwrap(),
            AppSettings::HidePossibleValuesInHelp
        );
        assert_eq!(
            "helprequired".parse::<AppSettings>().unwrap(),
            AppSettings::HelpRequired
        );
        assert_eq!(
            "nobinaryname".parse::<AppSettings>().unwrap(),
            AppSettings::NoBinaryName
        );
        assert_eq!(
            "nextlinehelp".parse::<AppSettings>().unwrap(),
            AppSettings::NextLineHelp
        );
        assert_eq!(
            "subcommandsnegatereqs".parse::<AppSettings>().unwrap(),
            AppSettings::SubcommandsNegateReqs
        );
        assert_eq!(
            "subcommandrequired".parse::<AppSettings>().unwrap(),
            AppSettings::SubcommandRequired
        );
        assert_eq!(
            "subcommandrequiredelsehelp".parse::<AppSettings>().unwrap(),
            AppSettings::SubcommandRequiredElseHelp
        );
        assert_eq!(
            "uselongformatforhelpsubcommand"
                .parse::<AppSettings>()
                .unwrap(),
            AppSettings::UseLongFormatForHelpSubcommand
        );
        assert_eq!(
            "strictutf8".parse::<AppSettings>().unwrap(),
            AppSettings::StrictUtf8
        );
        assert_eq!(
            "trailingvararg".parse::<AppSettings>().unwrap(),
            AppSettings::TrailingVarArg
        );
        assert_eq!(
            "unifiedhelpmessage".parse::<AppSettings>().unwrap(),
            AppSettings::UnifiedHelpMessage
        );
        assert_eq!(
            "disableversionforsubcommands"
                .parse::<AppSettings>()
                .unwrap(),
            AppSettings::DisableVersionForSubcommands
        );
        assert_eq!(
            "waitonerror".parse::<AppSettings>().unwrap(),
            AppSettings::WaitOnError
        );
        assert_eq!(
            "validargfound".parse::<AppSettings>().unwrap(),
            AppSettings::ValidArgFound
        );
        assert_eq!("built".parse::<AppSettings>().unwrap(), AppSettings::Built);
        assert_eq!(
            "binnamebuilt".parse::<AppSettings>().unwrap(),
            AppSettings::BinNameBuilt
        );
        assert_eq!(
            "trailingvalues".parse::<AppSettings>().unwrap(),
            AppSettings::TrailingValues
        );
        assert_eq!(
            "infersubcommands".parse::<AppSettings>().unwrap(),
            AppSettings::InferSubcommands
        );
        assert!("hahahaha".parse::<AppSettings>().is_err());
    }
}
