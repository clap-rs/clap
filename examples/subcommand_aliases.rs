//! How to assign some aliases to subcommands

use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
// https://docs.rs/clap/2/clap/enum.AppSettings.html#variant.InferSubcommands
#[clap(setting = AppSettings::InferSubcommands)]
enum Opt {
    // https://docs.rs/clap/2/clap/struct.App.html#method.alias
    #[clap(alias = "foobar")]
    Foo,
    // https://docs.rs/clap/2/clap/struct.App.html#method.aliases
    #[clap(aliases = &["baz", "fizz"])]
    Bar,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
