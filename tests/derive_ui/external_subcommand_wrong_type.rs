use clap::Parser;
use std::ffi::CString;

#[derive(Parser, Debug)]
enum Opt {
    #[command(external_subcommand)]
    Other(Vec<CString>),
}

#[derive(Parser, Debug)]
enum Opt2 {
    #[command(external_subcommand)]
    Other(String),
}

#[derive(Parser, Debug)]
enum Opt3 {
    #[command(external_subcommand)]
    Other { a: String },
}

fn main() {
    let _ = Opt::parse();
    let _ = Opt2::parse();
    let _ = Opt3::parse();
}
