use clap::{App, Arg};
use clap_complete::generate;
use clap_complete_fig::Fig;
use std::io;

#[test]
fn generate_completions() {
    let mut app = App::new("test_app")
        .arg(Arg::new("config").short('c').global(true))
        .arg(Arg::new("v").short('v').conflicts_with("config"))
        .subcommand(
            App::new("test")
                .about("Subcommand")
                .arg(Arg::new("debug").short('d')),
        );

    generate(Fig, &mut app, "test_app", &mut io::sink());
}
