# StructOpt [![Build status](https://travis-ci.org/TeXitoi/structopt.svg?branch=master)](https://travis-ci.org/TeXitoi/structopt) [![](https://img.shields.io/crates/v/structopt.svg)](https://crates.io/crates/structopt) [![](https://docs.rs/structopt/badge.svg)](https://docs.rs/structopt)

Parse command line argument by defining a struct.  It combines [clap](https://crates.io/crates/clap) with custom derive.

## Documentation

Find it on [Docs.rs](https://docs.rs/structopt).  You can also check the [examples](https://github.com/TeXitoi/structopt/tree/master/examples) and the [changelog](https://github.com/TeXitoi/structopt/blob/master/CHANGELOG.md).

## Example

Add `structopt` to your dependencies of your `Cargo.toml`:
```toml
[dependencies]
structopt = "0.2"
```

And then, in your rust file:
```rust
#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    // The number of occurences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Set speed
    #[structopt(short = "s", long = "speed", default_value = "42")]
    speed: f64,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// Number of cars
    #[structopt(short = "c", long = "nb-cars")]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[structopt(short = "l", long = "level")]
    level: Vec<String>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
```

Using this example:
```
$ ./basic
error: The following required arguments were not provided:
    --output <output>

USAGE:
    basic --output <output> --speed <speed>

For more information try --help
$ ./basic --help
basic 0.2.0
Guillaume Pinot <texitoi@texitoi.eu>
A basic example

USAGE:
    basic [FLAGS] [OPTIONS] --output <output> [--] [FILE]...

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -c, --car <car>           Number of car
    -l, --level <level>...    admin_level to consider
    -o, --output <output>     Output file
    -s, --speed <speed>       Set speed [default: 42]

ARGS:
    <FILE>...    Files to process
$ ./basic -o foo.txt
Opt { debug: false, verbose: 0, speed: 42, output: "foo.txt", car: None, level: [], files: [] }
$ ./basic -o foo.txt -dvvvs 1337 -l alice -l bob --car 4 bar.txt baz.txt
Opt { debug: true, verbose: 3, speed: 1337, output: "foo.txt", car: Some(4), level: ["alice", "bob"], files: ["bar.txt", "baz.txt"] }
```

## Why

I use [docopt](https://crates.io/crates/docopt) since a long time (pre rust 1.0). I really like the fact that you have a structure with the parsed argument: no need to convert `String` to `f64`, no useless `unwrap`. But on the other hand, I don't like to write by hand the usage string. That's like going back to the golden age of WYSIWYG editors.  Field naming is also a bit artificial.

Today, the new standard to read command line arguments in Rust is [clap](https://crates.io/crates/clap).  This library is so feature full! But I think there is one downside: even if you can validate argument and expressing that an argument is required, you still need to transform something looking like a hashmap of string vectors to something useful for your application.

Now, there is stable custom derive. Thus I can add to clap the automatic conversion that I miss. Here is the result.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

