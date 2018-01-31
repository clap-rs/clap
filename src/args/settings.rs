// Std
#[allow(unused_imports)]
use std::ascii::AsciiExt;
use std::str::FromStr;

bitflags! {
    struct Flags: u32 {
        const REQUIRED         = 1;
        const MULTIPLE_OCC     = 1 << 1;
        const EMPTY_VALS       = 1 << 2 | Self::TAKES_VAL.bits;
        const GLOBAL           = 1 << 3;
        const HIDDEN           = 1 << 4;
        const TAKES_VAL        = 1 << 5;
        const USE_DELIM        = 1 << 6;
        const NEXT_LINE_HELP   = 1 << 7;
        const R_UNLESS_ALL     = 1 << 8;
        const REQ_DELIM        = 1 << 9 | Self::TAKES_VAL.bits | Self::USE_DELIM.bits;
        const DELIM_NOT_SET    = 1 << 10;
        const HIDE_POS_VALS    = 1 << 11 | Self::TAKES_VAL.bits;
        const ALLOW_TAC_VALS   = 1 << 12 | Self::TAKES_VAL.bits;
        const REQUIRE_EQUALS   = 1 << 13 | Self::TAKES_VAL.bits;
        const LAST             = 1 << 14 | Self::TAKES_VAL.bits;
        const HIDE_DEFAULT_VAL = 1 << 15 | Self::TAKES_VAL.bits;
        const CASE_INSENSITIVE = 1 << 16;
        const HIDE_ENV_VALS    = 1 << 17;
        const MULTIPLE_VALS    = 1 << 18 | Self::TAKES_VAL.bits;
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ArgFlags(Flags);

impl ArgFlags {
    pub fn new() -> Self { ArgFlags::default() }

    // @TODO @p6 @internal: Reorder alphabetically
    impl_settings!{ArgSettings,
        Required => Flags::REQUIRED,
        MultipleOccurrences => Flags::MULTIPLE_OCC,
        MultipleValues => Flags::MULTIPLE_VALS,
        AllowEmptyValues => Flags::EMPTY_VALS,
        Global => Flags::GLOBAL,
        Hidden => Flags::HIDDEN,
        TakesValue => Flags::TAKES_VAL,
        UseValueDelimiter => Flags::USE_DELIM,
        NextLineHelp => Flags::NEXT_LINE_HELP,
        RequiredUnlessAll => Flags::R_UNLESS_ALL,
        RequireDelimiter => Flags::REQ_DELIM,
        ValueDelimiterNotSet => Flags::DELIM_NOT_SET,
        HidePossibleValues => Flags::HIDE_POS_VALS,
        AllowHyphenValues => Flags::ALLOW_TAC_VALS,
        RequireEquals => Flags::REQUIRE_EQUALS,
        Last => Flags::LAST,
        IgnoreCase => Flags::CASE_INSENSITIVE,
        HideEnvValues => Flags::HIDE_ENV_VALS,
        HideDefaultValue => Flags::HIDE_DEFAULT_VAL
    }
}

impl Default for ArgFlags {
    fn default() -> Self { ArgFlags(Flags::DELIM_NOT_SET) }
}

/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods [`Arg::set`], [`Arg::unset`], and [`Arg::is_set`]
/// [`Arg::set`]: ./struct.Arg.html#method.set
/// [`Arg::unset`]: ./struct.Arg.html#method.unset
/// [`Arg::is_set`]: ./struct.Arg.html#method.is_set
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgSettings {
    /// Specifies that the argument is required by default. Required by default means it is
    /// required, when no other conflicting rules or overrides have been evaluated. Conflicting
    /// rules take precedence over being required.
    ///
    /// **Pro tip:** Flags (i.e. not positional, or arguments that take values) shouldn't be
    /// required by default. This is because if a flag were to be required, it should simply be
    /// implied. No additional information is required from user. Flags by their very nature are
    /// simply boolean on/off switches. The only time a user *should* be required to use a flag
    /// is if the operation is destructive in nature, and the user is essentially proving to you,
    /// "Yes, I know what I'm doing."
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Arg, ArgSettings};
    /// Arg::with_name("config")
    ///     .setting(ArgSettings::Required)
    /// # ;
    /// ```
    ///
    /// Setting [`Required`] requires that the argument be used at runtime.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .settings(&[ArgSettings::Required, ArgSettings::TakesValue])
    ///         .long("config"))
    ///     .try_get_matches_from(vec![
    ///         "prog", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Not setting [`Required`] and then *not* supplying that argument at runtime is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .settings(&[ArgSettings::Required, ArgSettings::TakesValue])
    ///         .long("config"))
    ///     .try_get_matches_from(vec![
    ///         "prog"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    /// ```
    /// [`Required`]: ./enum.ArgSettings.html#variant.Required
    Required,
    /// Specifies that the argument may have an unknown number of multiple values. Without any other
    /// settings, this argument may appear only *once*.
    ///
    /// For example, `--opt val1 val2` is allowed, but `--opt val1 val2 --opt val3` is not.
    ///
    /// **NOTE:** Implicitly sets [`ArgSettings::TakesValue`]
    ///
    /// **WARNING:**
    ///
    /// Setting `MultipleValues` for an argument that takes a value, but with no other details can
    /// be dangerous in some circumstances. Because multiple values are allowed,
    /// `--option val1 val2 val3` is perfectly valid. Be careful when designing a CLI where
    /// positional arguments are *also* expected as `clap` will continue parsing *values* until one
    /// of the following happens:
    ///
    /// * It reaches the [maximum number of values]
    /// * It reaches a [specific number of values]
    /// * It finds another flag or option (i.e. something that starts with a `-`)
    ///
    /// **WARNING:**
    ///
    /// When using args with `MultipleValues` and [subcommands], one needs to consider the
    /// posibility of an argument value being the same as a valid subcommand. By default `clap` will
    /// parse the argument in question as a value *only if* a value is possible at that moment.
    /// Otherwise it will be parsed as a subcommand. In effect, this means using `Multiple` with no
    /// additional parameters and a value that coincides with a subcommand name, the subcommand
    /// cannot be called unless another argument is passed between them.
    ///
    /// As an example, consider a CLI with an option `--ui-paths=<paths>...` and subcommand `signer`
    ///
    /// The following would be parsed as values to `--ui-paths`.
    ///
    /// ```notrust
    /// $ program --ui-paths path1 path2 signer
    /// ```
    ///
    /// This is because `--ui-paths` accepts multiple values. `clap` will continue parsing values
    /// until another argument is reached and it knows `--ui-paths` is done parsing.
    ///
    /// By adding additional parameters to `--ui-paths` we can solve this issue. Consider adding
    /// [`Arg::number_of_values(1)`] or using *only* [`MultipleOccurrences`]. The following are all
    /// valid, and `signer` is parsed as a subcommand in the first case, but a value in the second
    /// case.
    ///
    /// ```notrust
    /// $ program --ui-paths path1 signer
    /// $ program --ui-paths path1 --ui-paths signer signer
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("debug")
    ///     .short("d")
    ///     .setting(ArgSettings::MultipleValues)
    /// # ;
    /// ```
    /// An example with flags
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("verbose")
    ///         .setting(ArgSettings::MultipleValues)
    ///         .short("v"))
    ///     .get_matches_from(vec![
    ///         "prog", "-v", "-v", "-v"    // note, -vvv would have same result
    ///     ]);
    ///
    /// assert!(m.is_present("verbose"));
    /// assert_eq!(m.occurrences_of("verbose"), 3);
    /// ```
    ///
    /// An example with options
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .setting(ArgSettings::MultipleValues) // implies TakesValue
    ///         .short("F"))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "file2", "file3"
    ///     ]);
    ///
    /// assert!(m.is_present("file"));
    /// assert_eq!(m.occurrences_of("file"), 1); // notice only one occurrence
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// ```
    /// Although `MultipleVlaues` has been specified, we cannot use the argument more than once.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .setting(ArgSettings::MultipleValues) // implies TakesValue
    ///         .short("F"))
    ///     .try_get_matches_from(vec![
    ///         "prog", "-F", "file1", "-F", "file2", "-F", "file3"
    ///     ]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnexpectedMultipleUsage)
    /// ```
    ///
    /// A common mistake is to define an option which allows multiple values, and a positional
    /// argument.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .setting(ArgSettings::MultipleValues) // implies TakesValue
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "file2", "file3", "word"
    ///     ]);
    ///
    /// assert!(m.is_present("file"));
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3", "word"]); // wait...what?!
    /// assert!(!m.is_present("word")); // but we clearly used word!
    /// ```
    /// The problem is `clap` doesn't know when to stop parsing values for "files". This is further
    /// compounded by if we'd said `word -F file1 file2` it would have worked fine, so it would
    /// appear to only fail sometimes...not good!
    ///
    /// A solution for the example above is to limit how many values with a [maxium], or [specific]
    /// number, or to say [`MultipleOccurrences`] is ok, but multiple values is not.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .settings(&[ArgSettings::MultipleOccurrences, ArgSettings::TakesValue])
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "-F", "file2", "-F", "file3", "word"
    ///     ]);
    ///
    /// assert!(m.is_present("file"));
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// assert!(m.is_present("word"));
    /// assert_eq!(m.value_of("word"), Some("word"));
    /// ```
    /// As a final example, let's fix the above error and get a pretty message to the user :)
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .settings(&[ArgSettings::MultipleOccurrences, ArgSettings::TakesValue])
    ///         .short("F"))
    ///     .arg(Arg::with_name("word")
    ///         .index(1))
    ///     .try_get_matches_from(vec![
    ///         "prog", "-F", "file1", "file2", "file3", "word"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [option]: ./enum.ArgSettings.html#variant.TakesValue
    /// [options]: ./enum.ArgSettings.html#variant.TakesValue
    /// [subcommands]: ./struct.App.html#method.subcommand
    /// [positionals]: ./struct.Arg.html#method.index
    /// [`Arg::number_of_values(1)`]: ./struct.Arg.html#method.number_of_values
    /// [`MultipleOccurrences`]: ./enum.ArgSettings.html#variant.MultipleOccurrences
    /// [`MultipleValues`]: ./enum.ArgSettings.html#variant.MultipleValues
    /// [maximum number of values]: ./struct.Arg.html#method.max_values
    /// [specific number of values]: ./struct.Arg.html#method.number_of_values
    /// [maximum]: ./struct.Arg.html#method.max_values
    /// [specific]: ./struct.Arg.html#method.number_of_values
    MultipleValues,
    /// Specifies that the argument may appear more than once.
    /// For flags, this results
    /// in the number of occurrences of the flag being recorded. For example `-ddd` or `-d -d -d`
    /// would count as three occurrences. For options or arguments that take a value, this
    /// *does not* affect how many values they can accept. (i.e. only one at a time is allowed)
    ///
    /// For example, `--opt val1 --opt val2` is allowed, but `--opt val1 val2` is not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("debug")
    ///     .short("d")
    ///     .setting(ArgSettings::MultipleOccurrences)
    /// # ;
    /// ```
    /// An example with flags
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("verbose")
    ///         .setting(ArgSettings::MultipleOccurrences)
    ///         .short("v"))
    ///     .get_matches_from(vec![
    ///         "prog", "-v", "-v", "-v"    // note, -vvv would have same result
    ///     ]);
    ///
    /// assert!(m.is_present("verbose"));
    /// assert_eq!(m.occurrences_of("verbose"), 3);
    /// ```
    ///
    /// An example with options
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("file")
    ///         .settings(&[ArgSettings::MultipleOccurrences, ArgSettings::TakesValue])
    ///         .short("F"))
    ///     .get_matches_from(vec![
    ///         "prog", "-F", "file1", "-F", file2", "-F", "file3"
    ///     ]);
    ///
    /// assert!(m.is_present("file"));
    /// assert_eq!(m.occurrences_of("file"), 1); // notice only one occurrence
    /// let files: Vec<_> = m.values_of("file").unwrap().collect();
    /// assert_eq!(files, ["file1", "file2", "file3"]);
    /// ```
    /// [option]: ./enum.ArgSettings.html#variant.TakesValue
    /// [options]: ./enum.ArgSettings.html#variant.TakesValue
    /// [subcommands]: ./struct.App.html#method.subcommand
    /// [positionals]: ./struct.Arg.html#method.index
    /// [`Arg::number_of_values(1)`]: ./struct.Arg.html#method.number_of_values
    /// [`MultipleOccurrences`]: ./enum.ArgSettings.html#variant.MultipleOccurrences
    /// [`MultipleValues`]: ./enum.ArgSettings.html#variant.MultipleValues
    /// [maximum number of values]: ./struct.Arg.html#method.max_values
    /// [specific number of values]: ./struct.Arg.html#method.number_of_values
    /// [maximum]: ./struct.Arg.html#method.max_values
    /// [specific]: ./struct.Arg.html#method.number_of_values
    MultipleOccurrences,
    /// Allows an argument to accept explicitly empty values. An empty value must be specified at
    /// the command line with an explicit `""`, `''`, or `--option=`
    ///
    /// **NOTE:** By default empty values are *not* allowed
    ///
    /// **NOTE:** Implicitly sets [`ArgSettings::TakesValue`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("file")
    ///     .long("file")
    ///     .setting(ArgSettings::AllowEmptyValues)
    /// # ;
    /// ```
    /// The default is to *not* allow empty values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .short("v")
    ///         .setting(ArgSettings::TakesValue))
    ///     .try_get_matches_from(vec![
    ///         "prog", "--config="
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    /// By adding this setting, we can allow empty values
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .short("v")
    ///         .setting(ArgSettings::AllowEmptyValues)) // implies TakesValue
    ///     .try_get_matches_from(vec![
    ///         "prog", "--config="
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// assert_eq!(res.unwrap().value_of("config"), None);
    /// ```
    /// [`ArgSettings::TakesValue`]: ./enum.ArgSettings.html#variant.TakesValue
    AllowEmptyValues,
    /// Specifies that an argument can be matched to all child [`SubCommand`]s.
    ///
    /// **NOTE:** Global arguments *only* propagate down, **not** up (to parent commands), however
    /// their values once a user uses them will be propagated back up to parents. In effect, this
    /// means one should *define* all global arguments at the top level, however it doesn't matter
    /// where the user *uses* the global argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("debug")
    ///     .short("d")
    ///     .setting(ArgSettings::Global)
    /// # ;
    /// ```
    ///
    /// For example, assume an appliction with two subcommands, and you'd like to define a
    /// `--verbose` flag that can be called on any of the subcommands and parent, but you don't
    /// want to clutter the source with three duplicate [`Arg`] definitions.
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("verb")
    ///         .long("verbose")
    ///         .short("v")
    ///         .setting(ArgSettings::Global))
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .subcommand(SubCommand::with_name("do-stuff"))
    ///     .get_matches_from(vec![
    ///         "prog", "do-stuff", "--verbose"
    ///     ]);
    ///
    /// assert_eq!(m.subcommand_name(), Some("do-stuff"));
    /// let sub_m = m.subcommand_matches("do-stuff").unwrap();
    /// assert!(sub_m.is_present("verb"));
    /// ```
    /// [`SubCommand`]: ./struct.App.html#method.subcommand
    /// [required]: ./enum.ArgSettings.html#variant.Required
    /// [`ArgMatches`]: ./struct.ArgMatches.html
    /// [`ArgMatches::is_present("flag")`]: ./struct.ArgMatches.html#method.is_present
    /// [`Arg`]: ./struct.Arg.html
    Global,
    /// Hides an argument from help message output.
    ///
    /// **NOTE:** This does **not** hide the argument from usage strings on error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("debug")
    ///     .setting(ArgSettings::Hidden)
    /// # ;
    /// ```
    /// Setting `Hidden` will hide the argument when displaying help text
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .long("config")
    ///         .setting(ArgSettings::Hidden)
    ///         .help("Some help text describing the --config arg"))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays
    ///
    /// ```notrust
    /// helptest
    ///
    /// USAGE:
    ///    helptest [FLAGS]
    ///
    /// FLAGS:
    /// -h, --help       Prints help information
    /// -V, --version    Prints version information
    /// ```
    Hidden,
    /// Specifies that the argument takes a value at run time.
    ///
    /// **NOTE:** values for arguments may be specified in any of the following methods
    ///
    /// * Using a space such as `-o value` or `--option value`
    /// * Using an equals and no space such as `-o=value` or `--option=value`
    /// * Use a short and no space such as `-ovalue`
    ///
    /// **NOTE:** By default, args which allow [multiple values] are delimited by commas, meaning
    /// `--option=val1,val2,val3` is three values for the `--option` argument. If you wish to
    /// change the delimiter to another character you can use [`Arg::value_delimiter(char)`],
    /// alternatively you can turn delimiting values **OFF** by using
    /// [`Arg::unset_setting(ArgSettings::UseValueDelimiter`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("config")
    ///     .setting(ArgSettings::TakesValue)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .setting(ArgSettings::TakesValue))
    ///     .get_matches_from(vec![
    ///         "prog", "--mode", "fast"
    ///     ]);
    ///
    /// assert!(m.is_present("mode"));
    /// assert_eq!(m.value_of("mode"), Some("fast"));
    /// ```
    /// [`Arg::value_delimiter(char)`]: ./struct.Arg.html#method.value_delimiter
    /// [`Arg::unset_setting(ArgSettings::UseValueDelimiter`]: ./enum.ArgSettings.html#variant.UseValueDelimiter
    /// [multiple values]: ./enum.ArgSettings.html#variant.MultipleValues
    TakesValue,
    /// Specifies that an argument should allow grouping of multiple values via a
    /// delimiter. I.e. should `--option=val1,val2,val3` be parsed as three values (`val1`, `val2`,
    /// and `val3`) or as a single value (`val1,val2,val3`). Defaults to using `,` (comma) as the
    /// value delimiter for all arguments that accept values (options and positional arguments)
    ///
    /// **NOTE:** When this setting is used, it will default [`Arg::value_delimiter`]
    /// to the comma `,`.
    ///
    /// **NOTE:** Implicitly sets [`ArgSettings::TakesValue`]
    ///
    /// # Examples
    ///
    /// The following example shows the default behavior.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let delims = App::new("prog")
    ///     .arg(Arg::with_name("option")
    ///         .long("option")
    ///         .setting(ArgSettings::UseValueDelimiter)
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "prog", "--option=val1,val2,val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("option"));
    /// assert_eq!(delims.occurrences_of("option"), 1);
    /// assert_eq!(delims.values_of("option").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// The next example shows the difference when turning delimiters off. This is the default
    /// behavior
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let nodelims = App::new("prog")
    ///     .arg(Arg::with_name("option")
    ///         .long("option")
    ///         .setting(ArgSettings::TakesValue))
    ///     .get_matches_from(vec![
    ///         "prog", "--option=val1,val2,val3",
    ///     ]);
    ///
    /// assert!(nodelims.is_present("option"));
    /// assert_eq!(nodelims.occurrences_of("option"), 1);
    /// assert_eq!(nodelims.value_of("option").unwrap(), "val1,val2,val3");
    /// ```
    /// [`Arg::value_delimiter`]: ./struct.Arg.html#method.value_delimiter
    UseValueDelimiter,
    /// When set to `true` the help string will be displayed on the line after the argument and
    /// indented once. This can be helpful for arguments with very long or complex help messages.
    /// This can also be helpful for arguments with very long flag names, or many/long value names.
    ///
    /// **NOTE:** To apply this setting to all arguments consider using
    /// [`AppSettings::NextLineHelp`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .long("long-option-flag")
    ///         .short("o")
    ///         .settings(&[ArgSettings::TakesValue, ArgSettings::NextLineHelp])
    ///         .value_names(&["value1", "value2"])
    ///         .help("Some really long help and complex\n\
    ///                help that makes more sense to be\n\
    ///                on a line after the option"))
    ///     .get_matches_from(vec![
    ///         "prog", "--help"
    ///     ]);
    /// ```
    ///
    /// The above example displays the following help message
    ///
    /// ```notrust
    /// nlh
    ///
    /// USAGE:
    ///     nlh [FLAGS] [OPTIONS]
    ///
    /// FLAGS:
    ///     -h, --help       Prints help information
    ///     -V, --version    Prints version information
    ///
    /// OPTIONS:
    ///     -o, --long-option-flag <value1> <value2>
    ///         Some really long help and complex
    ///         help that makes more sense to be
    ///         on a line after the option
    /// ```
    /// [`AppSettings::NextLineHelp`]: ./enum.AppSettings.html#variant.NextLineHelp
    NextLineHelp,
    /// Specifies that *multiple values* may only be set using the delimiter. This means if an
    /// if an option is encountered, and no delimiter is found, it automatically assumed that no
    /// additional values for that option follow. This is unlike the default, where it is generally
    /// assumed that more values will follow regardless of whether or not a delimiter is used.
    ///
    /// **NOTE:** The default is `false`.
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::UseValueDelimiter`] and
    /// [`ArgSettings::TakesValue`]
    ///
    /// **NOTE:** It's a good idea to inform the user that use of a delimiter is required, either
    /// through help text or other means.
    ///
    /// # Examples
    ///
    /// These examples demonstrate what happens when `require_delimiter(true)` is used. Notice
    /// everything works in this first example, as we use a delimiter, as expected.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let delims = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .settings(&[ArgSettings::RequireDelimiter, ArgSettings::MultipleValues]))
    ///     .get_matches_from(vec![
    ///         "prog", "-o", "val1,val2,val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("opt"));
    /// assert_eq!(delims.values_of("opt").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// In this next example, we will *not* use a delimiter. Notice it's now an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .setting(ArgSettings::RequireDelimiter))
    ///     .try_get_matches_from(vec![
    ///         "prog", "-o", "val1", "val2", "val3",
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// assert_eq!(err.kind, ErrorKind::UnknownArgument);
    /// ```
    /// What's happening is `-o` is getting `val1`, and because delimiters are required yet none
    /// were present, it stops parsing `-o`. At this point it reaches `val2` and because no
    /// positional arguments have been defined, it's an error of an unexpected argument.
    ///
    /// In this final example, we contrast the above with `clap`'s default behavior where the above
    /// is *not* an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let delims = App::new("prog")
    ///     .arg(Arg::with_name("opt")
    ///         .short("o")
    ///         .setting(ArgSettings::MultipleValues))
    ///     .get_matches_from(vec![
    ///         "prog", "-o", "val1", "val2", "val3",
    ///     ]);
    ///
    /// assert!(delims.is_present("opt"));
    /// assert_eq!(delims.values_of("opt").unwrap().collect::<Vec<_>>(), ["val1", "val2", "val3"]);
    /// ```
    /// [`ArgSettings::UseValueDelimiter`]: ./enum.ArgSettings.html#variant.UseValueDelimiter
    /// [`ArgSettings::TakesValue`]: ./enum.ArgSettings.html#variant.TakesValue
    RequireDelimiter,
    /// Specifies if the possible values of an argument should be displayed in the help text or
    /// not. Defaults to `false` (i.e. show possible values)
    ///
    /// This is useful for args with many values, or ones which are explained elsewhere in the
    /// help text.
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::TakesValue`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("config")
    ///     .setting(ArgSettings::HidePossibleValues)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("mode")
    ///         .long("mode")
    ///         .possible_values(&["fast", "slow"])
    ///         .setting(ArgSettings::HidePossibleValues));
    /// ```
    /// If we were to run the above program with `--help` the `[values: fast, slow]` portion of
    /// the help text would be omitted.
    HidePossibleValues,
    /// Allows values which start with a leading hyphen (`-`)
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::TakesValue`]
    ///
    /// **WARNING**: Take caution when using this setting combined with
    /// [`ArgSettings::MultipleValues`], as this becomes ambiguous `$ prog --arg -- -- val`. All
    /// three `--, --, val` will be values when the user may have thought the second `--` would
    /// constitute the normal, "Only positional args follow" idiom. To fix this, consider using
    /// [`ArgSettings::MultipleOccurrences`] which only allows a single value at a time.
    ///
    /// **WARNING**: When building your CLIs, consider the effects of allowing leading hyphens and
    /// the user passing in a value that matches a valid short. For example `prog -opt -F` where
    /// `-F` is supposed to be a value, yet `-F` is *also* a valid short for another arg. Care should
    /// should be taken when designing these args. This is compounded by the ability to "stack"
    /// short args. I.e. if `-val` is supposed to be a value, but `-v`, `-a`, and `-l` are all valid
    /// shorts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Arg, ArgSettings};
    /// Arg::with_name("pattern")
    ///     .setting(ArgSettings::AllowHyphenValues)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("prog")
    ///     .arg(Arg::with_name("pat")
    ///         .setting(ArgSettings::AllowHyphenValues)
    ///         .long("pattern"))
    ///     .get_matches_from(vec![
    ///         "prog", "--pattern", "-file"
    ///     ]);
    ///
    /// assert_eq!(m.value_of("pat"), Some("-file"));
    /// ```
    ///
    /// Not setting [`Arg::allow_hyphen_values(true)`] and supplying a value which starts with a
    /// hyphen is an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("pat")
    ///         .setting(ArgSettings::TakesValue)
    ///         .long("pattern"))
    ///     .try_get_matches_from(vec![
    ///         "prog", "--pattern", "-file"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [`ArgSettings::AllowHyphenValues`]: ./enum.ArgSettings.html#variant.AllowHyphenValues
    /// [`ArgSettings::MultipleValues`]: ./enum.ArgSettings.html#variant.MultipleValues
    /// [`ArgSettings::MultipleOccurrences`]: ./enum.ArgSettings.html#variant.MultipleOccurrences
    /// [`Arg::number_of_values(1)`]: ./struct.Arg.html#method.number_of_values
    AllowHyphenValues,
    /// Requires that options use the `--option=val` syntax (i.e. an equals between the option and
    /// associated value) **Default:** `false`
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::TakesValue`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Arg, ArgSettings};
    /// Arg::with_name("config")
    ///     .long("config")
    ///     .setting(ArgSettings::RequireEquals)
    /// # ;
    /// ```
    ///
    /// Setting [`RequireEquals`] requires that the option have an equals sign between
    /// it and the associated value.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .setting(ArgSettings::RequireEquals)
    ///         .long("config"))
    ///     .try_get_matches_from(vec![
    ///         "prog", "--config=file.conf"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// ```
    ///
    /// Setting [`RequireEquals`] and *not* supplying the equals will cause an error
    /// unless [`ArgSettings::EmptyValues`] is set.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("cfg")
    ///         .setting(ArgSettings::RequireEquals)
    ///         .long("config"))
    ///     .try_get_matches_from(vec![
    ///         "prog", "--config", "file.conf"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::EmptyValue);
    /// ```
    /// [`RequireEquals`]: ./enum.ArgSettings.html#variant.RequireEquals
    /// [`ArgSettings::EmptyValues`]: ./enum.ArgSettings.html#variant.EmptyValues
    /// [`ArgSettings::EmptyValues`]: ./enum.ArgSettings.html#variant.TakesValue
    RequireEquals,
    /// Specifies that this arg is the last, or final, positional argument (i.e. has the highest
    /// index) and is *only* able to be accessed via the `--` syntax (i.e. `$ prog args --
    /// last_arg`). Even, if no other arguments are left to parse, if the user omits the `--` syntax
    /// they will receive an [`UnknownArgument`] error. Setting an argument to `.last(true)` also
    /// allows one to access this arg early using the `--` syntax. Accessing an arg early, even with
    /// the `--` syntax is otherwise not possible.
    ///
    /// **NOTE:** This will change the usage string to look like `$ prog [FLAGS] [-- <ARG>]` if
    /// `ARG` is marked as `.last(true)`.
    ///
    /// **NOTE:** This setting will imply [`AppSettings::DontCollapseArgsInUsage`] because failing
    /// to set this can make the usage string very confusing.
    ///
    /// **NOTE**: This setting only applies to positional arguments, and has no affect on FLAGS /
    /// OPTIONS
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::TakesValue`]
    ///
    /// **CAUTION:** Using this setting *and* having child subcommands is not
    /// recommended with the exception of *also* using [`AppSettings::ArgsNegateSubcommands`]
    /// (or [`AppSettings::SubcommandsNegateReqs`] if the argument marked `Last` is also
    /// marked [`ArgSettings::Required`])
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{Arg, ArgSettings};
    /// Arg::with_name("args")
    ///     .setting(ArgSettings::Last)
    /// # ;
    /// ```
    ///
    /// Setting [`Last`] ensures the arg has the highest [index] of all positional args
    /// and requires that the `--` syntax be used to access it early.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("first"))
    ///     .arg(Arg::with_name("second"))
    ///     .arg(Arg::with_name("third")
    ///         .setting(ArgSettings::Last))
    ///     .try_get_matches_from(vec![
    ///         "prog", "one", "--", "three"
    ///     ]);
    ///
    /// assert!(res.is_ok());
    /// let m = res.unwrap();
    /// assert_eq!(m.value_of("third"), Some("three"));
    /// assert!(m.value_of("second").is_none());
    /// ```
    ///
    /// Even if the positional argument marked `Last` is the only argument left to parse,
    /// failing to use the `--` syntax results in an error.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, ArgSettings};
    /// let res = App::new("prog")
    ///     .arg(Arg::with_name("first"))
    ///     .arg(Arg::with_name("second"))
    ///     .arg(Arg::with_name("third")
    ///         .setting(ArgSettings::Last))
    ///     .try_get_matches_from(vec![
    ///         "prog", "one", "two", "three"
    ///     ]);
    ///
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind, ErrorKind::UnknownArgument);
    /// ```
    /// [index]: ./struct.Arg.html#method.index
    /// [`AppSettings::DontCollapseArgsInUsage`]: ./enum.AppSettings.html#variant.DontCollapseArgsInUsage
    /// [`AppSettings::ArgsNegateSubcommands`]: ./enum.AppSettings.html#variant.ArgsNegateSubcommands
    /// [`AppSettings::SubcommandsNegateReqs`]: ./enum.AppSettings.html#variant.SubcommandsNegateReqs
    /// [`ArgSettings::Required`]: ./enum.ArgSetings.html#variant.Required
    /// [`UnknownArgument`]: ./enum.ErrorKind.html#variant.UnknownArgument
    Last,
    /// Specifies that the default value of an argument should not be displayed in the help text.
    ///
    /// This is useful when default behavior of an arg is explained elsewhere in the help text.
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::TakesValue`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("config")
    ///     .setting(ArgSettings::HideDefaultValue)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("connect")
    ///     .arg(Arg::with_name("host")
    ///         .long("host")
    ///         .default_value("localhost")
    ///         .setting(ArgSettings::HideDefaultValue));
    ///
    /// ```
    ///
    /// If we were to run the above program with `--help` the `[default: localhost]` portion of
    /// the help text would be omitted.
    HideDefaultValue,
    /// When used with [`Arg::possible_values`] it allows the argument value to pass validation even
    /// if the case differs from that of the specified `possible_value`.
    ///
    /// **Pro Tip:** Use this setting with [`arg_enum!`]
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::TakesValue`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// # use std::ascii::AsciiExt;
    /// let m = App::new("pv")
    ///     .arg(Arg::with_name("option")
    ///         .long("--option")
    ///         .setting(ArgSettings::IgnoreCase)
    ///         .possible_value("test123"))
    ///     .get_matches_from(vec![
    ///         "pv", "--option", "TeSt123",
    ///     ]);
    ///
    /// assert!(m.value_of("option").unwrap().eq_ignore_ascii_case("test123"));
    /// ```
    ///
    /// This setting also works when multiple values can be defined:
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("pv")
    ///     .arg(Arg::with_name("option")
    ///         .short("-o")
    ///         .long("--option")
    ///         .settings(&[ArgSettings::IgnoreCase, ArgSettings::MultipleValues])
    ///         .possible_value("test123")
    ///         .possible_value("test321"))
    ///     .get_matches_from(vec![
    ///         "pv", "--option", "TeSt123", "teST123", "tESt321"
    ///     ]);
    ///
    /// let matched_vals = m.values_of("option").unwrap().collect::<Vec<_>>();
    /// assert_eq!(&*matched_vals, &["TeSt123", "teST123", "tESt321"]);
    /// ```
    /// [`arg_enum!`]: ./macro.arg_enum.html
    IgnoreCase,
    /// Specifies that any values inside the associated ENV variables of an argument should not be
    /// displayed in the help text.
    ///
    /// This is useful when ENV vars contain sensitive values.
    ///
    /// **NOTE:** Setting this implies [`ArgSettings::TakesValue`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// Arg::with_name("config")
    ///     .setting(ArgSettings::HideDefaultValue)
    /// # ;
    /// ```
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("connect")
    ///     .arg(Arg::with_name("host")
    ///         .long("host")
    ///         .env("CONNECT")
    ///         .setting(ArgSettings::HideEnvValues));
    ///
    /// ```
    ///
    /// If we were to run the above program with `$ CONNECT=super_secret connect --help` the
    /// `[default: CONNECT=super_secret]` portion of the help text would be omitted.
    HideEnvValues,
    #[doc(hidden)] RequiredUnlessAll,
    #[doc(hidden)] ValueDelimiterNotSet,
}

