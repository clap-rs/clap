//! Usage example of `arg_enum`
//!
//! All the variants of the enum and the enum itself support `rename_all`

use clap::Clap;

#[derive(Clap, Debug, PartialEq)]
enum ArgChoice {
    Foo,
    Bar,
    // Aliases are supported
    #[clap(alias = "b", alias = "z")]
    Baz,
}

#[derive(Clap, PartialEq, Debug)]
struct Opt {
    #[clap(arg_enum)]
    arg: ArgChoice,
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
}
