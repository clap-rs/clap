// Copyright ⓒ 2015-2016 Kevin B. Knapp and [`clap-rs` contributors](https://github.com/kbknapp/clap-rs/blob/master/CONTRIBUTORS.md).
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

//! A simple to use, efficient, and full featured library for parsing command line arguments and subcommands when writing console, or terminal applications.
//!
//! ## About
//!
//! `clap` is used to parse *and validate* the string of command line arguments provided by the user at runtime. You provide the list of valid possibilities, and `clap` handles the rest. This means you focus on your *applications* functionality, and less on the parsing and validating of arguments.
//!
//! `clap` also provides the traditional version and help switches (or flags) 'for free' meaning automatically with no configuration. It does this by checking list of valid possibilities you supplied and adding only the ones you haven't already defined. If you are using subcommands, `clap` will also auto-generate a `help` subcommand for you in addition to the traditional flags.
//!
//! Once `clap` parses the user provided string of arguments, it returns the matches along with any applicable values. If the user made an error or typo, `clap` informs them of the mistake and exits gracefully (or returns a `Result` type and allows you to perform any clean up prior to exit). Because of this, you can make reasonable assumptions in your code about the validity of the arguments.
//!
//! ## FAQ
//!
//! For a full FAQ and more in depth details, see [the wiki page](https://github.com/kbknapp/clap-rs/wiki/FAQ)
//!
//! ### Comparisons
//!
//! First, let me say that these comparisons are highly subjective, and not meant in a critical or harsh manner. All the argument parsing libraries out there (to include `clap`) have their own strengths and weaknesses. Sometimes it just comes down to personal taste when all other factors are equal. When in doubt, try them all and pick one that you enjoy :) There's plenty of room in the Rust community for multiple implementations!
//!
//! #### How does `clap` compare to [getopts](https://github.com/rust-lang-nursery/getopts)?
//!
//! `getopts` is a very basic, fairly minimalist argument parsing library. This isn't a bad thing, sometimes you don't need tons of features, you just want to parse some simple arguments, and have some help text generated for you based on valid arguments you specify. The downside to this approach is that you must manually implement most of the common features (such as checking to display help messages, usage strings, etc.). If you want a highly custom argument parser, and don't mind writing the majority of the functionality yourself, `getopts` is an excellent base.
//!
//! `getopts` also doesn't allocate much, or at all. This gives it a very small performance boost. Although, as you start implementing additional features, that boost quickly disappears.
//!
//! Personally, I find many, many uses of `getopts` are manually implementing features that `clap` provides by default. Using `clap` simplifies your codebase allowing you to focus on your application, and not argument parsing.
//!
//! #### How does `clap` compare to [docopt.rs](https://github.com/docopt/docopt.rs)?
//!
//! I first want to say I'm a big a fan of BurntSushi's work, the creator of `Docopt.rs`. I aspire to produce the quality of libraries that this man does! When it comes to comparing these two libraries they are very different. `docopt` tasks you with writing a help message, and then it parsers that message for you to determine all valid arguments and their use. Some people LOVE this approach, others do not. If you're willing to write a detailed help message, it's nice that you can stick that in your program and have `docopt` do the rest. On the downside, it's far less flexible.
//!
//! `docopt` is also excellent at translating arguments into Rust types automatically. There is even a syntax extension which will do all this for you, if you're willing to use a nightly compiler (use of a stable compiler requires you to somewhat manually translate from arguments to Rust types). To use BurntSushi's words, `docopt` is also a sort of black box. You get what you get, and it's hard to tweak implementation or customize the experience for your use case.
//!
//! Because `docopt` is doing a ton of work to parse your help messages and determine what you were trying to communicate as valid arguments, it's also one of the more heavy weight parsers performance-wise. For most applications this isn't a concern and this isn't to say `docopt` is slow, in fact from it. This is just something to keep in mind while comparing.
//!
//! #### All else being equal, what are some reasons to use `clap`?
//!
//! `clap` is as fast, and as lightweight as possible while still giving all the features you'd expect from a modern argument parser. In fact, for the amount and type of features `clap` offers it remains about as fast as `getopts`. If you use `clap` when just need some simple arguments parsed, you'll find its a walk in the park. `clap` also makes it possible to represent extremely complex, and advanced requirements, without too much thought. `clap` aims to be intuitive, easy to use, and fully capable for wide variety use cases and needs.
//!
//! ## Quick Example
//!
//! The following examples show a quick example of some of the very basic functionality of `clap`. For more advanced usage, such as requirements, conflicts, groups, multiple values and occurrences see the [documentation](http://kbknapp.github.io/clap-rs/clap/index.html), [examples/](examples) directory of this repository or the [video tutorials](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv) (which are quite outdated by now).
//!
//!  **NOTE:** All these examples are functionally the same, but show three different styles in which to use `clap`
//!
//! The following example is show a method that allows more advanced configuration options (not shown in this small example), or even dynamically generating arguments when desired. The downside is it's more verbose.
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01b_quick_example.rs)
//! //
//! // This example demonstrates clap's full 'builder pattern' style of creating arguments which is
//! // more verbose, but allows easier editing, and at times more advanced options, or the possibility
//! // to generate arguments dynamically.
//! extern crate clap;
//! use clap::{Arg, App, SubCommand};
//!
//! fn main() {
//!     let matches = App::new("My Super Program")
//!                           .version("1.0")
//!                           .author("Kevin K. <kbknapp@gmail.com>")
//!                           .about("Does awesome things")
//!                           .arg(Arg::with_name("config")
//!                                .short("c")
//!                                .long("config")
//!                                .value_name("FILE")
//!                                .help("Sets a custom config file")
//!                                .takes_value(true))
//!                           .arg(Arg::with_name("INPUT")
//!                                .help("Sets the input file to use")
//!                                .required(true)
//!                                .index(1))
//!                           .arg(Arg::with_name("v")
//!                                .short("v")
//!                                .multiple(true)
//!                                .help("Sets the level of verbosity"))
//!                           .subcommand(SubCommand::with_name("test")
//!                                       .about("controls testing features")
//!                                       .version("1.3")
//!                                       .author("Someone E. <someone_else@other.com>")
//!                                       .arg(Arg::with_name("debug")
//!                                           .short("d")
//!                                           .help("print debug information verbosely")))
//!                           .get_matches();
//!
//!     // Gets a value for config if supplied by user, or defaults to "default.conf"
//!     let config = matches.value_of("config").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//!     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//!     // required we could have used an 'if let' to conditionally get the value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//!     // Vary the output based on how many times the user used the "verbose" flag
//!     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//!     match matches.occurrences_of("v") {
//!         0 => println!("No verbose info"),
//!         1 => println!("Some verbose info"),
//!         2 => println!("Tons of verbose info"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//!     // You can handle information about subcommands by requesting their matches by name
//!     // (as below), requesting just the name used, or both at the same time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("debug") {
//!             println!("Printing debug info...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more program logic goes here...
//! }
//! ```
//!
//! The following example is functionally the same as the one above, but shows a far less verbose method but sacrifices some of the advanced configuration options (not shown in this small example).
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01a_quick_example.rs)
//! //
//! // This example demonstrates clap's "usage strings" method of creating arguments which is less
//! // less verbose
//! extern crate clap;
//! use clap::{Arg, App, SubCommand};
//!
//! fn main() {
//!     let matches = App::new("myapp")
//!                           .version("1.0")
//!                           .author("Kevin K. <kbknapp@gmail.com>")
//!                           .about("Does awesome things")
//!                           .args_from_usage(
//!                               "-c, --config=[FILE] 'Sets a custom config file'
//!                               <INPUT>              'Sets the input file to use'
//!                               -v...                'Sets the level of verbosity'")
//!                           .subcommand(SubCommand::with_name("test")
//!                                       .about("controls testing features")
//!                                       .version("1.3")
//!                                       .author("Someone E. <someone_else@other.com>")
//!                                       .arg_from_usage("-d, --debug 'Print debug information'"))
//!                           .get_matches();
//!
//!     // Same as previous example...
//! }
//! ```
//!
//! The following combines the previous two examples by using the less verbose `from_usage` methods and the performance of the Builder Pattern.
//!
//! ```ignore
//! // (Full example with detailed comments in examples/01c_quick_example.rs)
//! // Must be compiled with `--features unstable`
//! //
//! // This example demonstrates clap's "usage strings" method of creating arguments which is less
//! // less verbose
//! #[macro_use]
//! extern crate clap;
//!
//! fn main() {
//!     let matches = clap_app!(myapp =>
//!         (version: "1.0")
//!         (author: "Kevin K. <kbknapp@gmail.com>")
//!         (about: "Does awesome things")
//!         (@arg config: -c --config +takes_value "Sets a custom config file")
//!         (@arg INPUT: +required "Sets the input file to use")
//!         (@arg verbose: -v ... "Sets the level of verbosity")
//!         (@subcommand test =>
//!             (about: "controls testing features")
//!             (version: "1.3")
//!             (author: "Someone E. <someone_else@other.com>")
//!             (@arg verbose: -d --debug "Print debug information")
//!         )
//!     ).get_matches();
//!
//! // Same as previous examples...
//! }
//! ```
//!
//! This final method shows how you can use a YAML file to build your CLI and keep your Rust source tidy or support multiple localized translations by having different YAML files for each localization. First, create the `cli.yml` file to hold your CLI options, but it could be called anything we like (we'll use the same both examples above to keep it functionally equivalent):
//!
//! ```yaml
//! name: myapp
//! version: 1.0
//! author: Kevin K. <kbknapp@gmail.com>
//! about: Does awesome things
//! args:
//!     - config:
//!         short: c
//!         long: config
//!         value_name: FILE
//!         help: Sets a custom config file
//!         takes_value: true
//!     - INPUT:
//!         help: Sets the input file to use
//!         required: true
//!         index: 1
//!     - verbose:
//!         short: v
//!         multiple: true
//!         help: Sets the level of verbosity
//! subcommands:
//!     - test:
//!         about: controls testing features
//!         version: 1.3
//!         author: Someone E. <someone_else@other.com>
//!         args:
//!             - debug:
//!                 short: d
//!                 help: print debug information
//! ```
//!
//! Now we create our `main.rs` file just like we would have with the previous two examples:
//!
//! ```ignore
//! // (Full example with detailed comments in examples/17_yaml.rs)
//! //
//! // This example demonstrates clap's building from YAML style of creating arguments which is far
//! // more clean, but takes a very small performance hit compared to the other two methods.
//! #[macro_use]
//! extern crate clap;
//! use clap::App;
//!
//! fn main() {
//!     // The YAML file is found relative to the current file, similar to how modules are found
//!     let yaml = load_yaml!("cli.yml");
//!     let matches = App::from_yaml(yaml).get_matches();
//!
//!     // Same as previous examples...
//! }
//! ```
//!
//! **NOTE**: The YAML and macro builder options require adding a special `features` flag when compiling `clap` because they are not compiled by default. Simply change your `clap = "2"` to `clap = {version = "2", features = ["yaml"]}` for YAML, or `features = ["unstable"]` for the macro builder, in your `Cargo.toml`.
//!
//! If you were to compile any of the above programs and run them with the flag `--help` or `-h` (or `help` subcommand, since we defined `test` as a subcommand) the following would be output
//!
//! ```ignore
//! $ myprog --help
//! My Super Program 1.0
//! Kevin K. <kbknapp@gmail.com>
//! Does awesome things
//!
//! USAGE:
//!     MyApp [FLAGS] [OPTIONS] <INPUT> [SUBCOMMAND]
//!
//! FLAGS:
//!     -h, --help       Prints this message
//!     -v               Sets the level of verbosity
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -c, --config <FILE>    Sets a custom config file
//!
//! ARGS:
//!     INPUT    The input file to use
//!
//! SUBCOMMANDS:
//!     help    Prints this message
//!     test    Controls testing features
//! ```
//!
//! **NOTE:** You could also run `myapp test --help` to see similar output and options for the `test` subcommand.
//!
//! ## Try it!
//!
//! ### Pre-Built Test
//!
//! To try out the pre-built example, use the following steps:
//!
//! * Clone the repository `$ git clone https://github.com/kbknapp/clap-rs && cd clap-rs/clap-tests`
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
//! clap = "2"
//! ```
//!
//! * Add the following to your `src/main.rs`
//!
//! ```ignore
//! extern crate clap;
//! use clap::App;
//!
//! fn main() {
//!   App::new("fake").version("v1.0-beta").get_matches();
//! }
//! ```
//!
//! * Build your program `$ cargo build --release`
//! * Run with help or version `$ ./target/release/fake --help` or `$ ./target/release/fake --version`
//!
//! ## Usage
//!
//! For full usage, add `clap` as a dependency in your `Cargo.toml` file to use from crates.io:
//!
//!  ```toml
//!  [dependencies]
//!  clap = "2"
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
//! Define a list of valid arguments for your program (see the [documentation](https://kbknapp.github.io/clap-rs/index.html) or [examples/](examples) directory of this repo)
//!
//! Then run `cargo build` or `cargo update && cargo build` for your project.
//!
//! ### Optional Dependencies / Features
//!
//! If you'd like to keep your dependency list to **only** `clap`, you can disable any features that require an additional dependency. To do this, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.clap]
//! version = "2"
//! default-features = false
//! ```
//!
//! You can also selectively enable only the features you'd like to include, by adding:
//!
//! ```toml
//! [dependencies.clap]
//! version = "2"
//! default-features = false
//!
//! # Cherry-pick the features you'd like to use
//! features = [ "suggestions", "color" ]
//! ```
//!
//! The following is a list of optional `clap` features:
//!
//! * **"suggestions"**: Turns on the `Did you mean '--myoption' ?` feature for when users make typos.
//! * **"color"**: Turns on colored error messages. This feature only works on non-Windows OSs.
//! * **"lints"**: This is **not** included by default and should only be used while developing to run basic lints against changes. This can only be used on Rust nightly.
//! * **"debug"**: This is **not** included by default and should only be used while developing to display debugging information.
//! * **"yaml"**: This is **not** included by default. Enables building CLIs from YAML documents.
//! * **"unstable"**: This is **not** included by default. Enables unstable features, unstable refers to whether or not they may change, not performance stability.
//!
//! ### More Information
//!
//! You can find complete documentation on the [github-pages site](http://kbknapp.github.io/clap-rs/clap/index.html) for this project.
//!
//! You can also find usage examples in the [examples/](examples) directory of this repo.
//!
//! #### Video Tutorials
//!
//! There's also the video tutorial series [Argument Parsing with Rust](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv) that I've been working on.
//!
//! *Note*: Two new videos have just been added ([08 From Usage](https://youtu.be/xc6VdedFrG0), and [09 Typed Values](https://youtu.be/mZn3C1DnD90)), if you're already familiar with `clap` but want to know more about these two details you can check out those videos without watching the previous few.
//!
//! *Note*: Apologies for the resolution of the first video, it will be updated to a better resolution soon. The other videos have a proper resolution.
//!
//! ### Running the tests
//!
//! If contributing, you can run the tests as follows (assuming you're in the `clap-rs` directory)
//!
//! ```ignore
//! $ cargo test && make -C clap-tests test
//! $ cargo test --features yaml
//!
//! # Only on nightly compiler:
//! $ cargo build --features lints
//! ```
//!
//! ## License
//!
//! `clap` is licensed under the MIT license. Please read the [LICENSE-MIT](LICENSE-MIT) file in this repository for more information.

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
        unused_allocation,
        unused_qualifications)]
