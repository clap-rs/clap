# clap

![Travis-CI](https://travis-ci.org/kbknapp/clap-rs.svg?branch=master) [![Crates.io](https://img.shields.io/crates/v/clap.svg)]() [![Crates.io](https://img.shields.io/crates/l/clap.svg)]() [![Join the chat at https://gitter.im/kbknapp/clap-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/clap-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Command Line Argument Parser for Rust

It is a simple to use, efficient, and full featured library for parsing command line arguments and subcommands when writing console, or terminal applications.

## About

`clap` is used to parse *and validate* the string of command line arguments provided by the user at runtime. You provide the list of valid possibilities, and `clap` handles the rest. This means you focus on your *applications* functionality, and less on the parsing and validating of arguments.

`clap` also provides the traditional version and help switches (or flags) 'for free' meaning automatically with no configuration. It does this by checking list of valid possibilities you supplied and if you haven't them already (or only defined some of them), `clap` will auto-generate the applicable ones. If you are using subcommands, `clap` will also auto-generate a `help` subcommand for you in addition to the traditional flags.

Once `clap` parses the user provided string of arguments, it returns the matches along with any applicable values. If the user made an error or typo, `clap` informs them of the mistake and exits gracefully. Because of this, you can make reasonable assumptions in your code about the validity of the arguments.

## Features

Below are a few of the features which `clap` supports, full descriptions and usage can be found in the [documentation](http://kbknapp.github.io/clap-rs/docs/clap/index.html) and `examples/` directory

* **Auto-generated Help, Version, and Usage information**
  - Can optionally be fully, or partially overridden if you want a custom help, version, or usag
* **Flags / Switches** (i.e. bool fields)
  - Both short and long versions supported (i.e. `-f` and `--flag` respectively)
  - Supports combining short versions (i.e. `-fBgoZ` is the same as `-f -B -g -o -Z`)
  - Optionally supports multiple occurrences (i.e. `-vvv` or `-v -v -v`)
* **Positional Arguments** (i.e. those which are based off an index from the program name)
  - Optionally supports multiple values (i.e. `myprog <file>...` such as `myprog file1.txt file2.txt` being two values for the same "file" argument)
  - Optionally supports Specific Value Sets (See below)
  - Supports the unix `--` meaning, only positional arguments follow
* **Option Arguments** (i.e. those that take values as options)
  - Both short and long versions supported (i.e. `-o value` and `--option value` or `--option=value` respectively)
  - Optionally supports multiple values (i.e. `-o <value> -o <other_value>`)
  - Optionally supports Specific Value Sets (See below)
* **Sub-Commands** (i.e. `git add <file>` where `add` is a sub-command of `git`)
  - Support their own sub-arguments, and sub-sub-commands independant of the parent
  - Get their own auto-generated Help, Version, and Usage independant of parent
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
* **Automatic Version from Cargo.toml**: `clap` is fully compatible with Rust's `env!()` macro for automatically setting the version of your application to the version in your Cargo.toml. See `examples/09_AutoVersion.rs` for how to do this (Thanks to [jhelwig](https://github.com/jhelwig) for pointing this out)
* **Typed Values**: You can use several convenience macros provided by `clap` to get typed values (i.e. `i32`, `u8`, etc.) from positional or option arguments so long as the type you request implements `std::str::FromStr` See the `examples/12_TypedValues.rs`. You can also use `clap`s `simple_enum!` or `arg_enum!` macro to create an enum with variants that automatically implements `std::str::FromStr`. See `examples/13a_EnumValuesAutomatic.rs` for details.

## Quick Example
 
The following two examples show a quick example of some of the very basic functionality of `clap`. For more advanced usage, such as requirements, exclusions, groups, multiple values and occurrences see the [video tutorials](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv), [documentation](http://kbknapp.github.io/clap-rs/docs/clap/index.html), or `examples/` directory of this repository.
 
 *NOTE:* Both examples are functionally the same, but show two different styles in which to use `clap`
 
```rust
// (Full example with detailed comments in examples/01a_QuickExample.rs)
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
                          .subcommand(SubCommand::new("test")
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

    // more porgram logic goes here...
}
```

The following example is functionally the same as the one above, but this method allows more advanced configuration options (not shown in this small example), or even dynamically generating arguments when desired. Both methods can be used together to get the best of both worlds (see the documentation, examples, or video tutorials).
 
```rust
// (Full example with detailed comments in examples/01b_QuickExample.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is 
// more verbose, but allows easier editting, and at times more advanced options, or the possibility
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
                          .subcommand(SubCommand::new("test")
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

    // more porgram logic goes here...
}
```

If you were to compile either of the above programs and run them with the flag `--help` or `-h` (or `help` subcommand, since we defined `test` as a subcommand) the following would be output

```sh
$ myapp --help
myapp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [FLAGS] [OPTIONS] <INPUT> [SUBCOMMANDS]

FLAGS:
    -d               Turn debugging information on
    -h, --help       Prints this message
    -v, --version    Prints version information
 
OPTIONS:
    -c, --config <CONFIG>    Sets a custom config file

POSITIONAL ARGUMENTS:
    INPUT    The input file to use

SUBCOMMANDS:
    help    Prints this message
    test    Controls testing features
```

*NOTE:* You could also run `myapp test --help` to see similar output and options for the `test` subcommand.

## Try it!

### Pre-Built Test

To try out the pre-built example use the following stes:

* Clone the repostiory `$ git clone https://github.com/kbknapp/clap-rs && cd clap-rs/clap-tests`
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
clap = "*"
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

For full usage, add `clap` as a dependecy in your `Cargo.toml` file to use from crates.io:

 ```toml
 [dependencies]
 clap = "*"
 ```
 Or track the latest on the master branch at github:

```toml
[dependencies.clap]
git = "https://github.com/kbknapp/clap-rs.git"
```

Add `extern crate clap;` to your crate root.

Define a list of valid arguments for your program (see the [documentation](https://kbknapp.github.io/clap-rs/index.html) or `examples/` directory of this repo)

Then run `cargo build` or `cargo update && cargo build` for your project.

### More Information

You can find complete documentation on the [github-pages site](http://kbknapp.github.io/clap-rs/docs/clap/index.html) for this project.

You can also find usage examples in the `examples/` directory of this repo.

#### Video Tutorials

There's also the video tutorial series [Argument Parsing with Rust](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv) that I've been working on.

*Note*: Two new videos have just been added ([08 From Usage](https://youtu.be/xc6VdedFrG0), and [09 Typed Values](https://youtu.be/mZn3C1DnD90)), if you're already familiar with `clap` but want to know more about these two details you can check out those videos without watching the previous few.

*Note*: Apologies for the resolution of the first video, it will be updated to a better resolution soon. The other videos have a proper resolution.

## How to Contribute

Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!

1. Fork the project
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/clap-rs && cd clap-rs`)
3. Create new branch (`git checkout -b new-branch`)
4. Make your changes, and commit (`git commit -am "your message"`) (I try to use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog format so I can update it using [clog](https://github.com/thoughtram/clog))
5. If applicable, run the tests (See below)
6. Push your changes back to your fork (`git push origin your-branch`)
7. Create a pull request! (You can also create the pull request right away, and we'll merge when ready. This a good way to discuss proposed changes.) 

Another really great way to help is if you find an interesting, or helpful way in which to use `clap`. You can either add it to the `examples/` directory, or file an issue and tell me. I'm all about giving credit where credit is due :)

### Running the tests

If contributing, you can run the tests as follows (assuming you've cloned the repo to `clap-rs/`

```
cd clap-rs && cargo test
cd clap-tests && make test
```

### Building the documentation

If your changes require re-building the documentation, run this instead of `cargo doc` to generate the proper module docstring:

```
cd clap-rs && make doc
```

Then browse to `clap-rs/docs/clap/index.html` in your web-browser of choice to check it out.

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

`clap` is licensed under the MIT license. Please the LICENSE-MIT file in this repository for more information.
