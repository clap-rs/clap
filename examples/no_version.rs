#[macro_use]
extern crate structopt;

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "no_version",
    about = "",
    version = "",
    author = "",
    raw(global_settings = "&[AppSettings::DisableVersion]")
)]
struct Opt {}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
