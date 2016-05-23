// There are multiple ways in which to use enums as the "Keys" to access args. This has the benefit
// of not being "stringly-typed" and is *far* less error prone. It also allows one to take full
// advantage of auto-completion with tools like Racer (https://github.com/phildawes/racer). It also
// lets rustc do the error checking for you when you decide to change arugment names and such.

// This first method we'll look at is the most simple version, where enum variants are used
// literally as the arg keys. For args like positional arguments, this means one must either make
// the variants how they'd like them to be displayed in help messages and such. This can be at odds
// with Rust best practices for style guides, so in other examples we'll look at other ways use
// enums with Args.

// On to the code...

// First we import the clap macros
#[macro_use]
extern crate clap;
use clap::{App, Arg};

// Next we declare the "keys" we'll use to access the arguments
//
// NOTE: The enums support use of `pub` or `#[meta]` style attributes (like derive(), or cfg()
// attributes), or any combinations of those two elements
args!{
    enum MyProg {
        Verbose,
        Config
    }
}

fn main() {
    // We declare the App struct like normal. We could also use a use statement such as
    // `use MyProg::*;` to save a few key-strokes, but for this simple example we'll leave
    // that out.
    let m = App::new("myprog")
        .arg(Arg::with_name(MyProg::Verbose)
            .short("v")
            .help("print verbose output"))
        .arg(Arg::with_name(MyProg::Config)
            .long("config")
            .value_name("FILE")
            .help("use a custom config file"))
        .get_matches();

    // Now to access the args we use the enum variants
    if m.is_present(MyProg::Verbose) {
        println!("Printing verbosely...");
    }

    //println!("Verbose used {} times", m.occurrences_of(MyProg::Verbos)) // ERROR typo! (Mising E)

    if let Some(file) = m.value_of(MyProg::Config) {
        println!("Using config file: {}", file);
    }
}
