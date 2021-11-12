use clap::{App, Arg};

fn main() {
    let matches = App::new("MyApp")
        .subcommand(
            App::new("ls")
                .aliases(&["list", "dir"])
                .about("Adds files to myapp")
                .version("0.1")
                .author("Kevin K.")
                .arg(
                    Arg::new("input")
                        .about("the file to add")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    // You can get the independent subcommand matches (which function exactly like App matches)
    if let Some(matches) = matches.subcommand_matches("ls") {
        // Safe to use unwrap() because of the required() option
        println!("Adding file: {}", matches.value_of("input").unwrap());
    }

    // You can also match on a subcommand's name
    match matches.subcommand_name() {
        Some("ls") => println!("'myapp add' was used"),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    // Continued program logic goes here...
}
