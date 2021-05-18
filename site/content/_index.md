+++
title = "Fast & Modern CLI Framework for Rust"
+++

**Clap** is a simple-to-use, efficient, and full-featured library for parsing command line arguments and subcommands when writing console/terminal applications.

Here is an example of a simple program:

```rust
use clap::Clap;

/// Simple program to greet a person
#[derive(Clap, Debug)]
#[clap(name = "hello")]
struct Hello {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value = "1")]
    count: u8,
}

fn main() {
    let hello = Hello::parse();

    for _ in 0..hello.count {
        println!("Hello {}!", hello.name)
    }
}
```

The above example program can be run as shown below:

```
$ hello --name John --count 3
Hello John!
Hello John!
Hello John!
```

The program also has automatically generated help message:

```
hello

Simple program to greet a person

USAGE:
    hello [OPTIONS] --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>    Number of times to greet [default: 1]
    -n, --name <name>      Name of the person to greet
```
