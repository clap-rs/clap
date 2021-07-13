//! How to use flattening.

use clap::Parser;

#[derive(Parser, Debug)]
struct Cmdline {
    /// switch verbosity on
    #[clap(short)]
    verbose: bool,

    #[clap(flatten)]
    daemon_opts: DaemonOpts,
}

#[derive(Parser, Debug)]
struct DaemonOpts {
    /// daemon user
    #[clap(short)]
    user: String,

    /// daemon group
    #[clap(short)]
    group: String,
}

fn main() {
    let opt = Cmdline::parse();
    println!("{:?}", opt);
}
