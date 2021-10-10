//! How to extract subcommands' args into external structs.

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Foo {
    pub bar: Option<String>,
}

#[derive(Debug, Parser)]
pub enum Command {
    #[clap(name = "foo")]
    Foo(Foo),
}

#[derive(Debug, Parser)]
#[clap(name = "classify")]
pub struct ApplicationArguments {
    #[clap(subcommand)]
    pub command: Command,
}

fn main() {
    let opt = ApplicationArguments::parse();
    println!("{:?}", opt);
}
