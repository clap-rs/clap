#[allow(dead_code)]
mod settings;
#[macro_use]
mod macros;

pub use self::settings::AppSettings;

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::env;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::process;
use std::ffi::OsStr;
use std::borrow::Borrow;
use std::fmt::Display;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;
use vec_map::VecMap;

use args::{Arg, AnyArg, ArgGroup, ArgMatches, ArgMatcher, SubCommand};
use args::{FlagBuilder, OptBuilder, PosBuilder};
use args::settings::{ArgFlags, ArgSettings};
use fmt::Format;
use self::settings::AppFlags;
use suggestions;
use errors::{ClapError, ClapErrorType, ClapResult, error_builder};

const INTERNAL_ERROR_MSG: &'static str = "Fatal internal error. Please consider filing a bug \
                                          report at https://github.com/kbknapp/clap-rs/issues";

/// Used to create a representation of a command line program and all possible command line
/// arguments.
///
/// Application settings are set using the "builder pattern" with `.get_matches()` being the
/// terminal method that starts the runtime-parsing process and returns information about
/// the user supplied arguments (or lack there of).
///
/// There aren't any mandatory "options" that one must set. The "options" may also appear in any
/// order (so long as `.get_matches()` is the last method called).
///
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg};
/// let matches = App::new("myprog")
///                   .author("Me, me@mail.com")
///                   .version("1.0.2")
///                   .about("Explains in brief what the program does")
///                   .arg(
///                            Arg::with_name("in_file").index(1)
///                    )
///                   .after_help("Longer explaination to appear after the options when \
///                                displaying the help information from --help or -h")
///                   .get_matches();
///
/// // Your program logic starts here...
/// ```
#[allow(missing_debug_implementations)]
pub struct App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    // The name displayed to the user when showing version and help/usage information
    name: String,
    name_slice: &'ar str,
    // A string of author(s) if desired. Displayed when showing help/usage information
    author: Option<&'a str>,
    // The version displayed to the user
    version: Option<&'v str>,
    // A brief explanation of the program that gets displayed to the user when shown
    // help/usage
    // information
    about: Option<&'ab str>,
    // Additional help information
    more_help: Option<&'h str>,
    // A list of possible flags
    flags: Vec<FlagBuilder<'ar>>,
    // A list of possible options
    opts: Vec<OptBuilder<'ar>>,
    // A list of positional arguments
    positionals: VecMap<PosBuilder<'ar>>,
    // A list of subcommands
    subcommands: Vec<App<'a, 'v, 'ab, 'u, 'h, 'ar>>,
    help_short: Option<char>,
    version_short: Option<char>,
    required: Vec<&'ar str>,
    short_list: Vec<char>,
    long_list: Vec<&'ar str>,
    blacklist: Vec<&'ar str>,
    usage_str: Option<&'u str>,
    bin_name: Option<String>,
    usage: Option<String>,
    groups: HashMap<&'ar str, ArgGroup<'ar, 'ar>>,
    global_args: Vec<Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>>,
    help_str: Option<&'u str>,
    settings: AppFlags,
    overrides: Vec<&'ar str>,
}

