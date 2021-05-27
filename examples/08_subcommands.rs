use clap::{App, Arg};

fn main() {
    // s function exactly like sub-Apps, because that's exactly what they are. Each
    // instance of a can have its own version, author(s), Args, and even its own
    // subcommands.
    //
    // # Help and Version
    // Just like Apps, each subcommand will get its own "help" and "version" flags automatically
    // generated. Also, like Apps, you can override "-V" or "-h" safely and still get "--help" and
    // "--version" auto generated.
    //
    // NOTE: If you specify a subcommand for your App, clap will also autogenerate a "help"
    // subcommand along with "-h" and "--help" (applies to sub-subcommands as well).
    //
    // Just like arg() and args(), subcommands can be specified one at a time via subcommand() or
    // multiple ones at once with a Vec<> provided to subcommands().
    let app = App::new("MyApp")
        // Normal App and Arg configuration goes here...
        // In the following example assume we wanted an application which
        // supported an "add" subcommand, this "add" subcommand also took
        // one positional argument of a file to add:
        .subcommand(
            App::new("add") // The name we call argument with
                .about("Adds files to myapp") // The message displayed in "myapp -h"
                // or "myapp help"
                .license("MIT OR Apache-2.0")
                .version("0.1") // Subcommands can have independent version
                .author("Kevin K.") // And authors
                .arg(
                    Arg::new("input") // And their own arguments
                        .about("the file to add")
                        .index(1)
                        .required(true),
                ),
        );

    let app = if true {
      let about_text = "i-am-an-about-text".to_string() + "-for-foobar";
      app.subcommand(
        App::new("foobar")
          .about(about_text)
      )
    } else {
      app
    };

    let matches = app.get_matches();

    // You can check if a subcommand was used like normal
    if matches.is_present("add") {
        println!("'myapp add' was run.");
    }

    // You can get the independent subcommand matches (which function exactly like App matches)
    if let Some(ref matches) = matches.subcommand_matches("add") {
        // Safe to use unwrap() because of the required() option
        println!("Adding file: {}", matches.value_of("input").unwrap());
    }

    // You can also match on a subcommand's name
    match matches.subcommand_name() {
        Some("add") => println!("'myapp add' was used"),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    // Continued program logic goes here...
}
