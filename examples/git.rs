//! `git.rs` serves as a demonstration of how to use subcommands,
//! as well as a demonstration of adding documentation to subcommands.
//! Documentation can be added either through doc comments or
//! `help`/`about` attributes.

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "git")]
/// the stupid content tracker
enum Opt {
    /// fetch branches from remote repository
    Fetch {
        #[clap(long)]
        dry_run: bool,
        #[clap(long)]
        all: bool,
        #[clap(default_value = "origin")]
        repository: String,
    },
    #[clap(override_help = "add files to the staging area")]
    Add {
        #[clap(short)]
        interactive: bool,
        #[clap(short)]
        all: bool,
        files: Vec<String>,
    },
}

fn main() {
    let matches = Opt::parse();

    println!("{:?}", matches);
}
