use std::io;

use clap::{Arg, Command};

use clap_complete::{generate, shells::*};

#[test]
fn generate_completions() {
    let mut cmd = Command::new("test_app")
        .arg(Arg::new("config").short('c').global(true))
        .arg(Arg::new("v").short('v').conflicts_with("config"))
        .subcommand(
            Command::new("test")
                .about("Subcommand")
                .arg(Arg::new("debug").short('d')),
        );

    generate(Bash, &mut cmd, "test_app", &mut io::sink());
    generate(Fish, &mut cmd, "test_app", &mut io::sink());
    generate(PowerShell, &mut cmd, "test_app", &mut io::sink());
    generate(Elvish, &mut cmd, "test_app", &mut io::sink());
    generate(Zsh, &mut cmd, "test_app", &mut io::sink());
}
