// You can use clap's value_t! macro with a custom enum by implementing the std::str::FromStr
// trait which is very straight forward. There are three ways to do this, for simple enums 
// meaning those that don't require 'pub' or any '#[derive()]' directives you can use clas's
// simple_enum! macro. For those that require 'pub' or any '#[derive()]'s you can use clap's
// arg_enum! macro. The third way is to implement std::str::FromStr manually. 
//
// In most circumstances using either simple_enum! or arg_enum! is fine.
//
// In the following example we will create two enums using macros, assign a positional argument
// that accepts only one of those values, and use clap to parse the argument.

// Add clap like normal
#[macro_use]
extern crate clap;

use clap::{App, Arg};

// Define your enum, the simple_num! macro takes a enum name followed by => and each value
// separated by a ','
simple_enum!{ Foo => Bar, Baz, Qux }

// Using arg_enum! is more like traditional enum declarations
//
// **NOTE:** Only bare variants are supported
arg_enum!{
    #[derive(Debug)]
    pub enum Oof {
        Rab,
        Zab,
        Xuq
    }
}

fn main() {
    // Create the application like normal
    let m = App::new("myapp")
                    // Use a single positional argument that is required
                    .arg(Arg::from_usage("<type> 'The Foo to use'")
                            // You can define a list of possible values if you want the values to be
                            // displayed in the help information. Whether you use possible_values() or
                            // not, the valid values will ALWAYS be displayed on a failed parse.
                            .possible_values(vec!["Bar", "Baz", "Qux"]))
                    // For the second positional, lets not use possible_values() just to show the difference
                    .arg_from_usage("<type2> 'The Oof to use'")
                    .get_matches();

    let t = value_t_or_exit!(m.value_of("type"), Foo);
    let t2 = value_t_or_exit!(m.value_of("type2"), Oof);


    // Now we can use our enum like normal.
    match t {
        Foo::Bar => println!("Found a Bar"),
        Foo::Baz => println!("Found a Baz"),
        Foo::Qux => println!("Found a Qux")
    }

    // Since our Oof derives Debug, we can do this:
    println!("Oof: {:?}", t2);
}