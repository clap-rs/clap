// You can use clap's value_t! macro with a custom enum by implementing the std::str::FromStr
// trait which is very straight forward. There are three ways to do this, for simple enums
// meaning those that don't require 'pub' or any '#[derive()]' directives you can use clap's
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

// Using arg_enum! is more like traditional enum declarations
//
// **NOTE:** Only bare variants are supported
arg_enum! {
    #[derive(Debug)]
    pub enum Oof {
        Rab,
        Zab,
        Xuq
    }
}

arg_enum! {
    #[derive(Debug)]
    enum Foo {
        Bar,
        Baz,
        Qux
    }
}

fn main() {
    // Create the application like normal
    let enum_vals = ["fast", "slow"];
    let m = App::new("myapp")
        // Use a single positional argument that is required
        .arg(Arg::from("<foo> 'The Foo to use'").possible_values(&Foo::variants()))
        .arg(
            Arg::from("<speed> 'The speed to use'")
                // You can define a list of possible values if you want the values to be
                // displayed in the help information. Whether you use possible_values() or
                // not, the valid values will ALWAYS be displayed on a failed parse.
                .possible_values(&enum_vals),
        )
        // For the second positional, lets not use possible_values() just to show the difference
        .arg("<oof> 'The Oof to use'")
        .get_matches();

    let t = value_t!(m.value_of("foo"), Foo).unwrap_or_else(|e| e.exit());
    let t2 = value_t!(m.value_of("oof"), Oof).unwrap_or_else(|e| e.exit());

    // Now we can use our enum like normal.
    match t {
        Foo::Bar => println!("Found a Bar"),
        Foo::Baz => println!("Found a Baz"),
        Foo::Qux => println!("Found a Qux"),
    }

    // Since our Oof derives Debug, we can do this:
    println!("Oof: {:?}", t2);
}