impl FromStr for ArgSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "required" => Ok(ArgSettings::Required),
            "global" => Ok(ArgSettings::Global),
            "allowemptyvalues" => Ok(ArgSettings::AllowEmptyValues),
            "hidden" => Ok(ArgSettings::Hidden),
            "takesvalue" => Ok(ArgSettings::TakesValue),
            "usevaluedelimiter" => Ok(ArgSettings::UseValueDelimiter),
            "nextlinehelp" => Ok(ArgSettings::NextLineHelp),
            "requiredunlessall" => Ok(ArgSettings::RequiredUnlessAll),
            "requiredelimiter" => Ok(ArgSettings::RequireDelimiter),
            "valuedelimiternotset" => Ok(ArgSettings::ValueDelimiterNotSet),
            "hidepossiblevalues" => Ok(ArgSettings::HidePossibleValues),
            "allowhyphenvalues" => Ok(ArgSettings::AllowHyphenValues),
            "requireequals" => Ok(ArgSettings::RequireEquals),
            "last" => Ok(ArgSettings::Last),
            "hidedefaultvalue" => Ok(ArgSettings::HideDefaultValue),
            "ignorecase" => Ok(ArgSettings::IgnoreCase),
            "hideenvvalues" => Ok(ArgSettings::HideEnvValues),
            _ => Err("unknown ArgSetting, cannot convert from str".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ArgSettings;

    #[test]
    fn arg_settings_fromstr() {
        assert_eq!(
            "allowhyphenvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowHyphenValues
        );
        assert_eq!(
            "allowemptyvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::AllowEmptyValues
        );
        assert_eq!(
            "global".parse::<ArgSettings>().unwrap(),
            ArgSettings::Global
        );
        assert_eq!(
            "hidepossiblevalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::HidePossibleValues
        );
        assert_eq!(
            "hidden".parse::<ArgSettings>().unwrap(),
            ArgSettings::Hidden
        );
        assert_eq!(
            "nextlinehelp".parse::<ArgSettings>().unwrap(),
            ArgSettings::NextLineHelp
        );
        assert_eq!(
            "requiredunlessall".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequiredUnlessAll
        );
        assert_eq!(
            "requiredelimiter".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequireDelimiter
        );
        assert_eq!(
            "required".parse::<ArgSettings>().unwrap(),
            ArgSettings::Required
        );
        assert_eq!(
            "takesvalue".parse::<ArgSettings>().unwrap(),
            ArgSettings::TakesValue
        );
        assert_eq!(
            "usevaluedelimiter".parse::<ArgSettings>().unwrap(),
            ArgSettings::UseValueDelimiter
        );
        assert_eq!(
            "valuedelimiternotset".parse::<ArgSettings>().unwrap(),
            ArgSettings::ValueDelimiterNotSet
        );
        assert_eq!(
            "requireequals".parse::<ArgSettings>().unwrap(),
            ArgSettings::RequireEquals
        );
        assert_eq!("last".parse::<ArgSettings>().unwrap(), ArgSettings::Last);
        assert_eq!(
            "hidedefaultvalue".parse::<ArgSettings>().unwrap(),
            ArgSettings::HideDefaultValue
        );
        assert_eq!(
            "ignorecase".parse::<ArgSettings>().unwrap(),
            ArgSettings::IgnoreCase
        );
        assert_eq!(
            "hideenvvalues".parse::<ArgSettings>().unwrap(),
            ArgSettings::HideEnvValues
        );
        assert!("hahahaha".parse::<ArgSettings>().is_err());
    }
}
