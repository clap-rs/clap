#[allow(dead_code)]
mod settings;
#[macro_use]
mod macros;
mod parser;
mod meta;

pub use self::settings::AppSettings;

use std::env;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::process;
use std::ffi::OsString;
use std::borrow::Borrow;

#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

use args::{Arg, AnyArg, ArgGroup, ArgMatches, ArgMatcher};
use app::parser::Parser;
use errors::Error;
use errors::Result as ClapResult;

/// Used to create a representation of a command line program and all possible command line
/// arguments. Application settings are set using the "builder pattern" with the
/// `.get_matches()` family of methods being the terminal methods that starts the runtime-parsing
/// process. These methods then return information about the user supplied arguments (or lack there
/// of).
///
/// **NOTE:** There aren't any mandatory "options" that one must set. The "options" may
/// also appear in any order (so long as one of the `App::get_matches*` methods is the last method
/// called).
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg};
/// let m = App::new("My Program")
///     .author("Me, me@mail.com")
///     .version("1.0.2")
///     .about("Explains in brief what the program does")
///     .arg(
///         Arg::with_name("in_file").index(1)
///     )
///     .after_help("Longer explaination to appear after the options when \
///                  displaying the help information from --help or -h")
///     .get_matches();
///
/// // Your program logic starts here...
/// ```
#[allow(missing_debug_implementations)]
pub struct App<'a, 'b>(Parser<'a, 'b>) where 'a: 'b;

impl<'a, 'b> App<'a, 'b> {
    /// Creates a new instance of an application requiring a name. The name may be, but doesn't
    /// have to be same as the binary. The name will be displayed to the user when they request to
    /// print version or help and usage information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let prog = App::new("My Program")
    /// # ;
    /// ```
    pub fn new<S: Into<String>>(n: S) -> Self { App(Parser::with_name(n.into())) }

