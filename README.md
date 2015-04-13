# clap

![Travis-CI](https://travis-ci.org/kbknapp/clap-rs.svg?branch=master) [![Join the chat at https://gitter.im/kbknapp/clap-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/clap-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Command Line Argument Parser written in Rust

It is a simple to use and efficient library for parsing command line arguments and subcommands when writing console, or terminal applications.

## Video Tutorials

I've been working on a few short video tutorials about using `clap`. They're located on [youtube](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv). 

*Note*: The videos have not been updated to include the new `from_usage()` methods which make building args significantly less verbose. New videos will be added soon.

*Note*: Apologies for the resolution of the first video, it will be updated to a better resolution soon. The other videos have a proper resolution.

## About

You can use `clap` to lay out a list of possible valid command line arguments and subcommands, then let `clap` parse *and validate* the string given by the user at runtime. This means you focus on your applications functionality, not parsing and validating arguments.

What is different about `clap` from other options available is the very simple almost 'Pythonic' style in which you define the valid available arguments for your program, while still giving advanced features. `clap` allows you express complex relationships between arguments in a very simple manner. This means you don't have to spend tons time learning an entirely new library's structures and use. The basics of `clap` can be learned almost intuitively.

`clap` also provides all the traditional version and help switches (or flags) 'for free' by parsing the list of developer supplied arguments. If the developer hasn't defined them already (or only defined some of them), `clap` will auto-generate the applicable "help" and "version" switches (as well as a "help" subcommand so long as other subcommands have been manually defined as well).

After defining a list of possible valid arguments and subcommands, `clap` parses the string given by the end-user at runtime then gives you a list of the valid matches and their values. If the user made an error or typo, `clap` informs them and exits gracefully. This means that you can simply use these matches and values to determine the functioning of your program.

## Features

Below are a few of the features which `clap` supports, full descriptions and usage can be found in the [documentation](http://kbknapp.github.io/clap-rs/docs/clap/index.html) and `examples/` directory

* **Auto-generated Help, Version, and Usage information**
  - Can optionally be fully, or partially overridden if you want a custom help, version, or usage
* **Flags / Switches** (i.e. bool fields)
  - Both short and long versions supported (i.e. `-f` and `--flag` respectively)
  - Supports combining short versions (i.e. `-fBgoZ` is the same as `-f -B -g -o -Z`)
  - Optionally supports multiple occurrences (i.e. `myprog -vvv` or `myprog -v -v -v`)
* **Positional Arguments** (i.e. those which are based off an index from the program name)
  - Optionally supports multiple values (i.e. `myprog <file>...` such as `myprog file1.txt file2.txt` being two values for the same "file" argument)
  - Optionally supports Specific Value Sets (See below)
* **Option Arguments** (i.e. those that take values as options)
  - Both short and long versions supported (i.e. `-o value` and `--option value` or `--option=value` respectively)
  - Optionally supports multiple values (i.e. `myprog --option <value> --option <other_value>`)
  - Optionally supports Specific Value Sets (See below)
* **Sub-Commands** (i.e. `git add <file>` where `add` is a sub-command of `git`)
  - Support their own sub-arguments, and sub-commands independant of the parent
  - Get their own auto-generated Help, Version, and Usage independant of parent
* **Requirement Rules**: Arguments can optionally define the following types of requirement rules
  - Required by default
  - Required only if certain arguments are present
  - Can require other arguments to be present
* **Exclusion Rules**: Arguments can optionally define the following types of exclusion rules
  - Can be disallowed when certain arguments are present
  - Can disallow use of other arguments when present
* **Specific Value Sets**: Positional or Option Arguments can optionally define a specific set of allowed values (i.e. imagine a `--mode` option which may *only* have one of two values `fast` or `slow` such as `--mode fast` or `--mode slow`)
* **Default Values**: Although not specifically provided by `clap` you can achieve this exact functionality from Rust's `Option<&str>.unwrap_or("some default")` method
* **Get Version from Cargo.toml**: `clap` is fully compatible with Rust's `env!()` macro for automatically getting the version from your Cargo.toml. See `examples/09_AutoVersion.rs` for how to do this (Thanks to [jhelwig](https://github.com/jhelwig) for pointing this out)

## Quick Example
 
The following two examples (which are functionally equivilant, but show two different ways to use `clap`) show a quick example of some of the basic functionality of `clap`. For more advanced usage, such as requirements, exclusions, multiple values and occurrences see the [video tutorials](https://www.youtube.com/playlist?list=PLza5oFLQGTl0Bc_EU_pBNcX-rhVqDTRxv), [documentation](http://kbknapp.github.io/clap-rs/docs/clap/index.html), or `examples/` directory of this repository.
 
 ```rust
// (Full example with comments in examples/01_QuickExample.rs)
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

While the following example is functionally the same as the one above, this method allows more advanced configuration options, or even dynamically generating arguments when desired. Both methods can be used together to get the best of both worlds (see the documentation).
 
```rust
// (Full example with comments in examples/01_QuickExample.rs)
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("myapp")
                          .version("1.0")
                          .author("Kevin K. <kbknapp@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::new("CONFIG")
                               .short("c")
                               .long("config")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::new("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::new("debug")
                               .short("d")
                               .multiple(true)
                               .help("Sets the level of debugging information"))
                          .subcommand(SubCommand::new("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::new("verbose")
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
    -c, --config=CONFIG    Sets a custom config file

POSITIONAL ARGUMENTS:
    INPUT    The input file to use

SUBCOMMANDS:
    help    Prints this message
    test    Controls testing features
```

*NOTE:* You could also run `myapp test --help` to see similar output and options for the `test` subcommand.

## Installation

Add `clap` as a dependecy in your `Cargo.toml` file to use from crates.io:

 ```
 [dependencies]
 clap = "*"
 ```
 Or track the latest on the master branch at github:

```
[dependencies.clap]
git = "https://github.com/kbknapp/clap-rs.git"
```

Add `extern crate clap;` to your crate root.

Define a list of valid arguments for your program (see the [documentation](https://kbknapp.github.io/clap-rs/index.html) or `examples/` directory of this repo)

Then run `cargo build` or `cargo update && cargo build` for your project.

### More Information

You can find complete documentation on the [github-pages site](http://kbknapp.github.io/clap-rs/docs/clap/index.html) for this project.

You can also find usage examples in the `examples/` directory of this repo.

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
