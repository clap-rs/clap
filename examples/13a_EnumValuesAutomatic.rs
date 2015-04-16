// You can use clap's value_t! macro with a custom enum by implementing the std::str::FromStr
// trait which is very straight forward. There are two ways to do this, for simple enums you
// can use clap's simple_enum! macro, but if you require additional functionality you can
// create and implement the trait manually.
//
// In the following example we will create an enum with 4 values, assign a positional argument
// that accepts only one of those values, and use clap to parse the argument.
//
// Start with bringing the trait into scope.

// Add clap like normal
#[macro_use]
extern crate clap;

use clap::{App, Arg};

// Define your enum, the simple_num! macro takes a enum name followed by => and each value
// separated by a ','
simple_enum!{ Vals => Foo, Bar, Baz, Qux }

fn main() {
    // Create the application like normal
    let m = App::new("myapp")
                    // Use a single positional argument that is required
                    .arg(Arg::from_usage("<type> 'The type to use'")
                            // Define the list of possible values
                            .possible_values(vec!["Foo", "Bar", "Baz", "Qux"]))
                    .get_matches();

    let t = value_t_or_exit!(m.value_of("type"), Vals);

    // Now we can use our enum like normal.
    match t {
        Vals::Foo => println!("Found a Foo"),
        Vals::Bar => println!("Found a Bar"),
        Vals::Baz => println!("Found a Baz"),
        Vals::Qux => println!("Found a Qux")
    }
}