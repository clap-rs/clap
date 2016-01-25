//! Command Line Argument Parser for Rust
//!
//! It is a simple to use, efficient, and full featured library for parsing
//! command line arguments
//! and subcommands when writing console, or terminal applications.
//!
//! ## About
//!
//! `clap` is used to parse *and validate* the string of command line arguments
//! provided by the
//! user at runtime. You provide the list of valid possibilities, and `clap`
//! handles the rest. This
//! means you focus on your *applications* functionality, and less on the
//! parsing and validating of
//! arguments.
//!
//! `clap` also provides the traditional version and help switches (or flags)
//! 'for free' meaning
//! automatically with no configuration. It does this by checking list of valid
//! possibilities you
//! supplied and if you haven't them already (or only defined some of them),
//! `clap` will auto-
//! generate the applicable ones. If you are using subcommands, `clap` will
//! also auto-generate a
//! `help` subcommand for you in addition to the traditional flags.
//!
//! Once `clap` parses the user provided string of arguments, it returns the
//! matches along with any
//! applicable values. If the user made an error or typo, `clap` informs them
//! of the mistake and
//! exits gracefully. Because of this, you can make reasonable assumptions in
//! your code about the
//! validity of the arguments.
//! ## FAQ
//!
//! For a full FAQ and more in depth details, see
//! [the wiki page](https://github.com/kbknapp/clap-rs/wiki/FAQ)
//!
//! ### Comparisons
//!
//! First, let me say that these comparisons are highly subjective, and not
//! meant
//! in a critical or harsh manner. All the argument parsing libraries out there
//! (to include `clap`) have their own strengths and weaknesses. Sometimes it
//! just
//! comes down to personal taste when all other factors are equal. When in
//! doubt,
//! try them all and pick one that you enjoy :) There's plenty of room in the
//! Rust
//! community for multiple implementations!
//!
//! #### How does `clap` compare to `getopts`?
//!
//! [getopts](https://github.com/rust-lang-nursery/getopts) is a very basic,
//! fairly
//! minimalist argument parsing library. This isn't a bad thing, sometimes you
//! don't need tons of features, you just want to parse some simple arguments,
//! and
//! have some help text generated for you based on valid arguments you specify.
//! When using `getopts` you must manually implement most of the common features
//! (such as checking to display help messages, usage strings, etc.). If you
//! want a
//! highly custom argument parser, and don't mind writing most the argument
//! parser
//! yourself, `getopts` is an excellent base.
//!
//! Due to it's lack of features, `getopts` also doesn't allocate much, or at
//! all.
//! This gives it somewhat of a performance boost. Although, as you start
//! implementing those features you need manually, that boost quickly
//! disappears.
//!
//! Personally, I find many, many people that use `getopts` are manually
//! implementing features that `clap` has by default. Using `clap` simplifies
//! your
//! codebase allowing you to focus on your application, and not argument
//! parsing.
//!
//! Reasons to use `getopts` instead of `clap`
//!
//!  * You need a few allocations as possible, don't plan on implementing any
//!    additional features
//!  * You want a highly custom argument parser, but want to use an established
//!    parser as a base
//!
//! #### How does `clap` compare to `docopt.rs`?
//!
//! I first want to say I'm a big a fan of BurntSushi's work, the creator of
//! [Docopt.rs](https://github.com/docopt/docopt.rs). I aspire to produce the
//! quality of libraries that this man does! When it comes to comparing these
//! two
//! libraries they are very different. `docopt` tasks you with writing a help
//! message, and then it parsers that message for you to determine all valid
//! arguments and their use. Some people LOVE this, others not so much. If
//! you're
//! willing to write a detailed help message, it's nice that you can stick that
//! in
//! your program and have `docopt` do the rest. On the downside, it's somewhat
//! less
//! flexible than other options out there, and requires the help message change
//! if
//! you need to make changes.
//!
//! `docopt` is also excellent at translating arguments into Rust types
//! automatically. There is even a syntax extension which will do all this for
//! you,
//! ifou to manually translate from arguments to Rust types). To use
//! BurntSushi's
//! words, `docopt` is also somewhat of a black box. You get what you get, and
//! it's
//! hard to tweak implementation or customise your experience for your use case.
//!
//! Because `docopt` is doing a ton of work to parse your help messages and
//! determine what you were trying to communicate as valid arguments, it's also
//! one
//! of the more heavy weight parsers performance-wise. For most applications
//! this
//! isn't a concern, but it's something to keep in mind.
//!
//! Reasons to use `docopt` instead of `clap`
//! * You want automatic translation from arguments to Rust types, and are
//! using a
//!    nightly compiler
//!  * Performance isn't a concern
//!  * You don't have any complex relationships between arguments
//!
//! #### All else being equal, what are some reasons to use `clap`?
//!
//! `clap` is fast, and as lightweight as possible while still giving all the
//! features you'd expect from a modern argument parser. If you use `clap` when
//! just need some simple arguments parsed, you'll find it a walk in the park.
//! But
//! `clap` also makes it possible to represent extremely complex, and advanced
//! requirements, without too much thought. `clap` aims to be intuitive, easy to
//! use, and fully capable for wide variety use cases and needs.
//!
//! ## Quick Examples
//!
//! The following examples show a quick example of some of the very basic
//! functionality of `clap`.
//! For more advanced usage, such as requirements, exclusions, groups, multiple
//! values and
//! occurrences see the [video tutorials][video tutorials],
//! [documentation][docs], or
//! [examples/][examples] directory of this crate's repository.
//!
//! **NOTE:** All these examples are functionally the same, but show three
//! different styles in
//! which to use `clap`
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01a_quick_example.rs)
//! //
//! // This example demonstrates clap's "usage strings" method of creating
//! // arguments which is less less verbose
//! extern crate clap;
//! use clap::{Arg, App, SubCommand};
//!
//! fn main() {
//!     let matches = App::new("myapp")
//!         .version("1.0")
//!         .author("Kevin K. <kbknapp@gmail.com>")
//!         .about("Does awesome things")
//!         .args_from_usage(
//!             "-c --config=[CONFIG] 'Sets a custom config file'
//!              <INPUT> 'Sets the input file to use'
//!              [debug]... -d 'Sets the level of debugging information'")
//!         .subcommand(SubCommand::with_name("test")
//!             .about("controls testing features")
//!             .version("1.3")
//!             .author("Someone E. <someone_else@other.com>")
//!             .arg_from_usage(
//!                 "-v --verbose 'Print test information verbosely'"))
//!         .get_matches();
//!
//!     // Calling .unwrap() is safe here because "INPUT" is required (if
//!     // "INPUT" wasn't required we could have used an 'if let' to
//!     // conditionally get the value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//!     // Gets a value for config if supplied by user, or defaults to
//!     // "default.conf"
//!     let config = matches.value_of("CONFIG").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//!     // Vary the output based on how many times the user used the "debug"
//!     // flag (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d')
//!     match matches.occurrences_of("debug") {
//!         0 => println!("Debug mode is off"),
//!         1 => println!("Debug mode is kind of on"),
//!         2 => println!("Debug mode is on"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//!     // You can information about subcommands by requesting their matches by
//!     // name (as below), requesting just the name used, or both at the same
//!     // time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("verbose") {
//!             println!("Printing verbosely...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more program logic goes here...
//! }
//! ```
//!
//! The following example is functionally the same as the one above, but this
//! method allows more
//! advanced configuration options (not shown in this small example), or even
//! dynamically
//! generating arguments when desired. Both methods can be used together to get
//! the best of both
//! worlds (see the documentation, [examples/][examples], or video tutorials).
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01b_quick_example.rs)
//! //
//! // This example demonstrates clap's full 'builder pattern' style of
//! // creating arguments which is
//! // more verbose, but allows easier editing, and at times more advanced
//! // options, or the possibility
//! // to generate arguments dynamically.
//! extern crate clap;
//! use clap::{Arg, App, SubCommand};
//!
//! fn main() {
//!     let matches = App::new("myapp")
//!         .version("1.0")
//!         .author("Kevin K. <kbknapp@gmail.com>")
//!         .about("Does awesome things")
//!         .arg(Arg::with_name("CONFIG")
//!             .short("c")
//!             .long("config")
//!             .help("Sets a custom config file")
//!             .takes_value(true))
//!         .arg(Arg::with_name("INPUT")
//!             .help("Sets the input file to use")
//!             .required(true)
//!             .index(1))
//!         .arg(Arg::with_name("debug")
//!             .short("d")
//!             .multiple(true)
//!             .help("Sets the level of debugging information"))
//!         .subcommand(SubCommand::with_name("test")
//!             .about("controls testing features")
//!             .version("1.3")
//!             .author("Someone E. <someone_else@other.com>")
//!             .arg(Arg::with_name("verbose")
//!                 .short("v")
//!                 .help("print test information verbosely")))
//!         .get_matches();
//!
//!     // Calling .unwrap() is safe here because "INPUT" is required (if
//!     // "INPUT" wasn't required we could have used an 'if let' to
//!     // conditionally get the value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//!     // Gets a value for config if supplied by user, or defaults to
//!     // "default.conf"
//!     let config = matches.value_of("CONFIG").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//!     // Vary the output based on how many times the user used the "debug"
//!     // flag (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d')
//!     match matches.occurrences_of("debug") {
//!         0 => println!("Debug mode is off"),
//!         1 => println!("Debug mode is kind of on"),
//!         2 => println!("Debug mode is on"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//!     // You can information about subcommands by requesting their matches by
//!     // name (as below), requesting just the name used, or both at the same
//!     // time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("verbose") {
//!             println!("Printing verbosely...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more program logic goes here...
//! }
//! ```
//!
//! The following combines the previous two examples by using the simplicity of
//! the `from_usage`
//! methods and the performance of the Builder Pattern.
//!
//! ```ignore
//! // (Full example with detailed comments in examples/01c_quick_example.rs)
//! //
//! // This example demonstrates clap's "usage strings" method of creating
//! // arguments which is less verbose
//! #[macro_use]
//! extern crate clap;
//!
//! fn main() {
//!     let matches = clap_app!(myapp =>
//!         (version: "1.0")
//!         (author: "Kevin K. <kbknapp@gmail.com>")
//!         (about: "Does awesome things")
//!         (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
//!         (@arg INPUT: +required "Sets the input file to use")
//!         (@arg debug: -d ... "Sets the level of debugging information")
//!         (@subcommand test =>
//!             (about: "controls testing features")
//!             (version: "1.3")
//!             (author: "Someone E. <someone_else@other.com>")
//!             (@arg verbose: -v --verbose "Print test information verbosely")
//!         )
//!     ).get_matches();
//!
//!     // Calling .unwrap() is safe here because "INPUT" is required (if
//!     // "INPUT" wasn't required we could have used an 'if let' to
//!     // conditionally get the value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//!     // Gets a value for config if supplied by user, or defaults to
//!     // "default.conf"
//!     let config = matches.value_of("CONFIG").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//!     // Vary the output based on how many times the user used the "debug"
//!     // flag (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d')
//!     match matches.occurrences_of("debug") {
//!         0 => println!("Debug mode is off"),
//!         1 => println!("Debug mode is kind of on"),
//!         2 => println!("Debug mode is on"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//!     // You can information about subcommands by requesting their matches by
//!     // name (as below), requesting just the name used, or both at the same
//!     // time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("verbose") {
//!             println!("Printing verbosely...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more program logic goes here...
//! }
//! ```
//!
//! This final method shows how you can use a YAML file to build your CLI and
//! keep your Rust source
//! tidy. First, create the `cli.yml` file to hold your CLI options, but it
//! could be called
//! anything we like (we'll use the same both examples above to keep it
//! functionally equivalent):
//!
//! ```yaml
//! name: myapp
//! version: 1.0
//! author: Kevin K. <kbknapp@gmail.com>
//! about: Does awesome things
//! args:
//!     - CONFIG:
//!         short: c
//!         long: config
//!         help: Sets a custom config file
//!         takes_value: true
//!     - INPUT:
//!         help: Sets the input file to use
//!         required: true
//!         index: 1
//!     - debug:
//!         short: d
//!         multiple: true
//!         help: Sets the level of debugging information
//! subcommands:
//!     - test:
//!         about: controls testing features
//!         version: 1.3
//!         author: Someone E. <someone_else@other.com>
//!         args:
//!             - verbose:
//!                 short: v
//!                 help: print test information verbosely
//! ```
//!
//! Now we create our `main.rs` file just like we would have with the previous
//! two examples:
//!
//! ```ignore
//! // (Full example with detailed comments in examples/17_yaml.rs)
//! //
//! // This example demonstrates clap's building from YAML style of creating
//! arguments which is far
//! // more clean, but takes a very small performance hit compared to the other
//! two methods.
//! #[macro_use]
//! extern crate clap;
//! use clap::App;
//!
//! fn main() {
//! // The YAML file is found relative to the current file, similar to how
//! modules are found
//!     let yaml = load_yaml!("cli.yml");
//!     let matches = App::from_yaml(yaml).get_matches();
//!
//! // Calling .unwrap() is safe here because "INPUT" is required (if
//! "INPUT" wasn't
//! // required we could have used an 'if let' to conditionally get the
//! value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//! // Gets a value for config if supplied by user, or defaults to
//! "default.conf"
//!     let config = matches.value_of("CONFIG").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//! // Vary the output based on how many times the user used the "debug"
//! flag
//!     // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
//!     match matches.occurrences_of("debug") {
//!         0 => println!("Debug mode is off"),
//!         1 => println!("Debug mode is kind of on"),
//!         2 => println!("Debug mode is on"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//! // You can information about subcommands by requesting their matches by
//! name
//!     // (as below), requesting just the name used, or both at the same time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("verbose") {
//!             println!("Printing verbosely...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more program logic goes here...
//! }
//! ```
//!
//! If you were to compile any of the above programs and run them with the flag
//! `--help` or `-h`
//! (or `help` subcommand, since we defined `test` as a subcommand) the
//! following would be output
//!
//! **NOTE**: The YAML option requires adding a special `features` flag when
//! compiling `clap`
//! because it is not compiled by default since it takes additional
//! dependencies that some people
//! may not need. Simply change your `clap = "1"` to `clap = {version = "1",
//! features = ["yaml"]}`
//! in your `Cargo.toml` to use the YAML version.
//!
//! ```text
//! $ myapp --help
//! myapp 1.0
//! Kevin K. <kbknapp@gmail.com>
//! Does awesome things
//!
//! USAGE:
//!     MyApp [FLAGS] [OPTIONS] <INPUT> [SUBCOMMAND]
//!
//! FLAGS:
//!     -d               Turn debugging information on
//!     -h, --help       Prints this message
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -c, --config <CONFIG>    Sets a custom config file
//!
//! ARGS:
//!     INPUT    The input file to use
//!
//! SUBCOMMANDS:
//!     help    Prints this message
//!     test    Controls testing features
//! ```
//!
//! **NOTE:** You could also run `myapp test --help` to see similar output and
//! options for the
//! `test` subcommand.
//!
//! ## Try it!
//!
//! ### Pre-Built Test
//!
//! To try out the pre-built example, use the following steps:
//!
//! * Clone the repo `$ git clone https://github.com/kbknapp/clap-rs && cd
//! clap-rs/clap-tests`
//! * Compile the example `$ cargo build --release`
//! * Run the help info `$ ./target/release/claptests --help`
//! * Play with the arguments!
//!
//! ### BYOB (Build Your Own Binary)
//!
//! To test out `clap`'s default auto-generated help/version follow these steps:
//! * Create a new cargo project `$ cargo new fake --bin && cd fake`
//! * Add `clap` to your `Cargo.toml`
//! *
//! ```toml
//! [dependencies]
//! clap = "1"
//! ```
//!
//! * Add the following to your `src/main.rs`
//!
//! ```no_run
//! extern crate clap;
//! use clap::App;
//!
//! fn main() {
//!   let _ = App::new("fake").version("v1.0-beta").get_matches();
//! }
//! ```
//!
//! * Build your program `$ cargo build --release`
//! * Run w/ help or version `$ ./target/release/fake --help` or `$
//! ./target/release/fake --version`
//!
//! ## Usage
//!
//! For full usage, add `clap` as a dependency in your `Cargo.toml` file to use
//! from crates.io:
//!
//!  ```toml
//!  [dependencies]
//!  clap = "1"
//!  ```
//!  Or track the latest on the master branch at github:
//!
//! ```toml
//! [dependencies.clap]
//! git = "https://github.com/kbknapp/clap-rs.git"
//! ```
//!
//! Add `extern crate clap;` to your crate root.
//!
//! Define a list of valid arguments for your program (see the
//! [documentation][docs] or
//! [examples/][examples] directory of this repo)
//!
//! Then run `cargo build` or `cargo update && cargo build` for your project.
//!
//! ### Optional Dependencies / Features
//!
//! If you'd like to keep your dependency list to **only** `clap`, you can
//! disable any features
//! that require an additional dependency. To do this, add this to your
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies.clap]
//! version = "1"
//! default-features = false
//! ```
//!
//! You can also selectively enable only the features you'd like to include, by
//! adding:
//!
//! ```toml
//! [dependencies.clap]
//! version = "1"
//! default-features = false
//!
//! # Cherry-pick the features you'd like to use
//! features = [ "suggestions", "color" ]
//! ```
//!
//! The following is a list of optional `clap` features:
//!
//! * **"suggestions"**: Turns on the `Did you mean '--myoption' ?` feature for
//! when users make
//! typos.
//! * **"color"**: Turns on red error messages. This feature only works on
//! non-Windows OSs.
//! * **"lints"**: This is **not** included by default and should only be used
//! while developing to
//! run basic lints against changes. This can only be used on Rust nightly.
//!
//! ### Dependencies Tree
//!
//! The following graphic depicts `clap`s dependency graph.
//!
//!  * **Dashed** Line: Optional dependency
//! * **Red** Color: **NOT** included by default (must use cargo `features` to
//! enable)
//!
//! ![clap dependencies](https://raw.githubusercontent.
//! com/kbknapp/clap-rs/master/clap.png)
//!
//! ### More Information
//!
//! You can find complete documentation on the [github-pages site][docs] for
//! this project.
//!
//! You can also find usage examples in the [examples/][examples] directory of
//! this repo.
//!
//! #### Video Tutorials
//!
//! There's also the video tutorial series [Argument Parsing with Rust][video
//! tutorials] that I've
//! been working on.
//!
//! *Note*: Two new videos have just been added ([08 From
//! Usage](https://youtu.be/xc6VdedFrG0), and
//! [09 Typed Values](https://youtu.be/mZn3C1DnD90)), if you're already
//! familiar with `clap` but
//! want to know more about these two details you can check out those videos
//! without watching the
//! previous few.
//!
//! *Note*: Apologies for the resolution of the first video, it will be updated
//! to a better
//! resolution soon. The other videos have a proper resolution.
//!
//! ### Running the tests
//!
//! If contributing, you can run the tests as follows (assuming you're in the
//! `clap-rs` directory)
//!
//! ```sh
//! cargo test --features yaml && make -C clap-tests test
//! ```
//!
//! ## License
//!
//! `clap` is licensed under the MIT license. Please read the
//! [LICENSE-MIT][license]
//! file in
//! this repository for more information.
//!
//! [examples]: https://github.com/kbknapp/clap-rs/tree/master/examples
//! [docs]: http://kbknapp.github.io/clap-rs/clap/index.html
//! [video tutorials]:
//! https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv
//! [license]:
//! https://raw.githubusercontent.com/kbknapp/clap-rs/master/LICENSE-MIT

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny(unstable_features))]
#![deny(
        missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]
// clippy false positives, or ones we're ok with...
#![cfg_attr(feature = "lints", allow(cyclomatic_complexity))]
// Only while bitflats uses "_first" inside it's macros
 #![cfg_attr(feature = "lints", allow(used_underscore_binding))]

#[cfg(feature = "suggestions")]
extern crate strsim;
#[cfg(feature = "color")]
extern crate ansi_term;
#[cfg(feature = "yaml")]
extern crate yaml_rust;
#[macro_use]
extern crate bitflags;
extern crate vec_map;

#[cfg(feature = "yaml")]
pub use yaml_rust::YamlLoader;
pub use args::{Arg, ArgGroup, ArgMatches, SubCommand, ArgSettings};
pub use app::{App, AppSettings};
pub use fmt::Format;
pub use errors::{Error, ErrorKind};

#[macro_use]
mod macros;
mod app;
mod args;
mod usage_parser;
mod fmt;
mod suggestions;
mod errors;
mod utf8;
mod osstringext;

const INTERNAL_ERROR_MSG: &'static str = "Fatal internal error. Please consider filing a bug \
                                          report at https://github.com/kbknapp/clap-rs/issues";
