<!-- omit in TOC -->
# clap

[![Crates.io](https://img.shields.io/crates/v/clap?style=flat-square)](https://crates.io/crates/clap)
[![Crates.io](https://img.shields.io/crates/d/clap?style=flat-square)](https://crates.io/crates/clap)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/master/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/master/LICENSE-MIT)
[![Coverage Status](https://img.shields.io/coveralls/github/clap-rs/clap/master?style=flat-square)](https://coveralls.io/github/clap-rs/clap?branch=master)
[![Linux Build Status](https://img.shields.io/travis/clap-rs/clap/master?style=flat-square&logo=linux&logoColor=fff)](https://travis-ci.org/clap-rs/clap)
[![Windows Build Status](https://img.shields.io/azure-devops/build/clap-rs/3b9343e1-29b0-47be-acec-1edcdab69de9/1/master?style=flat-square&logo=windows)](https://dev.azure.com/clap-rs/clap/_build/latest?definitionId=1&branchName=master)

Command Line Argument Parser for Rust

It is a simple-to-use, efficient, and full-featured library for parsing command line arguments and subcommands when writing command line, console or terminal applications.

* [Documentation][docs]
* [Questions & Discussions](https://github.com/clap-rs/clap/discussions)
* [Website](https://clap.rs/)

We are currently hard at work trying to release `3.0`. We have a `3.0.0-beta.2` prerelease out but we do not give any guarantees that it's API is stable. We do not have a changelog yet which will be written down after we are sure about the API stability. We recommend users to not update to the prerelease version yet and to wait for the official `3.0`.

> If you're looking for the readme & examples for `clap v2.33` - find it on [github](https://github.com/clap-rs/clap/tree/v2.33.0), [crates.io](https://crates.io/crates/clap/2.33.0), [docs.rs](https://docs.rs/clap/2.33.0/clap/).

1. [About](#about)
2. [FAQ](#faq)
3. [Features](#features)
4. [Quick Example](#quick-example)
      1. [Using Derive Macros](#using-derive-macros)
      2. [Using Builder Pattern](#using-builder-pattern)
      3. [Using YAML](#using-yaml)
      4. [Using Macros](#using-macros)
      5. [Running it](#running-it)
5. [Try it!](#try-it)
   1. [Pre-Built Test](#pre-built-test)
   2. [Build Your Own Binary](#build-your-own-binary)
6. [Usage](#usage)
   1. [Optional Dependencies / Features](#optional-dependencies--features)
      1. [Features enabled by default](#features-enabled-by-default)
      2. [Opt-in features](#opt-in-features)
   2. [More Information](#more-information)
7. [Sponsors](#sponsors)
8. [Contributing](#contributing)
   1. [Compatibility Policy](#compatibility-policy)
      1. [Minimum Supported Version of Rust (MSRV)](#minimum-supported-version-of-rust-msrv)
      2. [Breaking Changes](#breaking-changes)
9. [License](#license)
10. [Related Crates](#related-crates)

## About

`clap` is used to parse *and validate* the string of command line arguments provided by a user at runtime. You provide the list of valid possibilities, and `clap` handles the rest. This means you focus on your *applications* functionality, and less on the parsing and validating of arguments.

`clap` provides many things 'for free' (with no configuration) including the traditional version and help switches (or flags) along with associated messages. If you are using subcommands, `clap` will also auto-generate a `help` subcommand and separate associated help messages.

Once `clap` parses the user provided string of arguments, it returns the matches along with any applicable values. If the user made an error or typo, `clap` informs them with a friendly message and exits gracefully (or returns a `Result` type and allows you to perform any clean up prior to exit). Because of this, you can make reasonable assumptions in your code about the validity of the arguments prior to your applications main execution.

## FAQ

For a full FAQ, see [this](FAQ.md)

## Features

Below are a few of the features which `clap` supports, full descriptions and usage can be found in the [documentation][docs] and [examples][examples] directory

* Generate a CLI simply by defining a struct!
* **Auto-generated Help, Version, and Usage information**
  - Can optionally be fully, or partially overridden if you want a custom help, version, or usage statements
* **Auto-generated completion scripts (Bash, Zsh, Fish, Elvish and PowerShell)**
  - Using [`clap_generate`](https://github.com/clap-rs/clap/tree/master/clap_generate)
  - Even works through many multiple levels of subcommands
  - Works with options which only accept certain values
  - Works with subcommand aliases
* **Flags / Switches** (i.e. bool fields)
  - Both short and long versions supported (i.e. `-f` and `--flag` respectively)
  - Supports combining short versions (i.e. `-fBgoZ` is the same as `-f -B -g -o -Z`)
  - Supports multiple occurrences (i.e. `-vvv` or `-v -v -v`)
* **Positional Arguments** (i.e. those which are based off an index from the program name)
  - Supports multiple values (i.e. `myprog <file>...` such as `myprog file1.txt file2.txt` being two values for the same "file" argument)
  - Supports Specific Value Sets (See below)
  - Can set value parameters (such as the minimum number of values, the maximum number of values, or the exact number of values)
  - Can set custom validations on values to extend the argument parsing capability to truly custom domains
* **Option Arguments** (i.e. those that take values)
  - Both short and long versions supported (i.e. `-o value`, `-ovalue`, `-o=value` and `--option value` or `--option=value` respectively)
  - Supports multiple values (i.e. `-o <val1> -o <val2>` or `-o <val1> <val2>`)
  - Supports delimited values (i.e. `-o=val1,val2,val3`, can also change the delimiter)
  - Supports Specific Value Sets (See below)
  - Supports named values so that the usage/help info appears as `-o <FILE> <INTERFACE>` etc. for when you require specific multiple values
  - Can set value parameters (such as the minimum number of values, the maximum number of values, or the exact number of values)
  - Can set custom validations on values to extend the argument parsing capability to truly custom domains
* **Sub-Commands** (i.e. `git add <file>` where `add` is a sub-command of `git`)
  - Support their own sub-arguments, and sub-sub-commands independent of the parent
  - Get their own auto-generated Help, Version, and Usage independent of parent
* **Support for building CLIs from YAML** - This keeps your Rust source nice and tidy and makes supporting localized translation very simple!
* **Requirement Rules**: Arguments can define the following types of requirement rules
  - Can be required by default
  - Can be required only if certain arguments are present
  - Can require other arguments to be present
  - Can be required only if certain values of other arguments are used
* **Confliction Rules**: Arguments can optionally define the following types of exclusion rules
  - Can be disallowed when certain arguments are present
  - Can disallow use of other arguments when present
* **Groups**: Arguments can be made part of a group
  - Fully compatible with other relational rules (requirements, conflicts, and overrides) which allows things like requiring the use of any arg in a group, or denying the use of an entire group conditionally
* **Specific Value Sets**: Positional or Option Arguments can define a specific set of allowed values (i.e. imagine a `--mode` option which may *only* have one of two values `fast` or `slow` such as `--mode fast` or `--mode slow`)
* **Default Values**
  - Also supports conditional default values (i.e. a default which only applies if specific arguments are used, or specific values of those arguments)
* **Automatic Version from Cargo.toml**: `clap` is fully compatible with Rust's `env!()` macro for automatically setting the version of your application to the version in your Cargo.toml. See [09_auto_version example](examples/09_auto_version.rs) for how to do this (Thanks to [jhelwig](https://github.com/jhelwig) for pointing this out)
* **Typed Values**: You can use several convenience macros provided by `clap` to get typed values (i.e. `i32`, `u8`, etc.) from positional or option arguments so long as the type you request implements `std::str::FromStr` See the [12_typed_values example](examples/12_typed_values.rs). You can also use `clap`s `arg_enum!` macro to create an enum with variants that automatically implement `std::str::FromStr`. See [13_enum_values example](examples/13_enum_values.rs) for details
* **Suggestions**: Suggests corrections when the user enters a typo. For example, if you defined a `--myoption` argument, and the user mistakenly typed `--moyption` (notice `y` and `o` transposed), they would receive a `Did you mean '--myoption'?` error and exit gracefully. This also works for subcommands and flags. (Thanks to [Byron](https://github.com/Byron) for the implementation) (This feature can optionally be disabled, see 'Optional Dependencies / Features')
* **Colorized Errors (Non Windows OS only)**: Error message are printed in colored text (this feature can optionally be disabled, see 'Optional Dependencies / Features').
* **Global Arguments**: Arguments can optionally be defined once, and be available to all child subcommands. There values will also be propagated up/down throughout all subcommands.
* **Custom Validations**: You can define a function to use as a validator of argument values. Imagine defining a function to validate IP addresses, or fail parsing upon error. This means your application logic can be solely focused on *using* values.
* **POSIX Compatible Conflicts/Overrides** - In POSIX args can be conflicting, but not fail parsing because whichever arg comes *last* "wins" so to speak. This allows things such as aliases (i.e. `alias ls='ls -l'` but then using `ls -C` in your terminal which ends up passing `ls -l -C` as the final arguments. Since `-l` and `-C` aren't compatible, this effectively runs `ls -C` in `clap` if you choose...`clap` also supports hard conflicts that fail parsing). (Thanks to [Vinatorul](https://github.com/Vinatorul)!)
* Supports the Unix `--` meaning, only positional arguments follow

## Quick Example

The following examples show a quick example of some of the very basic functionality of `clap`. For more advanced usage, such as requirements, conflicts, groups, multiple values and occurrences see the [documentation][docs], [examples][examples] directory of this repository.

 **NOTE:** All of these examples are functionally the same, but show different styles in which to use `clap`. These different styles are purely a matter of personal preference.

Add `clap` to your `Cargo.toml`

```toml
[dependencies]
clap = "3.0.0-beta.2"
```

#### Using Derive Macros

The first example shows the simplest way to use `clap`, by defining a struct. If you're familiar with the `structopt` crate you're in luck, it's the same! (In fact it's the exact same code running under the covers!)

```rust,no_run
// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use clap::Clap;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Test(Test),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short)]
    debug: bool
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }
    }

    // more program logic goes here...
}
```

#### Using Builder Pattern

This second method shows a method using the 'Builder Pattern' which allows more advanced configuration options (not shown in this small example), or even dynamically generating arguments when desired. The downside is it's more verbose.

```rust,no_run
// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "builder pattern" method of creating arguments
// which the most flexible, but also most verbose.
use clap::{Arg, App};

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::new("debug")
                .short('d')
                .about("print debug information verbosely")))
        .get_matches();

    // Same as above examples...
}
```

The next example shows a far less verbose method, but sacrifices some of the advanced configuration options (not shown in this small example). This method also takes a *very* minor runtime penalty.

```rust,no_run
// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "usage strings" method of creating arguments
// which is less verbose
use clap::{Arg, App};

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .arg("<INPUT>              'Sets the input file to use'")
        .arg("-v...                'Sets the level of verbosity'")
        .subcommand(App::new("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg("-d, --debug 'Print debug information'"))
        .get_matches();

    // Same as previous example...
}
```

#### Using YAML

This third method shows how you can use a YAML file to build your CLI and keep your Rust source tidy
or support multiple localized translations by having different YAML files for each localization.

First, create the `cli.yaml` file to hold your CLI options, but it could be called anything we like:

```yaml
name: myapp
version: "1.0"
author: Kevin K. <kbknapp@gmail.com>
about: Does awesome things
args:
    - config:
        short: c
        long: config
        value_name: FILE
        about: Sets a custom config file
        takes_value: true
    - INPUT:
        about: Sets the input file to use
        required: true
        index: 1
    - verbose:
        short: v
        multiple: true
        about: Sets the level of verbosity
subcommands:
    - test:
        about: controls testing features
        version: "1.3"
        author: Someone E. <someone_else@other.com>
        args:
            - debug:
                short: d
                about: print debug information
```

Since this feature requires additional dependencies that not everyone may want, it is *not* compiled in by default and we need to enable a feature flag in Cargo.toml:

Simply add the `yaml` feature flag to your `Cargo.toml`.

```toml
[dependencies]
clap = { version = "3.0.0-beta.2", features = ["yaml"] }
```

Finally we create our `main.rs` file just like we would have with the previous two examples:

```rust,ignore
// (Full example with detailed comments in examples/17_yaml.rs)
//
// This example demonstrates clap's building from YAML style of creating arguments which is far
// more clean, but takes a very small performance hit compared to the other two methods.
use clap::{App, load_yaml};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // Same as previous examples...
}
```

#### Using Macros

Finally there is a macro version, which is like a hybrid approach offering the speed of the
builder pattern (the first example), but without all the verbosity.

```rust,no_run
use clap::clap_app;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (about: "Does awesome things")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg verbose: -v --verbose "Print test information verbosely")
        (@subcommand test =>
            (about: "controls testing features")
            (version: "1.3")
            (author: "Someone E. <someone_else@other.com>")
            (@arg debug: -d ... "Sets the level of debugging information")
        )
    ).get_matches();

    // Same as previous examples...
}
```

#### Running it

If you were to compile any of the above programs and run them with the flag `--help` or `-h` (or `help` subcommand, since we defined `test` as a subcommand) the following would be output (except the first example where the help message sort of explains the Rust code).

```bash
$ myprog --help
My Super Program 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

ARGS:
    INPUT    The input file to use

USAGE:
    MyApp [FLAGS] [OPTIONS] <INPUT> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Controls testing features
```

**NOTE:** You could also run `myapp test --help` or `myapp help test` to see the help message for the `test` subcommand.

## Try it!

### Pre-Built Test

To try out the pre-built [examples][examples], use the following steps:

* Clone the repository `$ git clone https://github.com/clap-rs/clap && cd clap/`
* Compile the example `$ cargo build --example <EXAMPLE>`
* Run the help info `$ ./target/debug/examples/<EXAMPLE> --help`
* Play with the arguments!
* You can also do a onetime run via `$ cargo run --example <EXAMPLE> -- [args to example]`

### Build Your Own Binary

To test out `clap`'s default auto-generated help/version follow these steps:
* Create a new cargo project `$ cargo new fake --bin && cd fake`
* Write your program as described in the quick example section.
* Build your program `$ cargo build --release`
* Run with help or version `$ ./target/release/fake --help` or `$ ./target/release/fake --version`

## Usage

For full usage, add `clap` as a dependency in your `Cargo.toml` to use from crates.io:

```toml
[dependencies]
clap = "3.0.0-beta.2"
```

Define a list of valid arguments for your program (see the [documentation][docs] or [examples][examples] directory of this repo)

Then run `cargo build` or `cargo update && cargo build` for your project.

### Optional Dependencies / Features

#### Features enabled by default

* **derive**: Enables the custom derive (i.e. `#[derive(Clap)]`). Without this you must use one of the other methods of creating a `clap` CLI listed above
* **suggestions**: Turns on the `Did you mean '--myoption'?` feature for when users make typos. (builds dependency `strsim`)
* **color**: Turns on colored error messages. You still have to turn on colored help by setting `AppSettings::ColoredHelp`. (builds dependency `termcolor`)

To disable these, add this to your `Cargo.toml`:

```toml
[dependencies.clap]
version = "3.0.0-beta.2"
default-features = false
```

You can also selectively enable only the features you'd like to include, by adding:

```toml
[dependencies.clap]
version = "3.0.0-beta.2"
default-features = false

# Cherry-pick the features you'd like to use
features = [ "suggestions", "color" ]
```

#### Opt-in features

* **"wrap_help"**: Turns on the help text wrapping feature, based on the terminal size. (builds dependency `term-size`)
* **"yaml"**: Enables building CLIs from YAML documents. (builds dependency `yaml-rust`)
* **"regex"**: Enables regex validators. (builds dependency `regex`)
* **"unstable"**: Enables unstable `clap` features that may change from release to release

### More Information

You can find complete documentation on the [docs.rs][docs] for this project.

You can also find usage examples in the [examples][examples] directory of this repo.

## Sponsors

<!-- omit in TOC -->
### Gold

[![](https://opencollective.com/clap/tiers/gold.svg?avatarHeight=36&width=600)](https://opencollective.com/clap)

<!-- omit in TOC -->
### Silver

[![](https://opencollective.com/clap/tiers/silver.svg?avatarHeight=36&width=600)](https://opencollective.com/clap)

<!-- omit in TOC -->
### Bronze

[![](https://opencollective.com/clap/tiers/bronze.svg?avatarHeight=36&width=600)](https://opencollective.com/clap)

<!-- omit in TOC -->
### Backer

[![](https://opencollective.com/clap/tiers/backer.svg?avatarHeight=36&width=600)](https://opencollective.com/clap)

## Contributing

Details on how to contribute can be found in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

### Compatibility Policy

Because `clap` takes SemVer and compatibility seriously, this is the official policy regarding breaking changes and minimum required versions of Rust.

`clap` will pin the minimum required version of Rust to the CI builds. Bumping the minimum version of Rust is considered a minor breaking change, meaning *at a minimum* the minor version of `clap` will be bumped.

In order to keep from being surprised of breaking changes, it is **highly** recommended to use the `~major.minor.patch` style in your `Cargo.toml` only if you wish to target a version of Rust that is *older* than current stable minus two releases:

```toml
[dependencies]
clap = "~3.0.0-beta.2"
```

This will cause *only* the patch version to be updated upon a `cargo update` call, and therefore cannot break due to new features, or bumped minimum versions of Rust.

#### Minimum Supported Version of Rust (MSRV)

`clap` will officially support current stable Rust, minus two releases, but may work with prior releases as well. For example, current stable Rust at the time of this writing is 1.38.0, meaning `clap` is guaranteed to compile with 1.36.0 and beyond.

At the 1.39.0 stable release, `clap` will be guaranteed to compile with 1.37.0 and beyond, etc.

The following is a list of the minimum required version of Rust to compile `clap` by our `MAJOR.MINOR` version number:

|  clap  |  MSRV  |
| :----: | :----: |
| >=3.0  | 1.46.0 |
| >=2.21 | 1.24.0 |
| >=2.2  | 1.12.0 |
| >=2.1  | 1.6.0  |
| >=1.5  | 1.4.0  |
| >=1.4  | 1.2.0  |
| >=1.2  | 1.1.0  |
| >=1.0  | 1.0.0  |

#### Breaking Changes

`clap` takes a similar policy to Rust and will bump the major version number upon breaking changes with only the following exceptions:

 * The breaking change is to fix a security concern
 * The breaking change is to be fixing a bug (i.e. relying on a bug as a feature)
 * The breaking change is a feature isn't used in the wild, or all users of said feature have given approval *prior* to the change

## License

`clap` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository for more information.

## Related Crates

There are several excellent crates which can be used with `clap`, I recommend checking them all out! If you've got a crate that would be a good fit to be used with `clap` open an issue and let me know, I'd love to add it!

* [`assert_cmd`](https://github.com/assert-rs/assert_cmd) - This crate allows you test your CLIs in a very intuitive and functional way!

[docs]: https://docs.rs/clap
[examples]: examples
