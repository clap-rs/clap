#[macro_use]
extern crate structopt;

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
// https://docs.rs/clap/2/clap/enum.AppSettings.html#variant.InferSubcommands
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
enum Opt {
    // https://docs.rs/clap/2/clap/struct.App.html#method.alias
    #[structopt(name = "foo", alias = "foobar")]
    Foo,
    // https://docs.rs/clap/2/clap/struct.App.html#method.aliases
    #[structopt(name = "bar", raw(aliases = r#"&["baz", "fizz"]"#))]
    Bar,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
