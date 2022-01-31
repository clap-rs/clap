use clap::{arg, App, Arg};
use clap_man::Man;
use std::io;

#[test]
fn render_manpage() {
    let app = App::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .long_about("With a longer description to help clarify some things.")
        .after_help("This is an extra section added to the end of the manpage.")
        .after_long_help("With even more text added.")
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                .long_help("Some more text about how to set a custom config file")
                .required(false)
                .takes_value(true),
        )
        .arg(arg!([output] "Sets an optional output file").index(1))
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "Lists test values")),
        );

    Man::new(app).render(&mut io::sink()).unwrap();
}

#[test]
fn verify_argument_ordering_is_preserved() {
    let app = App::new("prog")
        .arg(
            Arg::new("a") // Typically args are grouped alphabetically by name.
                // Args without a display_order have a value of 999 and are
                // displayed alphabetically with all other 999 valued args.
                .long("last")
                .short('o')
                .takes_value(true)
                .help("Some help and text: Should be last"),
        )
        .arg(
            Arg::new("e")
                .long("fourth")
                .short('R')
                .takes_value(true)
                .display_order(4)
                .help("I should be fourth!"),
        )
        .arg(
            Arg::new("d")
                .long("third")
                .short('Q')
                .takes_value(true)
                .display_order(3)
                .help("I should be third!"),
        )
        .arg(
            Arg::new("c")
                .long("second")
                .short('P')
                .takes_value(true)
                .display_order(2)
                .help("I should be second!"),
        )
        .arg(
            Arg::new("b")
                .long("first")
                .short('O')
                .takes_value(true)
                .display_order(1) // In order to force this arg to appear *first*
                // all we have to do is give it a value lower than 999.
                // Any other args with a value of 1 will be displayed
                // alphabetically with this one...then 2 values, then 3, etc.
                .help("I should be first!"),
        );

    let mut buf = Vec::new();
    Man::new(app).render(&mut buf).unwrap();
    let s = match std::str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // Split the string at the `DESCRIPTION` heading to access the
    // SYNOPSIS and OPTIONS sections separately
    let split = s.split("DESCRIPTION");
    let keywords = ["first", "second", "third", "fourth", "last"];
    // For the synopsis, and the full output, iterate and confirm that
    // the keywords appear in the correct order
    for mut s in split {
        println!("{}", s);
        for keyword in keywords {
            assert!(s.contains(keyword));
            s = s.split(keyword).last().unwrap();
        }
    }
}
