#[macro_use]
extern crate clap;

use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
// https://docs.rs/clap/2/clap/enum.AppSettings.html#variant.InferSubcommands
#[clap(raw(setting = "AppSettings::InferSubcommands"))]
enum Opt {
    // https://docs.rs/clap/2/clap/struct.App.html#method.alias
    #[clap(name = "foo", alias = "foobar")]
    Foo,
    // https://docs.rs/clap/2/clap/struct.App.html#method.aliases
    #[clap(name = "bar", raw(aliases = r#"&["baz", "fizz"]"#))]
    Bar,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
