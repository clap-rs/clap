//! How to use value hints and generate shell completions.
//!
//! Usage with zsh:
//! ```sh
//! cargo run --example value_hints_derive -- --generate=zsh > /usr/local/share/zsh/site-functions/_value_hints_derive
//! compinit
//! ./target/debug/examples/value_hints_derive --<TAB>
//! ```
//! fish:
//! ```sh
//! cargo run --example value_hints_derive -- --generate=fish > value_hints_derive.fish
//! . ./value_hints_derive.fish
//! ./target/debug/examples/value_hints_derive --<TAB>
//! ```
use clap::{App, AppSettings, Clap, IntoApp, ValueHint};
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_generate::{generate, Generator};
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

#[derive(Clap, Debug, PartialEq)]
enum GeneratorChoice {
    Bash,
    Elvish,
    Fish,
    #[clap(name = "powershell")]
    PowerShell,
    Zsh,
}

#[derive(Clap, Debug, PartialEq)]
#[clap(
    name = "value_hints_derive",
    // AppSettings::TrailingVarArg is required to use ValueHint::CommandWithArguments
    setting = AppSettings::TrailingVarArg,
    setting = AppSettings::DisableVersionFlag
)]
struct Opt {
    /// If provided, outputs the completion file for given shell
    #[clap(long = "generate", arg_enum)]
    generator: Option<GeneratorChoice>,
    // Showcasing all possible ValueHints:
    #[clap(long, value_hint = ValueHint::Unknown)]
    unknown: Option<String>,
    #[clap(long, value_hint = ValueHint::Other)]
    other: Option<String>,
    #[clap(short, long, value_hint = ValueHint::AnyPath)]
    path: Option<PathBuf>,
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    dir: Option<PathBuf>,
    #[clap(short, long, value_hint = ValueHint::ExecutablePath)]
    exe: Option<PathBuf>,
    #[clap(long, value_hint = ValueHint::CommandName)]
    cmd_name: Option<OsString>,
    #[clap(short, long, value_hint = ValueHint::CommandString)]
    cmd: Option<String>,
    #[clap(value_hint = ValueHint::CommandWithArguments)]
    command_with_args: Vec<String>,
    #[clap(short, long, value_hint = ValueHint::Username)]
    user: Option<String>,
    #[clap(short, long, value_hint = ValueHint::Hostname)]
    host: Option<String>,
    #[clap(long, value_hint = ValueHint::Url)]
    url: Option<String>,
    #[clap(long, value_hint = ValueHint::EmailAddress)]
    email: Option<String>,
}

fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let opt = Opt::parse();

    if let Some(generator) = opt.generator {
        let mut app = Opt::into_app();
        eprintln!("Generating completion file for {:?}...", generator);
        match generator {
            GeneratorChoice::Bash => print_completions::<Bash>(&mut app),
            GeneratorChoice::Elvish => print_completions::<Elvish>(&mut app),
            GeneratorChoice::Fish => print_completions::<Fish>(&mut app),
            GeneratorChoice::PowerShell => print_completions::<PowerShell>(&mut app),
            GeneratorChoice::Zsh => print_completions::<Zsh>(&mut app),
        }
    } else {
        println!("{:#?}", opt);
    }
}
