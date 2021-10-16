// Std
use std::{ops::BitOr, str::FromStr};

// Third party
use bitflags::bitflags;

bitflags! {
    struct Flags: u64 {
        const SC_NEGATE_REQS                 = 1;
        const SC_REQUIRED                    = 1 << 1;
        const ARG_REQUIRED_ELSE_HELP         = 1 << 2;
        const PROPAGATE_VERSION              = 1 << 3;
        const DISABLE_VERSION_FOR_SC         = 1 << 4;
        const WAIT_ON_ERROR                  = 1 << 6;
        const SC_REQUIRED_ELSE_HELP          = 1 << 7;
        const NO_AUTO_HELP                   = 1 << 8;
        const NO_AUTO_VERSION                = 1 << 9;
        const DISABLE_VERSION_FLAG           = 1 << 10;
        const HIDDEN                         = 1 << 11;
        const TRAILING_VARARG                = 1 << 12;
        const NO_BIN_NAME                    = 1 << 13;
        const ALLOW_UNK_SC                   = 1 << 14;
        const SC_UTF8_NONE                   = 1 << 15;
        const LEADING_HYPHEN                 = 1 << 16;
        const NO_POS_VALUES                  = 1 << 17;
        const NEXT_LINE_HELP                 = 1 << 18;
        const DERIVE_DISP_ORDER              = 1 << 19;
        const DONT_DELIM_TRAIL               = 1 << 24;
        const ALLOW_NEG_NUMS                 = 1 << 25;
        const DISABLE_HELP_SC                = 1 << 27;
        const DONT_COLLAPSE_ARGS             = 1 << 28;
        const ARGS_NEGATE_SCS                = 1 << 29;
        const PROPAGATE_VALS_DOWN            = 1 << 30;
        const ALLOW_MISSING_POS              = 1 << 31;
        const TRAILING_VALUES                = 1 << 32;
        const BUILT                          = 1 << 33;
        const BIN_NAME_BUILT                 = 1 << 34;
        const VALID_ARG_FOUND                = 1 << 35;
        const INFER_SUBCOMMANDS              = 1 << 36;
        const CONTAINS_LAST                  = 1 << 37;
        const ARGS_OVERRIDE_SELF             = 1 << 38;
        const HELP_REQUIRED                  = 1 << 39;
        const SUBCOMMAND_PRECEDENCE_OVER_ARG = 1 << 40;
        const DISABLE_HELP_FLAG              = 1 << 41;
        const USE_LONG_FORMAT_FOR_HELP_SC    = 1 << 42;
        const INFER_LONG_ARGS                = 1 << 43;
        const IGNORE_ERRORS                  = 1 << 44;
        #[cfg(feature = "unstable-multicall")]
        const MULTICALL                      = 1 << 45;
    }
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppFlags(Flags);

impl Default for AppFlags {
    fn default() -> Self {
        Self::empty()
    }
}

impl_settings! { AppSettings, AppFlags,
    ArgRequiredElseHelp("argrequiredelsehelp")
        => Flags::ARG_REQUIRED_ELSE_HELP,
    SubcommandPrecedenceOverArg("subcommandprecedenceoverarg")
        => Flags::SUBCOMMAND_PRECEDENCE_OVER_ARG,
    ArgsNegateSubcommands("argsnegatesubcommands")
        => Flags::ARGS_NEGATE_SCS,
    AllowExternalSubcommands("allowexternalsubcommands")
        => Flags::ALLOW_UNK_SC,
    AllowInvalidUtf8ForExternalSubcommands("allowinvalidutf8forexternalsubcommands")
        => Flags::SC_UTF8_NONE,
    AllowLeadingHyphen("allowleadinghyphen")
        => Flags::LEADING_HYPHEN,
    AllowNegativeNumbers("allownegativenumbers")
        => Flags::ALLOW_NEG_NUMS,
    AllowMissingPositional("allowmissingpositional")
        => Flags::ALLOW_MISSING_POS,
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
    PropagateVersion("propagateversion")
        => Flags::PROPAGATE_VERSION,
    HidePossibleValuesInHelp("hidepossiblevaluesinhelp")
        => Flags::NO_POS_VALUES,
    HelpRequired("helprequired")
        => Flags::HELP_REQUIRED,
    Hidden("hidden")
        => Flags::HIDDEN,
    #[cfg(feature = "unstable-multicall")]
    Multicall("multicall")
        => Flags::MULTICALL,
    NoAutoHelp("noautohelp")
        => Flags::NO_AUTO_HELP,
    NoAutoVersion("noautoversion")
        => Flags::NO_AUTO_VERSION,
    NoBinaryName("nobinaryname")
        => Flags::NO_BIN_NAME,
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
    NextLineHelp("nextlinehelp")
        => Flags::NEXT_LINE_HELP,
    IgnoreErrors("ignoreerrors")
        => Flags::IGNORE_ERRORS,
    WaitOnError("waitonerror")
        => Flags::WAIT_ON_ERROR,
    Built("built")
        => Flags::BUILT,
    BinNameBuilt("binnamebuilt")
        => Flags::BIN_NAME_BUILT,
    InferSubcommands("infersubcommands")
        => Flags::INFER_SUBCOMMANDS,
    AllArgsOverrideSelf("allargsoverrideself")
        => Flags::ARGS_OVERRIDE_SELF,
    InferLongArgs("inferlongargs")
        => Flags::INFER_LONG_ARGS
}

/// Application level settings, which affect how [`App`] operates
///
/// **NOTE:** When these settings are used, they apply only to current command, and are *not*
/// propagated down or up through child or parent subcommands
///
/// [`App`]: crate::App
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AppSettings {
    /// Specifies that external subcommands that are invalid UTF-8 should *not* be treated as an error.
    ///
    /// **NOTE:** Using external subcommand argument values with invalid UTF-8 requires using
    /// [`ArgMatches::values_of_os`] or [`ArgMatches::values_of_lossy`] for those particular
    /// arguments which may contain invalid UTF-8 values
    ///
    /// **NOTE:** Setting this requires [`AppSettings::AllowExternalSubcommands`]
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
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
    ///     .setting(AppSettings::AllowExternalSubcommands)
    ///     .get_matches_from(vec![
    ///         "myprog", "subcmd", "--option", "value", "-fff", "--flag"
    ///     ]);
    ///
    /// // All trailing arguments will be stored under the subcommand's sub-matches using an empty
    /// // string argument name
    /// match m.subcommand() {
    ///     Some((external, ext_m)) => {
    ///          let ext_args: Vec<&std::ffi::OsStr> = ext_m.values_of_os("").unwrap().collect();
    ///          assert_eq!(external, "subcmd");
    ///          assert_eq!(ext_args, ["--option", "value", "-fff", "--flag"]);
    ///     },
    ///     _ => {},
    /// }
    /// ```
    ///
    /// [`ArgMatches::values_of_os`]: crate::ArgMatches::values_of_os()
    /// [`ArgMatches::values_of_lossy`]: crate::ArgMatches::values_of_lossy()
    /// [`subcommands`]: crate::App::subcommand()
    AllowInvalidUtf8ForExternalSubcommands,

    /// Specifies that leading hyphens are allowed in all argument *values*, such as negative numbers
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
    ///     .arg(Arg::new("neg"))
    ///     .get_matches_from(vec![
    ///         "nums", "-20"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("neg"), Some("-20"));
    /// # ;
    /// ```
    /// [`Arg::allow_hyphen_values`]: crate::Arg::allow_hyphen_values()
    AllowLeadingHyphen,

    /// Specifies that all arguments override themselves. This is the equivalent to saying the `foo`
    /// arg using [`Arg::overrides_with("foo")`] for all defined arguments.
    ///
    /// [`Arg::overrides_with("foo")`]: crate::Arg::overrides_with()
    AllArgsOverrideSelf,

    /// Allows negative numbers to pass as values. This is similar to
    /// [`AppSettings::AllowLeadingHyphen`] except that it only allows numbers, all
    /// other undefined leading hyphens will fail to parse.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// let res = App::new("myprog")
    ///     .setting(AppSettings::AllowNegativeNumbers)
    ///     .arg(Arg::new("num"))
    ///     .try_get_matches_from(vec![
    ///         "myprog", "-20"
    ///     ]);
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert_eq!(m.value_of("num").unwrap(), "-20");
    /// ```
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
    ///
    /// Style number two from above:
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let m = App::new("myprog")
    ///     .setting(AppSettings::AllowMissingPositional)
    ///     .arg(Arg::new("foo"))
    ///     .arg(Arg::new("bar"))
    ///     .arg(Arg::new("baz").takes_value(true).multiple_values(true))
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
    ///     .arg(Arg::new("baz").takes_value(true).multiple_values(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--", "baz1", "baz2", "baz3"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("foo"), None);
    /// assert_eq!(m.value_of("bar"), None);
    /// assert_eq!(m.values_of("baz").unwrap().collect::<Vec<_>>(), &["baz1", "baz2", "baz3"]);
    /// ```
    ///
    /// [required]: crate::Arg::required()
    AllowMissingPositional,

    /// Specifies that an unexpected positional argument,
    /// which would otherwise cause a [`ErrorKind::UnknownArgument`] error,
    /// should instead be treated as a [`subcommand`] within the [`ArgMatches`] struct.
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
    ///
    /// [`subcommand`]: crate::App::subcommand()
    /// [`ArgMatches`]: crate::ArgMatches
    /// [`ErrorKind::UnknownArgument`]: crate::ErrorKind::UnknownArgument
    AllowExternalSubcommands,

    /// Specifies that use of a valid argument negates [`subcommands`] being
    /// used after. By default `clap` allows arguments between subcommands such
    /// as `<cmd> [cmd_args] <subcmd> [subcmd_args] <subsubcmd> [subsubcmd_args]`.
    ///
    /// This setting disables that functionality and says that arguments can
    /// only follow the *final* subcommand. For instance using this setting
    /// makes only the following invocations possible:
    ///
    /// * `<cmd> <subcmd> <subsubcmd> [subsubcmd_args]`
    /// * `<cmd> <subcmd> [subcmd_args]`
    /// * `<cmd> [cmd_args]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ArgsNegateSubcommands);
    /// ```
    ///
    /// [`subcommands`]: crate::App::subcommand()
    ArgsNegateSubcommands,

    /// Specifies that the help text should be displayed (and then exit gracefully),
    /// if no arguments are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **NOTE:** [`subcommands`] count as arguments
    ///
    /// **NOTE:** Setting [`Arg::default_value`] effectively disables this option as it will
    /// ensure that some argument is always present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::ArgRequiredElseHelp);
    /// ```
    ///
    /// [`subcommands`]: crate::App::subcommand()
    /// [`Arg::default_value`]: crate::Arg::default_value()
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
    /// **Note:** Make sure you apply it as `global_setting` if you want this setting
    /// to be propagated to subcommands and sub-subcommands!
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, Arg};
    /// let app = App::new("app").subcommand(App::new("sub")).arg(
    ///     Arg::new("arg")
    ///         .long("arg")
    ///         .multiple_values(true)
    ///         .takes_value(true),
    /// );
    ///
    /// let matches = app
    ///     .clone()
    ///     .try_get_matches_from(&["app", "--arg", "1", "2", "3", "sub"])
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     matches.values_of("arg").unwrap().collect::<Vec<_>>(),
    ///     &["1", "2", "3", "sub"]
    /// );
    /// assert!(matches.subcommand_matches("sub").is_none());
    ///
    /// let matches = app
    ///     .setting(AppSettings::SubcommandPrecedenceOverArg)
    ///     .try_get_matches_from(&["app", "--arg", "1", "2", "3", "sub"])
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     matches.values_of("arg").unwrap().collect::<Vec<_>>(),
    ///     &["1", "2", "3"]
    /// );
    /// assert!(matches.subcommand_matches("sub").is_some());
    /// ```
    SubcommandPrecedenceOverArg,

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
    ///
    /// [`Arg::use_delimiter(false)`]: crate::Arg::use_delimiter()
    DontDelimitTrailingValues,

    /// Disables `-h` and `--help` flag.
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
    DisableHelpFlag,

    /// Disables the `help` [`subcommand`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind, };
    /// let res = App::new("myprog")
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
    ///
    /// [`subcommand`]: crate::App::subcommand()
    DisableHelpSubcommand,

    /// Disables `-V` and `--version` flag.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let res = App::new("myprog")
    ///     .setting(AppSettings::DisableVersionFlag)
    ///     .try_get_matches_from(vec![
    ///         "myprog", "-V"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    DisableVersionFlag,

    /// Displays the arguments and [`subcommands`] in the help message in the order that they were
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
    ///
    /// [`subcommands`]: crate::App::subcommand()
    DeriveDisplayOrder,

    /// Parse the bin name (argv[0]) as a subcommand
    ///
    /// This adds a small performance penalty to startup
    /// as it requires comparing the bin name against every subcommand name.
    ///
    /// A "multicall" executable is a single executable
    /// that contains a variety of applets,
    /// and decides which applet to run based on the name of the file.
    /// The executable can be called from different names by creating hard links
    /// or symbolic links to it.
    ///
    /// This is desirable when it is convenient to store code
    /// for many programs in the same file,
    /// such as deduplicating code across multiple programs
    /// without loading a shared library at runtime.
    ///
    /// Multicall can't be used with [`NoBinaryName`] since they interpret
    /// the command name in incompatible ways.
    ///
    /// # Examples
    ///
    /// Multicall applets are defined as subcommands
    /// to an app which has the Multicall setting enabled.
    ///
    /// Busybox is a common example of a "multicall" executable
    /// with a subcommmand for each applet that can be run directly,
    /// e.g. with the `cat` applet being run by running `busybox cat`,
    /// or with `cat` as a link to the `busybox` binary.
    ///
    /// This is desirable when the launcher program has additional options
    /// or it is useful to run the applet without installing a symlink
    /// e.g. to test the applet without installing it
    /// or there may already be a command of that name installed.
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// let mut app = App::new("busybox")
    ///     .setting(AppSettings::Multicall)
    ///     .subcommand(App::new("true"))
    ///     .subcommand(App::new("false"));
    /// // When called from the executable's canonical name
    /// // its applets can be matched as subcommands.
    /// let m = app.try_get_matches_from_mut(&["busybox", "true"]).unwrap();
    /// assert_eq!(m.subcommand_name(), Some("true"));
    /// // When called from a link named after an applet that applet is matched.
    /// let m = app.get_matches_from(&["true"]);
    /// assert_eq!(m.subcommand_name(), Some("true"));
    /// ```
    ///
    /// `hostname` is another example of a multicall executable.
    /// It differs from busybox by not supporting running applets via subcommand
    /// and is instead only runnable via links.
    ///
    /// This is desirable when the executable has a primary purpose
    /// rather than being a collection of varied applets,
    /// so it is appropriate to name the executable after its purpose,
    /// but there is other related functionality that would be convenient to provide
    /// and it is convenient for the code to implement it to be in the same executable.
    ///
    /// This behaviour can be opted-into
    /// by naming a subcommand with the same as the program
    /// as applet names take priority.
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let mut app = App::new("hostname")
    ///     .setting(AppSettings::Multicall)
    ///     .subcommand(App::new("hostname"))
    ///     .subcommand(App::new("dnsdomainname"));
    /// let m = app.try_get_matches_from_mut(&["hostname", "dnsdomainname"]);
    /// assert!(m.is_err());
    /// assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// let m = app.get_matches_from(&["hostname"]);
    /// assert_eq!(m.subcommand_name(), Some("hostname"));
    /// ```
    ///
    /// [`subcommands`]: crate::App::subcommand()
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    /// [`NoBinaryName`]: crate::AppSettings::NoBinaryName
    /// [`try_get_matches_from_mut`]: crate::App::try_get_matches_from_mut()
    #[cfg(feature = "unstable-multicall")]
    Multicall,

    /// Specifies to use the version of the current command for all [`subcommands`].
    ///
    /// Defaults to `false`; subcommands have independent version strings from their parents.
    ///
    /// **Note:** Make sure you apply it as `global_setting` if you want this setting
    /// to be propagated to subcommands and sub-subcommands!
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .setting(AppSettings::PropagateVersion)
    ///     .subcommand(App::new("test"))
    ///     .get_matches();
    /// // running `$ myprog test --version` will display
    /// // "myprog-test v1.1"
    /// ```
    ///
    /// [`subcommands`]: crate::App::subcommand()
    PropagateVersion,

    /// Specifies that this [`subcommand`] should be hidden from help messages
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
    ///
    /// [`subcommand`]: crate::App::subcommand()
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

    /// Try not to fail on parse errors like missing option values.
    ///
    /// **Note:** Make sure you apply it as `global_setting` if you want this setting
    /// to be propagated to subcommands and sub-subcommands!
    ///
    /// Issue: [#1880 Partial / Pre Parsing a
    /// CLI](https://github.com/clap-rs/clap/issues/1880)
    ///
    /// This is the basis for:
    ///
    /// * [Changing app settings based on
    ///   flags](https://github.com/clap-rs/clap/issues/1880#issuecomment-637779787)
    /// * [#1232 Dynamic completion
    ///   support](https://github.com/clap-rs/clap/issues/1232)
    ///
    /// Support is not complete: Errors are still possible but they can be
    /// avoided in many cases.
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// let app = App::new("app")
    ///   .setting(AppSettings::IgnoreErrors)
    ///   .arg("-c, --config=[FILE] 'Sets a custom config file'")
    ///   .arg("-x, --stuff=[FILE] 'Sets a custom stuff file'")
    ///   .arg("-f 'Flag'");
    ///
    /// let r = app.try_get_matches_from(vec!["app", "-c", "file", "-f", "-x"]);
    ///
    /// assert!(r.is_ok(), "unexpected error: {:?}", r);
    /// let m = r.unwrap();
    /// assert_eq!(m.value_of("config"), Some("file"));
    /// assert!(m.is_present("f"));
    /// assert_eq!(m.value_of("stuff"), None);
    /// ```
    IgnoreErrors,

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
    ///
    /// [`subcommands`]: crate::App::subcommand()
    /// [positional/free arguments]: crate::Arg::index()
    /// [aliases]: crate::App::alias()
    InferSubcommands,

    /// Tries to match unknown args to partial long arguments or their [aliases]. For example, to
    /// match an argument named `--test`, one could use `--t`, `--te`, `--tes`, and `--test`.
    ///
    /// **NOTE:** The match *must not* be ambiguous at all in order to succeed. i.e. to match
    /// `--te` to `--test` there could not also be another argument or alias `--temp` because both
    /// start with `--te`
    ///
    /// [aliases]: crate::App::alias()
    InferLongArgs,

    /// Specifies that the parser should not assume the first argument passed is the binary name.
    /// This is normally the case when using a "daemon" style mode, or an interactive CLI where
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
    /// [`try_get_matches_from_mut`]: crate::App::try_get_matches_from_mut()
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

    /// Allows [`subcommands`] to override all requirements of the parent command.
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
    ///
    /// [`Arg::required(true)`]: crate::Arg::required()
    /// [`subcommands`]: crate::App::subcommand()
    SubcommandsNegateReqs,

    /// Specifies that the help text should be displayed (before exiting gracefully) if no
    /// [`subcommands`] are present at runtime (i.e. an empty run such as `$ myprog`).
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
    ///     .setting(AppSettings::SubcommandRequiredElseHelp);
    /// ```
    ///
    /// [`subcommands`]: crate::App::subcommand()
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
    /// [long format]: crate::App::long_about
    UseLongFormatForHelpSubcommand,

    /// Allows specifying that if no [`subcommand`] is present at runtime,
    /// error and exit gracefully.
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
    ///
    /// [`subcommand`]: crate::App::subcommand()
    SubcommandRequired,

    /// Specifies that the final positional argument is a "VarArg" and that `clap` should not
    /// attempt to parse any further args.
    ///
    /// The values of the trailing positional argument will contain all args from itself on.
    ///
    /// **NOTE:** The final positional argument **must** have [`Arg::multiple_values(true)`] or the usage
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
    /// [`Arg::multiple_values(true)`]: crate::Arg::multiple_values()
    TrailingVarArg,

    /// Will display a message "Press \[ENTER\]/\[RETURN\] to continue..." and wait for user before
    /// exiting
    ///
    /// This is most useful when writing an application which is run from a GUI shortcut, or on
    /// Windows where a user tries to open the binary by double-clicking instead of using the
    /// command line.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::WaitOnError);
    /// ```
    WaitOnError,

    /// Tells clap to treat the auto-generated `-h, --help` flags just like any other flag, and
    /// *not* print the help message. This allows one to handle printing of the help message
    /// manually.
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// let result = App::new("myprog")
    ///     .setting(AppSettings::NoAutoHelp)
    ///     .try_get_matches_from("myprog --help".split(" "));
    ///
    /// // Normally, if `--help` is used clap prints the help message and returns an
    /// // ErrorKind::DisplayHelp
    /// //
    /// // However, `--help` was treated like a normal flag
    ///
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().is_present("help"));
    /// ```
    NoAutoHelp,

    /// Tells clap to treat the auto-generated `-V, --version` flags just like any other flag, and
    /// *not* print the version message. This allows one to handle printing of the version message
    /// manually.
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// let result = App::new("myprog")
    ///     .version("3.0")
    ///     .setting(AppSettings::NoAutoVersion)
    ///     .try_get_matches_from("myprog --version".split(" "));
    ///
    /// // Normally, if `--version` is used clap prints the version message and returns an
    /// // ErrorKind::DisplayVersion
    /// //
    /// // However, `--version` was treated like a normal flag
    ///
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().is_present("version"));
    /// ```
    NoAutoVersion,

    #[doc(hidden)]
    /// If the app is already built, used for caching.
    Built,

    #[doc(hidden)]
    /// If the app's bin name is already built, used for caching.
    BinNameBuilt,
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
            "allowinvalidutf8forexternalsubcommands"
                .parse::<AppSettings>()
                .unwrap(),
            AppSettings::AllowInvalidUtf8ForExternalSubcommands
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
            "propagateversion".parse::<AppSettings>().unwrap(),
            AppSettings::PropagateVersion
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
            "trailingvararg".parse::<AppSettings>().unwrap(),
            AppSettings::TrailingVarArg
        );
        assert_eq!(
            "waitonerror".parse::<AppSettings>().unwrap(),
            AppSettings::WaitOnError
        );
        assert_eq!("built".parse::<AppSettings>().unwrap(), AppSettings::Built);
        assert_eq!(
            "binnamebuilt".parse::<AppSettings>().unwrap(),
            AppSettings::BinNameBuilt
        );
        assert_eq!(
            "infersubcommands".parse::<AppSettings>().unwrap(),
            AppSettings::InferSubcommands
        );
        assert!("hahahaha".parse::<AppSettings>().is_err());
    }
}
