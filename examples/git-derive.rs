// Note: this requires the `derive` feature

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{AppSettings, Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "git")]
#[clap(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clones repos
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Clone {
        /// The remote to clone
        remote: String,
    },
    /// pushes things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Push {
        /// The remote to target
        remote: String,
    },
    /// adds things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Add {
        /// Stuff to add
        #[clap(required = true, parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    #[clap(external_subcommand)]
    External(Vec<OsString>),
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Clone { remote } => {
            println!("Cloning {}", remote);
        }
        Commands::Push { remote } => {
            println!("Pushing to {}", remote);
        }
        Commands::Add { path } => {
            println!("Adding {:?}", path);
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }

    // Continued program logic goes here...
}
