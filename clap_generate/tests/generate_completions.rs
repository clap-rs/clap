use clap::{App, Arg};
use clap_generate::{generate, generators::*};
use std::io;

#[test]
fn generate_completions() {
    let mut app = App::new("test_app")
        .arg(
            Arg::new("config")
                .short('c')
                .conflicts_with("v")
                .global(true),
        )
        .arg(Arg::new("v").short('v'))
        .subcommand(
            App::new("test")
                .about("Subcommand")
                .arg(Arg::new("debug").short('d')),
        );

    generate::<Bash, _>(&mut app, "test_app", &mut io::sink());
    generate::<Fish, _>(&mut app, "test_app", &mut io::sink());
    generate::<PowerShell, _>(&mut app, "test_app", &mut io::sink());
    generate::<Elvish, _>(&mut app, "test_app", &mut io::sink());
    generate::<Zsh, _>(&mut app, "test_app", &mut io::sink());
}
