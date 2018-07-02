#[macro_use]
extern crate clap;

use clap::Clap;

#[derive(Debug, Clap)]
pub struct Foo {
    pub bar: Option<String>,
}

#[derive(Debug, Clap)]
pub enum Command {
    #[clap(name = "foo")]
    Foo(Foo),
}

#[derive(Debug, Clap)]
#[clap(name = "classify")]
pub struct ApplicationArguments {
    #[clap(subcommand)]
    pub command: Command,
}

fn main() {
    let opt = ApplicationArguments::parse();
    println!("{:?}", opt);
}
