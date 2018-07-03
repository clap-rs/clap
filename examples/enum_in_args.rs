#[macro_use]
extern crate clap;

use clap::Clap;

arg_enum! {
    #[derive(Debug)]
    enum Baz {
        Foo,
        Bar,
        FooBar
    }
}

#[derive(Clap, Debug)]
struct Opt {
    /// Important argument.
    #[clap(raw(possible_values = "&Baz::variants()", case_insensitive = "true"))]
    i: Baz,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
