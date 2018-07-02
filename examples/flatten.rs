#[macro_use]
extern crate clap;

use clap::Clap;

#[derive(Clap, Debug)]
struct Cmdline {
    #[clap(short = "v", help = "switch on verbosity")]
    verbose: bool,
    #[clap(flatten)]
    daemon_opts: DaemonOpts,
}

#[derive(Clap, Debug)]
struct DaemonOpts {
    #[clap(short = "u", help = "daemon user")]
    user: String,
    #[clap(short = "g", help = "daemon group")]
    group: String,
}

fn main() {
    let opt = Cmdline::parse();
    println!("{:?}", opt);
}
