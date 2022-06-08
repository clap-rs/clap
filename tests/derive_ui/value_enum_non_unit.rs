use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
enum Opt {
    Foo(usize),
}

fn main() {
    println!("{:?}", Opt::Foo(42));
}
