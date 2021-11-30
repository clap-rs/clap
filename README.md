<!-- omit in TOC -->
# clap

> **Command Line Argument Parser for Rust**

[![Crates.io](https://img.shields.io/crates/v/clap?style=flat-square)](https://crates.io/crates/clap)
[![Crates.io](https://img.shields.io/crates/d/clap?style=flat-square)](https://crates.io/crates/clap)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/master/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/master/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/workflow/status/clap-rs/clap/CI/staging?style=flat-square)](https://github.com/clap-rs/clap/actions/workflows/ci.yml?query=branch%3Astaging)
[![Coverage Status](https://img.shields.io/coveralls/github/clap-rs/clap/master?style=flat-square)](https://coveralls.io/github/clap-rs/clap?branch=master)
[![Contributors](https://img.shields.io/github/contributors/clap-rs/clap?style=flat-square)](https://github.com/clap-rs/clap/graphs/contributors)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

1. [About](#about)
5. [API Reference](https://docs.rs/clap)
    - [Feature Flags](#feature-flags)
2. [CHANGELOG](https://github.com/clap-rs/clap/blob/master/docs/CHANGELOG.md)
2. [FAQ](https://github.com/clap-rs/clap/blob/master/docs/FAQ.md)
3. [Questions & Discussions](https://github.com/clap-rs/clap/discussions)
8. [Contributing](https://github.com/clap-rs/clap/blob/master/CONTRIBUTING.md)

## About

Create your command-line parser, with all of the bells and whistles, declaratively or procedurally.

### Example

<!-- Copied from examples/demo.{rs,md} -->
```rust,no_run
use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct Cli {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.toml", value_name = "PATH")]
    config: std::path::PathBuf,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let args = Cli::parse();

    println!("Value for config: {}", args.config.display());
    println!("Using input file: {}", args.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match args.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    // more program logic goes here...
}
```
```bash
$ demo --help
clap [..]

Kevin K. <kbknapp@gmail.com>, Clap Maintainers

A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    demo[EXE] [OPTIONS] <INPUT>

ARGS:
    <INPUT>    Some input. Because this isn't an Option<T> it's required to be used

OPTIONS:
    -c, --config <PATH>    Sets a custom config file. Could have been an Option<T> with no default
                           too [default: default.toml]
    -h, --help             Print help information
    -v, --verbose          A level of verbosity, and can be used multiple times
    -V, --version          Print version information
```
*(version number and `.exe` extension on windows replaced by placeholders)*

### Aspirations

- Out of the box, users get a polished CLI experience
  - Including common argument behavior, help generation, suggested fixes for users, colored output, [shell completions](https://github.com/clap-rs/clap/tree/master/clap_generate), etc
- Flexible enough to port your existing CLI interface
  - However, we won't necessarily streamline support for each use case
- Reasonable parse performance
- We will support the last two minor Rust releases (MSRV)
- We follow semver and will wait about 6 months between major breaking changes

While these aspirations can be at odds with fast build times and low binary
size, we will still strive to keep these reasonable.  Check out the
[argparse-benchmarks](https://github.com/rust-cli/argparse-benchmarks-rs) for
CLI parsers optimized for other use cases.

### Related Projects

- [Command-line Apps for Rust](https://rust-cli.github.io/book/index.html) book
- [`trycmd`](https://github.com/epage/trycmd):  Snapshot testing
  - Or for more control, [`assert_cmd`](https://github.com/assert-rs/assert_cmd) and [`assert_fs`](https://github.com/assert-rs/assert_fs)

## Feature Flags

### Default Features

* **std**: _Not Currently Used._ Placeholder for supporting `no_std` environments in a backwards compatible manner.
* **color**: Turns on colored error messages.
* **suggestions**: Turns on the `Did you mean '--myoption'?` feature for when users make typos.

#### Optional features

* **derive**: Enables the custom derive (i.e. `#[derive(Parser)]`). Without this you must use one of the other methods of creating a `clap` CLI listed above.
* **cargo**: Turns on macros that read values from `CARGO_*` environment variables.
* **env**: Turns on the usage of environment variables during parsing.
* **regex**: Enables regex validators.
* **unicode**: Turns on support for unicode characters (including emoji) in arguments and help messages.
* **wrap_help**: Turns on the help text wrapping feature, based on the terminal size.

#### Experimental features

**Warning:** These may contain breaking changes between minor releases.

* **unstable-replace**: Enable [`App::replace`](https://github.com/clap-rs/clap/issues/2836)
* **unstable-multicall**: Enable [`AppSettings::Multicall`](https://github.com/clap-rs/clap/issues/2861)
* **unstable-grouped**: Enable [`ArgMatches::grouped_values_of`](https://github.com/clap-rs/clap/issues/2924)
