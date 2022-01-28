use clap::{arg, App};
use clap_man::{Man, Meta};

// Run this example as `cargo run --example man-builder | man -l -`.

fn main() -> Result<(), std::io::Error> {
    let app = App::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .subcommand_help_heading("Commands")
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
        )
        .after_help(
            "\
For more information about the config file syntax, look up the INI format.
To see the debug information, visit our website on github.com",
        );

    let meta = Meta::from_clap("1", "GNU", &app);
    let man = Man::new(meta, app);
    man.render(&mut std::io::stdout())
}
