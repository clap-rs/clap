//! `git.rs` serves as a demonstration of how to use subcommands,
//! as well as a demonstration of adding documentation to subcommands.
//! Documentation can be added either through doc comments or the
//! `about` attribute.

#[macro_use]
extern crate clap;

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "git")]
/// the stupid content tracker
enum Opt {
    #[clap(name = "fetch")]
    /// fetch branches from remote repository
    Fetch {
        #[clap(long = "dry-run")]
        dry_run: bool,
        #[clap(long = "all")]
        all: bool,
        #[clap(default_value = "origin")]
        repository: String,
    },
    #[clap(name = "add")]
    /// add files to the staging area
    Add {
        #[clap(short = "i")]
        interactive: bool,
        #[clap(short = "a")]
        all: bool,
        files: Vec<String>,
    },
}

fn main() {
    let matches = Opt::parse();

    println!("{:?}", matches);
}
