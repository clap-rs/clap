#![allow(deprecated)]

use clap::{Arg, Command};
use clap_generate::{generate, generators::*};
use std::io;

#[test]
fn generate_completions() {
    let mut app = Command::new("test_app")
        .arg(Arg::new("config").short('c').global(true))
        .arg(Arg::new("v").short('v').conflicts_with("config"))
        .subcommand(
            Command::new("test")
                .about("Subcommand")
                .arg(Arg::new("debug").short('d')),
        );

    generate(Bash, &mut app, "test_app", &mut io::sink());
    generate(Fish, &mut app, "test_app", &mut io::sink());
    generate(PowerShell, &mut app, "test_app", &mut io::sink());
    generate(Elvish, &mut app, "test_app", &mut io::sink());
    generate(Zsh, &mut app, "test_app", &mut io::sink());
}
