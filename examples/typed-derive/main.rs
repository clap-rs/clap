use clap::Parser;

mod builtin;
mod custom;
mod fn_parser;
mod foreign_crate;
mod implicit;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(term_width = 0)] // Just to make testing across clap features easier
enum Cli {
    Implicit(implicit::ImplicitParsers),
    Builtin(builtin::BuiltInParsers),
    FnParser(fn_parser::FnParser),
    Custom(custom::CustomParser),
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
}