impl<'a, 'v, 'ab, 'u, 'h, 'ar> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    /// Creates a new instance of an application requiring a name (such as the binary). The name
    /// will be displayed to the user when they request to print version or help and usage
    /// information. The name should not contain spaces (hyphens '-' are ok).
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let prog = App::new("myprog")
    /// # .get_matches();
    /// ```
    pub fn new(n: &'ar str) -> Self {
        App {
            name: n.to_owned(),
            name_slice: n,
            author: None,
            about: None,
            more_help: None,
            version: None,
            flags: vec![],
            opts: vec![],
            positionals: VecMap::new(),
            subcommands: vec![],
            help_short: None,
            version_short: None,
            required: vec![],
            short_list: vec![],
            long_list: vec![],
            usage_str: None,
            usage: None,
            blacklist: vec![],
            bin_name: None,
            groups: HashMap::new(),
            global_args: vec![],
            help_str: None,
            overrides: vec![],
            settings: AppFlags::new(),
        }
    }

    /// Creates a new instace of `App` from a .yml (YAML) file. The YAML file must be properly
    /// formatted or this function will panic!(). A full example of supported YAML objects can be
    /// found in `examples/17_yaml.rs` and `examples/17_yaml.yml`.
    ///
    /// In order to use this function you must compile with the `features = ["yaml"]` in your
    /// settings for `[dependencies.clap]` table of your `Cargo.toml`
    ///
    /// Note, due to how the YAML objects are built there is a convienience macro for loading the
    /// YAML file (relative to the current file, like modules work). That YAML object can then be
    /// passed to this function.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::App;
    /// let yml = load_yaml!("app.yml");
    /// let app = App::from_yaml(yml);
    /// ```
    #[cfg(feature = "yaml")]
    pub fn from_yaml<'y>(mut yaml: &'y Yaml) -> App<'y, 'y, 'y, 'y, 'y, 'y> {
        // We WANT this to panic on error...so expect() is good.
        let mut is_sc = None;
        let mut a = if let Some(name) = yaml["name"].as_str() {
            App::new(name)
        } else {
            let yaml_hash = yaml.as_hash().unwrap();
            let sc_key = yaml_hash.keys().nth(0).unwrap();
            is_sc = Some(yaml_hash.get(sc_key).unwrap());
            App::new(sc_key.as_str().unwrap())
        };
        yaml = if let Some(sc) = is_sc {
            sc
        } else {
            yaml
        };
        if let Some(v) = yaml["version"].as_str() {
            a = a.version(v);
        }
        if let Some(v) = yaml["author"].as_str() {
            a = a.author(v);
        }
        if let Some(v) = yaml["bin_name"].as_str() {
            a = a.bin_name(v);
        }
        if let Some(v) = yaml["about"].as_str() {
            a = a.about(v);
        }
        if let Some(v) = yaml["after_help"].as_str() {
            a = a.after_help(v);
        }
        if let Some(v) = yaml["usage"].as_str() {
            a = a.usage(v);
        }
        if let Some(v) = yaml["help"].as_str() {
            a = a.help(v);
        }
        if let Some(v) = yaml["help_short"].as_str() {
            a = a.help_short(v);
        }
        if let Some(v) = yaml["version_short"].as_str() {
            a = a.version_short(v);
        }
        if let Some(v) = yaml["settings"].as_vec() {
            for ys in v {
                if let Some(s) = ys.as_str() {
                    a = a.setting(s.parse().ok().expect("unknown AppSetting found in YAML file"));
                }
            }
        }
        if let Some(v) = yaml["args"].as_vec() {
            for arg_yaml in v {
                a = a.arg(Arg::from_yaml(&arg_yaml.as_hash().unwrap()));
            }
        }
        if let Some(v) = yaml["subcommands"].as_vec() {
            for sc_yaml in v {
                a = a.subcommand(SubCommand::from_yaml(&sc_yaml));
            }
        }
        if let Some(v) = yaml["arg_groups"].as_vec() {
            for ag_yaml in v {
                a = a.arg_group(ArgGroup::from_yaml(&ag_yaml.as_hash().unwrap()));
            }
        }

        a
    }

    /// Sets a string of author(s) and will be displayed to the user when they request the help
    /// information with `--help` or `-h`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///      .author("Me, me@mymain.com")
    /// # ;
    /// ```
    pub fn author(mut self, a: &'a str) -> Self {
        self.author = Some(a);
        self
    }

    /// Overrides the system-determined binary name. This should only be used when absolutely
    /// neccessary, such as the binary name for your application is misleading, or perhaps *not*
    /// how the user should invoke your program.
    ///
    /// **NOTE:** This command **should not** be used for SubCommands.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///      .bin_name("my_binary")
    /// # ;
    /// ```
    pub fn bin_name(mut self, a: &str) -> Self {
        self.bin_name = Some(a.to_owned());
        self
    }

    /// Sets a string briefly describing what the program does and will be displayed when
    /// displaying help information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .about("Does really amazing things to great people")
    /// # ;
    /// ```
    pub fn about(mut self, a: &'ab str) -> Self {
        self.about = Some(a);
        self
    }

    /// Adds additional help information to be displayed in addition to and directly after
    /// auto-generated help. This information is displayed **after** the auto-generated help
    /// information. This additional help is often used to describe how to use the arguments,
    /// or caveats to be noted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .after_help("Does really amazing things to great people")
    /// # ;
    /// ```
    pub fn after_help(mut self, h: &'h str) -> Self {
        self.more_help = Some(h);
        self
    }

    /// Allows subcommands to override all requirements of the parent (this command). For example
    /// if you had a subcommand or even top level application which had a required arguments that
    /// are only required as long as there is no subcommand present.
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::SubcommandsNegateReqs` instead.
    /// This method will be removed at 2.x
    ///
    /// **NOTE:** This defaults to false (using subcommand does *not* negate requirements)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .subcommands_negate_reqs(true)
    /// # ;
    /// ```
    pub fn subcommands_negate_reqs(mut self, n: bool) -> Self {
        if n {
            self.settings.set(&AppSettings::SubcommandsNegateReqs);
        } else {
            self.settings.unset(&AppSettings::SubcommandsNegateReqs);
        }
        self
    }

    /// Allows specifying that if no subcommand is present at runtime, error and exit gracefully
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::SubcommandRequired` instead. This
    /// method will be removed at 2.x
    ///
    /// **NOTE:** This defaults to false (subcommands do *not* need to be present)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .subcommand_required(true)
    /// # ;
    /// ```
    pub fn subcommand_required(mut self, n: bool) -> Self {
        if n {
            self.settings.set(&AppSettings::SubcommandRequired);
        } else {
            self.settings.unset(&AppSettings::SubcommandRequired);
        }
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .version("v0.1.24")
    /// # ;
    /// ```
    pub fn version(mut self, v: &'v str) -> Self {
        self.version = Some(v);
        self
    }

    /// Sets a custom usage string to override the auto-generated usage string.
    ///
    /// This will be displayed to the user when errors are found in argument parsing, or when you
    /// call `ArgMatcher::usage()`
    ///
    /// **NOTE:** You do not need to specify the "USAGE: \n\t" portion, as that will
    /// still be applied by `clap`, you only need to specify the portion starting
    /// with the binary name.
    ///
    /// **NOTE:** This will not replace the entire help message, *only* the portion
    /// showing the usage.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .usage("myapp [-clDas] <some_file>")
    /// # ;
    /// ```
    pub fn usage(mut self, u: &'u str) -> Self {
        self.usage_str = Some(u);
        self
    }

    /// Sets a custom help message and overrides the auto-generated one. This should only be used
    /// when the auto-generated message does not suffice.
    ///
    /// This will be displayed to the user when they use the default `--help` or `-h`
    ///
    /// **NOTE:** This replaces the **entire** help message, so nothing will be auto-generated.
    ///
    /// **NOTE:** This **only** replaces the help message for the current command, meaning if you
    /// are using subcommands, those help messages will still be auto-generated unless you
    /// specify a `.help()` for them as well.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myapp")
    ///     .help("myapp v1.0\n\
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
    pub fn help(mut self, h: &'u str) -> Self {
        self.help_str = Some(h);
        self
    }

    /// Sets the short version of the `help` argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `h`, but this can be overridden
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     // Using an uppercase `H` instead of the default lowercase `h`
    ///     .help_short("H")
    /// # ;
    pub fn help_short(mut self, s: &str) -> Self {
        self.help_short = s.trim_left_matches(|c| c == '-')
                           .chars()
                           .nth(0);
        self
    }

    /// Sets the short version of the `version` argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `V`, but this can be overridden
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     // Using a lowercase `v` instead of the default capital `V`
    ///     .version_short("v")
    /// # ;
    pub fn version_short(mut self, s: &str) -> Self {
        self.version_short = s.trim_left_matches(|c| c == '-')
                              .chars()
                              .nth(0);
        self
    }

    /// Specifies that the help text sould be displayed (and then exit gracefully), if no
    /// arguments are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::ArgRequiredElseHelp` instead. This
    /// method will be removed at 2.x
    ///
    /// **NOTE:** Subcommands count as arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .arg_required_else_help(true)
    /// # ;
    /// ```
    pub fn arg_required_else_help(mut self, tf: bool) -> Self {
        if tf {
            self.settings.set(&AppSettings::ArgRequiredElseHelp);
        } else {
            self.settings.unset(&AppSettings::ArgRequiredElseHelp);
        }
        self
    }

    /// Hides a subcommand from help message output.
    ///
    /// **NOTE:** This does **not** hide the subcommand from usage strings on error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, SubCommand};
    /// # let matches = App::new("myprog")
    /// #                 .subcommand(
    /// # SubCommand::with_name("debug")
    /// .hidden(true)
    /// # ).get_matches();
    pub fn hidden(mut self, h: bool) -> Self {
        if h {
            self.settings.set(&AppSettings::Hidden);
        } else {
            self.settings.unset(&AppSettings::Hidden);
        }
        self
    }

    /// Uses version of the current command for all subcommands. (Defaults to false; subcommands
    /// have independant version strings)
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::GlobalVersion` instead. This
    /// method will be removed at 2.x
    ///
    /// **NOTE:** The version for the current command and this setting must be set **prior** to
    /// adding any subcommands
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .global_version(true)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches();
    /// // running `myprog test --version` will display
    /// // "myprog-test v1.1"
    /// ```
    pub fn global_version(mut self, gv: bool) -> Self {
        if gv {
            self.settings.set(&AppSettings::GlobalVersion);
        } else {
            self.settings.unset(&AppSettings::GlobalVersion);
        }
        self
    }

    /// Disables `-V` and `--version` for all subcommands (Defaults to false; subcommands have
    /// version flags)
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::VersionlessSubcommands` instead.
    /// This method will be removed at 2.x
    ///
    /// **NOTE:** This setting must be set **prior** adding any subcommands
    ///
    /// **NOTE:** Do not set this value to false, it will have undesired results!
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .version("v1.1")
    ///     .versionless_subcommands(true)
    ///     .subcommand(SubCommand::with_name("test"))
    ///     .get_matches();
    /// // running `myprog test --version` will display unknown argument error
    /// ```
    pub fn versionless_subcommands(mut self, vers: bool) -> Self {
        if vers {
            self.settings.set(&AppSettings::VersionlessSubcommands);
        } else {
            self.settings.unset(&AppSettings::VersionlessSubcommands);
        }
        self
    }

    /// By default the auto-generated help message groups flags, options, and positional arguments
    /// separately. This setting disable that and groups flags and options together presenting a
    /// more unified help message (a la getopts or docopt style).
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::UnifiedHelpMessage` instead. This
    /// method will be removed at 2.x
    ///
    /// **NOTE:** This setting is cosmetic only and does not affect any functionality.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .unified_help_message(true)
    ///     .get_matches();
    /// // running `myprog --help` will display a unified "docopt" or "getopts" style help message
    /// ```
    pub fn unified_help_message(mut self, uni_help: bool) -> Self {
        if uni_help {
            self.settings.set(&AppSettings::UnifiedHelpMessage);
        } else {
            self.settings.unset(&AppSettings::UnifiedHelpMessage);
        }
        self
    }

    /// Will display a message "Press [ENTER]/[RETURN] to continue..." and wait user before
    /// exiting
    ///
    /// This is most useful when writing an application which is run from a GUI shortcut, or on
    /// Windows where a user tries to open the binary by double-clicking instead of using the
    /// command line (i.e. set `.arg_required_else_help(true)` and `.wait_on_error(true)` to
    /// display the help in such a case).
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::WaitOnError` instead. This
    /// method will be removed at 2.x
    ///
    /// **NOTE:** This setting is **not** recursive with subcommands, meaning if you wish this
    /// behavior for all subcommands, you must set this on each command (needing this is extremely
    /// rare)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .arg_required_else_help(true)
    /// # ;
    /// ```
    pub fn wait_on_error(mut self, w: bool) -> Self {
        if w {
            self.settings.set(&AppSettings::WaitOnError);
        } else {
            self.settings.unset(&AppSettings::WaitOnError);
        }
        self
    }

    /// Specifies that the help text sould be displayed (and then exit gracefully), if no
    /// subcommands are present at runtime (i.e. an empty run such as, `$ myprog`.
    ///
    /// **Deprecated:** Use `App::setting()` with `AppSettings::SubcommandRequiredElseHelp`
    /// instead. This method will be removed at 2.x
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
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .subcommand_required_else_help(true)
    /// # ;
    /// ```
    pub fn subcommand_required_else_help(mut self, tf: bool) -> Self {
        if tf {
            self.settings.set(&AppSettings::SubcommandRequiredElseHelp);
        } else {
            self.settings.unset(&AppSettings::SubcommandRequiredElseHelp);
        }
        self
    }

    /// Enables Application level settings, passed as argument
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .setting(AppSettings::WaitOnError)
    /// # ;
    /// ```
    pub fn setting(mut self, setting: AppSettings) -> Self {
        self.settings.set(&setting);
        self
    }

    /// Enables multiple Application level settings, passed as argument
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .settings( &[AppSettings::SubcommandRequired,
    ///                  AppSettings::WaitOnError])
    /// # ;
    /// ```
    pub fn settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.settings.set(s);
        }
        self
    }

    /// Adds an argument to the list of valid possibilties manually. This method allows you full
    /// control over the arguments settings and options (as well as dynamic generation). It also
    /// allows you specify several more advanced configuration options such as relational rules
    /// (exclusions and requirements).
    ///
    /// The only disadvantage to this method is that it's more verbose, and arguments must be added
    /// one at a time. Using `Arg::from_usage` helps with the verbosity, and still allows full
    /// control over the advanced configuration options.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     // Adding a single "flag" argument with a short and help text, using Arg::with_name()
    ///     .arg(
    ///         Arg::with_name("debug")
    ///            .short("d")
    ///            .help("turns on debugging mode")
    ///     )
    ///     // Adding a single "option" argument with a short, a long, and help text using the less
    ///     // verbose Arg::from_usage()
    ///     .arg(
    ///         Arg::from_usage("-c --config=[CONFIG] 'Optionally sets a config file to use'")
    ///     )
    /// # ;
    /// ```
    pub fn arg(mut self, a: Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>) -> Self {
        self.add_arg(a);
        self
    }

    // actually adds the arguments
    fn add_arg(&mut self, a: Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>) {
        if self.flags.iter().any(|f| &f.name == &a.name) ||
           self.opts.iter().any(|o| o.name == a.name) ||
           self.positionals.values().any(|p| p.name == a.name) {
            panic!("Non-unique argument name: {} is already in use", a.name);
        }
        if let Some(grp) = a.group {
            let ag = self.groups.entry(grp).or_insert(ArgGroup::with_name(grp));
            ag.args.push(a.name);
        }
        if let Some(s) = a.short {
            if self.short_list.contains(&s) {
                panic!("Argument short must be unique\n\n\t-{} is already in use",
                       s);
            } else {
                self.short_list.push(s);
            }
        }
        if let Some(l) = a.long {
            if self.long_list.contains(&l) {
                panic!("Argument long must be unique\n\n\t--{} is already in use",
                       l);
            } else {
                self.long_list.push(l);
            }
            if l == "help" {
                self.settings.set(&AppSettings::NeedsLongHelp);
            } else if l == "version" {
                self.settings.set(&AppSettings::NeedsLongVersion);
            }
        }
        if a.required {
            self.required.push(a.name);
        }
        if a.index.is_some() || (a.short.is_none() && a.long.is_none()) {
            let i = if a.index.is_none() {
                (self.positionals.len() + 1)
            } else {
                a.index.unwrap() as usize
            };
            if self.positionals.contains_key(&i) {
                panic!("Argument \"{}\" has the same index as another positional \
                    argument\n\n\tPerhaps try .multiple(true) to allow one positional argument \
                    to take multiple values",
                       a.name);
            }
            let pb = PosBuilder::from_arg(&a, i as u8, &mut self.required);
            // self.positionals_name.insert(pb.name, i);
            self.positionals.insert(i, pb);
        } else if a.takes_value {
            let ob = OptBuilder::from_arg(&a, &mut self.required);
            self.opts.push(ob);
        } else {
            let fb = FlagBuilder::from(&a);
            self.flags.push(fb);
        }
        if a.global {
            if a.required {
                panic!("Global arguments cannot be required.\n\n\t'{}' is marked as global and \
                        required",
                       a.name);
            }
            self.global_args.push(a);
        }
    }

    /// Adds multiple arguments to the list of valid possibilties by iterating over a Vec of Args
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args(
    ///         vec![Arg::from_usage("[debug] -d 'turns on debugging info"),
    ///              Arg::with_name("input").index(1).help("the input file to use")]
    ///     )
    /// # ;
    /// ```
    pub fn args(mut self, args: Vec<Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>>) -> Self {
        for arg in args.into_iter() {
            self = self.arg(arg);
        }
        self
    }

    /// A convienience method for adding a single basic argument (one without advanced
    /// relational rules) from a usage type string. The string used follows the same rules and
    /// syntax as `Arg::from_usage()`
    ///
    /// The downside to using this method is that you can not set any additional properties of the
    /// `Arg` other than what `Arg::from_usage()` supports.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .arg_from_usage("-c --conf=<config> 'Sets a configuration file to use'")
    /// # ;
    /// ```
    pub fn arg_from_usage(mut self, usage: &'ar str) -> Self {
        self = self.arg(Arg::from_usage(usage));
        self
    }

    /// Adds multiple arguments at once from a usage string, one per line. See `Arg::from_usage()`
    /// for details on the syntax and rules supported.
    ///
    /// Like `App::arg_from_usage()` the downside is you only set properties for the `Arg`s which
    /// `Arg::from_usage()` supports. But here the benefit is pretty strong, as the readability is
    /// greatly enhanced, especially if you don't need any of the more advanced configuration
    /// options.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args_from_usage(
    ///         "-c --conf=[config] 'Sets a configuration file to use'
    ///          [debug]... -d 'Sets the debugging level'
    ///          <input> 'The input file to use'"
    ///     )
    /// # ;
    /// ```
    pub fn args_from_usage(mut self, usage: &'ar str) -> Self {
        for l in usage.lines() {
            self = self.arg(Arg::from_usage(l.trim()));
        }
        self
    }

    /// Adds an ArgGroup to the application. ArgGroups are a family of related arguments. By
    /// placing them in a logical group, you make easier requirement and exclusion rules. For
    /// instance, you can make an ArgGroup required, this means that one (and *only* one) argument
    /// from that group must be present. Using more than one argument from an ArgGroup causes a
    /// failure (graceful exit).
    ///
    /// You can also do things such as name an ArgGroup as a confliction, meaning any of the
    /// arguments that belong to that group will cause a failure if present.
    ///
    /// Perhaps the most common use of ArgGroups is to require one and *only* one argument to be
    /// present out of a given set. For example, lets say that you were building an application
    /// where one could set a given version number by supplying a string using an option argument,
    /// such as `--set-ver v1.2.3`, you also wanted to support automatically using a previous
    /// version numer and simply incrementing one of the three numbers, so you create three flags
    /// `--major`, `--minor`, and `--patch`. All of these arguments shouldn't be used at one time
    /// but perhaps you want to specify that *at least one* of them is used. You can create a
    /// group
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # App::new("app")
    /// .args_from_usage("--set-ver [ver] 'set the version manually'
    ///                   --major         'auto increase major'
    ///                   --minor         'auto increase minor'
    ///                   --patch         'auto increase patch")
    /// .arg_group(ArgGroup::with_name("vers")
    ///                     .add_all(&["ver", "major", "minor","patch"])
    ///                     .required(true))
    /// # ;
    pub fn arg_group(mut self, group: ArgGroup<'ar, 'ar>) -> Self {
        if group.required {
            self.required.push(group.name);
            if let Some(ref reqs) = group.requires {
                for r in reqs {
                    self.required.push(r);
                }
            }
            if let Some(ref bl) = group.conflicts {
                for b in bl {
                    self.blacklist.push(b);
                }
            }
        }
        let mut found = false;
        if let Some(ref mut grp) = self.groups.get_mut(group.name) {
            for a in &group.args {
                grp.args.push(a);
            }
            grp.requires = group.requires.clone();
            grp.conflicts = group.conflicts.clone();
            grp.required = group.required;
            found = true;
        }
        if !found {
            self.groups.insert(group.name, group);
        }
        self
    }

    /// Adds a ArgGroups to the application. ArgGroups are a family of related arguments. By
    /// placing them in a logical group, you make easier requirement and exclusion rules. For
    /// instance, you can make an ArgGroup required, this means that one (and *only* one) argument
    /// from that group must be present. Using more than one argument from an ArgGroup causes a
    /// failure (graceful exit).
    ///
    /// You can also do things such as name an ArgGroup as a confliction, meaning any of the
    /// arguments that belong to that group will cause a failure if present.
    ///
    /// Perhaps the most common use of ArgGroups is to require one and *only* one argument to be
    /// present out of a given set. For example, lets say that you were building an application
    /// where one could set a given version number by supplying a string using an option argument,
    /// such as `--set-ver v1.2.3`, you also wanted to support automatically using a previous
    /// version numer and simply incrementing one of the three numbers, so you create three flags
    /// `--major`, `--minor`, and `--patch`. All of these arguments shouldn't be used at one time
    /// but perhaps you want to specify that *at least one* of them is used. You can create a
    /// group
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// # App::new("app")
    /// .args_from_usage("--set-ver [ver] 'set the version manually'
    ///                   --major         'auto increase major'
    ///                   --minor         'auto increase minor'
    ///                   --patch         'auto increase patch")
    /// .arg_group(ArgGroup::with_name("vers")
    ///                     .add_all(&["ver", "major", "minor","patch"])
    ///                     .required(true))
    /// # ;
    pub fn arg_groups(mut self, groups: Vec<ArgGroup<'ar, 'ar>>) -> Self {
        for g in groups {
            self = self.arg_group(g);
        }
        self
    }

    /// Adds a subcommand to the list of valid possibilties. Subcommands are effectively sub apps,
    /// because they can contain their own arguments, subcommands, version, usage, etc. They also
    /// function just like apps, in that they get their own auto generated help, version, and
    /// usage.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # App::new("myprog")
    /// .subcommand(SubCommand::with_name("config")
    ///                .about("Controls configuration features")
    ///                .arg_from_usage("<config> 'Required configuration file to use'"))
    ///             // Additional subcommand configuration goes here, such as other arguments...
    /// # ;
    /// ```
    pub fn subcommand(mut self, mut subcmd: App<'a, 'v, 'ab, 'u, 'h, 'ar>) -> Self {
        if subcmd.name == "help" {
            self.settings.set(&AppSettings::NeedsSubcommandHelp);
        }
        if self.settings.is_set(&AppSettings::VersionlessSubcommands) {
            subcmd.settings.set(&AppSettings::DisableVersion);
        }
        if self.settings.is_set(&AppSettings::GlobalVersion) && subcmd.version.is_none() &&
           self.version.is_some() {
            subcmd.version = Some(self.version.unwrap());
        }
        self.subcommands.push(subcmd);
        self
    }

    /// Adds multiple subcommands to the list of valid possibilties by iterating over a Vec of
    /// `SubCommand`s
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # App::new("myprog")
    /// .subcommands( vec![
    ///        SubCommand::with_name("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::with_name("config_file").index(1)),
    ///        SubCommand::with_name("debug").about("Controls debug functionality")])
    /// # ;
    /// ```
    pub fn subcommands(mut self, subcmds: Vec<App<'a, 'v, 'ab, 'u, 'h, 'ar>>) -> Self {
        for subcmd in subcmds.into_iter() {
            self = self.subcommand(subcmd);
        }
        self
    }

    // Creates a usage string if one was not provided by the user manually. This
    // happens just
    // after all arguments were parsed, but before any subcommands have been parsed
    // (so as to
    // give subcommands their own usage recursively)
    fn create_usage(&self, matcher: &[&'ar str]) -> ClapResult<String> {
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n\t");
        if let Some(u) = self.usage_str {
            usage.push_str(u);
        } else if !matcher.is_empty() {
            try!(self.usage_from_matcher(&mut usage, matcher));
        } else {
            usage.push_str(&*self.usage
                                 .as_ref()
                                 .unwrap_or(self.bin_name
                                                .as_ref()
                                                .unwrap_or(&self.name)));

            let reqs = self.required.iter().map(|n| *n).collect::<Vec<_>>();
            let req_strings = self.get_required_from(&reqs, None);
            let req_string = req_strings.iter()
                                        .fold(String::new(), |a, s| a + &format!(" {}", s)[..]);

            if !self.flags.is_empty() && !self.settings.is_set(&AppSettings::UnifiedHelpMessage) {
                usage.push_str(" [FLAGS]");
            } else {
                usage.push_str(" [OPTIONS]");
            }
            if !self.settings.is_set(&AppSettings::UnifiedHelpMessage) && !self.opts.is_empty() &&
               self.opts.iter().any(|a| !a.settings.is_set(&ArgSettings::Required)) {
                usage.push_str(" [OPTIONS]");
            }

            usage.push_str(&req_string[..]);

            // places a '--' in the usage string if there are args and options
            // supporting multiple values
            if !self.positionals.is_empty() &&
               (self.opts.iter().any(|a| a.settings.is_set(&ArgSettings::Multiple)) ||
                self.positionals.values().any(|a| a.settings.is_set(&ArgSettings::Multiple))) &&
               !self.opts.iter().any(|a| a.settings.is_set(&ArgSettings::Required)) &&
               self.subcommands.is_empty() {
                usage.push_str(" [--]")
            }
            if !self.positionals.is_empty() &&
               self.positionals
                   .values()
                   .any(|a| !a.settings.is_set(&ArgSettings::Required)) {
                usage.push_str(" [ARGS]");
            }


            if !self.subcommands.is_empty() &&
               !self.settings.is_set(&AppSettings::SubcommandRequired) {
                usage.push_str(" [SUBCOMMAND]");
            } else if self.settings.is_set(&AppSettings::SubcommandRequired) &&
               !self.subcommands.is_empty() {
                usage.push_str(" <SUBCOMMAND>");
            }
        }

        usage.shrink_to_fit();
        Ok(usage)
    }

    // Creates a context aware usage string, or "smart usage" from currently used
    // args, and
    // requirements
    fn usage_from_matcher(&self, usage: &mut String, matcher: &[&'ar str]) -> ClapResult<()> {
        use std::fmt::Write;
        let mut hs: Vec<&str> = self.required.iter().map(|n| *n).collect();
        for n in matcher {
            hs.push(*n);
        }
        let reqs = self.get_required_from(&hs, None);

        let r_string = reqs.iter().fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

        try!(write!(usage,
                    "{}{}",
                    self.usage
                        .as_ref()
                        .unwrap_or(self.bin_name.as_ref().unwrap_or(&self.name)),
                    r_string));
        if self.settings.is_set(&AppSettings::SubcommandRequired) {
            try!(write!(usage, " <SUBCOMMAND>"));
        }

        Ok(())
    }

    /// Prints the full help message to `io::stdout()` using a `BufWriter`
    ///
    /// # Examples
    /// ```no_run
    /// # use clap::App;
    /// # use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_help(&mut out).ok().expect("failed to write to stdout");
    /// ```
    pub fn print_help(&self) -> ClapResult<()> {
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        self.write_help(&mut buf_w)
    }

    /// Writes the full help message to the user to a `io::Write` object
    ///
    /// ```no_run
    /// # use clap::App;
    /// # use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_help(&mut out).ok().expect("failed to write to stdout");
    /// ```
    pub fn write_help<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        if let Some(h) = self.help_str {
            return writeln!(w, "{}", h).map_err(ClapError::from);
        }

        // Print the version
        try!(write!(w,
                    "{} {}\n",
                    &self.bin_name.as_ref().unwrap_or(&self.name)[..].replace(" ", "-"),
                    self.version.unwrap_or("")));
        let flags = !self.flags.is_empty();
        let pos = !self.positionals.is_empty();
        let opts = !self.opts.is_empty();
        let subcmds = !self.subcommands.is_empty();
        let unified_help = self.settings.is_set(&AppSettings::UnifiedHelpMessage);

        let mut longest_flag = 0;
        for fl in self.flags
                      .iter()
                      .filter(|f| f.long.is_some() && !f.settings.is_set(&ArgSettings::Hidden))
                      .map(|a| a.to_string().len()) {
            if fl > longest_flag {
                longest_flag = fl;
            }
        }
        let mut longest_opt = 0;
        for ol in self.opts
                      .iter()
                      .filter(|o| !o.settings.is_set(&ArgSettings::Hidden))
                      .map(|a| a.to_string().len()) {
            if ol > longest_opt {
                longest_opt = ol;
            }
        }
        let mut longest_pos = 0;
        for pl in self.positionals
                      .values()
                      .filter(|p| !p.settings.is_set(&ArgSettings::Hidden))
                      .map(|f| f.to_string().len()) {
            if pl > longest_pos {
                longest_pos = pl;
            }
        }
        let mut longest_sc = 0;
        for scl in self.subcommands
                       .iter()
                       .filter(|s| !s.settings.is_set(&AppSettings::Hidden))
                       .map(|f| f.name.len()) {
            if scl > longest_sc {
                longest_sc = scl;
            }
        }

        if let Some(author) = self.author {
            try!(write!(w, "{}\n", author));
        }
        if let Some(about) = self.about {
            try!(write!(w, "{}\n", about));
        }

        try!(write!(w, "\n{}", try!(self.create_usage(&[]))));

        if flags || opts || pos || subcmds {
            try!(write!(w, "\n"));
        }

        let tab = "    ";
        let longest = if !unified_help || longest_opt == 0 {
            longest_flag
        } else {
            longest_opt
        };
        if unified_help && (flags || opts) {
            try!(write!(w, "\nOPTIONS:\n"));
            let mut combined = BTreeMap::new();
            for f in self.flags.iter().filter(|f| !f.settings.is_set(&ArgSettings::Hidden)) {
                let mut v = vec![];
                try!(f.write_help(&mut v, tab, longest));
                combined.insert(f.name, v);
            }
            for o in self.opts.iter().filter(|o| !o.settings.is_set(&ArgSettings::Hidden)) {
                let mut v = vec![];
                try!(o.write_help(&mut v, tab, longest));
                combined.insert(o.name, v);
            }
            for (_, a) in combined {
                try!(write!(w, "{}", String::from_utf8_lossy(&*a)));
            }
        } else {
            if flags {
                try!(write!(w, "\nFLAGS:\n"));
                for (_, f) in self.flags
                                  .iter()
                                  .filter(|f| !f.settings.is_set(&ArgSettings::Hidden))
                                  .map(|f| (f.name, f))
                                  .collect::<BTreeMap<_, _>>() {
                    try!(f.write_help(w, tab, longest));
                }
            }
            if opts {
                try!(write!(w, "\nOPTIONS:\n"));
                for (_, o) in self.opts
                                  .iter()
                                  .filter(|o| !o.settings.is_set(&ArgSettings::Hidden))
                                  .map(|o| (o.name, o))
                                  .collect::<BTreeMap<_, _>>() {
                    try!(o.write_help(w, tab, longest_opt));
                }
            }
        }
        if pos {
            try!(write!(w, "\nARGS:\n"));
            for v in self.positionals
                         .values()
                         .filter(|p| !p.settings.is_set(&ArgSettings::Hidden)) {
                try!(v.write_help(w, tab, longest_pos));
            }
        }
        if subcmds {
            try!(write!(w, "\nSUBCOMMANDS:\n"));
            for (name, sc) in self.subcommands
                                  .iter()
                                  .filter(|s| !s.settings.is_set(&AppSettings::Hidden))
                                  .map(|s| (s.name_slice, s))
                                  .collect::<BTreeMap<_, _>>() {
                try!(write!(w, "{}{}", tab, name));
                write_spaces!((longest_sc + 4) - (name.len()), w);
                if let Some(a) = sc.about {
                    if a.contains("{n}") {
                        let mut ab = a.split("{n}");
                        while let Some(part) = ab.next() {
                            try!(write!(w, "{}\n", part));
                            write_spaces!(longest_sc + 8, w);
                            try!(write!(w, "{}", ab.next().unwrap_or("")));
                        }
                    } else {
                        try!(write!(w, "{}", a));
                    }
                }
                try!(write!(w, "\n"));
            }
        }

        if let Some(h) = self.more_help {
            try!(write!(w, "\n{}", h));
        }

        // flush the buffer
        debugln!("Flushing the buffer...");
        w.flush().map_err(ClapError::from)
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands
    ///
    /// # Panics
    ///
    /// If any arguments contain invalid unicode characters. If this is not desired it is
    /// recommended to use the `*_safe()` or `*_lossy()` versions of this method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches();
    /// ```
    pub fn get_matches(self) -> ArgMatches<'ar, 'ar> {
        // Start the parsing
        self.get_matches_from(env::args())
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands. Invalid unicode characters are replaced with
    /// `U+FFFD REPLACEMENT CHARACTER`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches();
    /// ```
    pub fn get_matches_lossy(self) -> ArgMatches<'ar, 'ar> {
        // Start the parsing
        self.get_matches_from_lossy(env::args_os())
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return an error, where the `error_type` is a `ClapErrorType::HelpDisplayed`
    /// or `ClapErrorType::VersionDisplayed` respectively. You must call `error.exit()` or
    /// perform a `std::process::exit` yourself.
    ///
    /// **NOTE:** This method should only be used when is absolutely necessary to handle errors
    /// manually.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches_safe()
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    pub fn get_matches_safe(self) -> ClapResult<ArgMatches<'ar, 'ar>> {
        // Start the parsing
        self.get_matches_from_safe(env::args_os())
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands. Invalid unicode characters are replaced with
    /// `U+FFFD REPLACEMENT CHARACTER`
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return an error, where the `error_type` is a `ClapErrorType::HelpDisplayed`
    /// or `ClapErrorType::VersionDisplayed` respectively. You must call `error.exit()` or
    /// perform a `std::process::exit` yourself.
    ///
    /// **NOTE:** This method should only be used when is absolutely necessary to handle errors
    /// manually.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches_safe()
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    pub fn get_matches_safe_lossy(self) -> ClapResult<ArgMatches<'ar, 'ar>> {
        // Start the parsing
        self.get_matches_from_safe_lossy(env::args_os())
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands
    ///
    /// **NOTE:** The first argument will be parsed as the binary name.
    ///
    /// **NOTE:** This method should only be used when absolutely necessary, such as needing to
    /// parse arguments from something other than `std::env::args()`. If you are unsure, use
    /// `App::get_matches()`
    ///
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
    pub fn get_matches_from<I, T>(mut self, itr: I) -> ArgMatches<'ar, 'ar>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        self.get_matches_from_safe_borrow(itr).unwrap_or_else(|e| {
            // Otherwise, write to stderr and exit
            self.maybe_wait_for_exit(e);
        })
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands. Invalid unicode characters are replaced with
    /// `U+FFFD REPLACEMENT CHARACTER`
    ///
    /// **NOTE:** The first argument will be parsed as the binary name.
    ///
    /// **NOTE:** This method should only be used when absolutely necessary, such as needing to
    /// parse arguments from something other than `std::env::args()`. If you are unsure, use
    /// `App::get_matches()`
    ///
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
    pub fn get_matches_from_lossy<I, T>(mut self, itr: I) -> ArgMatches<'ar, 'ar>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        self.get_matches_from_safe_borrow_lossy(itr).unwrap_or_else(|e| {
            // Otherwise, write to stderr and exit
            self.maybe_wait_for_exit(e);
        })
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return an error, where the `error_type` is a `ClapErrorType::HelpDisplayed`
    /// or `ClapErrorType::VersionDisplayed` respectively. You must call `error.exit()` or
    /// perform a `std::process::exit` yourself.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name.
    ///
    /// **NOTE:** This method should only be used when absolutely necessary, such as needing to
    /// parse arguments from something other than `std::env::args()`. If you are unsure, use
    /// `App::get_matches_safe()`
    ///
    /// **NOTE:** This method should only be used when is absolutely necessary to handle errors
    /// manually.
    ///
    /// **NOTE:** Invalid unicode characters will result in an `Err` with type
    /// `ClapErrorType::InvalidUnicode`
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches_from_safe(arg_vec)
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    pub fn get_matches_from_safe<I, T>(mut self, itr: I) -> ClapResult<ArgMatches<'ar, 'ar>>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        self.get_matches_from_safe_borrow(itr)
    }

    /// Starts the parsing process. Called on top level parent app **ONLY** then recursively calls
    /// the real parsing function for all subcommands. Invalid unicode characters are replaced with
    /// `U+FFFD REPLACEMENT CHARACTER`
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return an error, where the `error_type` is a `ClapErrorType::HelpDisplayed`
    /// or `ClapErrorType::VersionDisplayed` respectively. You must call `error.exit()` or
    /// perform a `std::process::exit` yourself.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name.
    ///
    /// **NOTE:** This method should only be used when absolutely necessary, such as needing to
    /// parse arguments from something other than `std::env::args()`. If you are unsure, use
    /// `App::get_matches_safe()`
    ///
    /// **NOTE:** This method should only be used when is absolutely necessary to handle errors
    /// manually.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches_from_safe(arg_vec)
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    pub fn get_matches_from_safe_lossy<I, T>(mut self, itr: I) -> ClapResult<ArgMatches<'ar, 'ar>>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        self._get_matches_from_safe_borrow(itr, true)
    }

    /// Starts the parsing process without consuming the `App` struct `self`. This is normally not
    /// the desired functionality, instead prefer `App::get_matches_from_safe` which *does*
    /// consume `self`.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name.
    ///
    /// **NOTE:** This method should only be used when absolutely necessary, such as needing to
    /// parse arguments from something other than `std::env::args()`. If you are unsure, use
    /// `App::get_matches_safe()`
    ///
    /// **NOTE:** This method should only be used when is absolutely necessary to handle errors
    /// manually.
    ///
    /// **NOTE:** Invalid unicode characters will result in an `Err` with type
    /// `ClapErrorType::InvalidUnicode`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let mut app = App::new("myprog");
    ///     // Args and options go here...
    /// let matches = app.get_matches_from_safe_borrow(arg_vec)
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    pub fn get_matches_from_safe_borrow<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches<'ar, 'ar>>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        self._get_matches_from_safe_borrow(itr, false)
    }

    /// Starts the parsing process without consuming the `App` struct `self`. This is normally not
    /// the desired functionality, instead prefer `App::get_matches_from_safe` which *does*
    /// consume `self`. Invalid unicode characters are replaced with `U+FFFD REPLACEMENT CHARACTER`
    ///
    /// **NOTE:** The first argument will be parsed as the binary name.
    ///
    /// **NOTE:** This method should only be used when absolutely necessary, such as needing to
    /// parse arguments from something other than `std::env::args()`. If you are unsure, use
    /// `App::get_matches_safe()`
    ///
    /// **NOTE:** This method should only be used when is absolutely necessary to handle errors
    /// manually.
    ///
    /// **NOTE:** Invalid unicode characters will result in an `Err` with type
    /// `ClapErrorType::InvalidUnicode`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];
    ///
    /// let mut app = App::new("myprog");
    ///     // Args and options go here...
    /// let matches = app.get_matches_from_safe_borrow(arg_vec)
    ///     .unwrap_or_else( |e| { panic!("An error occurs: {}", e) });
    /// ```
    pub fn get_matches_from_safe_borrow_lossy<I, T>(&mut self,
                                                    itr: I)
                                                    -> ClapResult<ArgMatches<'ar, 'ar>>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        self._get_matches_from_safe_borrow(itr, true)
    }

    fn _get_matches_from_safe_borrow<I, T>(&mut self,
                                           itr: I,
                                           lossy: bool)
                                           -> ClapResult<ArgMatches<'ar, 'ar>>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        // Verify all positional assertions pass
        self.verify_positionals();
        // If there are global arguments, we need to propgate them down to subcommands
        // before parsing incase we run into a subcommand
        self.propogate_globals();

        let mut matcher = ArgMatcher::new();

        let mut it = itr.into_iter();
        // Get the name of the program (argument 1 of env::args()) and determine the
        // actual file
        // that was used to execute the program. This is because a program called
        // ./target/release/my_prog -a
        // will have two arguments, './target/release/my_prog', '-a' but we don't want
        // to display
        // the full path when displaying help messages and such
        if !self.settings.is_set(&AppSettings::NoBinaryName) {
            if let Some(name) = it.next() {
                let p = Path::new(name.as_ref());
                if let Some(f) = p.file_name() {
                    if let Ok(s) = f.to_os_string().into_string() {
                        if let None = self.bin_name {
                            self.bin_name = Some(s);
                        }
                    }
                }
            }
        }

        // do the real parsing
        if let Err(e) = self.get_matches_with(&mut matcher, &mut it, lossy) {
            return Err(e);
        }

        Ok(matcher.into())
    }

    // The actual parsing function
    #[cfg_attr(feature="lints", allow(while_let_on_iterator))]
    fn get_matches_with<I, T>(&mut self,
                              matcher: &mut ArgMatcher<'ar>,
                              it: &mut I,
                              lossy: bool)
                              -> ClapResult<()>
        where I: Iterator<Item = T>,
              T: AsRef<OsStr>
    {
        // First we create the `--help` and `--version` arguments and add them if
        // necessary
        self.create_help_and_version();

        let mut pos_only = false;
        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: Option<&str> = None;
        let mut pos_counter = 1;
        while let Some(arg) = it.next() {
            let arg_cow = match arg.as_ref().to_str() {
                Some(s) => s.into(),
                None => {
                    if !lossy {
                        return Err(
                            error_builder::InvalidUnicode(
                                &*try!(self.create_current_usage(matcher)))
                        );
                    }
                    arg.as_ref().to_string_lossy()
                }
            };
            let arg_slice: &str = arg_cow.borrow();

            // we need to know if we're parsing a new argument, or the value of previous
            // argument,
            // perhaps one with multiple values such as --option val1 val2. We determine
            // when to
            // stop parsing multiple values by finding a '-'
            // let new_arg = if arg_slice.starts_with("-") {
            //     // If we come to a single `-` it's a value, not a new argument...this happens
            //     // when
            //     // one wants to use the Unix standard of '-' to mean 'stdin'
            //     arg_slice.len() > 1
            // } else {
            //     true
            // };

            // pos_only is determined later, and set true when a user uses the Unix
            // standard of
            // '--' to mean only positionals follow
            // If the user passed `--` we don't check for subcommands, because the argument
            // they
            // may be trying to pass might match a subcommand name

            let starts_new_arg = if arg_slice.starts_with("-") {
                !(arg_slice.len() == 1)
            } else {
                false
            };
            if !pos_only {
                if !starts_new_arg {
                    // Check to see if parsing a value from an option
                    if let Some(nvo) = needs_val_of {
                        // get the OptBuilder so we can check the settings
                        if let Some(opt) = self.opts.iter().filter(|o| &o.name == &nvo).next() {
                            needs_val_of = try!(self.add_val_to_arg(opt, arg_slice, matcher));
                            // get the next value from the iterator
                            continue;
                        }
                    }
                }
                let mut skip = false;
                if arg_slice.starts_with("--") {
                    if arg_slice.len() == 2 {
                        // The user has passed '--' which means only positional args follow no matter
                        // what they start with
                        pos_only = true;
                        continue;
                    }

                    needs_val_of = try!(self.parse_long_arg(matcher, arg_slice));
                } else if arg_slice.starts_with("-") && arg_slice.len() != 1 {
                    needs_val_of = try!(self.parse_short_arg(matcher, arg_slice));
                } else {
                    skip = true;
                }
                if !skip { continue; }
                if self.subcommands.iter().any(|s| s.name_slice == arg_slice) {
                    if arg_slice == "help" &&
                       self.settings.is_set(&AppSettings::NeedsSubcommandHelp) {
                        return self._help();
                    }
                    subcmd_name = Some(arg_slice.to_owned());
                    break;
                } else if let Some(candidate) = suggestions::did_you_mean(
                                                        arg_slice,
                                                        self.subcommands.iter().map(|s| &s.name)) {
                    return Err(error_builder::InvalidSubcommand(
                                                        arg_slice,
                                                        candidate,
                                                        self.bin_name.as_ref().unwrap_or(&self.name),
                                                        &*try!(self.create_current_usage(matcher))));
                }
            }


            if let Some(p) = self.positionals.get(&pos_counter) {
                try!(self.validate_arg(p, matcher));

                try!(self.add_val_to_arg(p, arg_slice, matcher));

                if !pos_only &&
                   (self.settings.is_set(&AppSettings::TrailingVarArg) &&
                    pos_counter == self.positionals.len()) {
                    pos_only = true;
                }
                arg_post_processing!(self, p, matcher);

                // Only increment the positional counter if it doesn't allow multiples
                if !p.settings.is_set(&ArgSettings::Multiple) {
                    pos_counter += 1;
                }
            } else {
                return Err(error_builder::UnexpectedArgument(
                    arg_slice,
                    self.bin_name.as_ref().unwrap_or(&self.name),
                    &*try!(self.create_current_usage(matcher))));
            }
        }

        if let Some(a) = needs_val_of {
            if let Some(o) = self.opts.iter().filter(|o| &o.name == &a).next() {
                if (o.settings.is_set(&ArgSettings::Multiple) && self.required.is_empty()) ||
                    !o.settings.is_set(&ArgSettings::Multiple){
                    let should_err = match matcher.values_of(o.name) {
                        Some(ref v) => v.is_empty(),
                        None => true,
                    };
                    if should_err {
                        return Err(error_builder::EmptyValue(
                            &*o.to_string(),
                            &*try!(self.create_current_usage(matcher))
                        ));
                    }
                } else {
                    return Err(error_builder::MissingRequiredArgument(
                        &*self.get_required_from(&self.required, Some(matcher))
                            .iter()
                            .fold(String::new(),
                                |acc, s| acc + &format!("\n\t{}", Format::Error(s))[..]),
                        &*try!(self.create_current_usage(matcher))));
                }
            } else {
                return Err(error_builder::EmptyValue(&*format!("{}",
                                                               self.positionals
                                                                   .values()
                                                                   .filter(|p| &p.name == &a)
                                                                   .next()
                                                                   .expect(INTERNAL_ERROR_MSG)),
                                                     &*try!(self.create_current_usage(matcher))));
            }
        }

        if let Err(e) = self.validate_blacklist(matcher) {
            return Err(e);
        }

        if let Err(e) = self.validate_num_args(matcher) {
            return Err(e);
        }

        matcher.usage(try!(self.create_usage(&[])));

        if !(self.settings.is_set(&AppSettings::SubcommandsNegateReqs) && subcmd_name.is_some()) {
                try!(self.validate_required(&matcher));
        }
        if let Some(sc_name) = subcmd_name {
            use std::fmt::Write;
            let mut mid_string = String::new();
            if !self.settings.is_set(&AppSettings::SubcommandsNegateReqs) {
                let mut hs = self.required.iter().map(|n| *n).collect::<Vec<_>>();
                for k in matcher.arg_names() {
                    hs.push(*k);
                }
                let reqs = self.get_required_from(&hs, Some(matcher));

                for s in reqs.iter() {
                    write!(&mut mid_string, " {}", s).ok().expect(INTERNAL_ERROR_MSG);
                }
            }
            mid_string.push_str(" ");
            if let Some(ref mut sc) = self.subcommands
                                          .iter_mut()
                                          .filter(|s| s.name_slice == &sc_name)
                                          .next() {
                let mut sc_matcher = ArgMatcher::new();
                // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by
                // a space
                sc.usage = Some(format!("{}{}{}",
                                        self.bin_name.as_ref().unwrap_or(&String::new()),
                                        if self.bin_name.is_some() {
                                            &*mid_string
                                        } else {
                                            ""
                                        },
                                        &*sc.name));
                sc.bin_name = Some(format!("{}{}{}",
                                           self.bin_name.as_ref().unwrap_or(&String::new()),
                                           if self.bin_name.is_some() {
                                               " "
                                           } else {
                                               ""
                                           },
                                           &*sc.name));
                if let Err(e) = sc.get_matches_with(&mut sc_matcher, it, lossy) {
                    e.exit();
                }
                matcher.subcommand(SubCommand {
                    name: sc.name_slice,
                    matches: sc_matcher.into(),
                });
            }
        } else if self.settings.is_set(&AppSettings::SubcommandRequired) {
            let bn = self.bin_name.as_ref().unwrap_or(&self.name);
            return Err(error_builder::MissingSubcommand(bn,
                                                        &try!(self.create_current_usage(matcher))));
        } else if self.settings.is_set(&AppSettings::SubcommandRequiredElseHelp) {
            let mut out = vec![];
            try!(self.write_help(&mut out));
            return Err(ClapError {
                error: String::from_utf8_lossy(&*out).into_owned(),
                error_type: ClapErrorType::MissingArgumentOrSubcommand,
            });
        }
        if matcher.is_empty() && matcher.subcommand_name().is_none() &&
           self.settings.is_set(&AppSettings::ArgRequiredElseHelp) {
            let mut out = vec![];
            try!(self.write_help(&mut out));
            return Err(ClapError {
                error: String::from_utf8_lossy(&*out).into_owned(),
                error_type: ClapErrorType::MissingArgumentOrSubcommand,
            });
        }
        Ok(())
    }

    // basically re-implements ClapError::exit except it checks if we should wait
    // for input before
    // exiting since ClapError doesn't have that info and the error message must be
    // printed before
    // exiting
    fn maybe_wait_for_exit(&self, e: ClapError) -> ! {
        if e.use_stderr() {
            wlnerr!("{}", e.error);
            if self.settings.is_set(&AppSettings::WaitOnError) {
                wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                let mut s = String::new();
                let i = io::stdin();
                i.lock().read_line(&mut s).unwrap();
            }
            process::exit(1);
        }

        e.exit()
    }

    // Prints the version to the user and exits if quit=true
    fn print_version<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        // Print the binary name if existing, but replace all spaces with hyphens in
        // case we're
        // dealing with subcommands i.e. git mv is translated to git-mv
        try!(writeln!(w,
                      "{} {}",
                      &self.bin_name.as_ref().unwrap_or(&self.name)[..].replace(" ", "-"),
                      self.version.unwrap_or("")));

        w.flush().map_err(ClapError::from)
    }

    #[doc(hidden)]
    pub fn create_current_usage(&self, matcher: &ArgMatcher) -> ClapResult<String> {
        self.create_usage(&matcher.arg_names()
                                  .filter(|k| {
                                      if let Some(o) = self.opts
                                                           .iter()
                                                           .filter(|o| &&o.name == k)
                                                           .next() {
                                          !o.settings.is_set(&ArgSettings::Required)
                                      } else if let Some(p) = self.positionals
                                                           .values()
                                                           .filter(|p| &&p.name == k)
                                                           .next() {
                                          !p.settings.is_set(&ArgSettings::Required)
                                      } else {
                                          true
                                      }
                                  })
                                  .map(|k| *k)
                                  .collect::<Vec<_>>())
    }

    fn groups_for_arg(&self, name: &str) -> Option<Vec<&'ar str>> {
        if self.groups.is_empty() {
            return None;
        }
        let mut res = vec![];
        for (g_name, grp) in &self.groups {
            for a in &grp.args {
                if a == &name {
                    res.push(*g_name);
                }
            }
        }
        if res.is_empty() {
            return None;
        }

        Some(res)
    }

    fn args_in_group(&self, group: &str) -> Vec<String> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.groups.get(group).unwrap().args {
            if let Some(f) = self.flags.iter().filter(|f| &f.name == n).next() {
                args.push(f.to_string());
            } else if let Some(f) = self.opts.iter().filter(|o| &o.name == n).next() {
                args.push(f.to_string());
            } else if self.groups.contains_key(n) {
                g_vec.push(*n);
            } else {
                if let Some(p) = self.positionals
                                     .values()
                                     .filter(|p| &p.name == n)
                                     .next() {
                    args.push(p.to_string());
                }
            }
        }

        if !g_vec.is_empty() {
            for av in g_vec.iter().map(|g| self.args_in_group(g)) {
                for a in av {
                    args.push(a);
                }
            }
        }
        assert!(!args.is_empty(),
                "ArgGroup '{}' doesn't contain any args",
                group);
        args.dedup();
        args.iter().map(ToOwned::to_owned).collect()
    }

    fn arg_names_in_group(&self, group: &'ar str) -> Vec<&'ar str> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.groups.get(group).unwrap().args {
            if self.flags.iter().any(|f| &f.name == n) {
                args.push(*n);
            } else if self.opts.iter().any(|o| &o.name == n) {
                args.push(*n);
            } else if self.groups.contains_key(n) {
                g_vec.push(*n);
            } else if self.positionals.values().any(|p| &p.name == n) {
                args.push(*n);
            }
        }

        if !g_vec.is_empty() {
            for av in g_vec.iter().map(|g| self.arg_names_in_group(g)) {
                for a in av {
                    args.push(a);
                }
            }
        }
        assert!(!args.is_empty(),
                "ArgGroup '{}' doesn't contain any args",
                group);
        args.dedup();
        args.iter().map(|s| *s).collect()
    }

    fn get_required_from(&self,
                         reqs: &[&'ar str],
                         matcher: Option<&ArgMatcher>)
                         -> VecDeque<String> {
        let mut c_flags = vec![];
        let mut c_pos = vec![];
        let mut c_opt = vec![];
        let mut grps = vec![];
        for name in reqs {
            if self.flags.iter().any(|f| &f.name == name) {
                c_flags.push(name);
            } else if self.opts.iter().any(|o| &o.name == name) {
                c_opt.push(name);
            } else if self.groups.contains_key(name) {
                grps.push(*name);
            } else {
                c_pos.push(name);
            }
        }
        let mut tmp_f = vec![];
        for f in &c_flags {
            if let Some(f) = self.flags.iter().filter(|flg| &&flg.name == f).next() {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.iter().any(|f| &f.name == r) {
                                tmp_f.push(r);
                            } else if self.opts.iter().any(|o| &o.name == r) {
                                c_opt.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(*r);
                            } else {
                                c_pos.push(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_f.into_iter() {
            c_flags.push(f);
        }
        let mut tmp_o = vec![];
        for f in &c_opt {
            if let Some(f) = self.opts.iter().filter(|o| &&o.name == f).next() {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.iter().any(|f| &f.name == r) {
                                c_flags.push(r);
                            } else if self.opts.iter().any(|o| &o.name == r) {
                                tmp_o.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(*r);
                            } else {
                                c_pos.push(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_o.into_iter() {
            c_opt.push(f);
        }
        let mut tmp_p = vec![];
        for p in c_pos.iter() {
            if let Some(p) = self.positionals.values().filter(|pos| &&pos.name == p).next() {
                if let Some(ref rl) = p.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.iter().any(|f| &f.name == r) {
                                c_flags.push(r);
                            } else if self.opts.iter().any(|o| &o.name == r) {
                                c_opt.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(*r);
                            } else {
                                tmp_p.push(r);
                            }
                        }
                    }
                }
            }
        }
        for f in tmp_p.into_iter() {
            c_pos.push(f);
        }

        let mut ret_val = VecDeque::new();

        let mut pmap = BTreeMap::new();
        for p in c_pos.into_iter() {
            if matcher.is_some() && matcher.as_ref().unwrap().contains(p) {
                continue;
            }
            if let Some(p) = self.positionals.values().filter(|x| &x.name == p).next() {
                pmap.insert(p.index, format!("{}", p));
            }
        }
        for (_, s) in pmap {
            ret_val.push_back(s);
        }
        for f in c_flags.into_iter() {
            if matcher.is_some() && matcher.as_ref().unwrap().contains(f) {
                continue;
            }
            ret_val.push_back(format!("{}",
                                      self.flags
                                          .iter()
                                          .filter(|flg| &flg.name == f)
                                          .next()
                                          .unwrap()));
        }
        for o in c_opt.into_iter() {
            if matcher.is_some() && matcher.as_ref().unwrap().contains(o) {
                continue;
            }
            ret_val.push_back(format!("{}",
                                      self.opts
                                          .iter()
                                          .filter(|opt| &opt.name == o)
                                          .next()
                                          .unwrap()));
        }
        for g in grps.into_iter() {
            let g_string = self.args_in_group(g)
                               .iter()
                               .fold(String::new(), |acc, s| acc + &format!(" {} |", s)[..]);
            ret_val.push_back(format!("[{}]", &g_string[..g_string.len() - 1]));
        }

        ret_val
    }

    fn verify_positionals(&mut self) {
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // Firt we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)
        if let Some((idx, ref p)) = self.positionals.iter().rev().next() {
            if idx != self.positionals.len() {
                panic!("Found positional argument \"{}\" who's index is {} but there are only {} \
                    positional arguments defined",
                       p.name,
                       idx,
                       self.positionals.len());
            }
        }

        // Next we verify that only the highest index has a .multiple(true) (if any)
        assert!(!self.positionals
                     .values()
                     .any(|a| {
                         a.settings.is_set(&ArgSettings::Multiple) &&
                         (a.index as usize != self.positionals.len())
                     }),
                "Only the positional argument with the highest index may accept multiple values");

        // If it's required we also need to ensure all previous positionals are
        // required too
        let mut found = false;
        for p in self.positionals.values().rev() {
            if !found {
                if p.settings.is_set(&ArgSettings::Required) {
                    found = true;
                    continue;
                }
            } else {
                assert!(p.settings.is_set(&ArgSettings::Required),
                    "Found positional argument which is not required with a lower index than a \
                    required positional argument: {:?} index {}",
                    p.name,
                    p.index);
            }
        }
    }

    fn propogate_globals(&mut self) {
        for sc in self.subcommands.iter_mut() {
            // We have to create a new scope in order to tell rustc the borrow of `sc` is
            // done and
            // to recursively call this method
            {
                for a in self.global_args.iter() {
                    sc.add_arg(a.into());
                }
            }
            sc.propogate_globals();
        }
    }

    fn blacklisted_from(&self, name: &str, matcher: &ArgMatcher) -> Option<String> {
        for k in matcher.arg_names() {
            if let Some(f) = self.flags.iter().filter(|f| &f.name == k).next() {
                if let Some(ref bl) = f.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", f));
                    }
                }
            }
            if let Some(o) = self.opts.iter().filter(|o| &o.name == k).next() {
                if let Some(ref bl) = o.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", o));
                    }
                }
            }
            if let Some(pos) = self.positionals.values().filter(|p| &p.name == k).next() {
                if let Some(ref bl) = pos.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", pos));
                    }
                }
            }
        }
        None
    }

    fn overriden_from(&self, name: &'ar str, matcher: &ArgMatcher) -> Option<&'ar str> {
        for k in matcher.arg_names() {
            if let Some(f) = self.flags.iter().filter(|f| &f.name == k).next() {
                if let Some(ref bl) = f.overrides {
                    if bl.contains(&name) {
                        return Some(f.name);
                    }
                }
            }
            if let Some(o) = self.opts.iter().filter(|o| &o.name == k).next() {
                if let Some(ref bl) = o.overrides {
                    if bl.contains(&name) {
                        return Some(o.name);
                    }
                }
            }
            if let Some(pos) = self.positionals.values().filter(|p| &p.name == k).next() {
                if let Some(ref bl) = pos.overrides {
                    if bl.contains(&name) {
                        return Some(pos.name);
                    }
                }
            }
        }
        None
    }

    fn create_help_and_version(&mut self) {
        // name is "hclap_help" because flags are sorted by name
        if !self.flags.iter().any(|a| a.long.is_some() && a.long.unwrap() == "help") {
            if self.help_short.is_none() && !self.short_list.contains(&'h') {
                self.help_short = Some('h');
            }
            let arg = FlagBuilder {
                name: "hclap_help",
                short: self.help_short,
                long: Some("help"),
                help: Some("Prints help information"),
                blacklist: None,
                requires: None,
                overrides: None,
                settings: ArgFlags::new(),
            };
            self.long_list.push("help");
            self.flags.push(arg);
        }
        if !self.settings.is_set(&AppSettings::DisableVersion) &&
           !self.flags.iter().any(|a| a.long.is_some() && a.long.unwrap() == "version") {
            if self.version_short.is_none() && !self.short_list.contains(&'V') {
                self.version_short = Some('V');
            }
            // name is "vclap_version" because flags are sorted by name
            let arg = FlagBuilder {
                name: "vclap_version",
                short: self.version_short,
                long: Some("version"),
                help: Some("Prints version information"),
                blacklist: None,
                requires: None,
                overrides: None,
                settings: ArgFlags::new(),
            };
            self.long_list.push("version");
            self.flags.push(arg);
        }
        if !self.subcommands.is_empty() &&
           !self.subcommands
                .iter()
                .any(|a| a.name_slice == "help") {
            self.subcommands.push(App::new("help").about("Prints this message"));
        }
    }

    fn check_for_help_and_version_str(&self, arg: &str) -> ClapResult<()> {
        if arg == "help" && self.settings.is_set(&AppSettings::NeedsLongHelp) {
            try!(self._help());
        }
        if arg == "version" && self.settings.is_set(&AppSettings::NeedsLongVersion) {
            try!(self._version());
        }

        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        if let Some(h) = self.help_short { if arg == h { try!(self._help()); } }
        if let Some(v) = self.version_short { if arg == v { try!(self._version()); } }
        Ok(())
    }

    fn _help(&self) -> ClapResult<()> {
        try!(self.print_help());
        Err(ClapError {
            error: String::new(),
            error_type: ClapErrorType::HelpDisplayed,
        })
    }

    fn _version(&self) -> ClapResult<()> {
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        try!(self.print_version(&mut buf_w));
        Err(ClapError {
            error: String::new(),
            error_type: ClapErrorType::VersionDisplayed,
        })
    }

    fn parse_long_arg<'av>(&mut self,
                           matcher: &mut ArgMatcher<'ar>,
                           full_arg: &'av str)
                           -> ClapResult<Option<&'ar str>> {
        let mut val = "";
        let arg = if full_arg.contains("=") {
            let parts: Vec<_> = full_arg.trim_left_matches(|c| c == '-').splitn(2, "=").collect();
            val = parts[1];
            parts[0]
        } else {
            full_arg.trim_left_matches(|c| c == '-')
        };

        if let Some(opt) = self.opts
                    .iter()
                    .filter(|v| v.long.is_some() && v.long.unwrap() == arg)
                    .next() {
            let ret = try!(self.parse_opt(val, opt, matcher));
            arg_post_processing!(self, opt, matcher);

            return Ok(ret);
        } else if let Some(flag) = self.flags
                    .iter()
                    .filter(|v| v.long.is_some() && v.long.unwrap() == arg)
                    .next() {
            // Only flags could be help or version, and we need to check the raw long
            // so this is the first point to check
            try!(self.check_for_help_and_version_str(arg));

            try!(self.parse_flag(flag, matcher));

            // Handle conflicts, requirements, etc.
            arg_post_processing!(self, flag, matcher);

            return Ok(None);
        }

        self.did_you_mean_error(arg, matcher).map(|_| None)
    }

    fn parse_short_arg(&mut self,
                       matcher: &mut ArgMatcher<'ar>,
                       full_arg: &str)
                       -> ClapResult<Option<&'ar str>> {
        let arg = &*full_arg.trim_left_matches(|c| c == '-');
        for c in arg.chars() {
            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(opt) = self.opts
                        .iter()
                        .filter(|&v| v.short.is_some() && v.short.unwrap() == c)
                        .next() {
                // Check for trailing concatenated value
                let val = arg.splitn(2, c).collect::<Vec<_>>()[1];

                // Default to "we're expecting a value later"
                let ret = try!(self.parse_opt(val, opt, matcher));

                arg_post_processing!(self, opt, matcher);

                return Ok(ret);
            }

            try!(self.check_for_help_and_version_char(c));
            if let Some(flag) = self.flags.iter().filter(|&v| v.short.is_some() && v.short.unwrap() == c).next() {
                // Only flags can be help or version
                try!(self.parse_flag(flag, matcher));
                // Handle conflicts, requirements, overrides, etc.
                // Must be called here due to mutablilty
                arg_post_processing!(self, flag, matcher);
            }
        }
        Ok(None)
    }

    fn parse_opt(&self, val: &str, opt: &OptBuilder<'ar>, matcher: &mut ArgMatcher<'ar>) -> ClapResult<Option<&'ar str>> {
        try!(self.validate_arg(opt, matcher));

        if matcher.contains(opt.name) && !opt.settings.is_set(&ArgSettings::Multiple) {
            // Not the first time, but we don't allow multiples
            return Err(error_builder::UnexpectedMultipleUsage(
                &*opt.to_string(),
                &*try!(self.create_current_usage(matcher))));
        }

        if val.len() != 0 {
            if opt.is_set(ArgSettings::EmptyValues) {
                try!(self.add_val_to_arg(opt, val, matcher));
                return Ok(None);
            }
            return Err(error_builder::EmptyValue(
                &*opt.to_string(),
                &*try!(self.create_current_usage(matcher))));
        }

        // If it doesn't allow mutliples, (or a specific number of values), or we don't have any
        // values yet, we want to return the name of this arg so the next arg is parsed as a value
        // otherwise we're done getting values
        if opt.settings.is_set(&ArgSettings::Multiple) || opt.num_vals.is_some() || !matcher.contains(opt.name) {
            return Ok(Some(opt.name));
        }
        Ok(None)
    }

    fn add_val_to_arg<A>(&self,
                        arg: &A,
                        val: &str,
                        matcher: &mut ArgMatcher<'ar>)
                        -> ClapResult<Option<&'ar str>>
        where A: AnyArg<'ar> + Display {
        matcher.add_val_to(arg.name(), val.to_owned());

        // Increment or create the group "args"
        if let Some(grps) = self.groups_for_arg(arg.name()) {
            for grp in grps {
                matcher.add_val_to(grp, val.to_owned());
            }
        }

        // The validation must come AFTER inserting into 'matcher' or the usage string
        // can't be built
        self.validate_value(arg, val, matcher)
    }

    fn validate_value<A>(&self, arg: &A, val: &str, matcher: &ArgMatcher<'ar>) -> ClapResult<Option<&'ar str>>
        where A: AnyArg<'ar> + Display {
        if let Some(ref p_vals) = arg.possible_vals() {
            if !p_vals.contains(&val) {
                return Err(
                    error_builder::InvalidValue(val,
                                                p_vals,
                                                &arg.to_string(),
                                                &*try!(self.create_current_usage(matcher))));
            }
        }
        if !arg.is_set(ArgSettings::EmptyValues) &&
            val.is_empty() &&
            matcher.contains(arg.name()) {
            return Err(error_builder::EmptyValue(&*arg.to_string(),
                                                 &*try!(self.create_current_usage(matcher))));
        }
        if let Some(ref vtor) = arg.validator() {
            if let Err(e) = vtor(val.to_owned()) {
                return Err(error_builder::ValueValidationError(&*e));
            }
        }
        let vals = matcher.get(arg.name())
                          .expect(INTERNAL_ERROR_MSG)
                          .values.as_ref()
                                 .expect(INTERNAL_ERROR_MSG)
                                 .len();
        if let Some(max) = arg.max_vals() {
            if (vals as u8) < max {
                return Ok(Some(arg.name()));
            } else {
                return Ok(None);
            }
        }
        if let Some(..) = arg.min_vals() {
            return Ok(Some(arg.name()));
        }
        if let Some(num) = arg.num_vals() {
            if arg.is_set(ArgSettings::Multiple) {
                if (vals as u8) < num {
                    return Ok(Some(arg.name()));
                } 
            } else {
                if (vals as u8 % num) != 0 {
                    return Ok(Some(arg.name()));
                }
            }
        }
        if arg.is_set(ArgSettings::Multiple) {
            return Ok(Some(arg.name()));
        }
        Ok(None)
    }

    fn parse_flag(&self, flag: &FlagBuilder<'ar>, matcher: &mut ArgMatcher<'ar>) -> ClapResult<()> {
        // Validate that we can actually accept this arg
        try!(self.validate_arg(flag, matcher));

        // First occurrence or not?
        if !matcher.contains(flag.name) {
            // If this is the first, then add this flag itself
            matcher.insert(flag.name);
        } else if !flag.settings.is_set(&ArgSettings::Multiple) {
            // Not the first time, but we don't allow multiples
            return Err(error_builder::UnexpectedMultipleUsage(
                &*flag.to_string(),
                &*try!(self.create_current_usage(matcher))));
        } else {
            matcher.inc_occurrence_of(flag.name);
        }

        // Increment or create the group "args"
        self.groups_for_arg(flag.name).and_then(|vec| Some(matcher.inc_occurrences_of(&vec)));

        Ok(())
    }

    fn validate_arg<A>(&self, arg: &A, matcher: &mut ArgMatcher<'ar>) -> ClapResult<()>
        where A: AnyArg<'ar> + Display {
        // May not be needed since we can't get here without being an Arg of some type
        //
        // if arg.has_switch() && (!self.flags.iter().any(|f| f.name == arg.name()) &&
        //     !self.opts.iter().any(|f| f.name == arg.name())) {
        //     return Err(error_builder::InvalidArgument(
        //         &*format!("{}", arg),
        //         None,
        //         &*try!(self.create_current_usage(matcher))));
        // }

        // Ensure this arg isn't on the mutually excludes list
        if self.blacklist.contains(&arg.name()) {
            matcher.remove(arg.name());
            return Err(error_builder::ArgumentConflict(
                arg.to_string(),
                self.blacklisted_from(arg.name(), &matcher),
                try!(self.create_current_usage(matcher))));
        }

        // Make sure this isn't one being added multiple times if it doesn't support it
        if matcher.contains(arg.name()) && !arg.is_set(ArgSettings::Multiple) {
            return Err(error_builder::UnexpectedMultipleUsage(
                &*format!("{}", arg),
                &*try!(self.create_current_usage(matcher))));
        }

        Ok(())
    }

    fn validate_blacklist(&self, matcher: &mut ArgMatcher<'ar>) -> ClapResult<()> {
        for name in self.blacklist.iter() {
            if matcher.contains(name) {
                matcher.remove(name);
                return Err(error_builder::ArgumentConflict(
                    format!("{}", Format::Warning(
                        if let Some(f) = self.flags.iter().filter(|f| &f.name == name).next() {
                            f.to_string()
                        } else if let Some(o) = self.opts.iter()
                                                         .filter(|o| &o.name == name)
                                                         .next() {
                            o.to_string()
                        } else {
                            match self.positionals.values()
                                                    .filter(|p| p.name == *name)
                                                    .next() {
                                Some(p) => p.to_string(),
                                None    => format!("'{}'", name)
                            }
                        }
                    )),
                    self.blacklisted_from(name, &matcher),
                    try!(self.create_current_usage(matcher))));
            } else if self.groups.contains_key(name) {
                for n in self.arg_names_in_group(name) {
                    if matcher.contains(n) {
                        matcher.remove(n);
                        return Err(error_builder::ArgumentConflict(
                            format!("{}", Format::Warning(
                                if let Some(f) = self.flags.iter()
                                                           .filter(|f| &f.name == name)
                                                           .next() {
                                    f.to_string()
                                } else if let Some(o) = self.opts
                                                         .iter()
                                                         .filter(|o| &o.name == name)
                                                         .next() {
                                    o.to_string()
                                } else {
                                    match self.positionals.values()
                                                            .filter(|p| p.name == n)
                                                            .next() {
                                        Some(p) => p.to_string(),
                                        None    => format!("'{}'", name)
                                    }
                                }
                            )),
                            self.blacklisted_from(name, &matcher),
                            try!(self.create_current_usage(matcher))));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_num_args(&self, matcher: &mut ArgMatcher<'ar>) -> ClapResult<()> {
        for (name, ma) in matcher.iter() {
            if self.groups.contains_key(name) {
                continue;
            } else if let Some(ref vals) = ma.values {
                if let Some(f) = self.opts
                                     .iter()
                                     .filter(|o| &o.name == name)
                                     .next() {
                    if let Some(num) = f.num_vals {
                        let should_err = if f.settings.is_set(&ArgSettings::Multiple) {
                            ((vals.len() as u8) % num) != 0
                        } else {
                            num != (vals.len() as u8)
                        };
                        if should_err {
                            return Err(error_builder::WrongNumValues(
                                &*f.to_string(),
                                num,
                                if f.settings.is_set(&ArgSettings::Multiple) {
                                    (vals.len() % num as usize)
                                } else {
                                    vals.len()
                                },
                                if vals.len() == 1 ||
                                    (f.settings.is_set(&ArgSettings::Multiple) &&
                                        (vals.len() % num as usize) == 1) {
                                    "as"
                                } else {
                                    "ere"
                                },
                                &*try!(self.create_current_usage(matcher))));
                        }
                    }
                    if let Some(num) = f.max_vals {
                        if (vals.len() as u8) > num {
                            return Err(error_builder::TooManyValues(
                                vals.get(&vals.keys()
                                             .last()
                                             .expect(INTERNAL_ERROR_MSG))
                                    .expect(INTERNAL_ERROR_MSG),
                                &f.to_string(),
                                &try!(self.create_current_usage(matcher))));
                        }
                    }
                    if let Some(num) = f.min_vals {
                        if (vals.len() as u8) < num {
                            return Err(error_builder::TooFewValues(
                                &*f.to_string(),
                                num,
                                vals.len(),
                                &*try!(self.create_current_usage(matcher))));
                        }
                    }
                } else if let Some(f) = self.positionals
                                     .values()
                                     .filter(|p| &p.name == name)
                                     .next() {
                    if let Some(num) = f.num_vals {
                        if num != vals.len() as u8 {
                            return Err(error_builder::WrongNumValues(
                                &*f.to_string(),
                                num,
                                vals.len(),
                                if vals.len() == 1 {
                                    "as"
                                } else {
                                    "ere"
                                },
                                &*try!(self.create_current_usage(matcher))));
                        }
                    }
                    if let Some(max) = f.max_vals {
                        if (vals.len() as u8) > max {
                            return Err(error_builder::TooManyValues(
                                vals.get(&vals.keys()
                                             .last()
                                             .expect(INTERNAL_ERROR_MSG))
                                    .expect(INTERNAL_ERROR_MSG),
                                &f.to_string(),
                                &try!(self.create_current_usage(matcher))));
                        }
                    }
                    if let Some(min) = f.min_vals {
                        if (vals.len() as u8) < min {
                            return Err(error_builder::TooFewValues(
                                &*f.to_string(),
                                min,
                                vals.len(),
                                &*try!(self.create_current_usage(matcher))));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_required(&self, matcher: &ArgMatcher<'ar>) -> ClapResult<()> {
        'outer: for name in self.required.iter() {
            if matcher.contains(name) {
                continue 'outer;
            }
            if let Some(grp) = self.groups.get(name) {
                for arg in &grp.args {
                    if matcher.contains(arg) { continue 'outer; }
                }
            }
            if self.groups.values().any(|g| g.args.contains(name)) {
                continue 'outer;
            }
            if let Some(a) = self.flags.iter().filter(|f| &f.name == name).next() {
                if self._validate_blacklist_required(a, matcher) { continue 'outer; }
            }
            if let Some(a) = self.opts.iter().filter(|o| &o.name == name).next() {
                if self._validate_blacklist_required(a, matcher) { continue 'outer; }
            }
            if let Some(a) = self.positionals.values().filter(|p| &p.name == name).next() {
                if self._validate_blacklist_required(a, matcher) { continue 'outer; }
            }
            return Err(error_builder::MissingRequiredArgument(
                &*self.get_required_from(&self.required, Some(matcher))
                      .iter()
                      .fold(String::new(),
                          |acc, s| acc + &format!("\n\t{}", Format::Error(s))[..]),
                &*try!(self.create_current_usage(matcher))));
        }
        Ok(())
    }

    fn _validate_blacklist_required<A>(&self, a: &A, matcher: &ArgMatcher) -> bool where A: AnyArg<'ar> {
        if let Some(bl) = a.blacklist() {
            for n in bl.iter() {
                if matcher.contains(n) {
                    return true;
                } else if self.groups
                              .get(n)
                              .map(|g| g.args.iter().any(|an| matcher.contains(an)))
                              .unwrap_or(false) {
                    return true;
                }
            }
        }
        false
    }

    fn did_you_mean_error(&self, arg: &str, matcher: &mut ArgMatcher<'ar>) -> ClapResult<()> {
        // Didn't match a flag or option...maybe it was a typo and close to one
        let suffix = suggestions::did_you_mean_suffix(arg,
                                              self.long_list.iter(),
                                              suggestions::DidYouMeanMessageStyle::LongFlag);

        // Add the arg to the matches to build a proper usage string
        if let Some(name) = suffix.1 {
            if let Some(opt) = self.opts
                                       .iter()
                                       .filter(|o| o.long.is_some() && o.long.unwrap() == name)
                                       .next() {
                self.groups_for_arg(opt.name).and_then(|grps| Some(matcher.inc_occurrences_of(&grps)));
                matcher.insert(opt.name);
            } else if let Some(flg) = self.flags
                                       .iter()
                                       .filter(|f| f.long.is_some() && f.long.unwrap() == name)
                                       .next() {
                self.groups_for_arg(flg.name).and_then(|grps| Some(matcher.inc_occurrences_of(&grps)));
                matcher.insert(flg.name);
            }
        }

        Err(error_builder::InvalidArgument(&*format!("--{}", arg),
                                        Some(&*suffix.0),
                                        &*try!(self.create_current_usage(matcher))))
    }
}
