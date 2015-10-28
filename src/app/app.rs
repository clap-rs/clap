use std::collections::{BTreeMap, HashMap, VecDeque};
use std::env;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::process;
use std::error::Error;
use std::ffi::OsStr;
use std::borrow::Borrow;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

use args::{ArgMatches, Arg, SubCommand, MatchedArg};
use args::{FlagBuilder, OptBuilder, PosBuilder};
use args::settings::{ArgSettings, ArgFlags};
use args::ArgGroup;
use fmt::Format;
use super::settings::{AppSettings, AppFlags};

use super::suggestions::{DidYouMeanMessageStyle, did_you_mean};
use super::errors::{ClapErrorType, ClapError};


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
    flags: BTreeMap<&'ar str, FlagBuilder<'ar>>,
    // A list of possible options
    opts: BTreeMap<&'ar str, OptBuilder<'ar>>,
    // A list of positional arguments
    positionals_idx: BTreeMap<u8, PosBuilder<'ar>>,
    positionals_name: HashMap<&'ar str, u8>,
    // A list of subcommands
    subcommands: BTreeMap<String, App<'a, 'v, 'ab, 'u, 'h, 'ar>>,
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

impl<'a, 'v, 'ab, 'u, 'h, 'ar> App<'a, 'v, 'ab, 'u, 'h, 'ar>{
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
            flags: BTreeMap::new(),
            opts: BTreeMap::new(),
            positionals_idx: BTreeMap::new(),
            positionals_name: HashMap::new(),
            subcommands: BTreeMap::new(),
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
    pub fn author(mut self,
                  a: &'a str)
                  -> Self {
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
    pub fn bin_name(mut self,
                    a: &str)
                    -> Self {
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
    pub fn about(mut self,
                 a: &'ab str)
                 -> Self {
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
    pub fn after_help(mut self,
                      h: &'h str)
                      -> Self {
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
    pub fn subcommands_negate_reqs(mut self,
                                   n: bool)
                                   -> Self {
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
    pub fn subcommand_required(mut self,
                               n: bool)
                               -> Self {
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
    pub fn version(mut self,
                   v: &'v str)
                   -> Self {
        self.version = Some(v);
        self
    }

    /// Sets a custom usage string to override the auto-generated usage string.
    ///
    /// This will be displayed to the user when errors are found in argument parsing, or when you
    /// call `ArgMatches::usage()`
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
    pub fn usage(mut self,
                 u: &'u str)
                 -> Self {
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
    pub fn help(mut self,
                h: &'u str)
                -> Self {
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
    pub fn help_short(mut self,
                      s: &str)
                      -> Self {
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
    pub fn version_short(mut self,
                         s: &str)
                         -> Self {
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
    pub fn arg_required_else_help(mut self,
                                  tf: bool)
                                  -> Self {
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
    pub fn hidden(mut self,
                  h: bool)
                  -> Self {
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
    pub fn global_version(mut self,
                          gv: bool)
                          -> Self {
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
    pub fn versionless_subcommands(mut self,
                                   vers: bool)
                                   -> Self {
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
    pub fn unified_help_message(mut self,
                                uni_help: bool)
                                -> Self {
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
    pub fn wait_on_error(mut self,
                         w: bool)
                         -> Self {
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
    pub fn subcommand_required_else_help(mut self,
                                         tf: bool)
                                         -> Self {
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
    pub fn setting(mut self,
                   setting: AppSettings)
                   -> Self {
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
    pub fn settings(mut self,
                    settings: &[AppSettings])
                    -> Self {
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
    pub fn arg(mut self,
               a: Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>)
               -> Self {
        self.add_arg(a);
        self
    }

    // actually adds the arguments
    fn add_arg(&mut self,
               a: Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>) {
        if self.flags.contains_key(a.name) || self.opts.contains_key(a.name) ||
           self.positionals_name.contains_key(a.name) {
            panic!("Argument name must be unique\n\n\t\"{}\" is already in use", a.name);
        }
        if let Some(grp) = a.group {
            let ag = self.groups.entry(grp).or_insert(ArgGroup::with_name(grp));
            ag.args.push(a.name);
        }
        if let Some(s) = a.short {
            if self.short_list.contains(&s) {
                panic!("Argument short must be unique\n\n\t-{} is already in use", s);
            } else {
                self.short_list.push(s);
            }
        }
        if let Some(l) = a.long {
            if self.long_list.contains(&l) {
                panic!("Argument long must be unique\n\n\t--{} is already in use", l);
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
                (self.positionals_idx.len() + 1) as u8
            } else {
                a.index.unwrap()
            };
            if self.positionals_idx.contains_key(&i) {
                panic!("Argument \"{}\" has the same index as another positional \
                    argument\n\n\tPerhaps try .multiple(true) to allow one positional argument \
                    to take multiple values", a.name);
            }
            let pb = PosBuilder::from_arg(&a, i, &mut self.required);
            self.positionals_name.insert(pb.name, i);
            self.positionals_idx.insert(i, pb);
        } else if a.takes_value {
            let ob = OptBuilder::from_arg(&a, &mut self.required);
            self.opts.insert(ob.name, ob);
        } else {
            let fb = FlagBuilder::from(&a);
            self.flags.insert(fb.name, fb);
        }
        if a.global {
            if a.required {
                panic!("Global arguments cannot be required.\n\n\t'{}' is marked as global and \
                        required", a.name);
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
    pub fn args(mut self,
                args: Vec<Arg<'ar, 'ar, 'ar, 'ar, 'ar, 'ar>>)
                -> Self {
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
    pub fn arg_from_usage(mut self,
                          usage: &'ar str)
                          -> Self {
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
    pub fn args_from_usage(mut self,
                           usage: &'ar str)
                           -> Self {
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
    pub fn arg_group(mut self,
                     group: ArgGroup<'ar, 'ar>)
                     -> Self {
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
    pub fn arg_groups(mut self,
                      groups: Vec<ArgGroup<'ar, 'ar>>)
                      -> Self {
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
    pub fn subcommand(mut self,
                      mut subcmd: App<'a, 'v, 'ab, 'u, 'h, 'ar>)
                      -> Self {
        if subcmd.name == "help" {
            self.settings.set(&AppSettings::NeedsSubcommandHelp);
        }
        if self.settings.is_set(&AppSettings::VersionlessSubcommands) {
            self.settings.set(&AppSettings::DisableVersion);
        }
        if self.settings.is_set(&AppSettings::GlobalVersion) &&
            subcmd.version.is_none() &&
            self.version.is_some() {
            subcmd.version = Some(self.version.unwrap());
        }
        self.subcommands.insert(subcmd.name.clone(), subcmd);
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
    pub fn subcommands(mut self,
                       subcmds: Vec<App<'a, 'v, 'ab, 'u, 'h, 'ar>>)
                       -> Self {
        for subcmd in subcmds.into_iter() {
            self = self.subcommand(subcmd);
        }
        self
    }

    fn groups_for(&self, name: &str) -> Option<Vec<&'ar str>> {
        if self.groups.is_empty() { return None; }
        let mut res = vec![];
        for (g_name, grp) in &self.groups {
            for a in &grp.args {
                if a == &name {
                    res.push(*g_name);
                }
            }
        }
        if res.is_empty() { return None }

        Some(res)
    }

    fn get_group_members(&self,
                         group: &str)
                         -> Vec<String> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.groups.get(group).unwrap().args {
            if let Some(f) = self.flags.get(n) {
                args.push(f.to_string());
            } else if let Some(f) = self.opts.get(n) {
                args.push(f.to_string());
            } else if self.groups.contains_key(n) {
                g_vec.push(*n);
            } else {
                if let Some(idx) = self.positionals_name.get(n) {
                    if let Some(p) = self.positionals_idx.get(&idx) {
                        args.push(p.to_string());
                    }
                }
            }
        }

        if !g_vec.is_empty() {
            for av in g_vec.iter().map(|g| self.get_group_members(g)) {
                for a in av {
                    args.push(a);
                }
            }
        }
        assert!(args.len() > 0, "ArgGroup '{}' doesn't contain any args", group);
        args.dedup();
        args.iter().map(ToOwned::to_owned).collect()
    }

    fn get_group_members_names(&self,
                               group: &'ar str)
                               -> Vec<&'ar str> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.groups.get(group).unwrap().args {
            if self.flags.contains_key(n) {
                args.push(*n);
            } else if self.opts.contains_key(n) {
                args.push(*n);
            } else if self.groups.contains_key(n) {
                g_vec.push(*n);
            } else {
                if self.positionals_name.contains_key(n) {
                    args.push(*n);
                }
            }
        }

        if !g_vec.is_empty() {
            for av in g_vec.iter().map(|g| self.get_group_members_names(g)) {
                for a in av {
                    args.push(a);
                }
            }
        }
        assert!(args.len() > 0, "ArgGroup '{}' doesn't contain any args", group);
        args.dedup();
        args.iter().map(|s| *s).collect()
    }

    fn get_required_from(&self,
                         reqs: Vec<&'ar str>,
                         matches: Option<&ArgMatches>)
                         -> VecDeque<String> {
        let mut c_flags = vec![];
        let mut c_pos = vec![];
        let mut c_opt = vec![];
        let mut grps = vec![];
        for name in &reqs {
            if self.flags.contains_key(name) {
                c_flags.push(name);
            } else if self.opts.contains_key(name) {
                c_opt.push(name);
            } else if self.groups.contains_key(name) {
                grps.push(*name);
            } else {
                c_pos.push(name);
            }
        }
        let mut tmp_f = vec![];
        for f in &c_flags {
            if let Some(f) = self.flags.get(*f) {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                tmp_f.push(r);
                            } else if self.opts.contains_key(r) {
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
            if let Some(f) = self.opts.get(*f) {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                c_flags.push(r);
                            } else if self.opts.contains_key(r) {
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
        for f in c_pos.iter() {
            if let Some(f) = self.flags.get(*f) {
                if let Some(ref rl) = f.requires {
                    for r in rl.iter() {
                        if !reqs.contains(r) {
                            if self.flags.contains_key(r) {
                                c_flags.push(r);
                            } else if self.opts.contains_key(r) {
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
            c_flags.push(f);
        }


        let mut ret_val = VecDeque::new();

        let mut pmap = BTreeMap::new();
        for p in c_pos.into_iter() {
            if matches.is_some() && matches.as_ref().unwrap().is_present(p) { continue }
            if let Some(idx) = self.positionals_name.get(p) {
                if let Some(ref p) = self.positionals_idx.get(&idx) {
                    pmap.insert(p.index, format!("{}", p));
                }
            }
        }
        pmap.into_iter().map(|(_, s)| ret_val.push_back(s)).collect::<Vec<_>>();
        for f in c_flags.into_iter() {
            if matches.is_some() && matches.as_ref().unwrap().is_present(f) { continue }
            ret_val.push_back(format!("{}", self.flags.get(*f).unwrap()));
        }
        for o in c_opt.into_iter() {
            if matches.is_some() && matches.as_ref().unwrap().is_present(o) { continue }
            ret_val.push_back(format!("{}", self.opts.get(*o).unwrap()));
        }
        for g in grps.into_iter() {
            let g_string = self.get_group_members(g).iter()
                                                    .fold(String::new(), |acc, s| {
                                                        acc + &format!(" {} |",s)[..]
                                                    });
            ret_val.push_back(format!("[{}]", &g_string[..g_string.len()-1]));
        }

        ret_val
    }

    // Creates a usage string if one was not provided by the user manually. This happens just
    // after all arguments were parsed, but before any subcommands have been parsed (so as to
    // give subcommands their own usage recursively)
    fn create_usage(&self,
                    matches: &[&'ar str])
                    -> String {
        use std::fmt::Write;
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n\t");
        if let Some(u) = self.usage_str {
            usage.push_str(u);
        } else if !matches.is_empty() {
            let mut hs: Vec<&str> = self.required.iter().map(|n| *n).collect();
            for n in matches {
                hs.push(*n);
            }
            let reqs = self.get_required_from(hs, None);

            let r_string = reqs.iter().fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

            write!(&mut usage, "{}{}",
                self.usage.clone().unwrap_or(self.bin_name.clone().unwrap_or(self.name.clone())),
                r_string
            ).ok().expect(INTERNAL_ERROR_MSG);
            if self.settings.is_set(&AppSettings::SubcommandRequired) {
                write!(&mut usage, " <SUBCOMMAND>").ok().expect(INTERNAL_ERROR_MSG);
            }
        } else {
            usage.push_str(&*self.usage.clone()
                                       .unwrap_or(self.bin_name.clone()
                                                               .unwrap_or(self.name.clone())));

            let mut reqs = self.required.iter().map(|n| *n).collect::<Vec<_>>();
            // If it's required we also need to ensure all previous positionals are required too
            let mut found = false;
            for p in self.positionals_idx.values().rev() {
                if found {
                    reqs.push(p.name);
                    continue;
                }
                if p.settings.is_set(&ArgSettings::Required) {
                    found = true;
                    reqs.push(p.name);
                }
            }
            let req_strings = self.get_required_from(reqs, None);
            let req_string = req_strings.iter()
                                        .fold(String::new(), |acc, s| {
                                            acc + &format!(" {}", s)[..]
                                        });

            if !self.flags.is_empty() && !self.settings.is_set(&AppSettings::UnifiedHelpMessage) {
                usage.push_str(" [FLAGS]");
            } else {
                usage.push_str(" [OPTIONS]");
            }
            if !self.settings.is_set(&AppSettings::UnifiedHelpMessage) && !self.opts.is_empty() &&
               self.opts.values().any(|a| !a.settings.is_set(&ArgSettings::Required)) {
                usage.push_str(" [OPTIONS]");
            }

            usage.push_str(&req_string[..]);

            // places a '--' in the usage string if there are args and options
            // supporting multiple values
            if !self.positionals_idx.is_empty() &&
                (self.opts.values().any(|a| a.settings.is_set(&ArgSettings::Multiple)) ||
                    self.positionals_idx.values().any(|a| a.settings.is_set(&ArgSettings::Multiple))) &&
                !self.opts.values().any(|a| a.settings.is_set(&ArgSettings::Required)) &&
                self.subcommands.is_empty() {
                usage.push_str(" [--]")
            }
            if !self.positionals_idx.is_empty() &&
               self.positionals_idx.values()
                                                                       .any(|a| !a.settings.is_set(&ArgSettings::Required)) {
                usage.push_str(" [ARGS]");
            }


            if !self.subcommands.is_empty() && !self.settings.is_set(&AppSettings::SubcommandRequired) {
                usage.push_str(" [SUBCOMMAND]");
            } else if self.settings.is_set(&AppSettings::SubcommandRequired) &&
                !self.subcommands.is_empty() {
                usage.push_str(" <SUBCOMMAND>");
            }
        }

        usage.shrink_to_fit();
        usage
    }

    /// Prints the full help message to `io::stdout()` using a `BufWriter`
    ///
    /// # Exampless
    /// ```no_run
    /// # use clap::App;
    /// # use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_help(&mut out).ok().expect("failed to write to stdout");
    /// ```
    pub fn print_help(&self) -> io::Result<()> {
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
    pub fn write_help<W: Write>(&self,
                            w: &mut W)
                            -> io::Result<()> {
        if let Some(h) = self.help_str {
            return writeln!(w, "{}", h)
        }

        // Print the version
        try!(write!(w, "{} {}\n", &self.bin_name.clone().unwrap_or(
            self.name.clone())[..].replace(" ", "-"),
            self.version.unwrap_or("")
        ));
        let flags = !self.flags.is_empty();
        let pos = !self.positionals_idx.is_empty();
        let opts = !self.opts.is_empty();
        let subcmds = !self.subcommands.is_empty();
        let unified_help = self.settings.is_set(&AppSettings::UnifiedHelpMessage);

        let mut longest_flag = 0;
        for fl in self.flags
            .values()
            .filter(|f| f.long.is_some() && !f.settings.is_set(&ArgSettings::Hidden))
            // 2='--'
            .map(|a| a.to_string().len() ) {
            if fl > longest_flag {
                longest_flag = fl;
            }
        }
        let mut longest_opt = 0;
        for ol in self.opts
            .values()
            .filter(|o| !o.settings.is_set(&ArgSettings::Hidden))
            .map(|a|
                a.to_string().len()
            ) {
            if ol > longest_opt {
                longest_opt = ol;
            }
        }
        let mut longest_pos = 0;
        for pl in self.positionals_idx
            .values()
            .filter(|p| !p.settings.is_set(&ArgSettings::Hidden))
            .map(|f| f.to_string().len() ) {
            if pl > longest_pos {
                longest_pos = pl;
            }
        }
        let mut longest_sc = 0;
        for scl in self.subcommands
            .values()
            .filter(|s| !s.settings.is_set(&AppSettings::Hidden))
            .map(|f| f.name.len() ) {
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

        try!(write!(w, "\n{}", self.create_usage(&[])));

        if flags || opts || pos || subcmds {
            try!(write!(w, "\n"));
        }

        let tab = "    ";
        if unified_help && (flags || opts) {
            try!(write!(w, "\nOPTIONS:\n"));
            let mut combined = vec![];
            for f in self.flags.values().filter(|f| !f.settings.is_set(&ArgSettings::Hidden)) {
                combined.push(f.name);
            }
            for o in self.opts.values().filter(|o| !o.settings.is_set(&ArgSettings::Hidden)) {
                combined.push(o.name);
            }
            combined.sort();
            for a in combined {
                if let Some(a) = self.flags.get(a) {
                    try!(a.write_help(w, tab, if !unified_help || longest_opt == 0 { longest_flag } else { longest_opt }));
                } else if let Some(a) = self.opts.get(a) {
                    try!(a.write_help(w, tab, if !unified_help || longest_opt == 0 { longest_flag } else { longest_opt }));
                }
            }
        } else {
            if flags {
                try!(write!(w, "\nFLAGS:\n"));
                for v in self.flags.values()
                                   .filter(|f| !f.settings.is_set(&ArgSettings::Hidden)) {
                    try!(v.write_help(w, tab, if !unified_help || longest_opt == 0 { longest_flag } else { longest_opt }));
                }
            }
            if opts {
                try!(write!(w, "\nOPTIONS:\n"));
                for v in self.opts
                             .values()
                             .filter(|o| !o.settings.is_set(&ArgSettings::Hidden)) {
                    try!(v.write_help(w, tab, longest_opt));
                }
            }
        }
        if pos {
            try!(write!(w, "\nARGS:\n"));
            for v in self.positionals_idx
                         .values()
                         .filter(|p| !p.settings.is_set(&ArgSettings::Hidden)) {
                try!(v.write_help(w, tab, longest_pos));
            }
        }
        if subcmds {
            try!(write!(w, "\nSUBCOMMANDS:\n"));
            for sc in self.subcommands
                          .values()
                          .filter(|s| !s.settings.is_set(&AppSettings::Hidden)) {
                try!(write!(w, "{}{}", tab, sc.name));
                write_spaces!((longest_sc + 4) - (sc.name.len()), w);
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
        w.flush()
    }

    // Prints the version to the user and exits if quit=true
    fn print_version<W: Write>(&self,
                               w: &mut W)
                               -> io::Result<()> {
        // Print the binary name if existing, but replace all spaces with hyphens in case we're
        // dealing with subcommands i.e. git mv is translated to git-mv
        try!(writeln!(w, "{} {}", &self.bin_name.clone().unwrap_or(
            self.name.clone())[..].replace(" ", "-"),
            self.version.unwrap_or("")
        ));

        w.flush()
    }

    // Reports and error to stderr along with an optional usage statement and optionally quits
    fn create_error<S: AsRef<str>>(&self,
                    data: &[S],
                    error_type: ClapErrorType,
                    matches: &ArgMatches)
                    -> ClapError {
        let msg = match error_type {
            ClapErrorType::ArgumentError => {
                assert_eq!(data.len(), 1);
                format!("{}", data[0].as_ref())
            },
            ClapErrorType::InvalidValue => {
                assert_eq!(data.len(), 4);
                format!("'{}' isn't a valid value for '{}'{}{}",
                    Format::Warning(data[0].as_ref()),
                    Format::Warning(data[1].as_ref()),
                    format!("\n\t[valid values:{}]\n", data[2].as_ref()),
                    data[3].as_ref())
            },
            ClapErrorType::InvalidArgument => {
                assert_eq!(data.len(), 2);
                format!("The argument '{}' isn't valid{}",
                    Format::Warning(data[0].as_ref()),
                    data[1].as_ref())
            },
            ClapErrorType::InvalidSubcommand => {
                assert_eq!(data.len(), 2);
                format!("The subcommand '{}' isn't valid\n\tDid you mean '{}' ?\n\n\
                         If you received this message in error, try \
                         re-running with '{} {} {}'",
                    Format::Warning(data[0].as_ref()),
                    Format::Good(data[1].as_ref()),
                    &*self.bin_name.as_ref().unwrap_or(&self.name),
                    Format::Good("--"),
                    data[0].as_ref())
            },
            ClapErrorType::EmptyValue => {
                assert_eq!(data.len(), 1);
                format!("The argument '{}' requires a value but none was supplied",
                    Format::Warning(data[0].as_ref()))
            },
            ClapErrorType::ValueValidationError => {
                assert_eq!(data.len(), 1);
                data[0].as_ref().to_owned()
            },
            ClapErrorType::TooManyArgs => {
                assert_eq!(data.len(), 2);
                format!("The argument '{}' was found, but '{}' wasn't expecting any more values",
                    Format::Warning(data[0].as_ref()),
                    Format::Warning(data[1].as_ref()))
            },
            ClapErrorType::TooFewValues => {
                assert_eq!(data.len(), 4);
                format!("The argument '{}' requires at least {} values, but {} w{} provided",
                    Format::Warning(data[0].as_ref()),
                    Format::Good(data[1].as_ref()),
                    Format::Error(data[2].as_ref()),
                    data[3].as_ref())
            },
            ClapErrorType::TooManyValues => {
                assert_eq!(data.len(), 4);
                format!("The argument '{}' only requires {} values, but {} w{} provided",
                    Format::Warning(data[0].as_ref()),
                    Format::Good(data[1].as_ref()),
                    Format::Error(data[2].as_ref()),
                    data[3].as_ref())
            },
            ClapErrorType::WrongNumValues => {
                assert_eq!(data.len(), 4);
                format!("The argument '{}' requires {} values, but {} w{} provided",
                    Format::Warning(data[0].as_ref()),
                    Format::Good(data[1].as_ref()),
                    Format::Error(data[2].as_ref()),
                    data[3].as_ref())
            },
            ClapErrorType::ArgumentConflict => {
                assert_eq!(data.len(), 2);
                format!("The argument '{}' cannot be used with {}",
                    Format::Warning(data[0].as_ref()),
                    match self.blacklisted_from(data[1].as_ref(), &matches) {
                        Some(name) => format!("'{}'", Format::Warning(name)),
                        None       => "one or more of the other specified \
                                       arguments".to_owned()
                    })
            },
            ClapErrorType::MissingRequiredArgument => {
                // Callers still use &[""]
                assert_eq!(data.len(), 1);
                format!("The following required arguments were not supplied:{}",
                        self.get_required_from(self.required.iter()
                                                            .map(|s| *s)
                                                            .collect(),
                                                            Some(matches))
                            .iter()
                            .fold(String::new(), |acc, s| acc + &format!("\n\t{}",
                                Format::Error(s))[..]))
            },
            ClapErrorType::MissingSubcommand => {
                assert_eq!(data.len(), 1);
                format!("'{}' requires a subcommand but none was provided",
                        Format::Warning(data[0].as_ref()))
            },
            ClapErrorType::MissingArgumentOrSubcommand => "".to_owned(),
            ClapErrorType::UnexpectedArgument => {
                assert_eq!(data.len(), 1);
                format!("Found argument '{}', but {} wasn't expecting any",
                    Format::Warning(data[0].as_ref()),
                    self.bin_name.as_ref().unwrap_or(&self.name))
            },
            ClapErrorType::UnexpectedMultipleUsage => {
                assert_eq!(data.len(), 1);
                format!("The argument '{}' was supplied more \
                        than once, but does not support multiple values",
                            Format::Warning(data[0].as_ref()))
            },
            ClapErrorType::InvalidUnicode => {
                // Callers still use &[""]
                assert_eq!(data.len(), 1);
                "Invalid unicode character in one or more arguments".to_owned()
            },
            // HelpDisplayed, VersionDisplayed
            _ => unreachable!()
        };

        ClapError {
            error: format!("{} {}\n\n{}\n\nFor more information try {}",
                            Format::Error("error:"),
                            msg,
                            self.create_usage(
                            &matches.args.keys()
                                        .map(|k| *k)
                                        .filter(|k| {
                                            if let Some(o) = self.opts.get(k) {
                                                !o.settings.is_set(&ArgSettings::Required)
                                            } else if let Some(p) = self.positionals_name.get(k) {
                                                if let Some(p) = self.positionals_idx.get(p) {
                                                    !p.settings.is_set(&ArgSettings::Required)
                                                } else {
                                                    true
                                                }
                                            } else {
                                                true
                                            }
                                        })
                                        .collect::<Vec<_>>()),
                            Format::Good("--help")
                        ),
            error_type: error_type,
        }
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
    /// # Exampless
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
    pub fn get_matches_safe(self) -> Result<ArgMatches<'ar, 'ar>, ClapError> {
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
    pub fn get_matches_safe_lossy(self) -> Result<ArgMatches<'ar, 'ar>, ClapError> {
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
    pub fn get_matches_from<I, T>(mut self,
                                  itr: I)
                                  -> ArgMatches<'ar, 'ar>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr> {
        match self.get_matches_from_safe_borrow(itr) {
            Ok(m) => return m,
            Err(e) => {
                match e.error_type {
                    ClapErrorType::HelpDisplayed | ClapErrorType::VersionDisplayed => e.exit(),
                    _ => ()
                }
                wlnerr!("{}", e.error);
                if self.settings.is_set(&AppSettings::WaitOnError) {
                    wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                    let mut s = String::new();
                    let i = io::stdin();
                    i.lock().read_line(&mut s).unwrap();
                }
                process::exit(1);
            }
        }
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
    pub fn get_matches_from_lossy<I, T>(mut self,
                                  itr: I)
                                  -> ArgMatches<'ar, 'ar>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr> {
        match self.get_matches_from_safe_borrow_lossy(itr) {
            Ok(m) => return m,
            Err(e) => {
                match e.error_type {
                    ClapErrorType::HelpDisplayed | ClapErrorType::VersionDisplayed => e.exit(),
                    _ => ()
                }
                wlnerr!("{}", e.error);
                if self.settings.is_set(&AppSettings::WaitOnError) {
                    wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                    let mut s = String::new();
                    let i = io::stdin();
                    i.lock().read_line(&mut s).unwrap();
                }
                process::exit(1);
            }
        }
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
    pub fn get_matches_from_safe<I, T>(mut self,
                                       itr: I)
                                       -> Result<ArgMatches<'ar, 'ar>, ClapError>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr> {
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
    pub fn get_matches_from_safe_lossy<I, T>(mut self,
                                       itr: I)
                                       -> Result<ArgMatches<'ar, 'ar>, ClapError>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr> {
        self._get_matches_from_safe_borrow(itr, true)
    }

    fn _get_matches_from_safe_borrow<I, T>(&mut self,
                                              itr: I,
                                              lossy: bool)
                                              -> Result<ArgMatches<'ar, 'ar>, ClapError>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr> {
        // Verify all positional assertions pass
        self.verify_positionals();
        // If there are global arguments, we need to propgate them down to subcommands before
        // parsing incase we run into a subcommand
        self.propogate_globals();

        let mut matches = ArgMatches::new();

        let mut it = itr.into_iter();
        // Get the name of the program (argument 1 of env::args()) and determine the actual file
        // that was used to execute the program. This is because a program called
        // ./target/release/my_prog -a
        // will have two arguments, './target/release/my_prog', '-a' but we don't want to display
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
        if let Err(e) = self.get_matches_with(&mut matches, &mut it, lossy) {
            return Err(e);
        }

        Ok(matches)
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
    pub fn get_matches_from_safe_borrow<I, T>(&mut self,
                                              itr: I)
                                              -> Result<ArgMatches<'ar, 'ar>, ClapError>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr> {
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
                                              -> Result<ArgMatches<'ar, 'ar>, ClapError>
        where I: IntoIterator<Item = T>,
              T: AsRef<OsStr> {
        self._get_matches_from_safe_borrow(itr, true)
    }


    fn verify_positionals(&mut self) {
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // Firt we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)
        //
        // Next we verify that only the highest index has a .multiple(true) (if any)
        if let Some((idx, ref p)) = self.positionals_idx.iter().rev().next() {
            if *idx as usize != self.positionals_idx.len() {
                panic!("Found positional argument \"{}\" who's index is {} but there are only {} \
                    positional arguments defined", p.name, idx, self.positionals_idx.len());
            }
        }
        if let Some(ref p) = self.positionals_idx.values()
                                                 .filter(|a| a.settings.is_set(&ArgSettings::Multiple))
                                                 .filter(|a| {
                                                    a.index as usize != self.positionals_idx.len()
                                                })
                                                 .next() {
            panic!("Found positional argument \"{}\" which accepts multiple values but it's not \
                the last positional argument (i.e. others have a higher index)", p.name);
        }

        // If it's required we also need to ensure all previous positionals are required too
        let mut found = false;
        for (_, p) in self.positionals_idx.iter_mut().rev() {
            if found {
                p.settings.set(&ArgSettings::Required);
                self.required.push(p.name);
                continue;
            }
            if p.settings.is_set(&ArgSettings::Required) {
                found = true;
            }
        }
    }

    fn propogate_globals(&mut self) {
        for (_,sc) in self.subcommands.iter_mut() {
            // We have to create a new scope in order to tell rustc the borrow of `sc` is done and
            // to recursively call this method
            {
                for a in self.global_args.iter() {
                    sc.add_arg(a.into());
                }
            }
            sc.propogate_globals();
        }
    }


    fn possible_values_error(&self,
                             arg: &str,
                             opt: &str,
                             p_vals: &[&str],
                             matches: &ArgMatches<'ar, 'ar>)
                             -> ClapError {
        let suffix = App::did_you_mean_suffix(arg,
                                              p_vals.iter(),
                                              DidYouMeanMessageStyle::EnumValue);

        let mut sorted = vec![];
        for v in p_vals {
            sorted.push(v.clone());
        }
        sorted.sort();
        let valid_values = sorted.iter()
                                 .fold(String::new(), |acc, name| {
                                     acc + &format!(" {}",name)[..]
                                 });
        return self.create_error(
            &[arg, opt, &*valid_values, &*suffix.0],
            ClapErrorType::InvalidValue,
            matches);
    }

    // The actual parsing function
    fn get_matches_with<I, T>(&mut self,
                              matches: &mut ArgMatches<'ar, 'ar>,
                              it: &mut I,
                              lossy: bool)
                              -> Result<(), ClapError>
        where I: Iterator<Item = T>,
              T: AsRef<OsStr> {
        // First we create the `--help` and `--version` arguments and add them if necessary
        self.create_help_and_version();

        let mut pos_only = false;
        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: Option<&str> = None;
        let mut pos_counter = 1;
        let mut val_counter = 0;
        while let Some(arg) = it.next() {
            let arg_cow = match arg.as_ref().to_str() {
                Some(s) => s.into(),
                None    => {
                    if !lossy {
                        return Err(self.create_error(&[""], ClapErrorType::InvalidUnicode, matches));
                    }
                    arg.as_ref().to_string_lossy()
                }
            };
            let arg_slice: &str = arg_cow.borrow();
            let mut skip = false;

            // we need to know if we're parsing a new argument, or the value of previous argument,
            // perhaps one with multiple values such as --option val1 val2. We determine when to
            // stop parsing multiple values by finding a '-'
            let new_arg = if arg_slice.starts_with("-") {
                // If we come to a single `-` it's a value, not a new argument...this happens when
                // one wants to use the Unix standard of '-' to mean 'stdin'
                !(arg_slice.len() == 1)
            } else {
                false
            };

            // pos_only is determined later, and set true when a user uses the Unix standard of
            // '--' to mean only positionals follow
            if !pos_only && !new_arg && !self.subcommands.contains_key(arg_slice) {
                // Check to see if parsing a value from an option
                if let Some(nvo) = needs_val_of {
                    // get the OptBuilder so we can check the settings
                    if let Some(ref opt) = self.opts.get(nvo) {
                        try!(self.validate_option(opt, arg_slice, matches));

                        if let Some(ref vec) = self.groups_for(opt.name) {
                            for grp in vec {
                                if let Some(ref mut o) = matches.args.get_mut(grp) {
                                    o.occurrences = if opt.settings.is_set(&ArgSettings::Multiple) {
                                        o.occurrences + 1
                                    } else {
                                        1
                                    };
                                    // Values must be inserted in order...the user may care about that!
                                    if let Some(ref mut vals) = o.values {
                                        let len = vals.len() as u8 + 1;
                                        vals.insert(len, arg_slice.to_owned());
                                    }
                                }

                            }
                        }
                        // save the value to matched option
                        if let Some(ref mut o) = matches.args.get_mut(opt.name) {
                            // if it's multiple; the occurrences are increased when originally
                            // found
                            o.occurrences = if opt.settings.is_set(&ArgSettings::Multiple) {
                                o.occurrences + 1
                            } else {
                                skip = true;
                                1
                            };

                            // Options always have values, even if it's empty, so we can unwrap()
                            if let Some(ref mut vals) = o.values {

                                // Values must be inserted in order...the user may care about that!
                                let len = vals.len() as u8 + 1;
                                vals.insert(len, arg_slice.to_owned());

                                // Now that the values have been added, we can ensure we haven't
                                // gone over any max_limits, or if we've reached the exact number
                                // of values we can stop parsing values, and go back to arguments.
                                //
                                // For example, if we define an option with exactly 2 values and
                                // the users passes:
                                // $ my_prog --option val1 val2 pos1
                                // we stop parsing values of --option after val2, if the user
                                // hadn't defined an exact or max value, pos1 would be parsed as a
                                // value of --option
                                if let Some(num) = opt.max_vals {
                                    if len != num {
                                        continue
                                    }
                                } else if let Some(num) = opt.num_vals {
                                    if opt.settings.is_set(&ArgSettings::Multiple) {
                                        val_counter += 1;
                                        if val_counter != num {
                                            continue
                                        } else {
                                            val_counter = 0;
                                        }
                                    } else {
                                        // if we need more values, get the next value
                                        if len != num {
                                            continue
                                        }
                                    }
                                } else if !skip {
                                    // get the next value from the iterator
                                    continue
                                }
                            }
                        }
                        skip = true;
                    }
                }
            }

            // if we're done getting values from an option, get the next arg from the iterator
            if skip {
                needs_val_of = None;
                continue;
            } else if let Some(ref name) = needs_val_of {
                // We've reached more values for an option than it possibly accepts
                if let Some(ref o) = self.opts.get(name) {
                    if !o.settings.is_set(&ArgSettings::Multiple) {
                        return Err(
                            self.create_error(&[&*o.to_string()],
                                ClapErrorType::EmptyValue,
                                matches));
                    }
                }
            }

            if arg_slice.starts_with("--") && !pos_only {
                if arg_slice.len() == 2 {
                    // The user has passed '--' which means only positional args follow no matter
                    // what they start with
                    pos_only = true;
                    continue;
                }

                // This arg is either an option or flag using a long (i.e. '--something')
                match self.parse_long_arg(matches, arg_slice) {
                    Ok(r) => needs_val_of = r,
                    Err(e) => return Err(e),
                }
            } else if arg_slice.starts_with("-") && arg_slice.len() != 1 && !pos_only {
                // Multiple or single flag(s), or single option (could be '-SbG' or '-o')
                match self.parse_short_arg(matches, arg_slice) {
                    Ok(r) => needs_val_of = r,
                    Err(e) => return Err(e),
                }
            } else {
                // Positional or Subcommand
                //
                // If the user pased `--` we don't check for subcommands, because the argument they
                // may be trying to pass might match a subcommand name
                if !pos_only {
                    if self.subcommands.contains_key(arg_slice) {
                        if arg_slice == "help" && self.settings.is_set(&AppSettings::NeedsSubcommandHelp) {
                            if let Err(e) = self.print_help() {
                                return Err(ClapError{
                                    error: format!("{} {}\n\terror message: {}\n",
                                                    Format::Error("error:"),
                                                    INTERNAL_ERROR_MSG, e.description()),
                                    error_type: ClapErrorType::MissingSubcommand
                                });
                            }
                            // process::exit(0);
                            return Err(ClapError{
                                error: String::new(),
                                error_type: ClapErrorType::HelpDisplayed
                            })
                        }
                        subcmd_name = Some(arg_slice.to_owned());
                        break;
                    } else if let Some(candidate_subcommand) = did_you_mean(arg_slice,
                                                                     self.subcommands.keys()) {
                        return Err(self.create_error(
                                &[arg_slice, candidate_subcommand],
                                ClapErrorType::InvalidSubcommand,
                                matches));
                    }
                }

                // Did the developer even define any valid positionals? Since we reached this far,
                // it's not a subcommand
                if self.positionals_idx.is_empty() {
                    return Err(
                        self.create_error(
                            &[arg_slice],
                            ClapErrorType::UnexpectedArgument,
                            matches));
                } else if let Some(p) = self.positionals_idx.get(&pos_counter) {
                    // Make sure this one doesn't conflict with anything
                    if self.blacklist.contains(&p.name) {
                        return Err(
                            self.create_error(
                                &[&*p.to_string(), p.name],
                                ClapErrorType::ArgumentConflict,
                                matches));
                    }

                    if let Some(ref p_vals) = p.possible_vals {
                        if !p_vals.contains(&arg_slice) {
                            return Err(self.possible_values_error(arg_slice, &p.to_string(),
                                                                   p_vals, matches));
                        }
                    }

                    // Have we made the update yet?
                    let mut done = false;
                    if p.settings.is_set(&ArgSettings::Multiple) {
                        if let Some(num) = p.num_vals {
                            if let Some(ref ma) = matches.args.get(p.name) {
                                if let Some(ref vals) = ma.values {
                                    if vals.len() as u8 == num {
                                        return Err(
                                            self.create_error(
                                                &[arg_slice, &*p.to_string()],
                                                ClapErrorType::TooManyArgs,
                                                matches));
                                    }
                                }
                            }
                        }
                        // Check if it's already existing and update if so...
                        if let Some(ref mut pos) = matches.args.get_mut(p.name) {
                            done = true;
                            pos.occurrences += 1;
                            if let Some(ref mut vals) = pos.values {
                                let len = (vals.len() + 1) as u8;
                                vals.insert(len, arg_slice.to_owned());
                            }
                        }

                        if !pos_only && (self.settings.is_set(&AppSettings::TrailingVarArg) &&
                            pos_counter == self.positionals_idx.len() as u8) {
                                pos_only = true;
                        }
                    } else {
                        // Only increment the positional counter if it doesn't allow multiples
                        pos_counter += 1;
                    }
                    // Was an update made, or is this the first occurrence?
                    if !done {
                        if self.overrides.contains(&p.name) {
                            if let Some(name) = self.overriden_from(p.name, matches) {
                                matches.args.remove(&*name);
                                remove_overriden!(self, &*name);
                            }
                        }
                        if let Some(ref or) = p.overrides {
                            for pa in or {
                                matches.args.remove(pa);
                                remove_overriden!(self, pa);
                                self.overrides.push(pa);
                            }
                        }
                        let mut bm = BTreeMap::new();
                        if let Some(ref vtor) = p.validator {
                            let f = &*vtor;
                            if let Err(ref e) = f(arg_slice.to_owned()) {
                                return Err(
                                    self.create_error(
                                        &[e],
                                        ClapErrorType::ValueValidationError,
                                        matches));
                            }
                        }
                        bm.insert(1, arg_slice.to_owned());
                        if let Some(ref vec) = self.groups_for(p.name) {
                            for grp in vec {
                                matches.args.insert(grp, MatchedArg{
                                    occurrences: 1,
                                    values: Some(bm.clone()),
                                });
                            }
                        }
                        matches.args.insert(p.name, MatchedArg{
                            occurrences: 1,
                            values: Some(bm),
                        });

                        if let Some(ref bl) = p.blacklist {
                            for name in bl {
                                self.blacklist.push(name);
                            }
                        }

                        // Because of the macro call, we have to create a temp variable
                        if let Some(ref reqs) = p.requires {
                            // Add all required args which aren't already found in matches to the
                            // final required list
                            for n in reqs {
                                if matches.args.contains_key(n) {
                                    continue;
                                }

                                self.required.push(n);
                            }
                        }

                        if p.settings.is_set(&ArgSettings::Required) {
                            // for macro call
                            let name = &p.name;
                            vec_remove!(self.required, name);
                        }

                        parse_group_reqs!(self, p);
                    }
                } else {
                    return Err(
                        self.create_error(
                            &[arg_slice],
                            ClapErrorType::UnexpectedArgument,
                            matches));
                }
            }
        }
        if let Some(ref a) = needs_val_of {
            if let Some(o) = self.opts.get(a) {
                if o.settings.is_set(&ArgSettings::Multiple) && self.required.is_empty() {
                    let should_err = match matches.values_of(o.name) {
                        Some(ref v) => v.is_empty(),
                        None => true,
                    };
                    if should_err {
                        return Err(
                            self.create_error(
                                &[&*o.to_string()],
                                ClapErrorType::EmptyValue,
                                matches));
                    }
                } else if !o.settings.is_set(&ArgSettings::Multiple) {
                        return Err(
                            self.create_error(
                                &[&*o.to_string()],
                                ClapErrorType::EmptyValue,
                                matches));
                } else {
                    debugln!("Remaining Required Arg...");
                    debugln!("required={:#?}", self.required);
                    return Err(
                        self.create_error(
                            &[""],
                            ClapErrorType::MissingRequiredArgument,
                            matches));
                }
            } else {
                return Err(
                    self.create_error(
                        &[&*format!("{}", self.positionals_idx.get(
                            self.positionals_name.get(a).unwrap()).unwrap())],
                        ClapErrorType::EmptyValue,
                        matches));
            }
        }

        let res = self.validate_blacklist(matches);
        if res.is_err() {
            return res;
        }

        let res = self.validate_num_args(matches);
        if res.is_err() {
            return res;
        }

        matches.usage = Some(self.create_usage(&[]));

        if let Some(sc_name) = subcmd_name {
            use std::fmt::Write;
            let mut mid_string = String::new();
            if !self.settings.is_set(&AppSettings::SubcommandsNegateReqs) {
                let mut hs = self.required.iter().map(|n| *n).collect::<Vec<_>>();
                matches.args.keys().map(|k| hs.push(*k)).collect::<Vec<_>>();
                let reqs = self.get_required_from(hs, Some(matches));

                for s in reqs.iter() {
                    write!(&mut mid_string, " {}", s).ok().expect(INTERNAL_ERROR_MSG);
                }
            }
            mid_string.push_str(" ");
            if let Some(ref mut sc) = self.subcommands.get_mut(&sc_name) {
                let mut new_matches = ArgMatches::new();
                // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by a
                // space
                sc.usage = Some(format!("{}{}{}",
                    self.bin_name.clone().unwrap_or("".to_owned()),
                    if self.bin_name.is_some() {
                        mid_string
                    } else {
                        "".to_owned()
                    },
                    sc.name.clone()));
                sc.bin_name = Some(format!("{}{}{}",
                    self.bin_name.clone().unwrap_or("".to_owned()),
                    if self.bin_name.is_some() {
                        " "
                    } else {
                        ""
                    },
                    sc.name.clone()));
                if let Err(e) = sc.get_matches_with(&mut new_matches, it, lossy) {
                    e.exit();
                }
                matches.subcommand = Some(Box::new(SubCommand {
                    name: sc.name_slice,
                    matches: new_matches,
                }));
            }
        } else if self.settings.is_set(&AppSettings::SubcommandRequired) {
            let bn = self.bin_name.clone().unwrap_or(self.name.clone());
            return Err(
                self.create_error(
                    &[&*bn],
                    ClapErrorType::MissingSubcommand,
                    matches));
        } else if self.settings.is_set(&AppSettings::SubcommandRequiredElseHelp) {
            let mut out = vec![];
            match self.write_help(&mut out) {
                Ok(..) => return Err(ClapError{
                    error: String::from_utf8_lossy(&*out).into_owned(),
                    error_type: ClapErrorType::MissingSubcommand
                }),
                Err(e) => return Err(ClapError{
                    error: format!("{} {}\n\terror message: {}\n",
                                    Format::Error("error:"),
                                    INTERNAL_ERROR_MSG, e.description()),
                    error_type: ClapErrorType::MissingSubcommand
                }),
            }
        }
        if (!self.settings.is_set(&AppSettings::SubcommandsNegateReqs) || matches.subcommand_name().is_none()) &&
           self.validate_required(&matches) {
            return Err(
                self.create_error(
                    &[""],
                    ClapErrorType::MissingRequiredArgument,
                    matches));
        }
        if matches.args.is_empty() && matches.subcommand_name().is_none() &&
            self.settings.is_set(&AppSettings::ArgRequiredElseHelp) {
            let mut out = vec![];
            match self.write_help(&mut out) {
                Ok(..) => return Err(ClapError{
                    error: String::from_utf8_lossy(&*out).into_owned(),
                    error_type: ClapErrorType::MissingSubcommand
                }),
                Err(e) => return Err(ClapError{
                    error: format!("{} {}\n\terror message: {}\n",
                                    Format::Error("error:"),
                                    INTERNAL_ERROR_MSG, e.description()),
                    error_type: ClapErrorType::MissingSubcommand
                }),
            }
        }
        Ok(())
    }

    fn blacklisted_from(&self,
                        name: &str,
                        matches: &ArgMatches)
                        -> Option<String> {
        for k in matches.args.keys() {
            if let Some(f) = self.flags.get(k) {
                if let Some(ref bl) = f.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", f))
                    }
                }
            }
            if let Some(o) = self.opts.get(k) {
                if let Some(ref bl) = o.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", o))
                    }
                }
            }
            if let Some(idx) = self.positionals_name.get(k) {
                if let Some(pos) = self.positionals_idx.get(idx) {
                    if let Some(ref bl) = pos.blacklist {
                        if bl.contains(&name) {
                            return Some(format!("{}", pos))
                        }
                    }
                }
            }
        }
        None
    }

    fn overriden_from(&self,
                      name: &'ar str,
                      matches: &ArgMatches)
                      -> Option<&'ar str> {
        for k in matches.args.keys() {
            if let Some(f) = self.flags.get(k) {
                if let Some(ref bl) = f.overrides {
                    if bl.contains(&name) {
                        return Some(f.name)
                    }
                }
            }
            if let Some(o) = self.opts.get(k) {
                if let Some(ref bl) = o.overrides {
                    if bl.contains(&name) {
                        return Some(o.name)
                    }
                }
            }
            if let Some(idx) = self.positionals_name.get(k) {
                if let Some(pos) = self.positionals_idx.get(idx) {
                    if let Some(ref bl) = pos.overrides {
                        if bl.contains(&name) {
                            return Some(pos.name)
                        }
                    }
                }
            }
        }
        None
    }

    fn create_help_and_version(&mut self) {
        // name is "hclap_help" because flags are sorted by name
        if !self.flags.values().any(|a| a.long.is_some() && a.long.unwrap() == "help") {
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
                settings: ArgFlags::new()
            };
            self.long_list.push("help");
            self.flags.insert("hclap_help", arg);
            // self.settings.unset(&AppSettings::NeedsLongHelp);
        }
        if !self.settings.is_set(&AppSettings::VersionlessSubcommands) ||
           (self.settings.is_set(&AppSettings::VersionlessSubcommands) &&
                self.settings.is_set(&AppSettings::DisableVersion)) &&
           !self.flags.values().any(|a| a.long.is_some() && a.long.unwrap() == "version") {
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
                settings: ArgFlags::new()
            };
            self.long_list.push("version");
            self.flags.insert("vclap_version", arg);
            // self.settings.unset(&AppSettings::NeedsLongVersion);
        }
        if !self.subcommands.is_empty() && !self.subcommands.keys().any(|a| a == "help") {
            self.subcommands.insert("help".to_owned(), App::new("help")
                                                            .about("Prints this message"));
            // self.settings.unset(&AppSettings::NeedsSubcommandHelp);
        }
    }

    fn check_for_help_and_version(&self,
                                  arg: char)
                                  -> Result<(), ClapError> {
        if let Some(h) = self.help_short {
            if h == arg {
                if let Err(e) = self.print_help() {
                    return Err(ClapError{
                        error: format!("{} {}\n\terror message: {}\n",
                                        Format::Error("error:"),
                                        INTERNAL_ERROR_MSG, e.description()),
                        error_type: ClapErrorType::MissingSubcommand
                    });
                }
                // process::exit(0);
                return Err(ClapError{
                    error: String::new(),
                    error_type: ClapErrorType::HelpDisplayed
                })
            }
        }
        if let Some(v) = self.version_short {
            if v == arg {
                let out = io::stdout();
                let mut buf_w = BufWriter::new(out.lock());
                if let Err(e) = self.print_version(&mut buf_w) {
                    return Err(ClapError{
                        error: format!("{} {}\n\terror message: {}\n",
                                        Format::Error("error:"),
                                        INTERNAL_ERROR_MSG, e.description()),
                        error_type: ClapErrorType::MissingSubcommand
                    });
                }
                // process::exit(0);
                return Err(ClapError{
                    error: String::new(),
                    error_type: ClapErrorType::VersionDisplayed
                })
            }
        }

        Ok(())
    }

    fn parse_long_arg<'av>(&mut self,
                           matches: &mut ArgMatches<'ar, 'ar>,
                           full_arg: &'av str)
                           -> Result<Option<&'ar str>, ClapError> {
        let mut arg = full_arg.trim_left_matches(|c| c == '-');

        if arg == "help" && self.settings.is_set(&AppSettings::NeedsLongHelp) {
            if let Err(e) = self.print_help() {
                return Err(ClapError{
                    error: format!("{} {}\n\terror message: {}\n",
                                    Format::Error("error:"),
                                    INTERNAL_ERROR_MSG, e.description()),
                    error_type: ClapErrorType::MissingSubcommand
                });
            }
            return Err(ClapError{
                error: String::new(),
                error_type: ClapErrorType::HelpDisplayed
            })
            // process::exit(0);
        } else if arg == "version" && self.settings.is_set(&AppSettings::NeedsLongVersion) {
            let out = io::stdout();
            let mut buf_w = BufWriter::new(out.lock());
            if let Err(e) = self.print_version(&mut buf_w) {
                return Err(ClapError{
                    error: format!("{} {}\n\terror message: {}\n",
                                    Format::Error("error:"),
                                    INTERNAL_ERROR_MSG, e.description()),
                    error_type: ClapErrorType::MissingSubcommand
                });
            }
            return Err(ClapError{
                error: String::new(),
                error_type: ClapErrorType::VersionDisplayed
            })
            // process::exit(0);
        }

        let mut arg_val: Option<&'av str> = None;

        if arg.contains("=") {
            let arg_vec: Vec<_> = arg.split("=").collect();
            arg = arg_vec[0];
            if let Some(ref v) = self.opts.values()
                                      .filter(|&v| v.long.is_some())
                                      .filter(|&v| v.long.unwrap() == arg).nth(0) {
                // prevents "--config= value" typo
                if arg_vec[1].len() == 0 && !v.settings.is_set(&ArgSettings::EmptyValues) {
                    if let Some(ref vec) = self.groups_for(v.name) {
                        for grp in vec {
                            matches.args.insert(grp, MatchedArg{
                                occurrences: 1,
                                values: None,
                            });
                        }
                    }
                    matches.args.insert(v.name, MatchedArg {
                        occurrences: 1,
                        values: None
                    });
                    return Err(
                        self.create_error(
                            &[&*format!("--{}", arg)],
                            ClapErrorType::EmptyValue,
                            matches));
                }
                arg_val = Some(arg_vec[1]);
            }
        }

        if let Some(ref v) = self.opts.values()
                                  .filter(|&v| v.long.is_some())
                                  .filter(|&v| v.long.unwrap() == arg).nth(0) {
            // Ensure this option isn't on the master mutually excludes list
            if self.blacklist.contains(&v.name) {
                matches.args.remove(v.name);
                return Err(
                    self.create_error(
                        &[&*format!("--{}", arg), "one or more of the other specified arguments"],
                        ClapErrorType::ArgumentConflict,
                        matches));
            }
            if self.overrides.contains(&v.name) {
                debugln!("it is...");
                debugln!("checking who defined it...");
                if let Some(name) = self.overriden_from(v.name, matches) {
                    debugln!("found {}", name);
                    if let Some(ref vec) = self.groups_for(v.name) {
                        for grp in vec {
                            matches.args.remove(grp);
                        }
                    }
                    matches.args.remove(name);
                    remove_overriden!(self, name);
                }
            }
            if let Some(ref or) = v.overrides {
                for pa in or {
                    if let Some(ref vec) = self.groups_for(v.name) {
                        for grp in vec {
                            matches.args.remove(grp);
                        }
                    }
                    matches.args.remove(pa);
                    remove_overriden!(self, pa);
                    self.overrides.push(pa);
                }
            }

            if matches.args.contains_key(v.name) {
                if !v.settings.is_set(&ArgSettings::Multiple) {
                    return Err(
                        self.create_error(
                            &[&*format!("--{}", arg)],
                            ClapErrorType::UnexpectedMultipleUsage,
                            matches));
                }
                if let Some(av) = arg_val {
                    if let Some(ref vec) = self.groups_for(v.name) {
                        for grp in vec {
                            if let Some(ref mut o) = matches.args.get_mut(grp) {
                                o.occurrences += 1;
                                if let Some(ref mut vals) = o.values {
                                    let len = (vals.len() + 1) as u8;
                                    vals.insert(len, av.to_owned());
                                }
                            }
                        }
                    }
                    if let Some(ref mut o) = matches.args.get_mut(v.name) {
                        o.occurrences += 1;
                        if let Some(ref mut vals) = o.values {
                            let len = (vals.len() + 1) as u8;
                            vals.insert(len, av.to_owned());
                        }
                    }
                    // The validation must come AFTER inserting into 'matches' or the usage string
                    // can't be built
                    if let Err(e) = self.validate_value(v, av, matches) {
                        return Err(e);
                    }
                }
            } else {
                let mut bm = BTreeMap::new();
                if let Some(val) = arg_val {
                    bm.insert(1, val.to_owned());
                    if let Some(ref vec) = self.groups_for(v.name) {
                        for grp in vec {
                            matches.args.insert(grp, MatchedArg{
                                occurrences: 1,
                                values: Some(bm.clone()),
                            });
                        }
                    }
                    matches.args.insert(v.name, MatchedArg{
                        occurrences: 1,
                        values: Some(bm)
                    });
                    // The validation must come AFTER inserting into 'matches' or the usage string
                    // can't be built
                    if let Err(e) = self.validate_value(v, val, matches) {
                        return Err(e);
                    }
                } else {
                    if let Some(ref vec) = self.groups_for(v.name) {
                        for grp in vec {
                            matches.args.insert(grp, MatchedArg{
                                occurrences: 1,
                                values: Some(bm.clone()),
                            });
                        }
                    }
                    matches.args.insert(v.name, MatchedArg{
                        occurrences: 0,
                        values: Some(bm)
                    });
                }
            }
            if let Some(ref bl) = v.blacklist {
                for name in bl {
                    self.blacklist.push(name);
                    vec_remove!(self.overrides, name);
                    vec_remove!(self.required, name);
                }
            }

            if let Some(ref reqs) = v.requires {
                // Add all required args which aren't already found in matches to the
                // final required list
                for n in reqs {
                    if matches.args.contains_key(n) {
                        continue;
                    }

                    self.required.push(n);
                }
            }

            parse_group_reqs!(self, v);

            match arg_val {
                None => {
                    return Ok(Some(v.name));
                }
                _ => {
                    return Ok(None);
                }
            }
        }

        if let Some(v) = self.flags.values()
                                   .filter(|&v| v.long.is_some())
                                   .filter(|&v| v.long.unwrap() == arg).nth(0) {
            // Ensure this flag isn't on the mutually excludes list
            if self.blacklist.contains(&v.name) {
                matches.args.remove(v.name);
                return Err(
                    self.create_error(
                        &[&*v.to_string(), v.name],
                        ClapErrorType::ArgumentConflict,
                        matches));
            }
            if self.overrides.contains(&v.name) {
                debugln!("it is...");
                debugln!("checking who defined it...");
                if let Some(name) = self.overriden_from(v.name, matches) {
                    debugln!("found {}", name);
                    matches.args.remove(name);
                    remove_overriden!(self, name);
                }
            }
            if let Some(ref or) = v.overrides {
                for pa in or {
                    matches.args.remove(pa);
                    remove_overriden!(self, pa);
                    self.overrides.push(pa);
                }
            }

            // Make sure this isn't one being added multiple times if it doesn't suppor it
            if matches.args.contains_key(v.name) && !v.settings.is_set(&ArgSettings::Multiple) {
                return Err(
                    self.create_error(
                        &[&*v.to_string()],
                        ClapErrorType::UnexpectedMultipleUsage,
                        matches));
            }

            let mut
            done = false;
            if let Some(ref mut f) = matches.args.get_mut(v.name) {
                done = true;
                f.occurrences = if v.settings.is_set(&ArgSettings::Multiple) {
                    f.occurrences + 1
                } else {
                    1
                };
            }
            if !done {
                if let Some(ref vec) = self.groups_for(v.name) {
                    for grp in vec {
                        matches.args.insert(grp, MatchedArg{
                            occurrences: 1,
                            values: None,
                        });
                    }
                }
                matches.args.insert(v.name, MatchedArg{
                    // name: v.name.to_owned(),
                    occurrences: 1,
                    values: None
                });
            }


            // Add all of this flags "mutually excludes" list to the master list
            if let Some(ref ov) = v.overrides {
                for name in ov {
                    self.overrides.push(name);
                }
            }
            if let Some(ref bl) = v.blacklist {
                for name in bl {
                    self.blacklist.push(name);
                    vec_remove!(self.overrides, name);
                    vec_remove!(self.required, name);
                }
            }

            // Add all required args which aren't already found in matches to the master list
            if let Some(ref reqs) = v.requires {
                for n in reqs {
                    if matches.args.contains_key(n) {
                        continue;
                    }

                    self.required.push(n);
                }
            }

            parse_group_reqs!(self, v);

            return Ok(None);
        }

        let suffix = App::did_you_mean_suffix(arg,
                                              self.long_list.iter(),
                                              DidYouMeanMessageStyle::LongFlag);
        if let Some(name) = suffix.1 {
            if let Some(ref opt) = self.opts.values()
                                          .filter_map(|o| {
                                              if o.long.is_some() && o.long.unwrap() == name {
                                                  Some(o.name)
                                              } else {
                                                  None
                                              }
                                          })
                                          .next() {
                if let Some(ref vec) = self.groups_for(opt) {
                    for grp in vec {
                        matches.args.insert(grp, MatchedArg{
                            occurrences: 1,
                            values: None,
                        });
                    }
                }
                matches.args.insert(opt, MatchedArg {
                    occurrences: 0,
                    values: None
                });
            } else if let Some(ref flg) = self.flags.values()
                                          .filter_map(|f| {
                                              if f.long.is_some() && f.long.unwrap() == name {
                                                  Some(f.name)
                                              } else {
                                                  None
                                              }
                                          })
                                          .next() {
                if let Some(ref vec) = self.groups_for(flg) {
                    for grp in vec {
                        matches.args.insert(grp, MatchedArg{
                            occurrences: 1,
                            values: None,
                        });
                    }
                }
                matches.args.insert(flg, MatchedArg {
                    occurrences: 0,
                    values: None
                });
            }
        }

        Err(
            self.create_error(
                    &[&*format!("--{}", arg), &*suffix.0],
                    ClapErrorType::InvalidArgument,
                    matches))
    }

    fn validate_value(&self,
                      v: &OptBuilder,
                      av: &str,
                      matches: &ArgMatches)
                      -> Result<(), ClapError> {
        if let Some(ref p_vals) = v.possible_vals {
            if !p_vals.contains(&av) {
                return Err(self.possible_values_error(av, &v.to_string(), p_vals, matches));
            }
        }
        if !v.settings.is_set(&ArgSettings::EmptyValues) && av.is_empty() && matches.args.contains_key(v.name) {
            return Err(
                self.create_error(
                    &[&*v.to_string()],
                    ClapErrorType::EmptyValue,
                    matches));
        }
        if let Some(ref vtor) = v.validator {
            if let Err(e) = vtor(av.to_owned()) {
                return Err(
                    self.create_error(
                        &[&*e],
                        ClapErrorType::ArgumentError,
                        matches));
            }
        }
        Ok(())
    }

    fn parse_short_arg(&mut self,
                       matches: &mut ArgMatches<'ar, 'ar>,
                       full_arg: &str)
                       -> Result<Option<&'ar str>, ClapError> {
        let arg = &full_arg[..].trim_left_matches(|c| c == '-');
        for c in arg.chars() {
            if let Err(e) = self.check_for_help_and_version(c) {
                return Err(ClapError{
                    error: format!("{} {}\n\terror message: {}\n",
                                    Format::Error("error:"),
                                    INTERNAL_ERROR_MSG, e.description()),
                    error_type: ClapErrorType::MissingSubcommand
                });
            }

            // Check for matching short in options, and return the name
            // (only ones with shorts, of course)
            if let Some(v) = self.opts.values()
                                 .filter(|&v| v.short.is_some())
                                 .filter(|&v| v.short == Some(c)).nth(0) {
                let mut ret = Some(v.name);
                // Ensure this option isn't on the master mutually excludes list
                if self.blacklist.contains(&v.name) {
                    matches.args.remove(v.name);
                    return Err(
                        self.create_error(
                            &[&*format!("-{}", arg), v.name],
                            ClapErrorType::ArgumentConflict,
                            matches));
                }
                if self.overrides.contains(&v.name) {
                    if let Some(name) = self.overriden_from(v.name, matches) {
                        matches.args.remove(&*name);
                        remove_overriden!(self, &*name);
                    }
                }
                if let Some(ref or) = v.overrides {
                    for pa in or {
                        matches.args.remove(pa);
                        remove_overriden!(self, pa);
                        self.overrides.push(pa);
                    }
                }

                if matches.args.contains_key(v.name) && 
                    !v.settings.is_set(&ArgSettings::Multiple) {
                    return Err(
                        self.create_error(
                            &[&*format!("-{}", arg)],
                            ClapErrorType::UnexpectedMultipleUsage,
                            matches));
                } 

                // New scope for lifetimes
                let val: Vec<&str> = arg.splitn(2, c).collect();
                {
                    let ma = matches.args.entry(v.name).or_insert(
                            MatchedArg{
                                // occurrences will be incremented on getting a value
                                occurrences: 0,
                                values: Some(BTreeMap::new())
                            });
                    if !val[1].is_empty() {
                        if !v.settings.is_set(&ArgSettings::Multiple) {
                            ret = None;
                        }
                        if let Some(ref mut vals) = ma.values {
                            let len = vals.len() as u8 + 1;
                            vals.insert(len, val[1].to_owned());
                        }
                        ma.occurrences += 1;
                    }
                }

                if let Some(ref vec) = self.groups_for(v.name) {
                    for grp in vec {
                        let ma_g = matches.args.entry(grp).or_insert(MatchedArg{
                            occurrences: 0,
                            values: Some(BTreeMap::new()),
                        });
                        if !val[1].is_empty() {
                            if let Some(ref mut vals) = ma_g.values {
                                let len = vals.len() as u8 + 1;
                                vals.insert(len, val[1].to_owned());
                            }
                            ma_g.occurrences += 1;
                        }
                    }
                }

                if let Some(ref bl) = v.blacklist {
                    for name in bl {
                        self.blacklist.push(name);
                        vec_remove!(self.overrides, name);
                        vec_remove!(self.required, name);
                    }
                }

                if let Some(ref reqs) = v.requires {
                    // Add all required args which aren't already found in matches to the
                    // final required list
                    for n in reqs {
                        if matches.args.contains_key(n) {
                            continue;
                        }

                        self.required.push(n);
                    }
                }

                parse_group_reqs!(self, v);

                return Ok(ret);
            }

            match self.parse_single_short_flag(matches, c) {
                Ok(b) => {
                    if !b {
                        return Err(
                            self.create_error(
                                &[&*format!("-{}", c), ""],
                                ClapErrorType::InvalidArgument,
                                matches));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(None);
    }

    fn parse_single_short_flag(&mut self,
                               matches: &mut ArgMatches<'ar, 'ar>,
                               arg: char)
                               -> Result<bool, ClapError> {
        if let Some(v) = self.flags.values()
                           .filter(|&v| v.short.is_some())
                           .filter(|&v| v.short.unwrap() == arg).nth(0) {
            // Ensure this flag isn't on the mutually excludes list
            if self.blacklist.contains(&v.name) {
                matches.args.remove(v.name);
                return Err(
                    self.create_error(
                        &[&*format!("-{}", arg), v.name],
                        ClapErrorType::ArgumentConflict,
                        matches));
            }
            if self.overrides.contains(&v.name) {
                debugln!("it is...");
                debugln!("checking who defined it...");
                if let Some(name) = self.overriden_from(v.name, matches) {
                    debugln!("found {}", name);
                    matches.args.remove(name);
                    remove_overriden!(self, name);
                }
            }
            if let Some(ref or) = v.overrides {
                for pa in or {
                    matches.args.remove(pa);
                    remove_overriden!(self, pa);
                    self.overrides.push(pa);
                }
            }

            // Make sure this isn't one being added multiple times if it doesn't suppor it
            if matches.args.contains_key(v.name) && !v.settings.is_set(&ArgSettings::Multiple) {
                return Err(
                    self.create_error(
                        &[&*format!("-{}", arg)],
                        ClapErrorType::UnexpectedMultipleUsage,
                        matches));
            }

            if let Some(ref vec) = self.groups_for(v.name) {
                for grp in vec {
                    if let Some(ref mut f) = matches.args.get_mut(grp) {
                        f.occurrences = if v.settings.is_set(&ArgSettings::Multiple) {
                            f.occurrences + 1
                        } else {
                            1
                        };
                    }
                }
            }
            let mut done = false;
            if let Some(ref mut f) = matches.args.get_mut(v.name) {
                done = true;
                f.occurrences = if v.settings.is_set(&ArgSettings::Multiple) {
                    f.occurrences + 1
                } else {
                    1
                };
            }
            if !done {
                if let Some(ref vec) = self.groups_for(v.name) {
                    for grp in vec {
                        matches.args.insert(grp, MatchedArg{
                            occurrences: 1,
                            values: None,
                        });
                    }
                }
                matches.args.insert(v.name, MatchedArg{
                    // name: v.name.to_owned(),
                    occurrences: 1,
                    values: None
                });
            }

            if let Some(ref bl) = v.blacklist {
                for name in bl {
                    self.blacklist.push(name);
                    vec_remove!(self.overrides, name);
                    vec_remove!(self.required, name);
                }
            }

            // Add all required args which aren't already found in matches to the master list
            if let Some(ref reqs) = v.requires {
                for n in reqs {
                    if matches.args.contains_key(n) {
                        continue;
                    }

                    self.required.push(n);
                }
            }

            parse_group_reqs!(self, v);

            return Ok(true);
        }
        Ok(false)
    }

    fn validate_blacklist(&self,
                          matches: &mut ArgMatches<'ar, 'ar>)
                          -> Result<(), ClapError> {
        for name in self.blacklist.iter() {
            if matches.args.contains_key(name) {
                matches.args.remove(name);
                return Err(
                    self.create_error(
                        &[
                            if let Some(ref flag) = self.flags.get(name) {
                                format!("{}", Format::Warning(flag.to_string()))
                            } else if let Some(ref opt) = self.opts.get(name) {
                                format!("{}", Format::Warning(opt.to_string()))
                            } else {
                                match self.positionals_idx.values().filter(|p| p.name == *name).next() {
                                    Some(pos) => format!("{}", Format::Warning(pos.to_string())),
                                    None      => format!("\"{}\"", Format::Warning(name))
                                }
                            }, match self.blacklisted_from(name, matches) {
                                Some(name) => format!("'{}'", Format::Warning(name)),
                                None       => "one or more of the other specified arguments".to_owned()
                            }
                        ],
                        ClapErrorType::ArgumentConflict,
                        matches));
            } else if self.groups.contains_key(name) {
                for n in self.get_group_members_names(name) {
                    if matches.args.contains_key(n) {
                        matches.args.remove(n);
                        return Err(
                            self.create_error(
                                &[
                                    if let Some(ref flag) = self.flags.get(n) {
                                        format!("{}", Format::Warning(flag.to_string()))
                                    } else if let Some(ref opt) = self.opts.get(n) {
                                        format!("{}", Format::Warning(opt.to_string()))
                                    } else {
                                        match self.positionals_idx.values()
                                                                  .filter(|p| p.name == *name)
                                                                  .next() {
                                            Some(pos) => format!("{}", Format::Warning(pos.to_string())),
                                            None      => format!("\"{}\"", Format::Warning(n))
                                        }
                                    },
                                    "one or more of the other specified arguments".to_owned()],
                                ClapErrorType::ArgumentConflict,
                                matches));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_num_args(&self,
                         matches: &mut ArgMatches<'ar, 'ar>)
                         -> Result<(), ClapError> {
        for (name, ma) in matches.args.iter() {
            if self.groups.contains_key(name) {
                continue;
            } else if let Some(ref vals) = ma.values {
                if let Some(f) = self.opts.get(name) {
                    if let Some(num) = f.num_vals {
                        let should_err = if f.settings.is_set(&ArgSettings::Multiple) {
                            ((vals.len() as u8) % num) != 0
                        } else {
                            num != (vals.len() as u8)
                        };
                        if should_err {
                            return Err(
                                self.create_error(
                                    &[f.to_string(),
                                    num.to_string(),
                                    if f.settings.is_set(&ArgSettings::Multiple) {
                                        (vals.len() % num as usize).to_string()
                                    } else {
                                        vals.len().to_string()
                                    },
                                    if vals.len() == 1 ||
                                        ( f.settings.is_set(&ArgSettings::Multiple) &&
                                            ( vals.len() % num as usize) == 1) {"as".to_owned()}else{"ere".to_owned()}],
                                    ClapErrorType::WrongNumValues,
                                    matches));
                        }
                    }
                    if let Some(num) = f.max_vals {
                        if (vals.len() as u8) > num {
                            return Err(
                                self.create_error(
                                    &[&*f.to_string(),
                                      &*num.to_string(),
                                      &*vals.len().to_string(),
                                      if vals.len() == 1 {"as"}else{"ere"}],
                                    ClapErrorType::TooManyValues,
                                    matches));
                        }
                    }
                    if let Some(num) = f.min_vals {
                        if (vals.len() as u8) < num {
                            return Err(
                                self.create_error(
                                    &[&*f.to_string(),
                                      &*num.to_string(),
                                      &*vals.len().to_string(),
                                      if vals.len() == 1 {"as"}else{"ere"}],
                                    ClapErrorType::TooFewValues,
                                    matches));
                        }
                    }
                } else if let Some(f) = self.positionals_idx.get(
                    self.positionals_name.get(name).unwrap()) {
                    if let Some(num) = f.num_vals {
                        if num != vals.len() as u8 {
                            return Err(
                                self.create_error(
                                    &[&*f.to_string(),
                                      &*num.to_string(),
                                      &*vals.len().to_string(),
                                      if vals.len() == 1 {"as"}else{"ere"}],
                                    ClapErrorType::WrongNumValues,
                                    matches));
                        }
                    }
                    if let Some(num) = f.max_vals {
                        if num > vals.len() as u8 {
                            return Err(
                                self.create_error(
                                    &[&*f.to_string(),
                                      &*num.to_string(),
                                      &*vals.len().to_string(),
                                      if vals.len() == 1 {"as"}else{"ere"}],
                                    ClapErrorType::TooFewValues,
                                    matches));
                        }
                    }
                    if let Some(num) = f.min_vals {
                        if num < vals.len() as u8 {
                            return Err(
                                self.create_error(
                                    &[&*f.to_string(),
                                      &*num.to_string(),
                                      &*vals.len().to_string(),
                                      if vals.len() == 1 {"as"}else{"ere"}],
                                    ClapErrorType::TooManyValues,
                                    matches));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_required(&self,
                         matches: &ArgMatches<'ar, 'ar>)
                         -> bool {
        'outer: for name in self.required.iter() {
            if matches.args.contains_key(name) {
                continue 'outer;
            }
            for grp in self.groups.values() {
                if grp.args.contains(name) {
                    continue 'outer;
                }
            }
            if let Some(a) = self.flags.get(name) {
                if let Some(ref bl) = a.blacklist {
                    for n in bl.iter() {
                        if matches.args.contains_key(n) {
                            continue 'outer
                        } else if self.groups.contains_key(n) {
                            let grp = self.groups.get(n).unwrap();
                            for an in grp.args.iter() {
                                if matches.args.contains_key(an) {
                                    continue 'outer
                                }
                            }
                        }
                    }
                }
            }
            if let Some(a) = self.opts.get(name) {
                if let Some(ref bl) = a.blacklist {
                    for n in bl.iter() {
                        if matches.args.contains_key(n) {
                            continue 'outer
                        } else if self.groups.contains_key(n) {
                            let grp = self.groups.get(n).unwrap();
                            for an in grp.args.iter() {
                                if matches.args.contains_key(an) {
                                    continue 'outer
                                }
                            }
                        }
                    }
                }
            }
            // because positions use different keys, we dont use the macro
            match self.positionals_idx.values().filter(|p| &p.name == name).next() {
                Some(p) => {
                    if let Some(ref bl) = p.blacklist {
                        for n in bl.iter() {
                            if matches.args.contains_key(n) {
                                continue 'outer;
                            } else if self.groups.contains_key(n) {
                                let grp = self.groups.get(n).unwrap();
                                for an in grp.args.iter() {
                                    if matches.args.contains_key(an) {
                                        continue 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
                None => (),
            }
            return true;
        }
        false
    }

    /// Returns a suffix that can be empty, or is the standard 'did you mean phrase
    #[cfg_attr(feature = "lints", allow(needless_lifetimes))]
    fn did_you_mean_suffix<'z, T, I>(arg: &str,
                                     values: I,
                                     style: DidYouMeanMessageStyle)
                                     -> (String, Option<&'z str>)
        where T: AsRef<str> + 'z,
              I: IntoIterator<Item = &'z T>
    {
        match did_you_mean(arg, values) {
            Some(candidate) => {
                let mut suffix = "\n\tDid you mean ".to_owned();
                match style {
                    DidYouMeanMessageStyle::LongFlag => suffix.push_str(
                            &*format!("{}", Format::Good("--"))
                        ),
                    DidYouMeanMessageStyle::EnumValue => suffix.push('\''),
                }
                suffix.push_str(&Format::Good(candidate).to_string()[..]);
                if let DidYouMeanMessageStyle::EnumValue = style {
                    suffix.push('\'');
                }
                suffix.push_str(" ?");
                (suffix, Some(candidate))
            }
            None => (String::new(), None),
        }
    }

    fn validate_option(&self, opt: &OptBuilder, arg_slice: &str, matches: &ArgMatches) -> Result<(), ClapError> {
        // Check the possible values
        if let Some(ref p_vals) = opt.possible_vals {
            if !p_vals.contains(&arg_slice) {
                return Err(self.possible_values_error(arg_slice, &opt.to_string(),
                                                      p_vals, matches));
            }
        }
        // Check the required number of values
        if let Some(num) = opt.num_vals {
            if let Some(ref ma) = matches.args.get(opt.name) {
                if let Some(ref vals) = ma.values {
                    if num == vals.len() as u8 && !opt.settings.is_set(&ArgSettings::Multiple) {
                        return Err(
                            self.create_error(
                                &[&*opt.to_string(),
                                  &*num.to_string(),
                                  &*vals.len().to_string(),
                                  if vals.len() == 1 {"as"}else{"ere"}],
                                ClapErrorType::TooManyValues,
                                matches));
                    }
                }
            }
        }

        // if it's an empty value, and we don't allow that, report the error
        if !opt.settings.is_set(&ArgSettings::EmptyValues) && matches.args.contains_key(opt.name) &&
           arg_slice.is_empty() {
            return Err(
                self.create_error(
                    &[&*opt.to_string()],
                    ClapErrorType::EmptyValue,
                    matches));
        }

        if let Some(ref vtor) = opt.validator {
            if let Err(e) = vtor(arg_slice.to_owned()) {
                return Err(
                    self.create_error(
                        &[&*e],
                        ClapErrorType::ValueValidationError,
                        matches));
            }
        }

        Ok(())
    }
}
