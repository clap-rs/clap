//! `git.rs` serves as a demonstration of how to use subcommands,
//! as well as a demonstration of adding documentation to subcommands.
//! Documentation can be added either through doc comments or the
//! `about` attribute.

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "git")]
/// the stupid content tracker
enum Opt {
    #[structopt(name = "fetch")]
    /// fetch branches from remote repository
    Fetch {
        #[structopt(long = "dry-run")]
        dry_run: bool,
        #[structopt(long = "all")]
        all: bool,
        #[structopt(default_value = "origin")]
        repository: String,
    },
    #[structopt(name = "add")]
    /// add files to the staging area
    Add {
        #[structopt(short = "i")]
        interactive: bool,
        #[structopt(short = "a")]
        all: bool,
        files: Vec<String>,
    },
}

fn main() {
    let matches = Opt::from_args();

    println!("{:?}", matches);
}
