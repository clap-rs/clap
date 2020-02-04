extern crate clap;

use clap::{App, Arg};

fn main() {
    // You can define a function (or a closure) to use as a validator to argument values. The
    // function must accept a String and return Result<(), String> where Err(String) is the message
    // displayed to the user.

    let matches = App::new("myapp")
        // Application logic goes here...
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .index(1)
                .required(true)
                // You can pass in a closure, or a function
                .validator(is_png),
        )
        .get_matches();

    // Here we can call .unwrap() because the argument is required.
    println!("The .PNG file is: {}", matches.value_of("input").unwrap());
}

fn is_png(val: String) -> Result<(), String> {
    // val is the argument value passed in by the user
    // val has type of String.
    if val.ends_with(".png") {
        Ok(())
    } else {
        // clap automatically adds "error: " to the beginning
        // of the message.
        Err(String::from("the file format must be png."))
    }
    // Of course, you can do more complicated validation as
    // well, but for the simplicity, this example only checks
    // if the value passed in ends with ".png" or not.
}
