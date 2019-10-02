// In the following example we will create an enum with 4 values, assign a positional argument
// that accepts only one of those values, and use clap to parse the argument.
//
// Start with bringing the trait into scope.
use std::str::FromStr;

// Add clap like normal
#[macro_use]
extern crate clap;

use clap::{App, Arg};

// Define your enum
enum Vals {
    Foo,
    Bar,
    Baz,
    Qux,
}

// Implement the trait
impl FromStr for Vals {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Foo" => Ok(Vals::Foo),
            "Bar" => Ok(Vals::Bar),
            "Baz" => Ok(Vals::Baz),
            "Qux" => Ok(Vals::Qux),
            _ => Err("no match"),
        }
    }
}

fn main() {
    // Create the application like normal
    let m = App::new("myapp")
        // Use a single positional argument that is required
        .arg(
            Arg::from("<type> 'The type to use'")
                // Define the list of possible values
                .possible_values(&["Foo", "Bar", "Baz", "Qux"]),
        )
        .get_matches();

    let t = value_t!(m, "type", Vals).unwrap_or_else(|e| e.exit());

    // Now we can use our enum like normal.
    match t {
        Vals::Foo => println!("Found a Foo"),
        Vals::Bar => println!("Found a Bar"),
        Vals::Baz => println!("Found a Baz"),
        Vals::Qux => println!("Found a Qux"),
    }
}
