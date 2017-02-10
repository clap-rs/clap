# StructOpt [![Build status](https://travis-ci.org/TeXitoi/structopt.svg?branch=master)](https://travis-ci.org/TeXitoi/structopt) [![](https://img.shields.io/crates/v/structopt.svg)](https://crates.io/crates/structopt) [![](https://docs.rs/structopt-derive/badge.svg)](https://docs.rs/structopt-derive)

Parse command line argument by defining a struct.  It combines [clap](https://crates.io/crates/clap) with custom derive.

## Documentation

Find it on Docs.rs: [structopt-derive](https://docs.rs/structopt-derive) and [structopt](https://docs.rs/structopt).

## Example

```rust
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// A flag, true if used in the command line.
    #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
    debug: bool,

    /// An argument of type float, with a default value.
    #[structopt(short = "s", long = "speed", help = "Set speed", default_value = "42")]
    speed: f64,

    /// Needed parameter, the first on the command line.
    #[structopt(help = "Input file")]
    input: String,

    /// An optional parameter, will be `None` if not present on the
    /// command line.
    #[structopt(help = "Output file, stdout if not present")]
    output: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
```

Using this example:
```
$ ./example
error: The following required arguments were not provided:
    <input>

USAGE:
    example [FLAGS] [OPTIONS] <input> [ARGS]

For more information try --help
$ ./example --help
example 0.0.0
Guillaume Pinot <texitoi@texitoi.eu>
An example of StructOpt usage.

USAGE:
    example [FLAGS] [OPTIONS] <input> [ARGS]

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --speed <speed>    Set speed [default: 42]

ARGS:
    <input>     Input file
    <output>    Output file, stdout if not present
$ ./example foo
Opt { debug: false, speed: 42, input: "foo", output: None }
$ ./example -ds 1337 foo bar
Opt { debug: true, speed: 1337, input: "foo", output: Some("bar") }
```

## Why

I use [docopt](https://crates.io/crates/docopt) since  long time. I really like the fact that you have a structure with the parsed argumentThat's like going back to the : no need to convert `String` to `f64`, no useless `unwrap`. But in another hand, I don't like to write by hand the usage string. That's like going back to the golden age of WYSIWYG editors.

Today, the new standard to read command line arguments in Rust is [clap](https://crates.io/crates/clap).  This library is so feature full! But I think there is one downside: even if you can validate arument, expressing that an argument is required, you still need to transform something looking like a hashmap of string vectors to something useful for your application.

Now, there is stable custom derive. Thus I can add to clap the automatic conversion that I miss. Here is the result.
