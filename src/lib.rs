#![crate_type= "lib"]

// DOCS
//! # clap
//!
//! ![Travis-CI](https://travis-ci.org/kbknapp/clap-rs.svg?branch=master) [![Crates.io](https://img.shields.io/crates/v/clap.svg)]() [![Crates.io](https://img.shields.io/crates/l/clap.svg)]() [![Join the chat at https://gitter.im/kbknapp/clap-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/clap-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
//!
//! Command Line Argument Parser for Rust
//!
//! It is a simple to use, efficient, and full featured library for parsing command line arguments and subcommands when writing console, or terminal applications.
//!
//! ## About
//!
//! `clap` is used to parse *and validate* the string of command line arguments provided by the user at runtime. You provide the list of valid possibilities, and `clap` handles the rest. This means you focus on your *applications* functionality, and less on the parsing and validating of arguments.
//!
//! `clap` also provides the traditional version and help switches (or flags) 'for free' meaning automatically with no configuration. It does this by checking list of valid possibilities you supplied and if you haven't them already (or only defined some of them), `clap` will auto-generate the applicable ones. If you are using subcommands, `clap` will also auto-generate a `help` subcommand for you in addition to the traditional flags.
//!
//! Once `clap` parses the user provided string of arguments, it returns the matches along with any applicable values. If the user made an error or typo, `clap` informs them of the mistake and exits gracefully. Because of this, you can make reasonable assumptions in your code about the validity of the arguments.
//!
//! ## Features
//!
//! Below are a few of the features which `clap` supports, full descriptions and usage can be found in the [documentation](http://kbknapp.github.io/clap-rs/clap/index.html) and `examples/` directory
//!
//! * **Auto-generated Help, Version, and Usage information**
//!   - Can optionally be fully, or partially overridden if you want a custom help, version, or usag
//! * **Flags / Switches** (i.e. bool fields)
//!   - Both short and long versions supported (i.e. `-f` and `--flag` respectively)
//!   - Supports combining short versions (i.e. `-fBgoZ` is the same as `-f -B -g -o -Z`)
//!   - Optionally supports multiple occurrences (i.e. `-vvv` or `-v -v -v`)
//! * **Positional Arguments** (i.e. those which are based off an index from the program name)
//!   - Optionally supports multiple values (i.e. `myprog <file>...` such as `myprog file1.txt file2.txt` being two values for the same "file" argument)
//!   - Optionally supports Specific Value Sets (See below)
//!   - Supports the unix `--` meaning, only positional arguments follow
//!   - Optionally sets value parameters (such as the minimum number of values, the maximum number of values, or the exact number of values)
//! * **Option Arguments** (i.e. those that take values as options)
//!   - Both short and long versions supported (i.e. `-o value` and `--option value` or `--option=value` respectively)
//!   - Optionally supports multiple values (i.e. `-o <value> -o <other_value>` or the shorthand `-o <value> <other_value>`)
//!   - Optionally supports Specific Value Sets (See below)
//!   - Optionally supports named values so that the usage/help info appears as `-o <name> <other_name>` etc. for when you require specific multiple values
//!   - Optionally sets value parameters (such as the minimum number of values, the maximum number of values, or the exact number of values)
//! * **Sub-Commands** (i.e. `git add <file>` where `add` is a sub-command of `git`)
//!   - Support their own sub-arguments, and sub-sub-commands independent of the parent
//!   - Get their own auto-generated Help, Version, and Usage independent of parent
//! * **Requirement Rules**: Arguments can optionally define the following types of requirement rules
//!   - Required by default
//!   - Required only if certain arguments are present
//!   - Can require other arguments to be present
//! * **Exclusion/Confliction Rules**: Arguments can optionally define the following types of exclusion rules
//!   - Can be disallowed when certain arguments are present
//!   - Can disallow use of other arguments when present
//! * **Groups**: Arguments can optionally be made part of a group which means one, and only one argument from this "group" may be present at runtime
//!   - Fully compatible with other relational rules (requirements and exclusions) which allows things like requiring the use of a group, or denying the use of a group conditionally
//! * **Specific Value Sets**: Positional or Option Arguments can optionally define a specific set of allowed values (i.e. imagine a `--mode` option which may *only* have one of two values `fast` or `slow` such as `--mode fast` or `--mode slow`)
//! * **Default Values**: Although not specifically provided by `clap` you can achieve this exact functionality from Rust's `Option<&str>.unwrap_or("some default")` method (or `Result<T,String>.unwrap_or(T)` when using typed values)
//! * **Automatic Version from Cargo.toml**: `clap` is fully compatible with Rust's `env!()` macro for automatically setting the version of your application to the version in your Cargo.toml. See `examples/09_AutoVersion.rs` for how to do this (Thanks to [jhelwig](https://github.com/jhelwig) for pointing this out)
//! * **Typed Values**: You can use several convenience macros provided by `clap` to get typed values (i.e. `i32`, `u8`, etc.) from positional or option arguments so long as the type you request implements `std::str::FromStr` See the `examples/12_TypedValues.rs`. You can also use `clap`s `simple_enum!` or `arg_enum!` macro to create an enum with variants that automatically implements `std::str::FromStr`. See `examples/13a_EnumValuesAutomatic.rs` for details and performs an ascii case insensitive parse from a `string`->`enum`.
//! * **Suggestions**: Suggests corrections when the user enter's a typo. For example, if you defined a `--myoption <value>` argument, and the user mistakenly typed `--moyption value` (notice `y` and `o` switched), they would receive a `Did you mean '--myoption' ?` error and exit gracefully. This also works for subcommands and flags. (Thanks to [Byron](https://github.com/Byron) for the implementation) (This feature can optionally be disabled, see 'Optional Dependencies / Features')
//! * **Colorized (Red) Errors**: Error message are printed in red text (this feature can optionally be disabled, see 'Optional Dependencies / Features').
//! * **Global Arguments**: Arguments can optionally be defined once, and be available to all child subcommands.
//!
//! ## Quick Example
//!
//! The following two examples show a quick example of some of the very basic functionality of `clap`. For more advanced usage, such as requirements, exclusions, groups, multiple values and occurrences see the [video tutorials](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv), [documentation](http://kbknapp.github.io/clap-rs/clap/index.html), or `examples/` directory of this repository.
//!
//!  *NOTE:* Both examples are functionally the same, but show two different styles in which to use `clap`
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01a_QuickExample.rs)
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
//!                               "-c --config=[CONFIG] 'Sets a custom config file'
//!                               <INPUT> 'Sets the input file to use'
//!                               [debug]... -d 'Sets the level of debugging information'")
//!                           .subcommand(SubCommand::with_name("test")
//!                                       .about("controls testing features")
//!                                       .version("1.3")
//!                                       .author("Someone E. <someone_else@other.com>")
//!                                       .arg_from_usage("-v --verbose 'Print test information verbosely'"))
//!                           .get_matches();
//!
//!     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//!     // required we could have used an 'if let' to conditionally get the value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//!     // Gets a value for config if supplied by user, or defaults to "default.conf"
//!     let config = matches.value_of("CONFIG").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//!     // Vary the output based on how many times the user used the "debug" flag
//!     // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
//!     match matches.occurrences_of("debug") {
//!         0 => println!("Debug mode is off"),
//!         1 => println!("Debug mode is kind of on"),
//!         2 => println!("Debug mode is on"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//!     // You can information about subcommands by requesting their matches by name
//!     // (as below), requesting just the name used, or both at the same time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("verbose") {
//!             println!("Printing verbosely...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more porgram logic goes here...
//! }
//! ```
//!
//! The following example is functionally the same as the one above, but this method allows more advanced configuration options (not shown in this small example), or even dynamically generating arguments when desired. Both methods can be used together to get the best of both worlds (see the documentation, examples, or video tutorials).
//!
//! ```no_run
//! // (Full example with detailed comments in examples/01b_QuickExample.rs)
//! //
//! // This example demonstrates clap's full 'builder pattern' style of creating arguments which is
//! // more verbose, but allows easier editting, and at times more advanced options, or the possibility
//! // to generate arguments dynamically.
//! extern crate clap;
//! use clap::{Arg, App, SubCommand};
//!
//! fn main() {
//!     let matches = App::new("myapp")
//!                           .version("1.0")
//!                           .author("Kevin K. <kbknapp@gmail.com>")
//!                           .about("Does awesome things")
//!                           .arg(Arg::with_name("CONFIG")
//!                                .short("c")
//!                                .long("config")
//!                                .help("Sets a custom config file")
//!                                .takes_value(true))
//!                           .arg(Arg::with_name("INPUT")
//!                                .help("Sets the input file to use")
//!                                .required(true)
//!                                .index(1))
//!                           .arg(Arg::with_name("debug")
//!                                .short("d")
//!                                .multiple(true)
//!                                .help("Sets the level of debugging information"))
//!                           .subcommand(SubCommand::with_name("test")
//!                                       .about("controls testing features")
//!                                       .version("1.3")
//!                                       .author("Someone E. <someone_else@other.com>")
//!                                       .arg(Arg::with_name("verbose")
//!                                           .short("v")
//!                                           .help("print test information verbosely")))
//!                           .get_matches();
//!
//!     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//!     // required we could have used an 'if let' to conditionally get the value)
//!     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//!
//!     // Gets a value for config if supplied by user, or defaults to "default.conf"
//!     let config = matches.value_of("CONFIG").unwrap_or("default.conf");
//!     println!("Value for config: {}", config);
//!
//!     // Vary the output based on how many times the user used the "debug" flag
//!     // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
//!     match matches.occurrences_of("debug") {
//!         0 => println!("Debug mode is off"),
//!         1 => println!("Debug mode is kind of on"),
//!         2 => println!("Debug mode is on"),
//!         3 | _ => println!("Don't be crazy"),
//!     }
//!
//!     // You can information about subcommands by requesting their matches by name
//!     // (as below), requesting just the name used, or both at the same time
//!     if let Some(matches) = matches.subcommand_matches("test") {
//!         if matches.is_present("verbose") {
//!             println!("Printing verbosely...");
//!         } else {
//!             println!("Printing normally...");
//!         }
//!     }
//!
//!     // more porgram logic goes here...
//! }
//! ```
//!
//! If you were to compile either of the above programs and run them with the flag `--help` or `-h` (or `help` subcommand, since we defined `test` as a subcommand) the following would be output
//!
//! ```ignore
//! $ myapp --help
//! myapp 1.0
//! Kevin K. <kbknapp@gmail.com>
//! Does awesome things
//!
//! USAGE:
//!     MyApp [FLAGS] [OPTIONS] <INPUT> [SUBCOMMANDS]
//!
//! FLAGS:
//!     -d               Turn debugging information on
//!     -h, --help       Prints this message
//!     -v, --version    Prints version information
//!
//! OPTIONS:
//!     -c, --config <CONFIG>    Sets a custom config file
//!
//! POSITIONAL ARGUMENTS:
//!     INPUT    The input file to use
//!
//! SUBCOMMANDS:
//!     help    Prints this message
//!     test    Controls testing features
//! ```
//!
//! *NOTE:* You could also run `myapp test --help` to see similar output and options for the `test` subcommand.
//!
//! ## Try it!
//!
//! ### Pre-Built Test
//!
//! To try out the pre-built example use the following stes:
//!
//! * Clone the repostiory `$ git clone https://github.com/kbknapp/clap-rs && cd clap-rs/clap-tests`
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
//! ```ignore
//! [dependencies]
//! clap = "*"
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
//! * Run with help or version `$ ./target/release/fake --help` or `$ ./target/release/fake --version`
//!
//! ## Usage
//!
//! For full usage, add `clap` as a dependecy in your `Cargo.toml` file to use from crates.io:
//!
//!  ```ignore
//!  [dependencies]
//!  clap = "*"
//!  ```
//!  Or track the latest on the master branch at github:
//!
//! ```ignore
//! [dependencies.clap]
//! git = "https://github.com/kbknapp/clap-rs.git"
//! ```
//!
//! Add `extern crate clap;` to your crate root.
//!
//! Define a list of valid arguments for your program (see the [documentation](https://kbknapp.github.io/clap-rs/index.html) or `examples/` directory of this repo)
//!
//! Then run `cargo build` or `cargo update && cargo build` for your project.
//!
//! ### Optional Dependencies / Features
//!
//! If you'd like to keep your dependency list to **only** `clap`, you can disable any features that require an additional dependency. To do this, add this to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies.clap]
//! version = "*"
//! default-features = false
//! ```
//!
//! You can also selectively enable only the features you'd like to include, by adding:
//!
//! ```ignore
//! [dependencies.clap]
//! version = "*"
//! default-features = false
//!
//! # Cherry-pick the features you'd like to use
//! features = [ "suggestions", "color" ]
//! ```
//!
//! The following is a list of optional `clap` features:
//!
//! * **"suggestions"**: Turns on the `Did you mean '--myoption' ?` feature for when users make typos.
//! * **"color"**: Turns on red error messages.
//!
//! ### More Information
//!
//! You can find complete documentation on the [github-pages site](http://kbknapp.github.io/clap-rs/clap/index.html) for this project.
//!
//! You can also find usage examples in the `examples/` directory of this repo.
//!
//! #### Video Tutorials
//!
//! There's also the video tutorial series [Argument Parsing with Rust](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv) that I've been working on.
//!
//! *Note*: Two new videos have just been added ([08 From Usage](https://youtu.be/xc6VdedFrG0), and [09 Typed Values](https://youtu.be/mZn3C1DnD90)), if you're already familiar with `clap` but want to know more about these two details you can check out those videos without watching the previous few.
//!
//! *Note*: Apologies for the resolution of the first video, it will be updated to a better resolution soon. The other videos have a proper resolution.
//!
//! ## How to Contribute
//!
//! Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!
//!
//! 1. Fork the project
//! 2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/clap-rs && cd clap-rs`)
//! 3. Create new branch (`git checkout -b new-branch`)
//! 4. Make your changes, and commit (`git commit -am "your message"`) (I try to use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog format so I can update it using [clog](https://github.com/thoughtram/clog))
//! 5. If applicable, run the tests (See below)
//! 6. Push your changes back to your fork (`git push origin your-branch`)
//! 7. Create a pull request! (You can also create the pull request right away, and we'll merge when ready. This a good way to discuss proposed changes.)
//!
//! Another really great way to help is if you find an interesting, or helpful way in which to use `clap`. You can either add it to the `examples/` directory, or file an issue and tell me. I'm all about giving credit where credit is due :)
//!
//! ### Running the tests
//!
//! If contributing, you can run the tests as follows (assuming you're in the `clap-rs/` directory)
//!
//! ```ignore
//! cargo test && make -C clap-tests test
//! ```
//!
//! ### Goals
//!
//! There are a few goals of `clap` that I'd like to maintain throughout contributions. If your proposed changes break, or go against any of these goals we'll discuss the changes further before merging (but will *not* be ignored, all contributes are welcome!). These are by no means hard-and-fast rules, as I'm no expert and break them myself from time to time (even if by mistake or ignorance :P).
//!
//! * Remain backwards compatible when possible
//!   - If backwards compatibility *must* be broken, use deprecation warnings if at all possible before removing legacy code
//!   - This does not apply for security concerns
//! * Parse arguments quickly
//!   - Parsing of arguments shouldn't slow down usage of the main program
//!   - This is also true of generating help and usage information (although *slightly* less stringent, as the program is about to exit)
//! * Try to be cognizant of memory usage
//!   - Once parsing is complete, the memory footprint of `clap` should be low since the  main program is the star of the show
//! * `panic!` on *developer* error, exit gracefully on *end-user* error
//!
//! ## License
//!
//! `clap` is licensed under the MIT license. Please the LICENSE-MIT file in this repository for more information.
//!
//! ## Recent Breaking Changes
//!
//! Although I do my best to keep breaking changes to a minimum, being that this a sub 1.0 library, there are breaking changes from time to time in order to support better features or implementation. For the full details see the changelog.md
//!
//! * As of 0.7.0
//!   - `Arg::possible_values()`, `Arg::value_names()`, `Arg::requires_all()`, `Arg::mutually_excludes_all()` [deprecated], `Arg::conflicts_with_all()`
//!     + No longer take a `Vec<&str>`, instead they take a generic `IntoIterator<Item=AsRef<str>>` which means you cannot use an inline `vec![]` but it means the methods are now far more flexible, especially for dynamic value generation.
//!     + Instead use something that conforms to the `IntoIterator` trait, or something like:
//!
//!     ```ignore
//!     let my_vals = ["value1", "value2", "value3"];
//!     ...
//!     .possible_values(&my_vals)
//!     ```
//!
//! ### Deprecations
//!
//! Old method names will be left around for some time.
//!
//! * As of 0.10.0
//!  - `SubCommand::with_name()` -> `SubCommand::with_name()`
//!  - `App::error_on_no_subcommand()` -> `App::subcommand_required()`
//! * As of 0.6.8
//!   - `Arg::with_name()` -> `Arg::with_name()`
//!   - `Arg::mutually_excludes()` -> `Arg::conflicts_with()`
//!   - `Arg::mutually_excludes_all()` -> `Arg::conflicts_with_all()`
#[cfg(feature = "suggestions")]
extern crate strsim;
#[cfg(feature = "color")]
extern crate ansi_term;

