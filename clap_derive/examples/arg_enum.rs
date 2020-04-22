use clap::Clap;

#[derive(Clap, Debug, PartialEq)]
enum ArgChoice {
    Foo,
    Bar,
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
