# clap

[![Build Status](https://travis-ci.org/kbknapp/clap-rs.svg?branch=master)](https://travis-ci.org/kbknapp/clap-rs) [![](http://meritbadge.herokuapp.com/clap)](https://crates.io/crates/clap) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/kbknapp/clap-rs/blob/master/LICENSE-MIT) [![Coverage Status](https://coveralls.io/repos/kbknapp/clap-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/kbknapp/clap-rs?branch=master) [![Join the chat at https://gitter.im/kbknapp/clap-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/clap-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Command Line Argument Parser for Rust

It is a simple to use, efficient, and full featured library for parsing command line arguments and subcommands when writing console, or terminal applications.

Table of Contents
=================

* [What's New](#whats-new)
* [About](#about)
* [FAQ](#faq)
* [Features](#features)
* [Quick Example](#quick-example)
* [Try it!](#try-it)
  * [Pre-Built Test](#pre-built-test)
  * [BYOB (Build Your Own Binary)](#byob-build-your-own-binary)
* [Usage](#usage)
  * [Optional Dependencies / Features](#optional-dependencies--features)
  * [Dependencies Tree](#dependencies-tree)
  * [More Information](#more-information)
    * [Video Tutorials](#video-tutorials)
* [How to Contribute](#how-to-contribute)
  * [Running the tests](#running-the-tests)
  * [Goals](#goals)
* [License](#license)
* [Recent Breaking Changes](#recent-breaking-changes)
  * [Deprecations](#deprecations)

Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)

## What's New

If you're already familiar with `clap` but just want to see some new highlights as of **1.4.6**

* **Major Bug Fixes in 1.4.6** We recommend everyone upgrade as soon as possible. See the [the changelog](https://github.com/kbknapp/clap-rs/blob/master/CHANGELOG.md) for details.
* Using `get_matches_safe_*` family of methods no longer exits the process when help or version is displayed, instead it returns an `ClapError` with an `error_type` field set to `ClapErrorType::HelpDisplayed` or `ClapErrorType::VersionDisplayed` respectively. You must then call `ClapError::exit` or `std::process::exit` giving you the control.
* Allows parsing without a binary name preceding (useful for daemon modes and interactive CLIs)
* `-Lvalue` style options are **now supported**! (i.e. `-L` is the short, and `value` is the value being passed. Equivalent to `-L value`). This can be combined with flag expansion. Example: `-lF2` could be parsed as `-l -F 2` where `-l` is a flag and `-F` is an option that takes a number.
* There is a **new opt-in setting** (`AppSettings::TrailingVarArg`) to allow the final positional argument to be a vararg and have `clap` not interpret the remaining arguments (i.e. useful when final argument should be a list of arguments for another command or process)
* You can now access values from an argument in a group via the group name, instead of having to check each arg name individually to find out which one was used. The same applies for checking if an arg from a group `is_present()`
* You now have the option to **not** `panic!` on invalid unicode. The `*_safe()` family of `get_matches` will return an `Err` with `ClapErrorType::InvalidUnicode`.
* You have the option to get lossy unicode values. By using the `*_lossy()` versions of the `get_matches` family of methods all invalid unicode will be replaced with `U+FFFD` and **not** `panic!` or fail parsing.
* Some documentation improvements
* A new macro has been designed by [james-darkfox](https://github.com/james-darkfox) to give the simplicity of `from_usage` methods, but the performance of the Builder Pattern. Huge thanks to him! Fair warning this is very new, and may still have some kinks and tweaks left as we experiment ;)
* Users can now print the help message programmatically using `App::write_help(io::Write)` and `App::print_help()`.
* Users can now simply print the error message to `stderr` and exit gracefully programmatically using `ClapError::exit()`
* You can now get argument matches **without** consuming your `App` struct using `App::get_matches_from_safe_borrow()`
* Some other minor bug fixes and improvements

For full details, see [CHANGELOG.md](https://github.com/kbknapp/clap-rs/blob/master/CHANGELOG.md)

## About

`clap` is used to parse *and validate* the string of command line arguments provided by the user at runtime. You provide the list of valid possibilities, and `clap` handles the rest. This means you focus on your *applications* functionality, and less on the parsing and validating of arguments.

`clap` also provides the traditional version and help switches (or flags) 'for free' meaning automatically with no configuration. It does this by checking list of valid possibilities you supplied and if you haven't them already (or only defined some of them), `clap` will auto-generate the applicable ones. If you are using subcommands, `clap` will also auto-generate a `help` subcommand for you in addition to the traditional flags.

Once `clap` parses the user provided string of arguments, it returns the matches along with any applicable values. If the user made an error or typo, `clap` informs them of the mistake and exits gracefully. Because of this, you can make reasonable assumptions in your code about the validity of the arguments.

## FAQ

For a full FAQ and more in depth details, see [the wiki page](https://github.com/kbknapp/clap-rs/wiki/FAQ)

### Comparisons

First, let me say that these comparisons are highly subjective, and not meant in a critical or harsh manner. All the argument parsing libraries out there (to include `clap`) have their own strengths and weaknesses. Sometimes it just comes down to personal taste when all other factors are equal. When in doubt, try them all and pick one that you enjoy :) There's plenty of room in the Rust community for multiple implementations!

#### How does `clap` compare to [getopts](https://github.com/rust-lang-nursery/getopts)?

`getopts` is a very basic, fairly minimalist argument parsing library. This isn't a bad thing, sometimes you don't need tons of features, you just want to parse some simple arguments, and have some help text generated for you based on valid arguments you specify. The downside to this approach is that you must manually implement most of the common features (such as checking to display help messages, usage strings, etc.). If you want a highly custom argument parser, and don't mind writing the majority of the functionality yourself, `getopts` is an excellent base.

`getopts` also doesn't allocate much, or at all. This gives it somewhat of a performance boost. Although, as you start implementing additional features, that boost quickly disappears.

Personally, I find many, many people that use `getopts` are manually implementing features that `clap` has by default. Using `clap` simplifies your codebase allowing you to focus on your application, and not argument parsing.

#### How does `clap` compare to [docopt.rs](https://github.com/docopt/docopt.rs)?

I first want to say I'm a big a fan of BurntSushi's work, the creator of `Docopt.rs`. I aspire to produce the quality of libraries that this man does! When it comes to comparing these two libraries they are very different. `docopt` tasks you with writing a help message, and then it parsers that message for you to determine all valid arguments and their use. Some people LOVE this approach, others not so much. If you're willing to write a detailed help message, it's nice that you can stick that in your program and have `docopt` do the rest. On the downside, it's somewhat less flexible, and requires you to change the help message if you need to make changes.

`docopt` is also excellent at translating arguments into Rust types automatically. There is even a syntax extension which will do all this for you, if you're willing to use a nightly compiler (use of a stable compiler requires you to somewhat manually translate from arguments to Rust types). To use BurntSushi's words, `docopt` is also a sort of black box. You get what you get, and it's hard to tweak implementation or customise the experience for your use case.

Because `docopt` is doing a ton of work to parse your help messages and determine what you were trying to communicate as valid arguments, it's also one of the more heavy weight parsers performance-wise. For most applications this isn't a concern and this isn't to say `docopt` is slow, in fact from it. This is just something to keep in mind while comparing.

#### All else being equal, what are some reasons to use `clap`?

`clap` is as fast, and as lightweight as possible while still giving all the features you'd expect from a modern argument parser. In fact, for the amount and type of features `clap` offers the fact it remains about as fast as `getopts` is great. If you use `clap` when just need some simple arguments parsed, you'll find it a walk in the park. But `clap` also makes it possible to represent extremely complex, and advanced requirements, without too much thought. `clap` aims to be intuitive, easy to use, and fully capable for wide variety use cases and needs.

## Features

Below are a few of the features which `clap` supports, full descriptions and usage can be found in the [documentation](http://kbknapp.github.io/clap-rs/clap/index.html) and [examples/](examples) directory

* **Auto-generated Help, Version, and Usage information**
  - Can optionally be fully, or partially overridden if you want a custom help, version, or usage
* **Flags / Switches** (i.e. bool fields)
  - Both short and long versions supported (i.e. `-f` and `--flag` respectively)
  - Supports combining short versions (i.e. `-fBgoZ` is the same as `-f -B -g -o -Z`)
  - Optionally supports multiple occurrences (i.e. `-vvv` or `-v -v -v`)
* **Positional Arguments** (i.e. those which are based off an index from the program name)
  - Optionally supports multiple values (i.e. `myprog <file>...` such as `myprog file1.txt file2.txt` being two values for the same "file" argument)
  - Optionally supports Specific Value Sets (See below)
  - Supports the unix `--` meaning, only positional arguments follow
  - Optionally sets value parameters (such as the minimum number of values, the maximum number of values, or the exact number of values)
* **Option Arguments** (i.e. those that take values as options)
  - Both short and long versions supported (i.e. `-o value` or `-ovalue` and `--option value` or `--option=value` respectively)
  - Optionally supports multiple values (i.e. `-o <value> -o <other_value>` or the shorthand `-o <value> <other_value>`)
  - Optionally supports Specific Value Sets (See below)
  - Optionally supports named values so that the usage/help info appears as `-o <name> <other_name>` etc. for when you require specific multiple values
  - Optionally sets value parameters (such as the minimum number of values, the maximum number of values, or the exact number of values)
* **Sub-Commands** (i.e. `git add <file>` where `add` is a sub-command of `git`)
  - Support their own sub-arguments, and sub-sub-commands independent of the parent
  - Get their own auto-generated Help, Version, and Usage independent of parent
* **Requirement Rules**: Arguments can optionally define the following types of requirement rules
  - Required by default
  - Required only if certain arguments are present
  - Can require other arguments to be present
* **Exclusion/Confliction Rules**: Arguments can optionally define the following types of exclusion rules
  - Can be disallowed when certain arguments are present
  - Can disallow use of other arguments when present
* **Groups**: Arguments can optionally be made part of a group which means one, and only one argument from this "group" may be present at runtime
  - Fully compatible with other relational rules (requirements and exclusions) which allows things like requiring the use of a group, or denying the use of a group conditionally
* **Specific Value Sets**: Positional or Option Arguments can optionally define a specific set of allowed values (i.e. imagine a `--mode` option which may *only* have one of two values `fast` or `slow` such as `--mode fast` or `--mode slow`)
* **Default Values**: Although not specifically provided by `clap` you can achieve this exact functionality from Rust's `Option<&str>.unwrap_or("some default")` method (or `Result<T,String>.unwrap_or(T)` when using typed values)
* **Automatic Version from Cargo.toml**: `clap` is fully compatible with Rust's `env!()` macro for automatically setting the version of your application to the version in your Cargo.toml. See [09_auto_version example](examples/09_auto_version.rs) for how to do this (Thanks to [jhelwig](https://github.com/jhelwig) for pointing this out)
* **Typed Values**: You can use several convenience macros provided by `clap` to get typed values (i.e. `i32`, `u8`, etc.) from positional or option arguments so long as the type you request implements `std::str::FromStr` See the [12_typed_values example](examples/12_typed_values.rs). You can also use `clap`s `simple_enum!` or `arg_enum!` macro to create an enum with variants that automatically implements `std::str::FromStr`. See [13a_enum_values_automatic example](examples/13a_enum_values_automatic.rs) for details and performs an ascii case insensitive parse from a `string`->`enum`.
* **Suggestions**: Suggests corrections when the user enter's a typo. For example, if you defined a `--myoption <value>` argument, and the user mistakenly typed `--moyption value` (notice `y` and `o` switched), they would receive a `Did you mean '--myoption' ?` error and exit gracefully. This also works for subcommands and flags. (Thanks to [Byron](https://github.com/Byron) for the implementation) (This feature can optionally be disabled, see 'Optional Dependencies / Features')
* **Colorized (Red) Errors (Non Windows OS only)**: Error message are printed in red text (this feature can optionally be disabled, see 'Optional Dependencies / Features').
* **Global Arguments**: Arguments can optionally be defined once, and be available to all child subcommands.
* **Custom Validations**: You can define a function to use as a validator of argument values. Imagine defining a function to validate IP addresses, or fail parsing upon error. This means your application logic can be solely focused on *using* values.
* **POSIX Compatible Conflicts** - In POSIX args can be conflicting, but not fail parsing because whichever arg comes *last* "wins" to to speak. This allows things such as aliases (i.e. `alias ls='ls -l'` but then using `ls -C` in your terminal which ends up passing `ls -l -C` as the final arguments. Since `-l` and `-C` aren't compatible, this effectively runs `ls -C` in `clap` if you choose...`clap` also supports hard conflicts that fail parsing). (Thanks to [Vinatorul](https://github.com/Vinatorul)!)
* **Support for building CLIs from YAML** - This keeps your Rust source nice and tidy!

## Quick Example

The following examples show a quick example of some of the very basic functionality of `clap`. For more advanced usage, such as requirements, exclusions, groups, multiple values and occurrences see the [video tutorials](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv), [documentation](http://kbknapp.github.io/clap-rs/clap/index.html), or [examples/](examples) directory of this repository.

 **NOTE:** All these examples are functionally the same, but show three different styles in which to use `clap`

```rust
// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "usage strings" method of creating arguments which is less
// less verbose
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("myapp")
                          .version("1.0")
                          .author("Kevin K. <kbknapp@gmail.com>")
                          .about("Does awesome things")
                          .args_from_usage(
                              "-c --config=[CONFIG] 'Sets a custom config file'
                              <INPUT> 'Sets the input file to use'
                              [debug]... -d 'Sets the level of debugging information'")
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg_from_usage("-v --verbose 'Print test information verbosely'"))
                          .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("CONFIG").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "debug" flag
    // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
}
```

The following example is functionally the same as the one above, but this method allows more advanced configuration options (not shown in this small example), or even dynamically generating arguments when desired. Both methods can be used together to get the best of both worlds (see the documentation, examples, or video tutorials).

```rust
// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("myapp")
                          .version("1.0")
                          .author("Kevin K. <kbknapp@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("CONFIG")
                               .short("c")
                               .long("config")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("debug")
                               .short("d")
                               .multiple(true)
                               .help("Sets the level of debugging information"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::with_name("verbose")
                                          .short("v")
                                          .help("print test information verbosely")))
                          .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("CONFIG").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "debug" flag
    // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
}
```

The following combines the previous two examples by using the simplicity of the `from_usage` methods and the performance of the Builder Pattern.

```rust
// (Full example with detailed comments in examples/01c_quick_example.rs)
//
// This example demonstrates clap's "usage strings" method of creating arguments which is less
// less verbose
#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (about: "Does awesome things")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand test =>
            (about: "controls testing features")
            (version: "1.3")
            (author: "Someone E. <someone_else@other.com>")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
    ).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("CONFIG").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "debug" flag
    // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
}
```

This final method shows how you can use a YAML file to build your CLI and keep your Rust source tidy. First, create the `cli.yml` file to hold your CLI options, but it could be called anything we like (we'll use the same both examples above to keep it functionally equivalent):

```yaml
name: myapp
version: 1.0
author: Kevin K. <kbknapp@gmail.com>
about: Does awesome things
args:
    - CONFIG:
        short: c
        long: config
        help: Sets a custom config file
        takes_value: true
    - INPUT:
        help: Sets the input file to use
        required: true
        index: 1
    - debug:
        short: d
        multiple: true
        help: Sets the level of debugging information
subcommands:
    - test:
        about: controls testing features
        version: 1.3
        author: Someone E. <someone_else@other.com>
        args:
            - verbose:
                short: v
                help: print test information verbosely
```

Now we create our `main.rs` file just like we would have with the previous two examples:

```rust
// (Full example with detailed comments in examples/17_yaml.rs)
//
// This example demonstrates clap's building from YAML style of creating arguments which is far
// more clean, but takes a very small performance hit compared to the other two methods.
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("CONFIG").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "debug" flag
    // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
}
```

If you were to compile any of the above programs and run them with the flag `--help` or `-h` (or `help` subcommand, since we defined `test` as a subcommand) the following would be output

**NOTE**: The YAML option requires adding a special `features` flag when compiling `clap` because it is not compiled by default since it takes additional dependencies that some people may not need. Simply change your `clap = "1"` to `clap = {version = "1", features = ["yaml"]}` in your `Cargo.toml` to use the YAML version.

```sh
$ myapp --help
myapp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [FLAGS] [OPTIONS] <INPUT> [SUBCOMMAND]

FLAGS:
    -d               Turn debugging information on
    -h, --help       Prints this message
    -V, --version    Prints version information

OPTIONS:
    -c, --config <CONFIG>    Sets a custom config file

ARGS:
    INPUT    The input file to use

SUBCOMMANDS:
    help    Prints this message
    test    Controls testing features
```

**NOTE:** You could also run `myapp test --help` to see similar output and options for the `test` subcommand.

## Try it!

### Pre-Built Test

To try out the pre-built example, use the following steps:

* Clone the repository `$ git clone https://github.com/kbknapp/clap-rs && cd clap-rs/clap-tests`
* Compile the example `$ cargo build --release`
* Run the help info `$ ./target/release/claptests --help`
* Play with the arguments!

### BYOB (Build Your Own Binary)

To test out `clap`'s default auto-generated help/version follow these steps:
* Create a new cargo project `$ cargo new fake --bin && cd fake`
* Add `clap` to your `Cargo.toml`
*
```toml
[dependencies]
clap = "1"
```

* Add the following to your `src/main.rs`

```rust
extern crate clap;
use clap::App;

fn main() {
  let _ = App::new("fake").version("v1.0-beta").get_matches();
}
```

* Build your program `$ cargo build --release`
* Run with help or version `$ ./target/release/fake --help` or `$ ./target/release/fake --version`

## Usage

For full usage, add `clap` as a dependency in your `Cargo.toml` file to use from crates.io:

 ```toml
 [dependencies]
 clap = "1"
 ```
 Or track the latest on the master branch at github:

```toml
[dependencies.clap]
git = "https://github.com/kbknapp/clap-rs.git"
```

Add `extern crate clap;` to your crate root.

Define a list of valid arguments for your program (see the [documentation](https://kbknapp.github.io/clap-rs/index.html) or [examples/](examples) directory of this repo)

Then run `cargo build` or `cargo update && cargo build` for your project.

### Optional Dependencies / Features

If you'd like to keep your dependency list to **only** `clap`, you can disable any features that require an additional dependency. To do this, add this to your `Cargo.toml`:

```toml
[dependencies.clap]
version = "1"
default-features = false
```

You can also selectively enable only the features you'd like to include, by adding:

```toml
[dependencies.clap]
version = "1"
default-features = false

# Cherry-pick the features you'd like to use
features = [ "suggestions", "color" ]
```

The following is a list of optional `clap` features:

* **"suggestions"**: Turns on the `Did you mean '--myoption' ?` feature for when users make typos.
* **"color"**: Turns on red error messages. This feature only works on non-Windows OSs.
* **"lints"**: This is **not** included by default and should only be used while developing to run basic lints against changes. This can only be used on Rust nightly.

### Dependencies Tree

The following graphic depicts `clap`s dependency graph.

 * **Dashed** Line: Optional dependency
 * **Red** Color: **NOT** included by default (must use cargo `features` to enable)

![clap dependencies](clap.png)

### More Information

You can find complete documentation on the [github-pages site](http://kbknapp.github.io/clap-rs/clap/index.html) for this project.

You can also find usage examples in the [examples/](examples) directory of this repo.

#### Video Tutorials

There's also the video tutorial series [Argument Parsing with Rust](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv) that I've been working on.

*Note*: Two new videos have just been added ([08 From Usage](https://youtu.be/xc6VdedFrG0), and [09 Typed Values](https://youtu.be/mZn3C1DnD90)), if you're already familiar with `clap` but want to know more about these two details you can check out those videos without watching the previous few.

*Note*: Apologies for the resolution of the first video, it will be updated to a better resolution soon. The other videos have a proper resolution.

## How to Contribute

Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!

Another really great way to help is if you find an interesting, or helpful way in which to use `clap`. You can either add it to the [examples/](examples) directory, or file an issue and tell me. I'm all about giving credit where credit is due :)

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before you start contributing.

### Running the tests

If contributing, you can run the tests as follows (assuming you're in the `clap-rs` directory)

```
cargo test --features yaml && make -C clap-tests test
```

### Goals

There are a few goals of `clap` that I'd like to maintain throughout contributions. If your proposed changes break, or go against any of these goals we'll discuss the changes further before merging (but will *not* be ignored, all contributes are welcome!). These are by no means hard-and-fast rules, as I'm no expert and break them myself from time to time (even if by mistake or ignorance :P).

* Remain backwards compatible when possible
  - If backwards compatibility *must* be broken, use deprecation warnings if at all possible before removing legacy code
  - This does not apply for security concerns
* Parse arguments quickly
  - Parsing of arguments shouldn't slow down usage of the main program
  - This is also true of generating help and usage information (although *slightly* less stringent, as the program is about to exit)
* Try to be cognizant of memory usage
  - Once parsing is complete, the memory footprint of `clap` should be low since the  main program is the star of the show
* `panic!` on *developer* error, exit gracefully on *end-user* error

## License

`clap` is licensed under the MIT license. Please read the [LICENSE-MIT](LICENSE-MIT) file in this repository for more information.

## Recent Breaking Changes

Although I do my best to keep breaking changes to a minimum, there are breaking changes from time to time in order to support better features or implementation. For the full details, see [CHANGELOG.md](./CHANGELOG.md).

* As of 1.3.0
 - `ArgGroup::add_all` now takes `&[&str]` instead of a `Vec<&str>`
 - `ArgGroup::requires_all` now takes `&[&str]` instead of a `Vec<&str>`
 - `ArgGroup::conflicts_with_all` now takes `&[&str]` instead of a `Vec<&str>`

* As of 0.11.0: The default short flag for `version` has changed from `-v` to `-V` (Uppercase). Although you can also now override the short flag for `help` and `version` using `App::help_short()` and `App::version_short()`
* As of 0.7.0
  - `Arg::possible_values()`, `Arg::value_names()`, `Arg::requires_all()`, `Arg::mutually_excludes_all()` [deprecated], `Arg::conflicts_with_all()`
    + No longer take a `Vec<&str>`, instead they take a generic `IntoIterator<Item=AsRef<str>>` which means you cannot use an inline `vec![]` but it means the methods are now far more flexible, especially for dynamic value generation.
    + Instead use something that conforms to the `IntoIterator` trait, or something like:

    ```rust
    let my_vals = ["value1", "value2", "value3"];
    ...
    .possible_values(&my_vals)
    ```

### Deprecations

Old method names will be left around for some time.

* As of 1.2.0 (Will **not** be removed until 2.x)
 - `App::subcommands_negate_reqs(bool)` -> `AppSettings::SubcommandsNegateReqs` passed to `App::setting()`
 - `App::subcommand_required(bool)` -> `AppSettings::SubcommandRequired` passed to `App::setting()`
 - `App::arg_required_else_help(bool)` -> `AppSettings::ArgRequiredElseHelp` passed to `App::setting()`
 - `App::global_version(bool)` -> `AppSettings::GlobalVersion` passed to `App::setting()`
 - `App::versionless_subcommands(bool)` -> `AppSettings::VersionlessSubcommands` passed to `App::setting()`
 - `App::unified_help_messages(bool)` -> `AppSettings::UnifiedHelpMessages` passed to `App::setting()`
 - `App::wait_on_error(bool)` -> `AppSettings::WaitOnError` passed to `App::setting()`
 - `App::subcommand_required_else_help(bool)` -> `AppSettings::SubcommandRequiredElseHelp` passed to `App::setting()`

* As of 0.10.0
 - `SubCommand::new()` -> `SubCommand::with_name()` (Removed as of 1.0.0)
 - `App::error_on_no_subcommand()` -> `App::subcommand_required()` (Removed as of 1.0.0)
* As of 0.6.8
 - `Arg::new()` -> `Arg::with_name()` (Removed as of 1.0.0)
 - `Arg::mutually_excludes()` -> `Arg::conflicts_with()` (Removed as of 1.0.0)
 - `Arg::mutually_excludes_all()` -> `Arg::conflicts_with_all()` (Removed as of 1.0.0)
