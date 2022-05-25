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
use clap::{Command, CommandFactory, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug, PartialEq)]
#[clap(
    name = "value_hints_derive",
    // Command::trailing_var_ar is required to use ValueHint::CommandWithArguments
    trailing_var_arg = true,
)]
struct Opt {
    /// If provided, outputs the completion file for given shell
    #[clap(long = "generate", arg_enum, value_parser)]
    generator: Option<Shell>,
    // Showcasing all possible ValueHints:
    #[clap(long, value_hint = ValueHint::Unknown, value_parser)]
    unknown: Option<String>,
    #[clap(long, value_hint = ValueHint::Other, value_parser)]
    other: Option<String>,
    #[clap(short, long, value_hint = ValueHint::AnyPath, value_parser)]
    path: Option<PathBuf>,
    #[clap(short, long, value_hint = ValueHint::FilePath, value_parser)]
    file: Option<PathBuf>,
    #[clap(short, long, value_hint = ValueHint::DirPath, value_parser)]
    dir: Option<PathBuf>,
    #[clap(short, long, value_hint = ValueHint::ExecutablePath, value_parser)]
    exe: Option<PathBuf>,
    #[clap(long, value_hint = ValueHint::CommandName, value_parser)]
    cmd_name: Option<OsString>,
    #[clap(short, long, value_hint = ValueHint::CommandString, value_parser)]
    cmd: Option<String>,
    #[clap(value_hint = ValueHint::CommandWithArguments, value_parser)]
    command_with_args: Vec<String>,
    #[clap(short, long, value_hint = ValueHint::Username, value_parser)]
    user: Option<String>,
    #[clap(short, long, value_hint = ValueHint::Hostname, value_parser)]
    host: Option<String>,
    #[clap(long, value_hint = ValueHint::Url, value_parser)]
    url: Option<String>,
    #[clap(long, value_hint = ValueHint::EmailAddress, value_parser)]
    email: Option<String>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let opt = Opt::parse();

    if let Some(generator) = opt.generator {
        let mut cmd = Opt::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
    } else {
        println!("{:#?}", opt);
    }
}