    /// Creates a new instace of `App` from a .yml (YAML) file. A full example of supported YAML
    /// objects can be found in `examples/17_yaml.rs` and `examples/17_yaml.yml`. One great use for
    /// using YAML is when supporting multiple languages and dialects, as each language could be a
    /// distinct YAML file and determined at compiletime via `cargo` "features" in your
    /// `Cargo.toml`
    ///
    /// In order to use this function you must compile `clap` with the `features = ["yaml"]` in
    /// your settings for the `[dependencies.clap]` table of your `Cargo.toml`
    ///
    /// **NOTE:** Due to how the YAML objects are built there is a convienience macro for loading
    /// the YAML file at compile time (relative to the current file, like modules work). That YAML
    /// object can then be passed to this function.
    ///
    /// # Panics
    ///
    /// The YAML file must be properly formatted or this function will panic!(). A good way to
    /// ensure this doesn't happen is to run your program with the `--help` switch. If this passes
    /// without error, you needn't worry because the YAML is properly formatted.
    ///
    /// # Examples
    ///
    /// The following example shows how to load a properly formatted YAML file to build an instnace
    /// of an `App` struct.
    ///
    /// ```ignore
    /// # use clap::App;
    /// let yml = load_yaml!("app.yml");
    /// let app = App::from_yaml(yml);
    ///
    /// // continued logic goes here, such as `app.get_matches()` etc.
    /// ```
    #[cfg(feature = "yaml")]
    pub fn from_yaml<'y>(mut yaml: &'y Yaml) -> App<'y, 'y> {
        use args::SubCommand;
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
        if let Some(v) = yaml["groups"].as_vec() {
            for ag_yaml in v {
                a = a.group(ArgGroup::from_yaml(&ag_yaml.as_hash().unwrap()));
            }
        }

        a
    }

    /// Sets a string of author(s) that will be displayed to the user when they request the help
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
    pub fn author<S: Into<&'b str>>(mut self, author: S) -> Self {
        self.0.meta.author = Some(author.into());
        self
    }

    /// Overrides the system-determined binary name. This should only be used when absolutely
    /// neccessary, such as when the binary name for your application is misleading, or perhaps
    /// *not* how the user should invoke your program.
    ///
    /// **Pro-tip:** When building things such as third party `cargo` subcommands, this setting
    /// **should** be used!
    ///
    /// **NOTE:** This command **should not** be used for `SubCommand`s.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("My Program")
    ///      .bin_name("my_binary")
    /// # ;
    /// ```
    pub fn bin_name<S: Into<String>>(mut self, name: S) -> Self {
        self.0.meta.bin_name = Some(name.into());
        self
    }

    /// Sets a string describing what the program does. This will be displayed when displaying help
    /// information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .about("Does really amazing things to great people")
    /// # ;
    /// ```
    pub fn about<S: Into<&'b str>>(mut self, about: S) -> Self {
        self.0.meta.about = Some(about.into());
        self
    }

    /// Adds additional help information to be displayed in addition to auto-generated help. This
    /// information is displayed **after** the auto-generated help information. This is often used
    /// to describe how to use the arguments, or caveats to be noted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// App::new("myprog")
    ///     .after_help("Does really amazing things to great people...but be careful with -R")
    /// # ;
    /// ```
    pub fn after_help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.0.meta.more_help = Some(help.into());
        self
    }

    /// Sets a string of the version number to be displayed when displaying version or help
    /// information.
    ///
    /// **Pro-tip:** Use `clap`s convienience macro `crate_version!` to automatically set your
    /// application's version to the same thing as your crate at compile time. See the `examples/`
    /// directory for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .version("v0.1.24")
    /// # ;
    /// ```
    pub fn version<S: Into<&'b str>>(mut self, ver: S) -> Self {
        self.0.meta.version = Some(ver.into());
        self
    }

    /// Sets a custom usage string to override the auto-generated usage string.
    ///
    /// This will be displayed to the user when errors are found in argument parsing, or when you
    /// call `ArgMatches::usage`
    ///
    /// **CAUTION:** Using this setting disables `clap`s "context-aware" usage strings. After this
    /// setting is set, this will be the only usage string displayed to the user!
    ///
    /// **NOTE:** You do not need to specify the "USAGE: \n\t" portion, as that will
    /// still be applied by `clap`, you only need to specify the portion starting
    /// with the binary name.
    ///
    /// **NOTE:** This will not replace the entire help message, *only* the portion
    /// showing the usage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .usage("myapp [-clDas] <some_file>")
    /// # ;
    /// ```
    pub fn usage<S: Into<&'b str>>(mut self, usage: S) -> Self {
        self.0.meta.usage_str = Some(usage.into());
        self
    }

    /// Sets a custom help message and overrides the auto-generated one. This should only be used
    /// when the auto-generated message does not suffice.
    ///
    /// This will be displayed to the user when they use `--help` or `-h`
    ///
    /// **NOTE:** This replaces the **entire** help message, so nothing will be auto-generated.
    ///
    /// **NOTE:** This **only** replaces the help message for the current command, meaning if you
    /// are using subcommands, those help messages will still be auto-generated unless you
    /// specify a `.help()` for them as well.
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
    pub fn help<S: Into<&'b str>>(mut self, help: S) -> Self {
        self.0.meta.help_str = Some(help.into());
        self
    }

    /// Sets the short version of the `help` argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `h`, but this can be overridden by defining your
    /// own argument with a lowercase `h` as the `short`. `clap` lazily generates these help
    /// arguments **after** you've defined any arguments of your own.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .help_short("H") // Using an uppercase `H` instead of the default lowercase `h`
    /// # ;
    /// ```
    pub fn help_short<S: AsRef<str> + 'b>(mut self, s: S) -> Self {
        self.0.help_short(s.as_ref());
        self
    }

    /// Sets the short version of the `version` argument without the preceding `-`.
    ///
    /// By default `clap` automatically assigns `V`, but this can be overridden by defining your
    /// own argument with a uppercase `V` as the `short`. `clap` lazily generates these version
    /// arguments **after** you've defined any arguments of your own.
    ///
    /// **NOTE:** Any leading `-` characters will be stripped, and only the first
    /// non `-` chacter will be used as the `short` version
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .version_short("v") // Using a lowercase `v` instead of the default capital `V`
    /// # ;
    /// ```
    pub fn version_short<S: AsRef<str>>(mut self, s: S) -> Self {
        self.0.version_short(s.as_ref());
        self
    }

    /// Enables a single Application level settings.
    ///
    /// See `AppSettings` for a full list of possibilities and examples.
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
        self.0.set(setting);
        self
    }

    /// Enables multiple Application level settings
    ///
    /// See `AppSettings` for a full list of possibilities and examples.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, AppSettings};
    /// App::new("myprog")
    ///     .settings(&[AppSettings::SubcommandRequired,
    ///                  AppSettings::WaitOnError])
    /// # ;
    /// ```
    pub fn settings(mut self, settings: &[AppSettings]) -> Self {
        for s in settings {
            self.0.set(*s);
        }
        self
    }

    /// Adds an argument to the list of valid possibilties.
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
    pub fn arg<A: Borrow<Arg<'a, 'b>> + 'a>(mut self, a: A) -> Self {
        self.0.add_arg(a.borrow());
        self
    }

    /// Adds multiple arguments to the list of valid possibilties
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args(
    ///         &[Arg::from_usage("[debug] -d 'turns on debugging info'"),
    ///          Arg::with_name("input").index(1).help("the input file to use")]
    ///     )
    /// # ;
    /// ```
    pub fn args(mut self, args: &[Arg<'a, 'b>]) -> Self {
        for arg in args {
            self.0.add_arg(arg);
        }
        self
    }

    /// A convienience method for adding a single argument from a usage type string. The string
    /// used follows the same rules and syntax as `Arg::from_usage()`
    ///
    /// **NOTE:** The downside to using this method is that you can not set any additional
    /// properties of the `Arg` other than what `Arg::from_usage()` supports.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .arg_from_usage("-c --config=<FILE> 'Sets a configuration file to use'")
    /// # ;
    /// ```
    pub fn arg_from_usage(mut self, usage: &'a str) -> Self {
        self.0.add_arg(&Arg::from_usage(usage));
        self
    }

    /// Adds multiple arguments at once from a usage string, one per line. See `Arg::from_usage()`
    /// for details on the syntax and rules supported.
    ///
    /// **NOTE:** Like `App::arg_from_usage()` the downside is you only set properties for the
    /// `Arg`s which `Arg::from_usage()` supports.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// App::new("myprog")
    ///     .args_from_usage(
    ///         "-c --config=[FILE] 'Sets a configuration file to use'
    ///          [debug]... -d 'Sets the debugging level'
    ///          <FILE> 'The input file to use'"
    ///     )
    /// # ;
    /// ```
    pub fn args_from_usage(mut self, usage: &'a str) -> Self {
        for l in usage.lines() {
            if l.len() == 0 { continue; }
            self.0.add_arg(&Arg::from_usage(l.trim()));
        }
        self
    }

    /// Adds an `ArgGroup` to the application. `ArgGroup`s are a family of related arguments. By
    /// placing them in a logical group, you can build easier requirement and exclusion rules. For
    /// instance, you can make an entire `ArgGroup` required, meaning that one (and *only* one) argument
    /// from that group must be present at runtime.
    ///
    /// You can also do things such as name an `ArgGroup` as a conflict to another argument.
    /// Meaning any of the arguments that belong to that group will cause a failure if present with
    /// the conflicting argument.
    ///
    /// Another added benfit of `ArgGroup`s is that you can extract a value from a group instead of
    /// determining exactly which argument was used.
    ///
    /// Finally, using `ArgGroup`s to ensure exclusion between arguments is another very common use
    ///
    /// # Examples
    ///
    /// The following example demonstrates using an `ArgGroup` to ensure that one, and only one, of
    /// the arguments from the specified group is present at runtime.
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// App::new("app")
    ///     .args_from_usage(
    ///         "--set-ver [ver] 'set the version manually'
    ///          --major         'auto increase major'
    ///          --minor         'auto increase minor'
    ///          --patch         'auto increase patch'")
    ///     .group(ArgGroup::with_name("vers")
    ///          .args(&["set-ver", "major", "minor","patch"])
    ///          .required(true))
    /// # ;
    /// ```
    pub fn group(mut self, group: ArgGroup<'a>) -> Self {
        self.0.add_group(group);
        self
    }

    /// Adds multiple `ArgGroup`s to the application at once.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, ArgGroup};
    /// App::new("app")
    ///     .args_from_usage(
    ///         "--set-ver [ver] 'set the version manually'
    ///          --major         'auto increase major'
    ///          --minor         'auto increase minor'
    ///          --patch         'auto increase patch'
    ///          -c [FILE]       'a config file'
    ///          -i [IFACE]      'an interface'")
    ///     .groups(&[
    ///         ArgGroup::with_name("vers")
    ///             .args(&["set-ver", "major", "minor","patch"])
    ///             .required(true),
    ///         ArgGroup::with_name("input")
    ///             .args(&["c", "i"])
    ///     ])
    /// # ;
    /// ```
    pub fn groups(mut self, groups: &[ArgGroup<'a>]) -> Self {
        for g in groups {
            self = self.group(g.into());
        }
        self
    }

    /// Adds a subcommand to the list of valid possibilties. Subcommands are effectively sub-apps,
    /// because they can contain their own arguments, subcommands, version, usage, etc. They also
    /// function just like apps, in that they get their own auto generated help, version, and
    /// usage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .subcommand(SubCommand::with_name("config")
    ///         .about("Controls configuration features")
    ///         .arg_from_usage("<config> 'Required configuration file to use'"))
    /// # ;
    /// ```
    pub fn subcommand(mut self, subcmd: App<'a, 'b>) -> Self {
        self.0.add_subcommand(subcmd);
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
    pub fn subcommands<I>(mut self, subcmds: I) -> Self
        where I: IntoIterator<Item = App<'a, 'b>>
    {
        for subcmd in subcmds.into_iter() {
            self.0.add_subcommand(subcmd);
        }
        self
    }


    /// Prints the full help message to `io::stdout()` using a `BufWriter`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// let app = App::new("myprog");
    /// app.print_help();
    /// ```
    pub fn print_help(&self) -> ClapResult<()> {
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        self.write_help(&mut buf_w)
    }

    /// Writes the full help message to the user to a `io::Write` object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::App;
    /// use std::io;
    /// let mut app = App::new("myprog");
    /// let mut out = io::stdout();
    /// app.write_help(&mut out).ok().expect("failed to write to stdout");
    /// ```
    pub fn write_help<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        self.0.write_help(w)
    }

    /// Starts the parsing process, upon a failed parse an error will be displayed to the user and
    /// the process with exit with the appropriate error code. By default this method gets matches
    /// from `env::args_os`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches();
    /// ```
    pub fn get_matches(self) -> ArgMatches<'a> {
        self.get_matches_from(&mut env::args_os())
    }

    /// Starts the parsing process. This method will return a `Result` type instead of exiting the
    /// the process on failed parse. By default this method gets matches
    /// from `env::args_os`
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return an error, where the `kind` is a `ErrorKind::HelpDisplayed`
    /// or `ErrorKind::VersionDisplayed` respectively. You must call `error.exit()` or
    /// perform a `std::process::exit`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog")
    ///     // Args and options go here...
    ///     .get_matches_safe()
    ///     .unwrap_or_else( |e| e.exit() );
    /// ```
    pub fn get_matches_safe(self) -> ClapResult<ArgMatches<'a>> {
        // Start the parsing
        self.get_matches_from_safe(&mut env::args_os())
    }

    /// Starts the parsing process. Like `App::get_matches` this method does not return a `Result`
    /// and will automatically exit with an error message. This method, however, lets you specify
    /// what iterator to use when performing matches, such as a `Vec` of your making.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// `AppSettings::NoBinaryName` is used
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
    pub fn get_matches_from<I, T>(mut self, itr: I) -> ArgMatches<'a>
        where I: IntoIterator<Item = T>,
              T: Into<OsString>
    {
        self.get_matches_from_safe_borrow(itr).unwrap_or_else(|e| {
            // Otherwise, write to stderr and exit
            self.maybe_wait_for_exit(e);
        })
    }

    /// Starts the parsing process. A combination of `App::get_matches_from`, and
    /// `App::get_matches_safe`
    ///
    /// **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are
    /// used. It will return an error, where the `kind` is a `ErrorKind::HelpDisplayed`
    /// or `ErrorKind::VersionDisplayed` respectively. You must call `error.exit()` or
    /// perform a `std::process::exit` yourself.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// `AppSettings::NoBinaryName` is used
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
    pub fn get_matches_from_safe<I, T>(mut self, itr: I) -> ClapResult<ArgMatches<'a>>
        where I: IntoIterator<Item = T>,
              T: Into<OsString>
    {
        self.get_matches_from_safe_borrow(itr)
    }

    /// Starts the parsing process without consuming the `App` struct `self`. This is normally not
    /// the desired functionality, instead prefer `App::get_matches_from_safe` which *does*
    /// consume `self`.
    ///
    /// **NOTE:** The first argument will be parsed as the binary name unless
    /// `AppSettings::NoBinaryName` is used
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
    pub fn get_matches_from_safe_borrow<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches<'a>>
        where I: IntoIterator<Item = T>,
              T: Into<OsString>
    {
        // Verify all positional assertions pass
        self.0.verify_positionals();
        // If there are global arguments, we need to propgate them down to subcommands
        // before parsing incase we run into a subcommand
        self.0.propogate_globals();

        let mut matcher = ArgMatcher::new();

        let mut it = itr.into_iter();
        // Get the name of the program (argument 1 of env::args()) and determine the
        // actual file
        // that was used to execute the program. This is because a program called
        // ./target/release/my_prog -a
        // will have two arguments, './target/release/my_prog', '-a' but we don't want
        // to display
        // the full path when displaying help messages and such
        if !self.0.is_set(AppSettings::NoBinaryName) {
            if let Some(name) = it.next() {
                let bn_os = name.into();
                let p = Path::new(&*bn_os);
                if let Some(f) = p.file_name() {
                    if let Some(s) = f.to_os_string().to_str() {
                        if let None = self.0.meta.bin_name {
                            self.0.meta.bin_name = Some(s.to_owned());
                        }
                    }
                }
            }
        }

        // do the real parsing
        if let Err(e) = self.0.get_matches_with(&mut matcher, &mut it) {
            return Err(e);
        }

        Ok(matcher.into())
    }

    // Re-implements ClapError::exit except it checks if we should wait for input before exiting
    // since ClapError doesn't have that info and the error message must be printed before exiting
    fn maybe_wait_for_exit(&self, e: Error) -> ! {
        if e.use_stderr() {
            wlnerr!("{}", e.message);
            if self.0.is_set(AppSettings::WaitOnError) {
                wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                let mut s = String::new();
                let i = io::stdin();
                i.lock().read_line(&mut s).unwrap();
            }
            process::exit(1);
        }

        e.exit()
    }
}
