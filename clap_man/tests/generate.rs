use clap::{arg, App};
use clap_man::generate_manpage;
use std::io;

#[test]
fn render_manpage() {
    let mut app = App::new("myapp")
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

    generate_manpage(&mut app, &mut io::sink());
}
