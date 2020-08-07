//! Example to test arguments with different ValueHint values.
//!
//! Usage with zsh:
//! ```sh
//! cargo run --example value_hints -- --generate=zsh > /usr/local/share/zsh/site-functions/_value_hints
//! compinit
//! ./target/debug/examples/value_hints --<TAB>
//! ```
//! fish:
//! ```sh
//! cargo run --example value_hints -- --generate=fish > value_hints.fish
//! . ./value_hints.fish
//! ./target/debug/examples/value_hints --<TAB>
//! ```
use clap::{App, AppSettings, Arg, ValueHint};
use clap_generate::generators::{Elvish, Fish, PowerShell, Zsh};
use clap_generate::{generate, generators::Bash};
use std::io;

const APPNAME: &str = "value_hints";

fn build_cli() -> App<'static> {
    App::new(APPNAME)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::new("generator").long("generate").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
        .arg(
            Arg::new("unknown")
                .long("unknown")
                .value_hint(ValueHint::Unknown),
        )
        .arg(Arg::new("other").long("other").value_hint(ValueHint::Other))
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .value_hint(ValueHint::DirPath),
        )
        .arg(
            Arg::new("exe")
                .long("exe")
                .short('e')
                .value_hint(ValueHint::ExecutablePath),
        )
        .arg(
            Arg::new("cmd_name")
                .long("cmd-name")
                .value_hint(ValueHint::CommandName),
        )
        .arg(
            Arg::new("cmd")
                .long("cmd")
                .short('c')
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("command_with_args")
                .multiple_values(true)
                .value_hint(ValueHint::CommandWithArguments),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_hint(ValueHint::Username),
        )
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .value_hint(ValueHint::Hostname),
        )
        .arg(Arg::new("url").long("url").value_hint(ValueHint::Url))
        .arg(
            Arg::new("email")
                .long("email")
                .value_hint(ValueHint::EmailAddress),
        )
}

fn main() {
    let matches = build_cli().get_matches();

    if let Some(generator) = matches.value_of("generator") {
        let mut app = build_cli();
        eprintln!("Generating completion file for {}...", generator);
        match generator {
            "bash" => generate::<Bash, _>(&mut app, APPNAME, &mut io::stdout()),
            "elvish" => generate::<Elvish, _>(&mut app, APPNAME, &mut io::stdout()),
            "fish" => generate::<Fish, _>(&mut app, APPNAME, &mut io::stdout()),
            "powershell" => generate::<PowerShell, _>(&mut app, APPNAME, &mut io::stdout()),
            "zsh" => generate::<Zsh, _>(&mut app, APPNAME, &mut io::stdout()),
            _ => panic!("Unknown generator"),
        }
    }
}
