//! How to use `arg_enum!` with `StructOpt`.
// TODO: make it work
fn main() {}
// use clap::Clap;

// arg_enum! {
//     #[derive(Debug)]
//     enum Baz {
//         Foo,
//         Bar,
//         FooBar
//     }
// }

// #[derive(Clap, Debug)]
// struct Opt {
//     /// Important argument.
//     #[clap(possible_values = &Baz::variants(), case_insensitive = true)]
//     i: Baz,
// }

// fn main() {
//     let opt = Opt::parse();
//     println!("{:?}", opt);
// }