// clippy false positives, or ones we're ok with...
#![cfg_attr(feature = "lints", allow(cyclomatic_complexity))]
#![cfg_attr(feature = "lints", allow(doc_markdown))]
// Only while bitflats uses "_first" inside it's macros
#![cfg_attr(feature = "lints", allow(used_underscore_binding))]
// Only while bitflats fails this lint
#![cfg_attr(feature = "lints", allow(if_not_else))]

#[cfg(feature = "suggestions")]
extern crate strsim;
#[cfg(feature = "color")]
extern crate ansi_term;
#[cfg(feature = "yaml")]
extern crate yaml_rust;
#[cfg(all(feature = "wrap_help", not(target_os = "windows")))]
extern crate libc;
#[cfg(all(feature = "wrap_help", not(target_os = "windows")))]
extern crate unicode_width;
#[macro_use]
extern crate bitflags;
extern crate vec_map;

#[cfg(feature = "yaml")]
pub use yaml_rust::YamlLoader;
pub use args::{Arg, ArgGroup, ArgMatches, ArgSettings, SubCommand, Values, OsValues};
pub use app::{App, AppSettings};
pub use fmt::Format;
pub use errors::{Error, ErrorKind, Result};

#[macro_use]
mod macros;
mod app;
mod args;
mod usage_parser;
mod fmt;
mod suggestions;
mod errors;
mod osstringext;
mod term;
mod strext;

const INTERNAL_ERROR_MSG: &'static str = "Fatal internal error. Please consider filing a bug \
                                          report at https://github.com/kbknapp/clap-rs/issues";
const INVALID_UTF8: &'static str = "unexpected invalid UTF-8 code point";
