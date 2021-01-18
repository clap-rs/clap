# clap_derive

Parse command line argument by defining a struct.  It combines [structopt](https://github.com/TeXitoi/structopt) and [clap](https://crates.io/crates/clap) into a single experience. This crate is used by clap, and not meant to be used directly by
consumers.

## Documentation

Find it on [Docs.rs](https://docs.rs/clap_derive).  You can also check the [examples](https://github.com/clap-rs/clap/tree/master/clap_derive/examples) and the [changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md).

## Example

Add `clap` to your dependencies of your `Cargo.toml`:

```toml
[dependencies]
clap = "3"
```

And then, in your rust file:
```rust
use std::path::PathBuf;
use clap::Clap;

/// A basic example
#[derive(Clap, Debug)]
#[clap(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Set speed
    #[clap(short, long, default_value = "42")]
    speed: f64,

    /// Output file
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[clap(short = "c", long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[clap(short, long)]
    level: Vec<String>,

    /// Files to process
    #[clap(name = "FILE", value_hint = ValueHint::AnyPath)]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
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
basic 0.3.0
Guillaume Pinot <texitoi@texitoi.eu>, others
A basic example

USAGE:
    basic [FLAGS] [OPTIONS] --output <output> [--] [file]...

ARGS:
    <FILE>...    Files to process

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
    -l, --level <level>...     admin_level to consider
    -c, --nb-cars <nb-cars>    Number of cars
    -o, --output <output>      Output file
    -s, --speed <speed>        Set speed [default: 42]

ARGS:
    <file>...    Files to process
$ ./basic -o foo.txt
Opt {
    debug: false,
    verbose: 0,
    speed: 42.0,
    output: "foo.txt",
    nb_cars: None,
    level: [],
    files: [],
}
$ ./basic -o foo.txt -dvvvs 1337 -l alice -l bob --nb-cars 4 bar.txt baz.txt
Opt {
    debug: true,
    verbose: 3,
    speed: 1337.0,
    output: "foo.txt",
    nb_cars: Some(
        4,
    ),
    level: [
        "alice",
        "bob",
    ],
    files: [
        "bar.txt",
        "baz.txt",
    ],
}
```

## clap_derive rustc version policy

- Minimum rustc version modification must be specified in the [changelog](https://github.com/clap-rs/clap_derive/blob/master/CHANGELOG.md) and in the [travis configuration](https://github.com/clap-rs/clap_derive/blob/master/.travis.yaml).
- Contributors can increment minimum rustc version without any justification if the new version is required by the latest version of one of clap_derive's depedencies (`cargo update` will not fail on clap_derive).
- Contributors can increment minimum rustc version if the library user experience is improved.

## Why

I've (@TeXitoi) used [docopt](https://crates.io/crates/docopt) for a long time (pre rust 1.0). I really like the fact that you have a structure with the parsed argument: no need to convert `String` to `f64`, no useless `unwrap`. But on the other hand, I don't like to write by hand the usage string. That's like going back to the golden age of WYSIWYG editors.  Field naming is also a bit artificial.

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