pub use args::{Arg, SubCommand, ArgMatches, ArgGroup};
pub use app::App;
pub use fmt::Format;

#[macro_use]
mod macros;
mod app;
mod args;
mod usageparser;
mod fmt;

#[cfg(test)]
mod tests {
    use super::{App, Arg, SubCommand};
    use std::collections::HashSet;

    arg_enum!{
        #[derive(Debug)]
        enum Val1 {
            ValOne,
            ValTwo
        }
    }
    arg_enum!{
        #[derive(Debug)]
        pub enum Val2 {
            ValOne,
            ValTwo
        }
    }
    arg_enum!{
        enum Val3 {
            ValOne,
            ValTwo
        }
    }
    arg_enum!{
        pub enum Val4 {
            ValOne,
            ValTwo
        }
    }

    #[test]
    fn test_enums() {
        let v1_lower = "valone";
        let v1_camel = "ValOne";

        let v1_lp = v1_lower.parse::<Val1>().unwrap();
        let v1_cp = v1_camel.parse::<Val1>().unwrap();
        match v1_lp {
            Val1::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val1::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        let v1_lp = v1_lower.parse::<Val2>().unwrap();
        let v1_cp = v1_camel.parse::<Val2>().unwrap();
        match v1_lp {
            Val2::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val2::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        let v1_lp = v1_lower.parse::<Val3>().unwrap();
        let v1_cp = v1_camel.parse::<Val3>().unwrap();
        match v1_lp {
            Val3::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val3::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        let v1_lp = v1_lower.parse::<Val4>().unwrap();
        let v1_cp = v1_camel.parse::<Val4>().unwrap();
        match v1_lp {
            Val4::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val4::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
    }

    #[test]
	fn create_app() {
        let _ = App::new("test").version("1.0").author("kevin").about("does awesome things").get_matches();
    }

    #[test]
	fn add_multiple_arg() {
        let _ = App::new("test")
	                .args( vec![
	                    Arg::with_name("test").short("s"),
	                    Arg::with_name("test2").short("l")])
	                .get_matches();
    }

    #[test]
	fn create_flag() {
        let _ = App::new("test")
	                .arg(Arg::with_name("test")
	                            .short("t")
	                            .long("test")
	                            .help("testing testing"))
	                .get_matches();
    }

    #[test]
	fn create_flag_usage() {
        let a = Arg::from_usage("[flag] -f 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("[flag] --flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[flag] -f --flag 'some help info'");
        assert_eq!(c.name, "flag");
        assert_eq!(c.short.unwrap(), 'f');
        assert_eq!(c.long.unwrap(), "flag");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("[flag] -f... 'some help info'");
        assert_eq!(d.name, "flag");
        assert_eq!(d.short.unwrap(), 'f');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let e = Arg::from_usage("[flag] -f --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("-f --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("--flags");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("--flags...");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("[flags] -f");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("[flags] -f...");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
    }

    #[test]
	fn create_positional() {
        let _ = App::new("test")
	                .arg(Arg::with_name("test")
	                            .index(1)
	                            .help("testing testing"))
	                .get_matches();
    }

    #[test]
	fn create_positional_usage() {
        let a = Arg::from_usage("[pos] 'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("<pos> 'some help info'");
        assert_eq!(b.name, "pos");
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[pos]... 'some help info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(!c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("<pos>... 'some help info'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let b = Arg::from_usage("<pos>");
        assert_eq!(b.name, "pos");
        assert!(!b.multiple);
        assert!(b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[pos]...");
        assert_eq!(c.name, "pos");
        assert!(c.multiple);
        assert!(!c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }

    #[test]
	fn create_args_tabs_usage() {
        let a = Arg::from_usage("[pos]\t'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("<pos>\t'some help info'");
        assert_eq!(b.name, "pos");
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[pos]...\t'some help info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(!c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("<pos>...\t'some help info'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
    }

    #[test]
	fn create_option() {
        let _ = App::new("test")
	                .arg(Arg::with_name("test")
	                            .short("t")
	                            .long("test")
	                            .takes_value(true)
	                            .help("testing testing"))
	                .get_matches();
    }

    #[test]
	fn create_option_usage() {
		// Short only
        let a = Arg::from_usage("[option] -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o [opt] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o <opt> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option] -o [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from_usage("[option]... -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o [opt]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o <opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let c = Arg::from_usage("<option>... -o <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o <opt>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long only

        let a = Arg::from_usage("[option] --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt [option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt <option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option] --opt [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from_usage("[option]... --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt [option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt <opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let c = Arg::from_usage("<option>... --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt <option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long only with '='

        let a = Arg::from_usage("[option] --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt=[option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt=<option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option] --opt=[opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from_usage("[option]... --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt=[option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt=<opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let c = Arg::from_usage("<option>... --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt=<option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long and Short

        let a = Arg::from_usage("[option] -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt [option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt <option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option]... -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt [option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option>... -o --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt <option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long and Short with '='

        let a = Arg::from_usage("[option] -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt=[option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt=<option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option]... -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt=[option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option>... -o --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt=<option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_with_vals() {
        let d = Arg::from_usage("-o <opt> <opt> 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("-o <opt> <opt>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("--opt <file> <mode>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        let mut v = d.val_names.unwrap().into_iter().collect::<HashSet<_>>();
        for name in &["mode", "file"] {
            assert!(v.remove(name));
        }
        assert!(v.is_empty());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("[myopt] --opt <file> <mode> 'some help info'");
        assert_eq!(d.name, "myopt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(!d.required);
        let mut v = d.val_names.unwrap().into_iter().collect::<HashSet<_>>();
        for name in &["mode", "file"] {
            assert!(v.remove(name));
        }
        assert!(v.is_empty());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("--opt <option> <option> 'some help info'");
        assert_eq!(d.name, "option");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert_eq!(d.num_vals.unwrap(), 2);
    }

    #[test]
	fn create_subcommand() {
        let _ = App::new("test")
	                .subcommand(SubCommand::with_name("some")
	                                        .arg(Arg::with_name("test")
	                                            .short("t")
	                                            .long("test")
	                                            .takes_value(true)
	                                            .help("testing testing")))
	                .arg(Arg::with_name("other").long("other"))
	                .get_matches();
    }

    #[test]
	fn create_multiple_subcommands() {
        let _ = App::new("test")
	                .subcommands(vec![ SubCommand::with_name("some")
	                                        .arg(Arg::with_name("test")
	                                            .short("t")
	                                            .long("test")
	                                            .takes_value(true)
	                                            .help("testing testing")),
	                                    SubCommand::with_name("add")
	                                        .arg(Arg::with_name("roster").short("r"))])
	                .arg(Arg::with_name("other").long("other"))
	                .get_matches();
    }

    #[test]
    #[should_panic]
	fn unique_arg_names() {
        App::new("some").args(vec![
	        Arg::with_name("arg").short("a"),
	        Arg::with_name("arg").short("b")
	    ]);
    }

    #[test]
    #[should_panic]
	fn unique_arg_shorts() {
        App::new("some").args(vec![
	        Arg::with_name("arg1").short("a"),
	        Arg::with_name("arg2").short("a")
	    ]);
    }

    #[test]
    #[should_panic]
	fn unique_arg_longs() {
        App::new("some").args(vec![
	        Arg::with_name("arg1").long("long"),
	        Arg::with_name("arg2").long("long")
	    ]);
    }
}
