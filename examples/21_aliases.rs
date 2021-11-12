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

    // You can also match on a subcommand's name
    match matches.subcommand() {
        Some(("ls", sub_matches)) => println!(
            "'myapp add' was used, input is: {}",
            // Safe to use unwrap() because of the required() option
            sub_matches.value_of("input").unwrap()
        ),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    // Continued program logic goes here...
}
