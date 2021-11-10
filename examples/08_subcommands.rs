use clap::{App, Arg};

fn main() {
    // Just like arg() and args(), subcommands can be specified one at a time via subcommand() or
    // multiple ones at once with a Vec<App> provided to subcommands().
    let matches = App::new("MyApp")
        .version("1.0")
        // Normal App and Arg configuration goes here...
        // In the following example assume we wanted an application which
        // supported an "add" subcommand, this "add" subcommand also took
        // one positional argument of a file to add:
        .subcommand(
            App::new("add") // The name we call argument with
                .about("Adds files to myapp") // The message displayed in "myapp -h"
                // or "myapp help"
                .version("0.1") // Subcommands can have independent version
                .author("Kevin K.") // And authors
                .arg(
                    Arg::new("input") // And their own arguments
                        .help("the file to add")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    // Most commonly, you'll get the name and matches at the same time
    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, input is: {}",
            sub_matches
                .value_of("input")
                .expect("'input' is required and parsing will fail if its missing")
        ),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    // Continued program logic goes here...
}
