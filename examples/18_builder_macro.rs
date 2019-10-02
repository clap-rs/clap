#[macro_use]
extern crate clap;

// Note, there isn't a need for "use clap::{ ... };" Because the clap_app! macro uses
// $crate:: internally

fn main() {
    // Validation example testing that a file exists
    let file_exists = |path| {
        if std::fs::metadata(path).is_ok() {
            Ok(())
        } else {
            Err(String::from("File doesn't exist"))
        }
    };

    // External module may contain this subcommand. If this exists in another module, a function is
    // required to access it. Recommend `fn clap() -> Clap::`.
    let external_sub_command = clap_app!( @subcommand foo =>
        (@arg bar: -b "Bar")
    );

    let matches = clap_app!(MyApp =>
        (@setting SubcommandRequiredElseHelp)
        (version: "1.0")
        (author: "Alice")
        (about: "Does awesome things")
        (@arg config: -c --config <conf> #{1, 2} {file_exists} "Sets a custom config file")
        (@arg proxyHostname: --("proxy-hostname") +takes_value "Sets the hostname of the proxy to use")
        (@arg input: * "Input file")
        (@group test =>
            (@attributes +required)
            (@arg output: "Sets an optional output file")
            (@arg debug: -d ... "Turn debugging information on")
        )
        (subcommand: external_sub_command)
        (@subcommand test =>
            (about: "does testing things")
            (version: "2.5")
            (@arg list: -l "Lists test values")
            (@arg test_req: -r requires[list] "Tests requirement for listing")
            (@arg aaaa: --aaaa +takes_value {
                    |a| if a.contains("a") {
                        Ok(())
                    } else {
                        Err(String::from("string does not contain at least one a"))
                    }
                } "Test if the argument contains an a")
        )
    ).get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    // Continued program logic goes here...
}
