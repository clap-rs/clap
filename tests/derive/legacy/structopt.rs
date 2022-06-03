#![allow(deprecated)]

use clap::{AppSettings, StructOpt};

#[test]
fn compatible() {
    #[derive(StructOpt)]
    #[structopt(author, version, about)]
    #[structopt(global_setting(AppSettings::PropagateVersion))]
    struct Cli {
        #[structopt(subcommand)]
        command: Commands,
    }

    #[derive(StructOpt)]
    #[structopt(setting(AppSettings::SubcommandRequiredElseHelp))]
    enum Commands {
        /// Adds files to myapp
        Add { name: Option<String> },
    }

    Cli::from_iter(["test", "add"]);
}
